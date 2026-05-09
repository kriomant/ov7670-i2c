#![no_std]

//! OV7670 Camera Module Driver
//!
//! A `no_std`, asynchronous driver for the OV7670 camera control registers,
//! using `embedded-hal-async` for I²C and typed bitfields for register values.
//!
//! # Basic usage
//!
//! ```no_run
//! use embedded_hal_async::i2c::I2c;
//! use ov7670_i2c::OV7670;
//!
//! async fn identify<I2C: I2c>(i2c: I2C) -> Result<OV7670<I2C>, I2C::Error> {
//!     let mut camera = OV7670::new(i2c);
//!
//!     let manufacturer_id = camera.read_manufacturer_id().await?;
//!     let product_id = camera.read_product_id().await?;
//!     // Inspect or log these IDs using your application's facilities.
//!     let _ = (manufacturer_id, product_id);
//!
//!     Ok(camera)
//! }
//! ```
//!
//! `OV7670::new` uses the default 7-bit address `0x21`. Use
//! `OV7670::new_with_address(i2c, address)` to supply a different address. Bus errors
//! are returned directly as `I2C::Error`.
//!
//! `camera.reset().await?` requests a software reset. The caller must wait at least
//! 1 ms before the next register access; the driver does not insert this delay.
//!
//! # Register access
//!
//! Register types, builders, enums, and the `addr` module are exported at the crate
//! root. Typed accessors follow the `read_*`, `write_*`, and `modify_*` naming
//! convention. Read-only ID registers expose typed read methods only.
//!
//! ```no_run
//! use embedded_hal_async::i2c::I2c;
//! use ov7670_i2c::{addr, ClkrcBuilder, OV7670};
//!
//! async fn configure<I2C: I2c>(camera: &mut OV7670<I2C>) -> Result<(), I2C::Error> {
//!     // Read, change one field, and write back the register.
//!     camera.modify_com7(|reg| reg.set_color_bar(true)).await?;
//!
//!     // Build and write an entire register value.
//!     let clock = ClkrcBuilder::new().with_divider(1).build();
//!     camera.write_clkrc(clock).await?;
//!
//!     // Access raw register bytes when needed.
//!     let raw = camera.read_register(addr::CLKRC).await?;
//!     camera.write_register(addr::CLKRC, raw).await?;
//!
//!     Ok(())
//! }
//! ```
//!
//! Use `modify_*` to retain the other fields of an existing register. Builders
//! construct whole register values; their defaults are not a sensor initialization
//! sequence. Bitfield types also provide `from_bits` and `into_bits` conversions.
//!
//! Register reads use separate I²C write and read operations rather than
//! `write_read`. No extra delay is inserted between them; the implementation notes
//! that some hardware may require one.

use embedded_hal_async::i2c::I2c;

mod registers;
pub mod combined;
pub use registers::*;

/// Default I2C address for OV7670
pub const DEFAULT_ADDRESS: u8 = 0x21;

/// OV7670 camera module driver
pub struct OV7670<I2C> {
    i2c: I2C,
    address: u8,
}

/// Macro to generate read, write, and modify methods for a register
macro_rules! register_accessors {
    (
        $(
            $reg_name:ident => {
                register: $reg_enum:ident,
                type: $reg_type:ident,
                read_only: $read_only:expr
            }
        ),* $(,)?
    ) => {
        $(
            paste::paste! {
                #[doc = "Read " $reg_enum " register"]
                pub async fn [<read_ $reg_name>](&mut self) -> Result<$reg_type, I2C::Error> {
                    let value = self.read_register(addr::$reg_enum).await?;
                    Ok($reg_type::from_bits(value))
                }

                register_accessors!(@write $reg_name, $reg_enum, $reg_type, $read_only);
                register_accessors!(@modify $reg_name, $reg_enum, $reg_type, $read_only);
            }
        )*
    };

    // Write method - only generated if not read-only
    (@write $reg_name:ident, $reg_enum:ident, $reg_type:ident, false) => {
        paste::paste! {
            #[doc = "Write " $reg_enum " register"]
            pub async fn [<write_ $reg_name>](&mut self, value: $reg_type) -> Result<(), I2C::Error> {
                self.write_register(addr::$reg_enum, value.into_bits()).await
            }
        }
    };
    (@write $reg_name:ident, $reg_enum:ident, $reg_type:ident, true) => {};

    // Modify method - only generated if not read-only
    (@modify $reg_name:ident, $reg_enum:ident, $reg_type:ident, false) => {
        paste::paste! {
            #[doc = "Modify " $reg_enum " register\n\n"]
            #[doc = "Reads the current register value, applies the modification function, and writes it back.\n\n"]
            #[doc = "# Arguments\n"]
            #[doc = "* `f` - Closure that takes a mutable reference to the register and modifies it"]
            pub async fn [<modify_ $reg_name>]<F>(&mut self, f: F) -> Result<(), I2C::Error>
            where
                F: FnOnce(&mut $reg_type),
            {
                let mut reg = self.[<read_ $reg_name>]().await?;
                f(&mut reg);
                self.[<write_ $reg_name>](reg).await
            }
        }
    };
    (@modify $reg_name:ident, $reg_enum:ident, $reg_type:ident, true) => {};
}

impl<I2C: I2c> OV7670<I2C> {
    /// Create a new OV7670 driver instance with default I2C address (0x21)
    ///
    /// # Arguments
    /// * `i2c` - I2C peripheral instance
    pub fn new(i2c: I2C) -> Self {
        Self::new_with_address(i2c, DEFAULT_ADDRESS)
    }

    /// Create a new OV7670 driver instance with a custom I2C address
    ///
    /// # Arguments
    /// * `i2c` - I2C peripheral instance
    /// * `address` - I2C address of the OV7670
    pub fn new_with_address(i2c: I2C, address: u8) -> Self {
        Self {
            i2c,
            address,
        }
    }

    /// Read a register from the OV7670
    ///
    /// # Arguments
    /// * `register` - Register address to read from
    ///
    /// Returns register value or I2C error
    pub async fn read_register(&mut self, register: u8) -> Result<u8, I2C::Error> {
        let mut buffer = [0u8; 1];

        // NOTE: `write_read` doesn't work!
        // It seems some delay is required between write and consequtive read.
        // No artificial delay is needed in my case, but it may be needed in general.

        // Write register address
        self.i2c.write(self.address, &[register]).await?;

        // Read register value
        self.i2c.read(self.address, &mut buffer).await?;

        Ok(buffer[0])
    }

    /// Write a register to the OV7670
    ///
    /// # Arguments
    /// * `register` - Register address to write to
    /// * `value` - Value to write
    ///
    /// Returns Ok(()) or I2C error
    pub async fn write_register(&mut self, register: u8, value: u8) -> Result<(), I2C::Error> {
        self.i2c.write(self.address, &[register, value]).await
    }

    /// Read the manufacturer ID from the OV7670
    ///
    /// Returns a 16-bit manufacturer ID. For OmniVision sensors, this is typically 0x7FA2.
    pub async fn read_manufacturer_id(&mut self) -> Result<u16, I2C::Error> {
        let midh = self.read_register(addr::MIDH).await?;
        let midl = self.read_register(addr::MIDL).await?;

        Ok((midh as u16) << 8 | midl as u16)
    }

    /// Read the product ID from the OV7670
    ///
    /// Returns a 16-bit product ID. For OV7670, this should be 0x7673.
    pub async fn read_product_id(&mut self) -> Result<u16, I2C::Error> {
        let pid = self.read_register(addr::PID).await?;
        let ver = self.read_register(addr::VER).await?;

        Ok((pid as u16) << 8 | ver as u16)
    }

    /// Reset the camera to default settings
    pub async fn reset(&mut self) -> Result<(), I2C::Error> {
        let com7 = Com7Builder::new()
            .with_reset(true)
            .build();
        self.write_register(addr::COM7, com7.into_bits()).await
    }

    /// Read the combined gain + vertical window setting (10-bit gain, VSTART, VSTOP)
    pub async fn read_gain_v_window(&mut self) -> Result<combined::GainVStartVStopVRef, I2C::Error> {
        let gain = self.read_gain().await?;
        let vstart = self.read_vstart().await?;
        let vstop = self.read_vstop().await?;
        let vref = self.read_vref().await?;
        Ok(combined::GainVStartVStopVRef::combine(gain, vstart, vstop, vref))
    }

    /// Write the combined gain + vertical window setting
    pub async fn write_gain_v_window(&mut self, setting: combined::GainVStartVStopVRef) -> Result<(), I2C::Error> {
        let (gain_reg, vstart, vstop, vref): (Gain, VStart, VStop, VRef) = setting.split();
        self.write_register(addr::GAIN, gain_reg.into_bits()).await?;
        self.write_register(addr::VSTART, vstart.into_bits()).await?;
        self.write_register(addr::VSTOP, vstop.into_bits()).await?;
        self.write_register(addr::VREF, vref.into_bits()).await
    }

    /// Modify the combined gain + vertical window setting
    pub async fn modify_gain_v_window<F>(&mut self, f: F) -> Result<(), I2C::Error>
    where
        F: FnOnce(&mut combined::GainVStartVStopVRef),
    {
        let mut setting = self.read_gain_v_window().await?;
        f(&mut setting);
        self.write_gain_v_window(setting).await
    }

    /// Read the combined AEC (auto exposure) setting (16-bit value)
    pub async fn read_aec_setting(&mut self) -> Result<combined::AecCombined, I2C::Error> {
        let com1 = self.read_com1().await?;
        let aech = self.read_aech().await?;
        let aechh = self.read_aechh().await?;
        Ok(combined::AecCombined::combine(com1, aech, aechh))
    }

    /// Write the combined AEC (auto exposure) setting (16-bit value)
    pub async fn write_aec_setting(&mut self, setting: combined::AecCombined) -> Result<(), I2C::Error> {
        let (com1, aech, aechh): (Com1, Aech, Aechh) = setting.split();
        self.write_register(addr::COM1, com1.into_bits()).await?;
        self.write_register(addr::AECH, aech.into_bits()).await?;
        self.write_register(addr::AECHH, aechh.into_bits()).await
    }

    /// Modify the combined AEC setting
    pub async fn modify_aec_setting<F>(&mut self, f: F) -> Result<(), I2C::Error>
    where
        F: FnOnce(&mut combined::AecCombined),
    {
        let mut aec = self.read_aec_setting().await?;
        f(&mut aec);
        self.write_aec_setting(aec).await
    }

    /// Read the combined horizontal window setting (HSTART and HSTOP)
    pub async fn read_h_window(&mut self) -> Result<combined::HStartStopHref, I2C::Error> {
        let hstart = self.read_hstart().await?;
        let hstop = self.read_hstop().await?;
        let href = self.read_href().await?;
        Ok(combined::HStartStopHref::combine(hstart, hstop, href))
    }

    /// Write the combined horizontal window setting (HSTART and HSTOP)
    pub async fn write_h_window(&mut self, setting: combined::HStartStopHref) -> Result<(), I2C::Error> {
        let (hstart, hstop, href): (Hstart, Hstop, HRef) = setting.split();
        self.write_register(addr::HSTART, hstart.into_bits()).await?;
        self.write_register(addr::HSTOP, hstop.into_bits()).await?;
        self.write_register(addr::HREF, href.into_bits()).await
    }

    /// Modify the combined horizontal window setting
    pub async fn modify_h_window<F>(&mut self, f: F) -> Result<(), I2C::Error>
    where
        F: FnOnce(&mut combined::HStartStopHref),
    {
        let mut window = self.read_h_window().await?;
        f(&mut window);
        self.write_h_window(window).await
    }

    /// Read the combined HSYNC delay setting
    pub async fn read_hsync_delay(&mut self) -> Result<combined::HsyncDelayCombined, I2C::Error> {
        let exhch = self.read_exhch().await?;
        let hsyst = self.read_hsyst().await?;
        let hsyen = self.read_hsyen().await?;
        Ok(combined::HsyncDelayCombined::combine(exhch, hsyst, hsyen))
    }

    /// Write the combined HSYNC delay setting
    pub async fn write_hsync_delay(&mut self, setting: combined::HsyncDelayCombined) -> Result<(), I2C::Error> {
        let (exhch, hsyst, hsyen): (ExhcH, HSySt, HSyEn) = setting.split();
        self.write_register(addr::EXHCH, exhch.into_bits()).await?;
        self.write_register(addr::HSYST, hsyst.into_bits()).await?;
        self.write_register(addr::HSYEN, hsyen.into_bits()).await
    }

    /// Read the combined scaling setting (horizontal and vertical scale factors + test pattern)
    pub async fn read_scaling(&mut self) -> Result<combined::ScalingXScYSc, I2C::Error> {
        let xsc = self.read_scaling_xsc().await?;
        let ysc = self.read_scaling_ysc().await?;
        Ok(combined::ScalingXScYSc::combine(xsc, ysc))
    }

    /// Write the combined scaling setting
    pub async fn write_scaling(&mut self, setting: combined::ScalingXScYSc) -> Result<(), I2C::Error> {
        let (xsc, ysc): (ScalingXSc, ScalingYSc) = setting.split();
        self.write_register(addr::SCALING_XSC, xsc.into_bits()).await?;
        self.write_register(addr::SCALING_YSC, ysc.into_bits()).await
    }

    /// Modify the combined scaling setting
    pub async fn modify_scaling<F>(&mut self, f: F) -> Result<(), I2C::Error>
    where
        F: FnOnce(&mut combined::ScalingXScYSc),
    {
        let mut scaling = self.read_scaling().await?;
        f(&mut scaling);
        self.write_scaling(scaling).await
    }

    /// Read the combined dummy lines setting
    pub async fn read_dummy_lines(&mut self) -> Result<combined::DummyLines, I2C::Error> {
        let advfl = self.read_advfl().await?;
        let advfh = self.read_advfh().await?;
        Ok(combined::DummyLines::combine(advfl, advfh))
    }

    /// Write the combined dummy lines setting
    pub async fn write_dummy_lines(&mut self, setting: combined::DummyLines) -> Result<(), I2C::Error> {
        let (advfl, advfh): (AdvfL, AdvfH) = setting.split();
        self.write_register(addr::ADVFL, advfl.into_bits()).await?;
        self.write_register(addr::ADVFH, advfh.into_bits()).await
    }

    /// Read the combined dummy line setting
    pub async fn read_dummy_line(&mut self) -> Result<combined::DummyLine, I2C::Error> {
        let dm_lnl = self.read_dm_lnl().await?;
        let dm_lnh = self.read_dm_lnh().await?;
        Ok(combined::DummyLine::combine(dm_lnl, dm_lnh))
    }

    /// Write the combined dummy line setting
    pub async fn write_dummy_line(&mut self, setting: combined::DummyLine) -> Result<(), I2C::Error> {
        let (dm_lnl, dm_lnh): (DmLnL, DmLnH) = setting.split();
        self.write_register(addr::DM_LNL, dm_lnl.into_bits()).await?;
        self.write_register(addr::DM_LNH, dm_lnh.into_bits()).await
    }

    /// Read the combined dummy pixels setting
    pub async fn read_dummy_pixels(&mut self) -> Result<combined::DummyPixels, I2C::Error> {
        let exhch = self.read_exhch().await?;
        let exhcl = self.read_exhcl().await?;
        Ok(combined::DummyPixels::combine(exhch, exhcl))
    }

    /// Write the combined dummy pixels setting
    pub async fn write_dummy_pixels(&mut self, setting: combined::DummyPixels) -> Result<(), I2C::Error> {
        let (exhch, exhcl): (ExhcH, ExhcL) = setting.split();
        self.write_register(addr::EXHCH, exhch.into_bits()).await?;
        self.write_register(addr::EXHCL, exhcl.into_bits()).await
    }

    // ========================================
    // Register accessor methods (generated by macro)
    // ========================================

    register_accessors! {
        // COM registers
        com7 => { register: COM7, type: Com7, read_only: false },
        com3 => { register: COM3, type: Com3, read_only: false },
        com10 => { register: COM10, type: Com10, read_only: false },
        com14 => { register: COM14, type: Com14, read_only: false },
        com15 => { register: COM15, type: Com15, read_only: false },
        com1 => { register: COM1, type: Com1, read_only: false },
        com2 => { register: COM2, type: Com2, read_only: false },
        com4 => { register: COM4, type: Com4, read_only: false },
        com5 => { register: COM5, type: Com5, read_only: false },
        com6 => { register: COM6, type: Com6, read_only: false },
        com8 => { register: COM8, type: Com8, read_only: false },
        com9 => { register: COM9, type: Com9, read_only: false },
        com11 => { register: COM11, type: Com11, read_only: false },
        com12 => { register: COM12, type: Com12, read_only: false },
        com13 => { register: COM13, type: Com13, read_only: false },
        com16 => { register: COM16, type: Com16, read_only: false },
        com17 => { register: COM17, type: Com17, read_only: false },

        // Clock and timing
        clkrc => { register: CLKRC, type: Clkrc, read_only: false },
        tslb => { register: TSLB, type: Tslb, read_only: false },
        href => { register: HREF, type: HRef, read_only: false },
        vref => { register: VREF, type: VRef, read_only: false },
        hstart => { register: HSTART, type: Hstart, read_only: false },
        hstop => { register: HSTOP, type: Hstop, read_only: false },
        vstart => { register: VSTART, type: VStart, read_only: false },
        vstop => { register: VSTOP, type: VStop, read_only: false },
        pshft => { register: PSHFT, type: PShft, read_only: false },

        // Scaling registers
        scaling_xsc => { register: SCALING_XSC, type: ScalingXSc, read_only: false },
        scaling_ysc => { register: SCALING_YSC, type: ScalingYSc, read_only: false },
        scaling_dcwctr => { register: SCALING_DCWCTR, type: ScalingDcwCtr, read_only: false },
        scaling_pclk_div => { register: SCALING_PCLK_DIV, type: ScalingPClkDiv, read_only: false },
        scaling_pclk_delay => { register: SCALING_PCLK_DELAY, type: ScalingPclkDelay, read_only: false },

        // Color gain registers
        gain => { register: GAIN, type: Gain, read_only: false },
        blue => { register: BLUE, type: Blue, read_only: false },
        red => { register: RED, type: Red, read_only: false },

        // Average registers
        bave => { register: BAVE, type: BAve, read_only: false },
        gbave => { register: GbAVE, type: GbAve, read_only: false },
        rave => { register: RAVE, type: Rave, read_only: false },
        yave => { register: YAVE, type: YAve, read_only: false },

        // Read-only ID registers
        pid => { register: PID, type: Pid, read_only: true },
        ver => { register: VER, type: Ver, read_only: true },
        midh => { register: MIDH, type: MidH, read_only: true },
        midl => { register: MIDL, type: MidL, read_only: true },

        // AEC/AGC registers
        aechh => { register: AECHH, type: Aechh, read_only: false },
        aech => { register: AECH, type: Aech, read_only: false },
        aew => { register: AEW, type: Aew, read_only: false },
        aeb => { register: AEB, type: Aeb, read_only: false },
        vpt => { register: VPT, type: Vpt, read_only: false },
        laec => { register: LAEC, type: Laec, read_only: false },

        // Bias registers
        bbias => { register: BBIAS, type: Bias, read_only: false },
        gbbias => { register: GbBIAS, type: Bias, read_only: false },
        rbias => { register: RBIAS, type: Bias, read_only: false },

        // MVFP register
        mvfp => { register: MVFP, type: Mvfp, read_only: false },

        // ADC control registers
        adcctr0 => { register: ADCCTR0, type: AdcCtr0, read_only: false },
        adcctr1 => { register: ADCCTR1, type: Adcctr1, read_only: false },
        adcctr2 => { register: ADCCTR2, type: Adcctr2, read_only: false },
        adcctr3 => { register: ADCCTR3, type: Adcctr3, read_only: false },

        // Exposure registers
        exhch => { register: EXHCH, type: ExhcH, read_only: false },
        exhcl => { register: EXHCL, type: ExhcL, read_only: false },
        advfl => { register: ADVFL, type: AdvfL, read_only: false },
        advfh => { register: ADVFH, type: AdvfH, read_only: false },

        // Sync registers
        hsyst => { register: HSYST, type: HSySt, read_only: false },
        hsyen => { register: HSYEN, type: HSyEn, read_only: false },

        // Miscellaneous registers
        chlf => { register: CHLF, type: Chlf, read_only: false },
        arblm => { register: ARBLM, type: Arblm, read_only: false },
        adc => { register: ADC, type: Adc, read_only: false },
        acom => { register: ACOM, type: ACom, read_only: false },
        ofon => { register: OFON, type: Ofon, read_only: false },
        edge => { register: EDGE, type: Edge, read_only: false },

        // AWB control registers
        awbc1 => { register: AWBC1, type: AwbC, read_only: false },
        awbc2 => { register: AWBC2, type: AwbC, read_only: false },
        awbc3 => { register: AWBC3, type: AwbC, read_only: false },
        awbc4 => { register: AWBC4, type: AwbC, read_only: false },
        awbc5 => { register: AWBC5, type: AwbC, read_only: false },
        awbc6 => { register: AWBC6, type: AwbC, read_only: false },
        awbctr0 => { register: AWBCTR0, type: Awbctr, read_only: false },
        awbctr1 => { register: AWBCTR1, type: Awbctr, read_only: false },
        awbctr2 => { register: AWBCTR2, type: Awbctr, read_only: false },
        awbctr3 => { register: AWBCTR3, type: Awbctr, read_only: false },

        // Special registers
        reg4b => { register: REG4B, type: Reg4b, read_only: false },
        reg74 => { register: REG74, type: Reg74, read_only: false },
        reg75 => { register: REG75, type: Reg75, read_only: false },
        reg76 => { register: REG76, type: Reg76, read_only: false },
        reg77 => { register: REG77, type: Reg77, read_only: false },

        // Denoise threshold
        dnsth => { register: DNSTH, type: DnSth, read_only: false },

        // Matrix coefficients
        mtx1 => { register: MTX1, type: Mtx, read_only: false },
        mtx2 => { register: MTX2, type: Mtx, read_only: false },
        mtx3 => { register: MTX3, type: Mtx, read_only: false },
        mtx4 => { register: MTX4, type: Mtx, read_only: false },
        mtx5 => { register: MTX5, type: Mtx, read_only: false },
        mtx6 => { register: MTX6, type: Mtx, read_only: false },
        mtxs => { register: MTXS, type: MtxS, read_only: false },

        // Brightness and contrast
        bright => { register: BRIGHT, type: Bright, read_only: false },
        contras => { register: CONTRAS, type: Contras, read_only: false },
        contras_center => { register: CONTRAS_CENTER, type: ContrasCenter, read_only: false },

        // Lens correction
        lcc1 => { register: LCC1, type: Lcc1, read_only: false },
        lcc2 => { register: LCC2, type: Lcc2, read_only: false },
        lcc3 => { register: LCC3, type: Lcc3, read_only: false },
        lcc4 => { register: LCC4, type: Lcc4, read_only: false },
        lcc5 => { register: LCC5, type: Lcc5, read_only: false },
        lcc6 => { register: LCC6, type: Lcc, read_only: false },
        lcc7 => { register: LCC7, type: Lcc, read_only: false },

        // Manual U/V
        manu => { register: MANU, type: ManU, read_only: false },
        manv => { register: MANV, type: ManV, read_only: false },

        // Gain registers
        gfix => { register: GFIX, type: GFix, read_only: false },
        ggain => { register: GGAIN, type: GGain, read_only: false },
        dblv => { register: DBLV, type: Dblv, read_only: false },

        // Slope
        slop => { register: SLOP, type: Slop, read_only: false },

        // Gamma curve
        gam1 => { register: GAM1, type: Gam, read_only: false },
        gam2 => { register: GAM2, type: Gam, read_only: false },
        gam3 => { register: GAM3, type: Gam, read_only: false },
        gam4 => { register: GAM4, type: Gam, read_only: false },
        gam5 => { register: GAM5, type: Gam, read_only: false },
        gam6 => { register: GAM6, type: Gam, read_only: false },
        gam7 => { register: GAM7, type: Gam, read_only: false },
        gam8 => { register: GAM8, type: Gam, read_only: false },
        gam9 => { register: GAM9, type: Gam, read_only: false },
        gam10 => { register: GAM10, type: Gam, read_only: false },
        gam11 => { register: GAM11, type: Gam, read_only: false },
        gam12 => { register: GAM12, type: Gam, read_only: false },
        gam13 => { register: GAM13, type: Gam, read_only: false },
        gam14 => { register: GAM14, type: Gam, read_only: false },
        gam15 => { register: GAM15, type: Gam, read_only: false },

        // RGB444
        rgb444 => { register: RGB444, type: Rgb444, read_only: false },

        // Dummy line
        dm_lnl => { register: DM_LNL, type: DmLnL, read_only: false },
        dm_lnh => { register: DM_LNH, type: DmLnH, read_only: false },

        // Banding filter
        bd50st => { register: BD50ST, type: Bd50St, read_only: false },
        bd60st => { register: BD60ST, type: Bd60St, read_only: false },
        bd50max => { register: BD50MAX, type: BdMax, read_only: false },
        bd60max => { register: BD60MAX, type: BdMax, read_only: false },

        // Histogram AEC
        haecc1 => { register: HAECC1, type: HAecCX, read_only: false },
        haecc2 => { register: HAECC2, type: HAecCX, read_only: false },
        haecc3 => { register: HAECC3, type: HAecCX, read_only: false },
        haecc4 => { register: HAECC4, type: HAecCX, read_only: false },
        haecc5 => { register: HAECC5, type: HAecCX, read_only: false },
        haecc6 => { register: HAECC6, type: HAecCX, read_only: false },
        haecc7 => { register: HAECC7, type: HAecC7, read_only: false },

        // Night mode control
        nt_ctrl => { register: NT_CTRL, type: NtCtrl, read_only: false },

        // Strobe control
        str_opt => { register: STR_OPT, type: StrOpt, read_only: false },
        str_r => { register: STR_R, type: StrX, read_only: false },
        str_g => { register: STR_G, type: StrX, read_only: false },
        str_b => { register: STR_B, type: StrX, read_only: false },

        // ABLC
        ablc1 => { register: ABLC1, type: Ablc1, read_only: false },
        thl_st => { register: THL_ST, type: ThlSt, read_only: false },
        thl_dlt => { register: THL_DLT, type: ThlDlt, read_only: false },

        // ADC offset
        ad_chb => { register: AD_CHB, type: AdChX, read_only: false },
        ad_chr => { register: AD_CHR, type: AdChX, read_only: false },
        ad_chgb => { register: AD_CHGb, type: AdChX, read_only: false },
        ad_chgr => { register: AD_CHGr, type: AdChX, read_only: false },

        // Saturation control
        satctr => { register: SATCTR, type: SatCtr, read_only: false },
    }
}

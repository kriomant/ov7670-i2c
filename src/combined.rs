//! Combined register structs for multi-register settings
//!
//! This module provides type-safe wrappers for OV7670 settings that span multiple registers.
//! Each combined struct contains the actual register structs it combines, with
//! accessor/mutator methods for the spanned settings and individual fields.
//!
//! # Design
//!
//! - Combined structs contain the actual register structs as public fields
//! - Accessor/mutator methods for spanned settings (e.g., `gain()`, `set_gain()`)
//! - Accessor/mutator methods for individual register fields (delegating to contained registers)
//! - `split()` method returns a tuple of individual register structs
//! - `combine()` static method creates combined struct from individual registers

use crate::registers::*;

/// Combined struct for [`Gain`], [`VStart`], [`VStop`], and [`VRef`] registers
///
/// Combined fields:
/// * 10-bit `gain` setting spans [`Gain`] and [`VRef`]
/// * 10-bit `vstart` settings spans [`VStart`] and [`VRef`]
/// * 10-bit `vstop` settings spans [`VStop`] and [`VRef`]
#[derive(Copy, Clone, Debug, Default)]
pub struct GainVStartVStopVRef {
    /// Gain register
    pub gain_reg: Gain,
    /// VStart register
    pub vstart: VStart,
    /// VStop register
    pub vstop: VStop,
    /// VRef register
    pub vref: VRef,
}

impl GainVStartVStopVRef {
    /// Get the 10-bit gain value
    pub fn gain(&self) -> u16 {
        ((self.vref.gain_high() as u16) << 8) | (self.gain_reg.gain_low() as u16)
    }

    /// Set the 10-bit gain value
    pub fn set_gain(&mut self, gain: u16) {
        self.gain_reg.set_gain_low((gain & 0xFF) as u8);
        self.vref.set_gain_high(((gain >> 8) & 0x03) as u8);
    }

    /// Get the 10-bit VSTART value
    pub fn vstart(&self) -> u16 {
        ((self.vstart.vstart_high() as u16) << 2) | (self.vref.vstart_low() as u16)
    }

    /// Set the 10-bit VSTART value
    pub fn set_vstart(&mut self, value: u16) {
        self.vstart.set_vstart_high(((value >> 2) & 0xFF) as u8);
        self.vref.set_vstart_low((value & 0x03) as u8);
    }

    /// Get the 10-bit VSTOP value
    pub fn vstop(&self) -> u16 {
        ((self.vstop.vstop_high() as u16) << 2) | ((self.vref.vstop_low() as u16) << 2)
    }

    /// Set the 10-bit VSTOP value
    pub fn set_vstop(&mut self, value: u16) {
        self.vstop.set_vstop_high(((value >> 2) & 0xFF) as u8);
        self.vref.set_vstop_low(((value >> 2) & 0x03) as u8);
    }

    /// Get VRef reserved bits
    pub fn vref_reserved(&self) -> u8 {
        self.vref.reserved0()
    }

    /// Set VRef reserved bits
    pub fn set_vref_reserved(&mut self, value: u8) {
        self.vref.set_reserved0(value)
    }

    /// Split into individual registers
    pub fn split(self) -> (Gain, VStart, VStop, VRef) {
        (self.gain_reg, self.vstart, self.vstop, self.vref)
    }

    /// Combine from individual registers
    pub fn combine(gain_reg: Gain, vstart: VStart, vstop: VStop, vref: VRef) -> Self {
        Self { gain_reg, vstart, vstop, vref }
    }
}

/// Combined struct for AEC setting spanning [`Com1`], [`Aech`], and [`Aechh`] registers
///
/// The 16-bit AEC value is split across three registers:
/// - Bits [1:0] in [`Com1::aec_low`]
/// - Bits [9:2] in [`Aech::aec_mid`]
/// - Bits [15:10] in [`Aechh::aec_high`]
#[derive(Copy, Clone, Debug, Default)]
pub struct AecCombined {
    /// Com1 register
    pub com1: Com1,
    /// Aech register
    pub aech: Aech,
    /// Aechh register
    pub aechh: Aechh,
}

impl AecCombined {
    /// Get the 16-bit AEC value
    pub fn aec(&self) -> u16 {
        ((self.aechh.aec_high() as u16) << 10)
            | ((self.aech.aec_mid() as u16) << 2)
            | (self.com1.aec_low() as u16)
    }

    /// Set the 16-bit AEC value
    pub fn set_aec(&mut self, aec: u16) {
        self.com1.set_aec_low((aec & 0x03) as u8);
        self.aech.set_aec_mid(((aec >> 2) & 0xFF) as u8);
        self.aechh.set_aec_high(((aec >> 10) & 0x3F) as u8);
    }

    /// Get CCIR656 format flag
    pub fn ccir656(&self) -> bool {
        self.com1.ccir656()
    }

    /// Set CCIR656 format flag
    pub fn set_ccir656(&mut self, value: bool) {
        self.com1.set_ccir656(value)
    }

    /// Get Com1 reserved bits
    pub fn com1_reserved0(&self) -> u8 {
        self.com1.reserved0()
    }

    /// Set Com1 reserved bits
    pub fn set_com1_reserved0(&mut self, value: u8) {
        self.com1.set_reserved0(value)
    }

    /// Get Com1 reserved bits
    pub fn com1_reserved1(&self) -> u8 {
        self.com1.reserved1()
    }

    /// Set Com1 reserved bits
    pub fn set_com1_reserved1(&mut self, value: u8) {
        self.com1.set_reserved1(value)
    }

    /// Get Aechh reserved bits
    pub fn aechh_reserved(&self) -> u8 {
        self.aechh.reserved0()
    }

    /// Set Aechh reserved bits
    pub fn set_aechh_reserved(&mut self, value: u8) {
        self.aechh.set_reserved0(value)
    }

    /// Split into individual registers
    pub fn split(self) -> (Com1, Aech, Aechh) {
        (self.com1, self.aech, self.aechh)
    }

    /// Combine from individual registers
    pub fn combine(com1: Com1, aech: Aech, aechh: Aechh) -> Self {
        Self { com1, aech, aechh }
    }
}

/// Combined struct for [`HStart`], [`HStop`], and [`HRef`] registers
///
/// HSTART and HSTOP are 11-bit values split across registers:
/// - HSTART: [`Hstart::hstart_high`] (bits [10:3]) + [`HRef::hstart_low`] (bits [2:0])
/// - HSTOP: [`Hstop::hstop_high`] (bits [10:3]) + [`HRef::hstop_low`] (bits [5:3])
#[derive(Copy, Clone, Debug, Default)]
pub struct HStartStopHref {
    /// Hstart register
    pub hstart: Hstart,
    /// Hstop register
    pub hstop: Hstop,
    /// HRef register
    pub href: HRef,
}

impl HStartStopHref {
    /// Get the 11-bit HSTART value
    pub fn hstart(&self) -> u16 {
        ((self.hstart.hstart_high() as u16) << 3) | (self.href.hstart_low() as u16)
    }

    /// Set the 11-bit HSTART value
    pub fn set_hstart(&mut self, value: u16) {
        self.hstart.set_hstart_high(((value >> 3) & 0xFF) as u8);
        self.href.set_hstart_low((value & 0x07) as u8);
    }

    /// Get the 11-bit HSTOP value
    pub fn hstop(&self) -> u16 {
        ((self.hstop.hstop_high() as u16) << 3) | ((self.href.hstop_low() as u16) << 3)
    }

    /// Set the 11-bit HSTOP value
    pub fn set_hstop(&mut self, value: u16) {
        self.hstop.set_hstop_high(((value >> 3) & 0xFF) as u8);
        self.href.set_hstop_low(((value >> 3) & 0x07) as u8);
    }

    /// Get HREF edge offset
    pub fn edge_offset(&self) -> u8 {
        self.href.edge_offset()
    }

    /// Set HREF edge offset
    pub fn set_edge_offset(&mut self, value: u8) {
        self.href.set_edge_offset(value)
    }

    /// Split into individual registers
    pub fn split(self) -> (Hstart, Hstop, HRef) {
        (self.hstart, self.hstop, self.href)
    }

    /// Combine from individual registers
    pub fn combine(hstart: Hstart, hstop: Hstop, href: HRef) -> Self {
        Self { hstart, hstop, href }
    }
}

/// Combined struct for HSYNC delays spanning [`ExhcH`], [`Hsyst`], and [`Hsyen`] registers
///
/// HSYNC delays are 10-bit values split across registers:
/// - Rising edge: [`ExhcH::hsync_raising_edge_delay`] (bits [3:2]) + [`Hsyst::hsync_raising_edge_delay_low`] (bits [7:0])
/// - Falling edge: [`ExhcH::hsync_falling_edge_delay`] (bits [3:2]) + [`Hsyen::hsync_falling_edge_delay_low`] (bits [7:0])
#[derive(Copy, Clone, Debug, Default)]
pub struct HsyncDelayCombined {
    /// ExhcH register
    pub exhch: ExhcH,
    /// Hsyst register
    pub hsyst: HSySt,
    /// Hsyen register
    pub hsyen: HSyEn,
}

impl HsyncDelayCombined {
    /// Get the 10-bit rising edge delay
    pub fn rising_delay(&self) -> u16 {
        ((self.exhch.hsync_raising_edge_delay() as u16) << 8)
            | (self.hsyst.hsync_raising_edge_delay_low() as u16)
    }

    /// Set the 10-bit rising edge delay
    pub fn set_rising_delay(&mut self, value: u16) {
        self.exhch.set_hsync_raising_edge_delay(((value >> 8) & 0x03) as u8);
        self.hsyst.set_hsync_raising_edge_delay_low((value & 0xFF) as u8);
    }

    /// Get the 10-bit falling edge delay
    pub fn falling_delay(&self) -> u16 {
        ((self.exhch.hsync_falling_edge_delay() as u16) << 8)
            | (self.hsyen.hsync_falling_edge_delay_low() as u16)
    }

    /// Set the 10-bit falling edge delay
    pub fn set_falling_delay(&mut self, value: u16) {
        self.exhch.set_hsync_falling_edge_delay(((value >> 8) & 0x03) as u8);
        self.hsyen.set_hsync_falling_edge_delay_low((value & 0xFF) as u8);
    }

    /// Get dummy pixels high
    pub fn dummy_pixels_high(&self) -> u8 {
        self.exhch.dummy_pixels_high()
    }

    /// Set dummy pixels high
    pub fn set_dummy_pixels_high(&mut self, value: u8) {
        self.exhch.set_dummy_pixels_high(value)
    }

    /// Split into individual registers
    pub fn split(self) -> (ExhcH, HSySt, HSyEn) {
        (self.exhch, self.hsyst, self.hsyen)
    }

    /// Combine from individual registers
    pub fn combine(exhch: ExhcH, hsyst: HSySt, hsyen: HSyEn) -> Self {
        Self { exhch, hsyst, hsyen }
    }
}

/// Combined struct for [`ScalingXSc`] and [`ScalingYSc`] registers
///
/// The 2-bit test pattern value is split across registers:
/// - Bit 0 in [`ScalingXSc::test_pattern_low`]
/// - Bit 1 in [`ScalingYSc::test_pattern_high`]
#[derive(Copy, Clone, Debug, Default)]
pub struct ScalingXScYSc {
    /// ScalingXSc register
    pub xsc: ScalingXSc,
    /// ScalingYSc register
    pub ysc: ScalingYSc,
}

impl ScalingXScYSc {
    /// Get the 2-bit test pattern value
    pub fn test_pattern(&self) -> TestPattern {
        let value = (self.ysc.test_pattern_high() << 1) | self.xsc.test_pattern_low();
        TestPattern::from_bits(value)
    }

    /// Set the 2-bit test pattern value
    pub fn set_test_pattern(&mut self, value: TestPattern) {
        self.xsc.set_test_pattern_low((value as u8) & 0x01);
        self.ysc.set_test_pattern_high(((value as u8) >> 1) & 0x01);
    }

    /// Get horizontal scale factor
    pub fn horizontal_scale_factor(&self) -> u8 {
        self.xsc.horizontal_scale_factor()
    }

    /// Set horizontal scale factor
    pub fn set_horizontal_scale_factor(&mut self, value: u8) {
        self.xsc.set_horizontal_scale_factor(value)
    }

    /// Get vertical scale factor
    pub fn vertical_scale_factor(&self) -> u8 {
        self.ysc.vertical_scale_factor()
    }

    /// Set vertical scale factor
    pub fn set_vertical_scale_factor(&mut self, value: u8) {
        self.ysc.set_vertical_scale_factor(value)
    }

    /// Split into individual registers
    pub fn split(self) -> (ScalingXSc, ScalingYSc) {
        (self.xsc, self.ysc)
    }

    /// Combine from individual registers
    pub fn combine(xsc: ScalingXSc, ysc: ScalingYSc) -> Self {
        Self { xsc, ysc }
    }
}

/// Combined struct for dummy lines setting spanning [`AdvfL`] and [`AdvfH`] registers
///
/// The 16-bit dummy lines value is split across two registers:
/// - Low byte in [`AdvfL::dummy_lines_low`]
/// - High byte in [`AdvfH::dummy_lines_high`]
#[derive(Copy, Clone, Debug, Default)]
pub struct DummyLines {
    /// AdvfL register
    pub advfl: AdvfL,
    /// AdvfH register
    pub advfh: AdvfH,
}

impl DummyLines {
    /// Get the 16-bit dummy lines value
    pub fn dummy_lines(&self) -> u16 {
        ((self.advfh.dummy_lines_high() as u16) << 8) | (self.advfl.dummy_lines_low() as u16)
    }

    /// Set the 16-bit dummy lines value
    pub fn set_dummy_lines(&mut self, value: u16) {
        self.advfl.set_dummy_lines_low((value & 0xFF) as u8);
        self.advfh.set_dummy_lines_high(((value >> 8) & 0xFF) as u8);
    }

    /// Split into individual registers
    pub fn split(self) -> (AdvfL, AdvfH) {
        (self.advfl, self.advfh)
    }

    /// Combine from individual registers
    pub fn combine(advfl: AdvfL, advfh: AdvfH) -> Self {
        Self { advfl, advfh }
    }
}

/// Combined struct for dummy line setting spanning [`DmLnL`] and [`DmLnH`] registers
///
/// The 16-bit dummy line value is split across two registers:
/// - Low byte in [`DmLnL::dummy_line_low`]
/// - High byte in [`DmLnH::dummy_line_high`]
#[derive(Copy, Clone, Debug, Default)]
pub struct DummyLine {
    /// DmLnL register
    pub dm_lnl: DmLnL,
    /// DmLnH register
    pub dm_lnh: DmLnH,
}

impl DummyLine {
    /// Get the 16-bit dummy line value
    pub fn dummy_line(&self) -> u16 {
        ((self.dm_lnh.dummy_line_high() as u16) << 8) | (self.dm_lnl.dummy_line_low() as u16)
    }

    /// Set the 16-bit dummy line value
    pub fn set_dummy_line(&mut self, value: u16) {
        self.dm_lnl.set_dummy_line_low((value & 0xFF) as u8);
        self.dm_lnh.set_dummy_line_high(((value >> 8) & 0xFF) as u8);
    }

    /// Split into individual registers
    pub fn split(self) -> (DmLnL, DmLnH) {
        (self.dm_lnl, self.dm_lnh)
    }

    /// Combine from individual registers
    pub fn combine(dm_lnl: DmLnL, dm_lnh: DmLnH) -> Self {
        Self { dm_lnl, dm_lnh }
    }
}

/// Combined struct for manufacturer ID spanning [`MidH`] and [`MidL`] registers
///
/// The 16-bit manufacturer ID is split across two registers:
/// - High byte in [`MidH::manufacturer_id_high`]
/// - Low byte in [`MidL::manufacturer_id_low`]
#[derive(Copy, Clone, Debug, Default)]
pub struct ManufacturerId {
    /// MidH register
    pub midh: MidH,
    /// MidL register
    pub midl: MidL,
}

impl ManufacturerId {
    /// Get the 16-bit manufacturer ID
    pub fn id(&self) -> u16 {
        ((self.midh.manufacturer_id_high() as u16) << 8) | (self.midl.manufacturer_id_low() as u16)
    }

    /// Set the 16-bit manufacturer ID
    pub fn set_id(&mut self, value: u16) {
        self.midh.set_manufacturer_id_high(((value >> 8) & 0xFF) as u8);
        self.midl.set_manufacturer_id_low((value & 0xFF) as u8);
    }

    /// Split into individual registers
    pub fn split(self) -> (MidH, MidL) {
        (self.midh, self.midl)
    }

    /// Combine from individual registers
    pub fn combine(midh: MidH, midl: MidL) -> Self {
        Self { midh, midl }
    }
}

/// Combined struct for product ID spanning [`Pid`] and [`Ver`] registers
///
/// The 16-bit product ID is split across two registers:
/// - MSB in [`Pid::product_id_msb`]
/// - LSB in [`Ver::product_id_lsb`]
#[derive(Copy, Clone, Debug, Default)]
pub struct ProductId {
    /// Pid register
    pub pid: Pid,
    /// Ver register
    pub ver: Ver,
}

impl ProductId {
    /// Get the 16-bit product ID
    pub fn id(&self) -> u16 {
        ((self.pid.product_id_msb() as u16) << 8) | (self.ver.product_id_lsb() as u16)
    }

    /// Set the 16-bit product ID
    pub fn set_id(&mut self, value: u16) {
        self.pid.set_product_id_msb(((value >> 8) & 0xFF) as u8);
        self.ver.set_product_id_lsb((value & 0xFF) as u8);
    }

    /// Split into individual registers
    pub fn split(self) -> (Pid, Ver) {
        (self.pid, self.ver)
    }

    /// Combine from individual registers
    pub fn combine(pid: Pid, ver: Ver) -> Self {
        Self { pid, ver }
    }
}

/// Combined struct for dummy pixels setting spanning [`ExhcH`] and [`ExhcL`] registers
///
/// The 12-bit dummy pixels value is split across registers:
/// - Bits [11:8] in [`ExhcH::dummy_pixels_high`]
/// - Bits [7:0] in [`ExhcL::dummy_pixels_low`]
#[derive(Copy, Clone, Debug, Default)]
pub struct DummyPixels {
    /// ExhcH register
    pub exhch: ExhcH,
    /// ExhcL register
    pub exhcl: ExhcL,
}

impl DummyPixels {
    /// Get the 12-bit dummy pixels value
    pub fn dummy_pixels(&self) -> u16 {
        ((self.exhch.dummy_pixels_high() as u16) << 8) | (self.exhcl.dummy_pixels_low() as u16)
    }

    /// Set the 12-bit dummy pixels value
    pub fn set_dummy_pixels(&mut self, value: u16) {
        self.exhch.set_dummy_pixels_high(((value >> 8) & 0x0F) as u8);
        self.exhcl.set_dummy_pixels_low((value & 0xFF) as u8);
    }

    /// Get HSYNC falling edge delay MSB
    pub fn hsync_falling_edge_delay_msb(&self) -> u8 {
        self.exhch.hsync_falling_edge_delay()
    }

    /// Set HSYNC falling edge delay MSB
    pub fn set_hsync_falling_edge_delay_msb(&mut self, value: u8) {
        self.exhch.set_hsync_falling_edge_delay(value)
    }

    /// Get HSYNC rising edge delay MSB
    pub fn hsync_rising_edge_delay_msb(&self) -> u8 {
        self.exhch.hsync_raising_edge_delay()
    }

    /// Set HSYNC rising edge delay MSB
    pub fn set_hsync_rising_edge_delay_msb(&mut self, value: u8) {
        self.exhch.set_hsync_raising_edge_delay(value)
    }

    /// Split into individual registers
    pub fn split(self) -> (ExhcH, ExhcL) {
        (self.exhch, self.exhcl)
    }

    /// Combine from individual registers
    pub fn combine(exhch: ExhcH, exhcl: ExhcL) -> Self {
        Self { exhch, exhcl }
    }
}

/// Test pattern
#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum TestPattern {
    /// No test output
    No = 0b00,
    /// Shifting "1"
    Shifting  = 0b01,
    /// 8-bar color bar
    ColorBar = 0b10,
    /// Fade to gray color bar
    FadeToGray = 0b11,
}

impl TestPattern {
    const fn from_bits(bits: u8) -> Self {
        match bits {
            0b00 => Self::No,
            0b01 => Self::Shifting,
            0b10 => Self::ColorBar,
            0b11 => Self::FadeToGray,
            _ => unreachable!()
        }
    }

    const fn into_bits(self) -> u8 {
        self as u8
    }
}

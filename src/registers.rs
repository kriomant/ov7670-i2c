//! OV7670 register definitions using bitfields
//!
//! This module provides type-safe bitfield definitions for all OV7670 camera registers.
//! It is based on "Advanced Information Preliminary Datasheet, OV7670/OV7171 CMOS VGA (640x480) CameraChip™
//! Sensor with OmniVision® Technology", Version 1.4, August 21, 2006.

// # Module code style
//
// ## Bit order
//
// Use MSB order: `#[bitfield(u8, order=msb)]`
// Use it even for struct with single field for consistency.
//
// *Rationale*: datasheet lists fields in MSB order.
//
// *Example*: [Gain]
//
// ## Fields visibility
//
// Register struct fields should not have any visibility keyword.
//
// *Rationale*: Explicit visibility keywords on fields are unnecessary since the bitfield
// macro generates public getter/setter methods with visibility inherited from the struct.
// Omitting visibility keywords keeps the code cleaner.
//
// *Examples*: [VRef], [Gain]
//
// ## Reserved fields
//
// If reserved field is the only field in registry, name it just 'reserved'.
// Otherwise name them 'reserved0', 'reserved1' and so on in MSB order.
//
// Always use 'u8' type for reserved fields, even for single-bit ones.
//
// *Rationale*: New datasheet versions may describe previously reserved bits, or users
// may discover their meaning. Reserved fields should be easy to use without waiting
// for library updates.
//
// *Examples*: [VRef::reserved0], [Laec::reserved]
//
// ## Bits annotation
//
// Annotate each field with their position inside register.
// Put annotation at the end of field comment's first line.
// It should be either `(bit N)` for single bit or `(bits N:M)`, where N>M for multi-bit.
//
// ## Spanned settings
//
// Some settings are spanned across several register fields.
// Field for such settings in different registers should be named consistently,
// with '_high' and '_low' siffixes for two-part settings,
// '_high', '_mid' and '_low' suffixes for three-part.
//
// Add references from each component to each other in comment.
//
// *Example*: [Aechh::aec_high], [Aech::aec_mid], [Com1::aec_low]
//
// ## Enums
//
// ### Total enums
//
// Use enum for field type if all possible field values are covered by enum.
// *Example*: [Com2::output_drive]
//
// ### Duplicated values
//
// Sometimes there are several bit patterns with same effect.
//
// Create enum for all effects (not possible patterns), but don't assign
// values to enum's variants. Just choose some bit pattern for enum variant
// if several are possible and return it from 'into_bits', but accept all
// possible in 'from_bits'.
//
// Add '_raw' suffix to field name. Add getter/setter methods to struct's 'impl' block,
// also add corresponding 'with_*' method to builder 'impl'.
//
// *Example*: [Com15::output_range_raw]
//
// ### Non-exhaustive enums
//
// If not all bit patterns of field are valid, create enum for valid ones.
//
// Add '_raw' suffix to field name.
//
// Add getter/setter methods to struct's 'impl' block,
// also add corresponding 'with_*' method to builder 'impl'.
// *Important*: getter and 'from_bits' should return `Option<EnumType>`.
//
// *Example*: [AdcCtr0::adc_reference_adjustment_raw]
//
// ## Register struct merging
//
// If several register use similar structure (consider not only field types, but names too),
// then single structure definition may be used for them
//
// *Example*: [Lcc]

use bitfields::bitfield;

/// Register addresses
#[allow(non_upper_case_globals)]
pub mod addr {
    pub const GAIN: u8 = 0x00;
    pub const BLUE: u8 = 0x01;
    pub const RED: u8 = 0x02;
    pub const VREF: u8 = 0x03;
    pub const COM1: u8 = 0x04;
    pub const BAVE: u8 = 0x05;
    pub const GbAVE: u8 = 0x06;
    pub const AECHH: u8 = 0x07;
    pub const RAVE: u8 = 0x08;
    pub const COM2: u8 = 0x09;
    pub const PID: u8 = 0x0A;
    pub const VER: u8 = 0x0B;
    pub const COM3: u8 = 0x0C;
    pub const COM4: u8 = 0x0D;
    pub const COM5: u8 = 0x0E;
    pub const COM6: u8 = 0x0F;
    pub const AECH: u8 = 0x10;
    pub const CLKRC: u8 = 0x11;
    pub const COM7: u8 = 0x12;
    pub const COM8: u8 = 0x13;
    pub const COM9: u8 = 0x14;
    pub const COM10: u8 = 0x15;
    pub const HSTART: u8 = 0x17;
    pub const HSTOP: u8 = 0x18;
    pub const VSTART: u8 = 0x19;
    pub const VSTOP: u8 = 0x1A;
    pub const PSHFT: u8 = 0x1B;
    pub const MIDH: u8 = 0x1C;
    pub const MIDL: u8 = 0x1D;
    pub const MVFP: u8 = 0x1E;
    pub const LAEC: u8 = 0x1F;
    pub const ADCCTR0: u8 = 0x20;
    pub const ADCCTR1: u8 = 0x21;
    pub const ADCCTR2: u8 = 0x22;
    pub const ADCCTR3: u8 = 0x23;
    pub const AEW: u8 = 0x24;
    pub const AEB: u8 = 0x25;
    pub const VPT: u8 = 0x26;
    pub const BBIAS: u8 = 0x27;
    pub const GbBIAS: u8 = 0x28;
    pub const EXHCH: u8 = 0x2A;
    pub const EXHCL: u8 = 0x2B;
    pub const RBIAS: u8 = 0x2C;
    pub const ADVFL: u8 = 0x2D;
    pub const ADVFH: u8 = 0x2E;
    pub const YAVE: u8 = 0x2F;
    pub const HSYST: u8 = 0x30;
    pub const HSYEN: u8 = 0x31;
    pub const HREF: u8 = 0x32;
    pub const CHLF: u8 = 0x33;
    pub const ARBLM: u8 = 0x34;
    pub const ADC: u8 = 0x37;
    pub const ACOM: u8 = 0x38;
    pub const OFON: u8 = 0x39;
    pub const TSLB: u8 = 0x3A;
    pub const COM11: u8 = 0x3B;
    pub const COM12: u8 = 0x3C;
    pub const COM13: u8 = 0x3D;
    pub const COM14: u8 = 0x3E;
    pub const EDGE: u8 = 0x3F;
    pub const COM15: u8 = 0x40;
    pub const COM16: u8 = 0x41;
    pub const COM17: u8 = 0x42;
    pub const AWBC1: u8 = 0x43;
    pub const AWBC2: u8 = 0x44;
    pub const AWBC3: u8 = 0x45;
    pub const AWBC4: u8 = 0x46;
    pub const AWBC5: u8 = 0x47;
    pub const AWBC6: u8 = 0x48;
    pub const REG4B: u8 = 0x4B;
    pub const DNSTH: u8 = 0x4C;
    pub const MTX1: u8 = 0x4F;
    pub const MTX2: u8 = 0x50;
    pub const MTX3: u8 = 0x51;
    pub const MTX4: u8 = 0x52;
    pub const MTX5: u8 = 0x53;
    pub const MTX6: u8 = 0x54;
    pub const BRIGHT: u8 = 0x55;
    pub const CONTRAS: u8 = 0x56;
    pub const CONTRAS_CENTER: u8 = 0x57;
    pub const MTXS: u8 = 0x58;
    pub const LCC1: u8 = 0x62;
    pub const LCC2: u8 = 0x63;
    pub const LCC3: u8 = 0x64;
    pub const LCC4: u8 = 0x65;
    pub const LCC5: u8 = 0x66;
    pub const MANU: u8 = 0x67;
    pub const MANV: u8 = 0x68;
    pub const GFIX: u8 = 0x69;
    pub const GGAIN: u8 = 0x6A;
    pub const DBLV: u8 = 0x6B;
    pub const AWBCTR3: u8 = 0x6C;
    pub const AWBCTR2: u8 = 0x6D;
    pub const AWBCTR1: u8 = 0x6E;
    pub const AWBCTR0: u8 = 0x6F;
    pub const SCALING_XSC: u8 = 0x70;
    pub const SCALING_YSC: u8 = 0x71;
    pub const SCALING_DCWCTR: u8 = 0x72;
    pub const SCALING_PCLK_DIV: u8 = 0x73;
    pub const REG74: u8 = 0x74;
    pub const REG75: u8 = 0x75;
    pub const REG76: u8 = 0x76;
    pub const REG77: u8 = 0x77;
    pub const SLOP: u8 = 0x7A;
    pub const GAM1: u8 = 0x7B;
    pub const GAM2: u8 = 0x7C;
    pub const GAM3: u8 = 0x7D;
    pub const GAM4: u8 = 0x7E;
    pub const GAM5: u8 = 0x7F;
    pub const GAM6: u8 = 0x80;
    pub const GAM7: u8 = 0x81;
    pub const GAM8: u8 = 0x82;
    pub const GAM9: u8 = 0x83;
    pub const GAM10: u8 = 0x84;
    pub const GAM11: u8 = 0x85;
    pub const GAM12: u8 = 0x86;
    pub const GAM13: u8 = 0x87;
    pub const GAM14: u8 = 0x88;
    pub const GAM15: u8 = 0x89;
    pub const RGB444: u8 = 0x8C;
    pub const DM_LNL: u8 = 0x92;
    pub const DM_LNH: u8 = 0x93;
    pub const LCC6: u8 = 0x94;
    pub const LCC7: u8 = 0x95;
    pub const BD50ST: u8 = 0x9D;
    pub const BD60ST: u8 = 0x9E;
    pub const HAECC1: u8 = 0x9F;
    pub const HAECC2: u8 = 0xA0;
    pub const SCALING_PCLK_DELAY: u8 = 0xA2;
    pub const NT_CTRL: u8 = 0xA4;
    pub const BD50MAX: u8 = 0xA5;
    pub const HAECC3: u8 = 0xA6;
    pub const HAECC4: u8 = 0xA7;
    pub const HAECC5: u8 = 0xA8;
    pub const HAECC6: u8 = 0xA9;
    pub const HAECC7: u8 = 0xAA;
    pub const BD60MAX: u8 = 0xAB;
    pub const STR_OPT: u8 = 0xAC;
    pub const STR_R: u8 = 0xAD;
    pub const STR_G: u8 = 0xAE;
    pub const STR_B: u8 = 0xAF;
    pub const ABLC1: u8 = 0xB1;
    pub const THL_ST: u8 = 0xB3;
    pub const THL_DLT: u8 = 0xB5;
    pub const AD_CHB: u8 = 0xBE;
    pub const AD_CHR: u8 = 0xBF;
    pub const AD_CHGb: u8 = 0xC0;
    pub const AD_CHGr: u8 = 0xC1;
    pub const SATCTR: u8 = 0xC9;
}

/// GAIN - AGC Gain Control (0x00)
#[bitfield(u8, order=msb)]
#[derive(Copy, Clone)]
pub struct Gain {
    /// Low bits of gain, high bits are in [Vref::gain_high]. (bits 7:0)
    #[bits(8)]
    gain_low: u8,
}

/// BLUE - AWB Blue Channel Gain (0x01)
/// Blue channel gain setting for Auto White Balance
#[bitfield(u8, order=msb)]
#[derive(Copy, Clone)]
pub struct Blue {
    /// Blue channel gain (bits 7:0)
    #[bits(8)]
    blue_gain: u8,
}

/// RED - AWB Red Channel Gain (0x02)
/// Red channel gain setting for Auto White Balance
#[bitfield(u8, order=msb)]
#[derive(Copy, Clone)]
pub struct Red {
    /// Red channel gain (bits 7:0)
    #[bits(8)]
    red_gain: u8,
}

/// VREF - Vertical Frame Control (0x03)
#[bitfield(u8, order=msb)]
#[derive(Copy, Clone)]
pub struct VRef {
    /// High bits of gain (bits 7:6)
    /// Low bits are in [Gain::gain_low]
    #[bits(2)]
    gain_high: u8,

    /// Reserved (bits 5:4)
    #[bits(2)]
    reserved0: u8,

    /// VSTOP low 2 bits (bits 3:2)
    #[bits(2)]
    vstop_low: u8,

    /// VSTART low 2 bits (bits 1:0)
    #[bits(2)]
    vstart_low: u8,
}

/// COM1 - Common Control 1 (0x04)
/// CCIR656 format, HREF, VSYNC control
#[bitfield(u8, order=msb)]
#[derive(Copy, Clone)]
pub struct Com1 {
    /// Reserved (bit 7)
    #[bits(1)]
    reserved0: u8,

    /// CCIR656 format (bit 6)
    ccir656: bool,

    /// Reserved (bits 5:2)
    #[bits(4)]
    reserved1: u8,

    /// AEC low 2 bits (bits 1:0)
    /// AEC[1:0], see [Aechh::aec_high] for bits 15:10 and [Aech::aec_mid] for bits 9:2.
    #[bits(2)]
    aec_low: u8,
}

/// BAVE - U/B Average Level (0x05)
/// Average level of blue channel
#[bitfield(u8, order=msb)]
#[derive(Copy, Clone)]
pub struct BAve {
    /// U/B average level (bits 7:0)
    #[bits(8)]
    u_b_average: u8,
}

/// GbAVE - Y/Gb Average Level (0x06)
/// Average level of green channel in bayer pattern
#[bitfield(u8, order=msb)]
#[derive(Copy, Clone)]
pub struct GbAve {
    /// Y/Bb average level (bits 7:0)
    #[bits(8)]
    y_gb_average: u8,
}

/// AECHH - Exposure Value MSB (0x07)
/// AEC Most Significant Bits
#[bitfield(u8, order=msb)]
#[derive(Copy, Clone)]
pub struct Aechh {
    /// Reserved (bits 7:6)
    #[bits(2)]
    reserved0: u8,

    /// AEC high bits (bits 5:0)
    /// AEC[15:10], see [Aech::aec_mid] for bits 9:2 and [Com1::aec_low] for bits 1:0.
    #[bits(6)]
    aec_high: u8,
}

/// RAVE - V/R Average Level (0x08)
#[bitfield(u8, order=msb)]
#[derive(Copy, Clone)]
pub struct Rave {
    /// V/R average level (bits 7:0)
    #[bits(8)]
    v_r_average: u8,
}

/// COM2 - Common Control 2 (0x09)
/// Drive capability and output drive selection
#[bitfield(u8, order=msb)]
#[derive(Copy, Clone)]
pub struct Com2 {
    #[bits(3)]
    reserved0: u8,

    /// Soft sleep mode (bit 4)
    #[bits(1)]
    soft_sleep: bool,

    /// Reserved (bits 3:2)
    #[bits(2)]
    reserved1: u8,

    /// Output drive capability (bits 1:0)
    #[bits(2)]
    output_drive: OutputDriveCapability,
}

/// PID - Product ID MSB (0x0A) - Read Only
/// Product ID number MSB
#[bitfield(u8, order=msb)]
#[derive(Copy, Clone)]
pub struct Pid {
    /// Product ID MSB (bits 7:0)
    #[bits(8)]
    product_id_msb: u8,
}

/// VER - Product ID LSB (0x0B) - Read Only
/// Product ID number LSB (version)
#[bitfield(u8, order=msb)]
#[derive(Copy, Clone)]
pub struct Ver {
    /// Product ID LSB (bits 7:0)
    #[bits(8)]
    product_id_lsb: u8,
}

/// COM3 - Common Control 3 (0x0C)
/// Scaling, DCW and variouspixel processing control
#[bitfield(u8, order=msb)]
#[derive(Copy, Clone)]
pub struct Com3 {
    /// Reserved (bit 7)
    #[bits(1)]
    reserved0: u8,

    /// Output data MSB and LSB swap (bit 6)
    #[bits(1)]
    output_msb_lsb_swap: bool,

    /// Tri-state option for output clock at power-down period (bit 5)
    #[bits(1)]
    output_clock_tristate: TriState,

    /// Tri-state option for output data at power-down period (bit 4)
    #[bits(1)]
    output_data_tristate: TriState,

    /// Scale enable.
    /// If set to a pre-defined format (see [Com7::cif], [Com7::qvga], [Com7::qcif])
    /// then [Com14::manual_scaling] must be set for manual adjustment.
    scale_enable: bool,

    /// DCW enable.
    /// If set to a pre-defined format (see [Com7::cif], [Com7::qvga], [Com7::qcif])
    /// then [Com14::manual_scaling] must be set for manual adjustment.
    dcw_enable: bool,

    #[bits(2)]
    reserved1: u8,
}

/// COM4 - Common Control 4 (0x0D)
/// Average option and window size selection
#[bitfield(u8, order=msb)]
#[derive(Copy, Clone)]
pub struct Com4 {
    /// Reserved (bits 7:6)
    #[bits(2)]
    reserved0: u8,

    /// Average option (bits 5:4)
    /// Must be same value as [Com17::aec_window]
    #[bits(2)]
    average_option: WindowSize,

    /// Reserved (bits 3:0)
    #[bits(4)]
    reserved1: u8,
}

/// COM5 - Common Control 5 (0x0E)
/// System clock and exposure options
#[bitfield(u8, order=msb)]
#[derive(Copy, Clone)]
pub struct Com5 {
    /// Reserved (bits 7:0)
    #[bits(8)]
    reserved: u8,
}

/// COM6 - Common Control 6 (0x0F)
/// HREF timing control
#[bitfield(u8, order=msb)]
#[derive(Copy, Clone)]
pub struct Com6 {
    /// Enable HREF at optical black (bit 7)
    #[bits(1)]
    href_at_optical_black: bool,

    /// Reserved (bits 6:2)
    #[bits(5)]
    reserved0: u8,

    /// Reset all timing when format changes (bit 1)
    #[bits(1)]
    reset_timing_on_format_change: bool,

    /// Reserved (bit 0)
    #[bits(1)]
    reserved1: u8,
}

/// AECH - Exposure Value (0x10)
/// AEC[9:2]
#[bitfield(u8, order=msb)]
#[derive(Copy, Clone)]
pub struct Aech {
    /// AEC middle bits [9:2] (bits 7:0)
    /// See [Aechh::aec_high] for AEC[15:10] and [Com1::aec_low] for AEC[1:0].
    #[bits(8)]
    aec_mid: u8,
}

/// CLKRC - Clock Rate Control (0x11)
/// Internal clock pre-scaler
#[bitfield(u8, order=msb)]
#[derive(Copy, Clone)]
pub struct Clkrc {
    /// Reserved (bit 7)
    #[bits(1)]
    reserved0: u8,

    /// Use external clock directly (bit 6)
    use_external_clk: bool,

    /// Clock divider (bits 5:0)
    /// There are two controversal statements abount divider in the
    /// same datasheet.
    /// Section 3.2.1 Clock Prescalar says that divider is `2*(Clkrc::divider+1)`
    /// But register descriptions says that F(internal clock) = F(input clock)/(Clkrc::divider+1)
    #[bits(6)]
    divider: u8,
}

/// COM7 - Common Control 7 (0x12)
/// Main control register for output format and resolution
///
/// |                      | [rgb]    | [raw_rgb] |
/// |----------------------------------------------
/// | YUV                  | false    | true      |
/// | RGB                  | true     | false     |
/// | Bayer RAW            | false    | true      |
/// | Processed Bayer RAW  | true     | true      |
#[bitfield(u8, order=msb)]
#[derive(Copy, Clone)]
pub struct Com7 {
    /// Software reset (bit 7)
    /// Set to `true` for reset. Wait 1ms before next register access.
    reset: bool,

    /// Reserved (bit 6)
    #[bits(1)]
    reserved0: u8,

    /// CIF resolution enable (bit 5)
    cif: bool,

    /// QVGA resolution enable (bit 4)
    qvga: bool,

    /// QCIF resolution enable (bit 3)
    qcif: bool,

    /// RGB format enable (bit 2)
    rgb: bool,

    /// Color bar test pattern enable (bit 1)
    color_bar: bool,

    /// Bayer raw format bit (bit 0)
    raw_rgb: bool,
}

/// Color format selection is spanned between non-consequtive
/// bits of register. These are convenience methods for working
/// with color format value.
impl Com7 {
    pub fn color_format(&self) -> ColorFormat {
        ColorFormat::from_bits(self.0 & 0b101)
    }

    pub fn set_color_format(&mut self, format: ColorFormat) {
        self.0 = self.0 & !0b101 | format.into_bits();
    }
}

/// There are three independent bits for choosing predefined resolutions.
/// They seem to be mutually exclusive, so here are convenience methods
/// to choose format with one call instead of setting three bits manually.
impl Com7 {
    pub fn predefined_resolution(&self) -> PredefinedResolution {
        PredefinedResolution::from_bits((self.0 & 0b111000) >> 3)
    }

    pub fn set_predefined_resolution(&mut self, resolution: PredefinedResolution) {
        self.0 = self.0 & !0b111000 | (resolution.into_bits() << 3);
    }
}

/// COM8 - Common Control 8 (0x13)
/// AGC, AWB and AEC control
#[bitfield(u8, order=msb)]
#[derive(Copy, Clone)]
pub struct Com8 {
    /// Enable fast AGC/AEC algorithm (bit 7)
    fast_aec_agc: bool,

    /// AEC - Step size limit (bit 6)
    /// false: Step size is limited to vertical blank
    /// true: Unlimited step size
    unlimited_step_size: bool,

    /// Banding filter (bit 5)
    /// In order to turn ON the banding filter, [BD50ST] (0x9D)
    /// or [BD60ST] (0x9E) must be set to a non-zero value.
    banding_filter: bool,

    /// Reserved (bits 4:3)
    #[bits(2)]
    reserved0: u8,

    /// AGC auto/manual control (bit 2)
    agc_enable: bool,

    /// AWB enable (bit 1)
    awb_enable: bool,

    /// AEC enable (bit 0)
    aec_enable: bool,
}

/// COM9 - Common Control 9 (0x14)
/// AGC gain ceiling, drop frame control, and freeze AGC/AEC
#[bitfield(u8, order=msb)]
#[derive(Copy, Clone)]
pub struct Com9 {
    /// Reserved (bit 7)
    #[bits(1)]
    reserved0: u8,

    /// Automatic gain ceiling - maximum AGC value (bits 6:4)
    #[bits(3)]
    max_agc: u8,

    /// Reserved (bits 3:1)
    #[bits(3)]
    reserved1: u8,

    /// Freeze AGC/AEC (bit 0)
    freeze_agc_aec: bool,
}

/// COM10 - Common Control 10 (0x15)
/// HSYNC, HREF, PCLK output control
#[bitfield(u8, order=msb)]
#[derive(Copy, Clone)]
pub struct Com10 {
    /// Reserved (bit 7)
    #[bits(1)]
    reserved0: u8,

    /// Use HREF instead of HSYNC (bit 6)
    use_href: bool,

    /// PCLK does not toggle during horizontal blank (bit 5)
    pclk_hb_mask: bool,

    /// PCLK reverse (bit 4)
    pclk_reverse: bool,

    /// HREF reverse (bit 3)
    href_reverse: bool,

    /// VSYNC edge option (bit 2)
    /// VSYNC changes on specified edge of PCLK signal
    #[bits(1)]
    vsync_edge: PClkEdge,

    /// VSYNC negative (bit 1)
    vsync_negative: bool,

    /// HSYNC negative (bit 0)
    hsync_negative: bool,
}

/// HSTART - Horizontal Frame Start (0x17)
/// Horizontal start high 8 bits
#[bitfield(u8, order=msb)]
#[derive(Copy, Clone)]
pub struct Hstart {
    /// HSTART[10:3] (bits 7:0)
    /// High bits of HSTART. Low 3 bits are at [HRef::hstart_low].
    #[bits(8)]
    hstart_high: u8,
}

/// HSTOP - Horizontal Frame End (0x18)
/// Horizontal end high 8 bits
#[bitfield(u8, order=msb)]
#[derive(Copy, Clone)]
pub struct Hstop {
    /// HSTOP[10:3] (bits 7:0)
    /// High bits of HSTOP. Low 3 bits are at [HRef::hstop_low].
    #[bits(8)]
    hstop_high: u8,
}

/// VSTART - Vertical Frame Start (0x19)
/// Vertical start high 8 bits
#[bitfield(u8, order=msb)]
#[derive(Copy, Clone)]
pub struct VStart {
    /// VSTART[9:2] (bits 7:0)
    /// High bits of VSTART. Low 2 bits are at [VRef::vstart_low].
    #[bits(8)]
    vstart_high: u8,
}

/// VSTOP - Vertical Frame End (0x1A)
/// Vertical end high 8 bits
#[bitfield(u8, order=msb)]
#[derive(Copy, Clone)]
pub struct VStop {
    /// VSTOP[9:2] (bits 7:0)
    /// High bits of VSTOP. Low 2 bits are at [VRef::vstop_low].
    #[bits(8)]
    vstop_high: u8,
}

/// PSHFT - Pixel Delay Select (0x1B)
/// Pixel shift for data output
#[bitfield(u8, order=msb)]
#[derive(Copy, Clone)]
pub struct PShft {
    /// Pixel Delay Select (bits 7:0)
    /// Delays timing of the D[7:0] data relative to HREF in pixel units.
    /// Range: 0x00-0xFF.
    #[bits(8)]
    pixel_delay: u8,
}

/// MIDH - Manufacturer ID High (0x1C) - Read Only
/// Manufacturer ID high byte
#[bitfield(u8, order=msb)]
#[derive(Copy, Clone)]
pub struct MidH {
    /// Manufacturer ID high byte (bits 7:0)
    /// Low byte is at [MidL::manufacturer_id_low].
    #[bits(8)]
    manufacturer_id_high: u8,
}

/// MIDL - Manufacturer ID Low (0x1D) - Read Only
/// Manufacturer ID low byte
#[bitfield(u8, order=msb)]
#[derive(Copy, Clone)]
pub struct MidL {
    /// Manufacturer ID low byte (bits 7:0)
    /// High byte is at [MidH::manufacturer_id_high].
    #[bits(8)]
    manufacturer_id_low: u8,
}

/// MVFP - Mirror/VFlip Enable (0x1E)
/// Mirror and vertical flip control
#[bitfield(u8, order=msb)]
#[derive(Copy, Clone)]
pub struct Mvfp {
    /// Reserved (bits 7:6)
    #[bits(2)]
    reserved0: u8,

    /// Mirror enable (bit 5)
    mirror: bool,

    /// Vertical flip enable (bit 4)
    vflip: bool,

    #[bits(1)]
    reserved1: u8,

    /// Black sun enable (bit 2)
    black_sun_enable: bool,

    /// Reserved (bits 1:0)
    #[bits(2)]
    reserved2: u8,
}

/// LAEC - Reserved (0x1F)
/// Reserved register
#[bitfield(u8, order=msb)]
#[derive(Copy, Clone)]
pub struct Laec {
    /// Reserved (bits 7:0)
    #[bits(8)]
    reserved: u8,
}

/// ADCCTR0 - ADC Control (0x20)
/// ADC range control
#[bitfield(u8, order=msb)]
#[derive(Copy, Clone)]
pub struct AdcCtr0 {
    /// Reserved (bits 7:4)
    #[bits(4)]
    reserved0: u8,

    /// ADC range adjustment (bit 3)
    #[bits(1)]
    adc_range_adjustment: AdcRangeAdjustment,

    /// ADC reference adjustment (bits 2:0)
    #[bits(3)]
    adc_reference_adjustment_raw: u8,
}

impl AdcCtr0 {
    pub fn adc_reference_adjustment(&self) -> Option<AdcReferenceAdjustment> {
        AdcReferenceAdjustment::from_bits(self.adc_reference_adjustment_raw())
    }

    pub fn set_adc_reference_adjustment(&mut self, adj: AdcReferenceAdjustment) {
        self.set_adc_reference_adjustment_raw(adj.into_bits());
    }
}

impl AdcCtr0Builder {
    pub fn with_adc_reference_adjustment(self, adj: AdcReferenceAdjustment) -> Self {
        self.with_adc_reference_adjustment_raw(adj.into_bits())
    }
}

/// ADCCTR1 - ADC Control 1 (0x21)
/// Reserved
#[bitfield(u8, order=msb)]
#[derive(Copy, Clone)]
pub struct Adcctr1 {
    /// Reserved (bits 7:0)
    #[bits(8)]
    reserved: u8,
}

/// ADCCTR2 - ADC Control 2 (0x22)
/// Reserved
#[bitfield(u8, order=msb)]
#[derive(Copy, Clone)]
pub struct Adcctr2 {
    /// Reserved (bits 7:0)
    #[bits(8)]
    reserved: u8,
}

/// ADCCTR3 - ADC Control 3 (0x23)
/// Reserved
#[bitfield(u8, order=msb)]
#[derive(Copy, Clone)]
pub struct Adcctr3 {
    /// Reserved (bits 7:0)
    #[bits(8)]
    reserved: u8,
}

/// AEW - AGC/AEC Stable Operating Region (Upper Limit) (0x24)
/// AGC/AEC stable upper limit
#[bitfield(u8, order=msb)]
#[derive(Copy, Clone)]
pub struct Aew {
    /// AEC/AGC Stable Operating Region, upper limit (bits 7:0)
    #[bits(8)]
    aew_value: u8,
}

/// AEB - AGC/AEC Stable Operating Region (Lower Limit) (0x25)
/// AGC/AEC stable lower limit
#[bitfield(u8, order=msb)]
#[derive(Copy, Clone)]
pub struct Aeb {
    /// AEC/AGC Stable Operating Region, lower limit (bits 7:0)
    #[bits(8)]
    aeb_value: u8,
}

/// VPT - AGC/AEC Fast Mode Operating Region (0x26)
/// AGC/AEC fast mode region
#[bitfield(u8, order=msb)]
#[derive(Copy, Clone)]
pub struct Vpt {
    /// High nibble of upper limit of fast mode control zone (bits 7:4)
    #[bits(4)]
    upper_limit: u8,

    /// High nibble of lower limit of fast mode control zone (bits 3:0)
    #[bits(4)]
    lower_limit: u8,
}

/// BBIAS - B Channel Signal Output Bias (0x27)
/// GbBIAS - Gb Channel Signal Output Bias (0x28)
/// RBIAS - R Channel Signal Output Bias (0x2C)
/// Effective only when COM6[3]=1.
#[bitfield(u8, order=msb)]
#[derive(Copy, Clone)]
pub struct Bias {
    /// Bias adjustment sign (bit 7)
    #[bits(1)]
    bias_adj_sign: BiasAdjustmentSign,

    /// Bias value of 10-bit range (bits 6:0)
    #[bits(7)]
    bias_value: u8,
}

/// EXHCH - Dummy Pixel Insert MSB (0x2A)
#[bitfield(u8, order=msb)]
#[derive(Copy, Clone)]
pub struct ExhcH {
    /// 4 MSB for dummy pixel insert in horizontal direction (bits 7:4)
    #[bits(4)]
    dummy_pixels_high: u8,

    /// HSYNC falling edge delay 2 MSB (bits 3:2)
    #[bits(2)]
    hsync_falling_edge_delay: u8,

    /// HSYNC rising edge delay 2 MSB (bits 1:0)
    #[bits(2)]
    hsync_raising_edge_delay: u8,
}

/// EXHCL - Dummy Pixel Insert LSB (0x2B)
#[bitfield(u8, order=msb)]
#[derive(Copy, Clone)]
pub struct ExhcL {
    /// 8 LSB for dummy pixel insert in horizontal direction (bits 7:0)
    #[bits(8)]
    dummy_pixels_low: u8,
}


/// ADVFL - LSB of Insert Dummy Lines in Vertical Direction (0x2D)
#[bitfield(u8, order=msb)]
#[derive(Copy, Clone)]
pub struct AdvfL {
    /// LSB of Insert Dummy Lines in Vertical Direction (1 bit equals 1 line) (bits 7:0)
    #[bits(8)]
    dummy_lines_low: u8,
}

/// ADVFH - MSB of Insert Dummy Rows (0x2E)
/// MSB of dummy row count
#[bitfield(u8, order=msb)]
#[derive(Copy, Clone)]
pub struct AdvfH {
    /// MSB of Insert Dummy Lines in Vertical Direction (bits 7:0)
    #[bits(8)]
    dummy_lines_high: u8,
}

/// YAVE - Y/G Channel Average Value (0x2F)
/// Y/G channel average
#[bitfield(u8, order=msb)]
#[derive(Copy, Clone)]
pub struct YAve {
    /// Y/G average (bits 7:0)
    #[bits(8)]
    yg_average: u8,
}

/// HSYST - HSYNC Rising Edge Delay (0x30)
#[bitfield(u8, order=msb)]
#[derive(Copy, Clone)]
pub struct HSySt {
    /// HSYNC Raising Edge Delay, low 8 bits (bits 7:0)
    #[bits(8)]
    hsync_raising_edge_delay_low: u8,
}

/// HSYEN - HSYNC Falling Edge Delay (0x31)
#[bitfield(u8, order=msb)]
#[derive(Copy, Clone)]
pub struct HSyEn {
    /// HSYNC Falling Edge Delay, low 8 bits (bits 7:0)
    #[bits(8)]
    hsync_falling_edge_delay_low: u8,
}

/// HREF - HREF Control (0x32)
/// HREF edge offset control
#[bitfield(u8, order=msb)]
#[derive(Copy, Clone)]
pub struct HRef {
    /// HREF edge offset to data output (bits 7:6)
    #[bits(2)]
    edge_offset: u8,

    /// HREF end 3 LSB (high 8 MSB at register [HStop] (bits 5:3)
    #[bits(3)]
    hstop_low: u8,

    /// HREF start 3 LSB (high 8 MSB at register [HStart] (bits 2:0)
    #[bits(3)]
    hstart_low: u8,
}

/// CHLF - Array Current Control (0x33)
#[bitfield(u8, order=msb)]
#[derive(Copy, Clone)]
pub struct Chlf {
    /// Reserved (bits 7:0)
    reserved: u8,
}

/// ARBLM - Array Reference Control (0x34)
#[bitfield(u8, order=msb)]
#[derive(Copy, Clone)]
pub struct Arblm {
    /// Reserved (bits 7:0)
    reserved: u8,
}

/// ADC - ADC Control (0x37)
#[bitfield(u8, order=msb)]
#[derive(Copy, Clone)]
pub struct Adc {
    /// Reserved (bits 7:0)
    #[bits(8)]
    reserved: u8,
}

/// ACOM - ADC and Analog Common Mode Control (0x38)
/// ADC common mode control
#[bitfield(u8, order=msb)]
#[derive(Copy, Clone)]
pub struct ACom {
    /// Reserved (bits 7:0)
    #[bits(8)]
    reserved: u8,
}

/// OFON - ADC Offset Control (0x39)
/// ADC offset control
#[bitfield(u8, order=msb)]
#[derive(Copy, Clone)]
pub struct Ofon {
    /// Reserved (bits 7:0)
    #[bits(8)]
    reserved: u8,
}

/// TSLB - Line Buffer Test Option (0x3A)
/// Sequence control and UV output
#[bitfield(u8, order=msb)]
#[derive(Copy, Clone)]
pub struct Tslb {
    /// Reserved (bits 7:6)
    #[bits(2)]
    reserved0: u8,

    /// Negative image enable (bit 5)
    negative_image: bool,

    /// UV output value (bit 4)
    /// Use fixed UV value set in registers [ManU] and [ManV] as UV output instead of chip output.
    fixed_uv_output: bool,

    /// Output Sequence (bit 3)
    /// Use with register [Com13::uv_swap]
    ///
    /// | Tslb::luma_chroma_order | Com13::uv_swap | order   |
    /// |-------------------------|----------------|---------|
    /// | LumaChroma              | false          | Y U Y V |
    /// | LumaChroma              | true           | Y V Y U |
    /// | ChromaLuma              | false          | U Y V Y |
    /// | ChromaLuma              | true           | V Y U Y |
    #[bits(1)]
    luma_chroma_order: LumaChromaOrder,

    /// Reserved (bits 2:1)
    #[bits(2)]
    reserved1: u8,

    /// Auto output window (bit 0)
    ///
    /// false: Sensor DOES NOT automatically set window after resolution change.
    ///        The companion backend processor can adjust the output window immediately
    ///        after changing the resolution
    /// true:  Sensor automatically sets output window when resolution changes.
    ///        After resolution changes, the companion backend processor must adjust
    ///        the output window after the next VSYNC pulse.
    auto_output_window: bool,
}

/// COM11 - Common Control 11 (0x3B)
/// Night mode, banding filter, D56_auto
#[bitfield(u8, order=msb)]
#[derive(Copy, Clone)]
pub struct Com11 {
    /// Night mode enable (bit 7)
    /// If true, the frame rate is reduced automatically while the minimum
    /// frame rate is limited by [Self::night_mode_min_frame_rate].
    /// Also, ADVFH and ADVFL will be automatically updated.
    night_mode: bool,

    /// Minimum frame rate of night mode (bits 6:5)
    /// `night_mode_frame_rate = normal_frame_rate / pow(2, [Self::night_mode_min_frame_rate])`
    #[bits(2)]
    night_mode_min_frame_rate: MinFrameRate,

    /// 50/60 Hz auto detection (bit 4)
    d56_auto: bool,

    /// Banding filter value select (bit 3)
    /// Effective only when [Self::d56_auto] == false
    #[bits(1)]
    banding_filter: BandingFilter,

    /// Reserved (bit 2)
    #[bits(1)]
    reserved0: u8,

    /// Exposure timing can be less than limit of banding filter when light is too strong (bit 1)
    #[bits(1)]
    can_reduce_exposure: bool,

    /// Reserved (bit 0)
    #[bits(1)]
    reserved1: u8,
}

/// COM12 - Common Control 12 (0x3C)
#[bitfield(u8, order=msb)]
#[derive(Copy, Clone)]
pub struct Com12 {
    /// HREF option (bit 7)
    #[bits(1)]
    href_option: HRefOption,

    /// Reserved (bits 6:0)
    #[bits(7)]
    reserved0: u8,
}

/// COM13 - Common Control 13 (0x3D)
/// Gamma enable, UV saturation level, and UV swap
#[bitfield(u8, order=msb)]
#[derive(Copy, Clone)]
pub struct Com13 {
    /// Gamma enable (bit 7)
    gamma_enable: bool,

    /// UV saturation level auto adjust (bit 6)
    /// Result is saved in [SatCtr::result]
    uv_saturation_auto: bool,

    /// Reserved (bits 5:1)
    #[bits(5)]
    reserved0: u8,

    /// UV swap (bit 0)
    /// Use together with [Tslb::luma_chroma_order] to define color components
    /// order.
    uv_swap: bool,
}

/// COM14 - Common Control 14 (0x3E)
/// DCW and scaling PCLK
#[bitfield(u8, order=msb)]
#[derive(Copy, Clone)]
pub struct Com14 {
    /// Reserved (bits 7:5)
    #[bits(3)]
    reserved0: u8,

    /// DCW and scaling PCLK enable (bit 4)
    /// false: Normal PCLK
    /// true:  DCW and scaling PCLK, controlled by register [Com14::pclk_div_raw]
    ///        and [ScalingPClkDiv::pclk_div_raw]
    dcw_scaling: bool,

    /// Manual scaling enable (bit 3)
    /// Manual scaling enable for pre-defined resolution modes such as CIF, QCIF, and QVGA.
    /// false: Scaling parameter cannot be adjusted manually
    /// true:  Scaling parameter can be adjusted manually
    manual_scaling: bool,

    /// PCLK divider control (bits 2:0)
    /// Only when [Com14::dcw_scaling] is `true`.
    #[bits(3)]
    pclk_div_raw: u8,
}

impl Com14 {
    pub fn pclk_div(&self) -> Option<PClkDivider> {
        PClkDivider::from_bits(self.pclk_div_raw())
    }

    pub fn set_pclk_div(&mut self, div: PClkDivider) {
        self.set_pclk_div_raw(div.into_bits());
    }
}

impl Com14Builder {
    pub fn with_pclk_div(self, div: PClkDivider) -> Self {
        self.with_pclk_div_raw(div.into_bits())
    }
}

/// EDGE - Edge Enhancement Adjustment (0x3F)
#[bitfield(u8, order=msb)]
#[derive(Copy, Clone)]
pub struct Edge {
    /// Reserved (bits 7:5)
    #[bits(3)]
    reserved0: u8,

    /// Edge enhancement factor (bits 4:0)
    #[bits(5)]
    edge_enh_factor: u8,
}

/// COM15 - Common Control 15 (0x40)
#[bitfield(u8, order=msb)]
#[derive(Copy, Clone)]
pub struct Com15 {
    /// Output range control (bits 7:6)
    #[bits(2)]
    output_range_raw: u8,

    /// RGB 555/565 option (bits 5:4)
    /// Must set [Com7::rgb] to `true` and [Com7::raw_rgb] to `false`.
    #[bits(2)]
    rgb_format_raw: u8,

    /// Reserved (bits 3:0)
    #[bits(4)]
    reserved0: u8,
}

impl Com15 {
    pub fn output_range(&self) -> OutputRange {
        OutputRange::from_bits(self.output_range_raw())
    }

    pub fn set_output_range(&mut self, range: OutputRange) {
        self.set_output_range_raw(range.into_bits());
    }

    pub fn rgb_format(&self) -> RgbFormat {
        RgbFormat::from_bits(self.rgb_format_raw())
    }

    pub fn set_rgb_format(&mut self, format: RgbFormat) {
        self.set_rgb_format_raw(format.into_bits());
    }
}

impl Com15Builder {
    pub fn with_output_range(self, div: OutputRange) -> Self {
        self.with_output_range_raw(div.into_bits())
    }

    pub fn with_rgb_format(self, div: RgbFormat) -> Self {
        self.with_rgb_format_raw(div.into_bits())
    }
}

/// COM16 - Common Control 16 (0x41)
#[bitfield(u8, order=msb)]
#[derive(Copy, Clone)]
pub struct Com16 {
    /// Reserved (bits 7:6)
    #[bits(2)]
    reserved0: u8,

    /// Enable edge enhancement threshold auto-adjustment for YUV output (bit 5)
    /// (result is saved in register [Edge::edge_enh_factor] and range
    /// is controlled by registers [Reg75::edge_enh_lower_limit]
    /// and [Reg76::edge_enh_higher_limit].
    edge_enh_threshold_adj: bool,

    /// De-noise threshold auto-adjustment (bit 4)
    /// Result is saved in register [DnsTh] and range is controlled by
    /// register [Reg77].
    denoise_threshold_auto_adj: bool,

    /// AWB gain enable (bit 3)
    awb_gain_enable: bool,

    /// Reserved (bit 2)
    #[bits(1)]
    reserved1: u8,

    /// Color matrix coefficient double option (bit 1)
    /// false: Original matrix
    /// true:  Double of original matrix
    double_color_matrix_coeffs: bool,

    /// Reserved (bit 0)
    #[bits(1)]
    reserved2: u8,
}

/// COM17 - Common Control 17 (0x42)
#[bitfield(u8, order=msb)]
#[derive(Copy, Clone)]
pub struct Com17 {
    /// AEC window (bits 7:6)
    /// Must be the same value as [Com4::average_option]
    #[bits(2)]
    aec_window: WindowSize,

    /// Reserved (bits 5:4)
    #[bits(2)]
    reserved0: u8,

    /// DSP color bar enable (bit 3)
    dsp_color_bar: bool,

    /// Reserved (bits 2:0)
    #[bits(3)]
    reserved1: u8,
}

/// AWBC1-6 - Auto White Balance Control (0x43-0x48)
/// AWB control registers
#[bitfield(u8, order=msb)]
#[derive(Copy, Clone)]
pub struct AwbC {
    /// Reserved (bits 7:0)
    #[bits(8)]
    reserved: u8,
}

/// REG4B - UV Average Enable (0x4B)
/// UV average enable
#[bitfield(u8, order=msb)]
#[derive(Copy, Clone)]
pub struct Reg4b {
    /// Reserved (bits 7:1)
    #[bits(7)]
    reserved: u8,

    /// UV average enable (bit 0)
    uv_average: bool,
}

/// DNSTH - De-noise Strength (0x4C)
#[bitfield(u8, order=msb)]
#[derive(Copy, Clone)]
pub struct DnSth {
    /// De-noise strength (bits 7:0)
    #[bits(8)]
    denoise_strength: u8,
}

/// MTX1-6 - Matrix Coefficient (0x4F-0x54)
/// Color matrix coefficients
#[bitfield(u8, order=msb)]
#[derive(Copy, Clone)]
pub struct Mtx {
    /// Matrix coefficient (bits 7:0)
    #[bits(8)]
    matrix_coeff: u8,
}

/// BRIGHT - Brightness Control (0x55)
#[bitfield(u8, order=msb)]
#[derive(Copy, Clone)]
pub struct Bright {
    /// Brightness value (bits 7:0)
    #[bits(8)]
    brightness: u8,
}

/// CONTRAS - Contrast Control (0x56)
#[bitfield(u8, order=msb)]
#[derive(Copy, Clone)]
pub struct Contras {
    /// Contrast value (bits 7:0)
    #[bits(8)]
    contrast: u8,
}

/// CONTRAS_CENTER - Contrast Center (0x57)
#[bitfield(u8, order=msb)]
#[derive(Copy, Clone)]
pub struct ContrasCenter {
    /// Contrast center (bits 7:0)
    #[bits(8)]
    contrast_center: u8,
}

/// MTXS - Matrix Coefficient Sign for Coefficient 5 to 0 (0x58)
#[bitfield(u8, order=msb)]
#[derive(Copy, Clone)]
pub struct MtxS {
    /// Auto contrast center enable (bit 7)
    /// false: Disable, center is set by register [ContrasCenter]
    /// true:  Enable, register [ContrasCenter] is updated automatically
    auto_contrast_center: bool,

    /// Reserved (bit 6)
    #[bits(1)]
    reserved: u8,

    /// Matrix coefficient sign for coefficients (bits 5:0)
    /// 0: plus, 1: minus
    #[bits(6)]
    matrix_sign: u8,
}

/// LCC1 - Lens Correction Option 1 (0x62)
#[bitfield(u8, order=msb)]
#[derive(Copy, Clone)]
pub struct Lcc1 {
    /// X Coordinate of Lens Correction Center Relative to Array Center (bits 7:0)
    #[bits(8)]
    x_coord: u8,
}

/// LCC2 - Lens Correction Option 2 (0x63)
#[bitfield(u8, order=msb)]
#[derive(Copy, Clone)]
pub struct Lcc2 {
    /// Y Coordinate of Lens Correction Center Relative to Array Center (bits 7:0)
    #[bits(8)]
    y_coord: u8,
}

/// LCC3 - Lens Correction Option 3 (0x64)
#[bitfield(u8, order=msb)]
#[derive(Copy, Clone)]
pub struct Lcc3 {
    /// G Channel Compensation Coefficient when [Lcc5::lens_correction_control] is set.
    /// R, G, and B Channel Compensation Coefficient when [Lcc5::lens_correction_control] is unset.
    #[bits(8)]
    compensation_coeff: u8,
}


/// LCC4 - Lens Correction Option 4 (0x65)
#[bitfield(u8, order=msb)]
#[derive(Copy, Clone)]
pub struct Lcc4 {
    /// Radius of the circular section where no compensation applies
    #[bits(8)]
    no_componsation_radius: u8,
}

/// LCC5 - Lens Correction Option 5 (0x66)
#[bitfield(u8)]
#[derive(Copy, Clone)]
pub struct Lcc5 {
    /// Reserved (bits 7:3)
    #[bits(5)]
    reserved0: u8,

    /// Lens correction control select (bit 2)
    #[bits(1)]
    lens_componsation_control: LensCompensationControl,

    /// Reserved (bit 1)
    #[bits(1)]
    reserved1: u8,

    /// Lens correction enable
    lens_correction: bool,
}

/// LCC1-7 - Lens Correction Control (0x62-0x66, 0x94-0x95)
/// Lens correction option
#[bitfield(u8, order=msb)]
#[derive(Copy, Clone)]
pub struct Lcc {
    /// Lens correction value (bits 7:0)
    #[bits(8)]
    lcc_value: u8,
}

/// MANU - Manual U Value (0x67)
/// Manual U value for output
#[bitfield(u8, order=msb)]
#[derive(Copy, Clone)]
pub struct ManU {
    /// Manual U value (bits 7:0)
    /// Effective only when [Tslb::fixed_uv_output] is set.
    #[bits(8)]
    u_value: u8,
}

/// MANV - Manual V Value (0x68)
/// Manual V value for output
#[bitfield(u8, order=msb)]
#[derive(Copy, Clone)]
pub struct ManV {
    /// Manual V value (bits 7:0)
    /// Effective only when [Tslb::fixed_uv_output] is set.
    #[bits(8)]
    v_value: u8,
}

/// GFIX - Fixed Gain Control (0x69)
/// Fixed gain value
#[bitfield(u8, order=msb)]
#[derive(Copy, Clone)]
pub struct GFix {
    /// Fix gain for Gr channel (bits 7:6)
    #[bits(2)]
    fix_gain_gr: FixGain,

    /// Fix gain for Gb channel (bits 5:4)
    #[bits(2)]
    fix_gain_gb: FixGain,

    /// Fix gain for R channel (bits 3:2)
    #[bits(2)]
    fix_gain_r: FixGain,

    /// Fix gain for B channel (bits 1:0)
    #[bits(2)]
    fix_gain_b: FixGain,
}

/// GGAIN - G Channel AWB Gain (0x6A)
#[bitfield(u8, order=msb)]
#[derive(Copy, Clone)]
pub struct GGain {
    /// G Channel AWB Gain (bits 7:0)
    #[bits(8)]
    g_gain: u8,
}

/// DBLV - PLL Control (0x6B)
/// PLL and clock control
#[bitfield(u8, order=msb)]
#[derive(Copy, Clone)]
pub struct Dblv {
    /// PLL control (bits 7:6)
    #[bits(2)]
    pll_control: PllControl,

    /// Reserved (bit 5)
    #[bits(1)]
    reserved0: u8,

    /// Regulator control, bypass internal regulator (bit 4)
    bypass_internal_regulator: bool,

    /// Reserved (bits 3:0)
    #[bits(4)]
    reserved1: u8,
}

/// AWBCTR0-3 - AWB Control (0x6C-0x6F)
/// Advanced AWB control
#[bitfield(u8, order=msb)]
#[derive(Copy, Clone)]
pub struct Awbctr {
    /// AWB control value (bits 7:0)
    awb_ctrl: u8,
}

/// SCALING_XSC - Horizontal Scaling Coefficient (0x70)
#[bitfield(u8, order=msb)]
#[derive(Copy, Clone)]
pub struct ScalingXSc {
    /// Test_pattern low bit (bit 7)
    /// High bit at [ScalingYSc::test_pattern_high]
    #[bits(1)]
    test_pattern_low: u8,

    /// Horizontal scale factor (bits 6:0)
    #[bits(7)]
    horizontal_scale_factor: u8,
}

/// SCALING_YSC - Vertical Scaling Coefficient (0x71)
#[bitfield(u8, order=msb)]
#[derive(Copy, Clone)]
pub struct ScalingYSc {
    /// Test_pattern high bit (bit 7)
    /// Low bit at [ScalingXSc::test_pattern_low]
    #[bits(1)]
    test_pattern_high: u8,

    /// Vertical scale factor (bits 6:0)
    #[bits(7)]
    vertical_scale_factor: u8,
}

/// SCALING_DCWCTR - DCW Control (0x72)
/// Down-sampling control
#[bitfield(u8, order=msb)]
#[derive(Copy, Clone)]
pub struct ScalingDcwCtr {
    /// Vertical average calculation option (bit 7)
    #[bits(1)]
    vertical_average_calc: Sampling,

    /// Vertical down sampling option (bit 6)
    #[bits(1)]
    vertical_downsampling_option: Sampling,

    /// Vertical down-sampling rate (bits 5:4)
    #[bits(2)]
    vertical_downsampling_rate: DownsamplingRate,

    /// Horizontal average calculation option (bit 3)
    #[bits(1)]
    horizontal_average_calc: Sampling,

    /// Horizontal down sampling option (bit 2)
    #[bits(1)]
    horizontal_downsampling_option: Sampling,

    /// Horizontal down-sampling rate (bits 1:0)
    #[bits(2)]
    horizontal_downsampling_rate: DownsamplingRate,
}

/// SCALING_PCLK_DIV - Pixel Clock Divider (0x73)
#[bitfield(u8, order=msb)]
#[derive(Copy, Clone)]
pub struct ScalingPClkDiv {
    /// Reserved (bits 7:4)
    #[bits(4)]
    reserved0: u8,

    /// Bypass clock divider for DSP scale control (bit 3)
    bypass_clock_divider: bool,

    /// Clock divider control for DSP scale control (bits 2:0)
    /// Valid only when [Com14::manual_scaling] is `true`.
    /// Should change with [Com14::pclk_div_raw].
    #[bits(3)]
    pclk_div_raw: u8,
}

impl ScalingPClkDiv {
    pub fn pclk_div(&self) -> Option<PClkDivider> {
        PClkDivider::from_bits(self.pclk_div_raw())
    }

    pub fn set_pclk_div(&mut self, div: PClkDivider) {
        self.set_pclk_div_raw(div.into_bits());
    }
}

impl ScalingPClkDivBuilder {
    pub fn with_pclk_div(self, div: PClkDivider) -> Self {
        self.with_pclk_div_raw(div.into_bits())
    }
}

/// REG74 - Register 74 (0x74)
/// Digital gain control
#[bitfield(u8)]
#[derive(Copy, Clone)]
pub struct Reg74 {
    /// Reserved (bits 7:5)
    #[bits(3)]
    reserved0: u8,

    /// DG_Manu (bit 4)
    /// Digital gain control
    #[bits(1)]
    digital_gain_control: DigitalGainControl,

    /// Reserved (bits 3:2)
    #[bits(2)]
    reserved1: u8,

    /// Digital gain manual control (bits 1:0)
    #[bits(2)]
    digital_gain: DigitalGain,
}

/// REG75 - Register 75 (0x75)
#[bitfield(u8, order=msb)]
#[derive(Copy, Clone)]
pub struct Reg75 {
    /// Reserved (bits 7:5)
    #[bits(3)]
    reserved0: u8,

    /// Edge enhancement lower limit (bits 4:0)
    #[bits(5)]
    edge_enh_lower_limit: u8,
}

/// REG76 - Register 76 (0x76)
#[bitfield(u8, order=msb)]
#[derive(Copy, Clone)]
pub struct Reg76 {
    /// Black pixel correction enable (bit 7)
    black_pixel_correction_enable: bool,

    /// White pixel correction enable (bit 6)
    white_pixel_correction_enable: bool,

    /// Reserved (bit 5)
    #[bits(1)]
    reserved0: u8,

    /// Edge enhancement higher limit (bits 4:0)
    #[bits(5)]
    edge_enh_higher_limit: u8,
}

/// REG77 - Register 77 (0x77)
#[bitfield(u8)]
#[derive(Copy, Clone)]
pub struct Reg77 {
    /// De-noise offset (bits 7:0)
    denoise_offset: u8,
}

/// SLOP - Gamma Curve Highest Segment Slope (0x7A)
#[bitfield(u8)]
#[derive(Copy, Clone)]
pub struct Slop {
    /// Gamma Curve Highest Segment Slope (bits 7:0)
    /// Calculated as follows: `(0x100 - [Gam15]) * 4/3)`
    gamma_slope: u8,
}

/// GAM1-15 - Gamma Curve (0x7B-0x89)
/// | addr | reg   | segment | desc                                                       |
/// |-------------------------------------------------------------------------------------|
/// |   7B | GAM1  |      04 | Gamma Curve 1st Segment Input End Point 0x04 Output Value  |
/// |   7C | GAM2  |      07 | Gamma Curve 2nd Segment Input End Point 0x08 Output Value  |
/// |   7D | GAM3  |      10 | Gamma Curve 3rd Segment Input End Point 0x10 Output Value  |
/// |   7E | GAM4  |      28 | Gamma Curve 4th Segment Input End Point 0x20 Output Value  |
/// |   7F | GAM5  |      36 | Gamma Curve 5th Segment Input End Point 0x28 Output Value  |
/// |   80 | GAM6  |      44 | Gamma Curve 6th Segment Input End Point 0x30 Output Value  |
/// |   81 | GAM7  |      52 | Gamma Curve 7th Segment Input End Point 0x38 Output Value  |
/// |   82 | GAM8  |      60 | Gamma Curve 8th Segment Input End Point 0x40 Output Value  |
/// |   83 | GAM9  |      6C | Gamma Curve 9th Segment Input End Point 0x48 Output Value  |
/// |   84 | GAM10 |      78 | Gamma Curve 10th Segment Input End Point 0x50 Output Value |
/// |   85 | GAM11 |      8C | Gamma Curve 11th Segment Input End Point 0x60 Output Value |
/// |   86 | GAM12 |      9E | Gamma Curve 12th Segment Input End Point 0x70 Output Value |
/// |   87 | GAM13 |      BB | Gamma Curve 13th Segment Input End Point 0x90 Output Value |
/// |   88 | GAM14 |      D2 | Gamma Curve 14th Segment Input End Point 0xB0 Output Value |
/// |   89 | GAM15 |      E5 | Gamma Curve 15th Segment Input End Point 0xD0 Output Value |
#[bitfield(u8)]
#[derive(Copy, Clone)]
pub struct Gam {
    /// Gamma value (bits 7:0)
    gamma_value: u8,
}

/// RGB444 - RGB444 Format Control (0x8C)
/// RGB444 Enable and word swap
#[bitfield(u8, order=msb)]
#[derive(Copy, Clone)]
pub struct Rgb444 {
    /// Reserved (bits 7:2)
    #[bits(6)]
    reserved0: u8,

    /// RGB444 enable (bit 1)
    /// Effective only when `[Com15::rgb_format_raw] & 1 != 0`
    rgb444_enable: bool,

    /// RGB444 word format (bit 0)
    #[bits(1)]
    rgb444_word_format: Rgb444WordFormat,
}

/// DM_LNL - Dummy Line Low (0x92)
#[bitfield(u8)]
#[derive(Copy, Clone)]
pub struct DmLnL {
    /// Dummy line low 8 bits (bits 7:0)
    /// High bits at [DmLnH::dummy_line_high]
    dummy_line_low: u8,
}

/// DM_LNH - Dummy Line High (0x93)
#[bitfield(u8)]
#[derive(Copy, Clone)]
pub struct DmLnH {
    /// Dummy line high 8 bits (bits 7:0)
    /// Low bits at [DmLnL::dummy_line_low]
    dummy_line_high: u8,
}

/// BD50ST - 50Hz Banding Filter Value (0x9D)
#[bitfield(u8)]
#[derive(Copy, Clone)]
pub struct Bd50St {
    /// 50Hz banding value (bits 7:0)
    /// Effective only when [Com8::banding_filter] is set
    /// and [Com11::banding_filter] is [BandingFilter::Filter50Hz]
    band_50hz: u8,
}

/// BD60ST - 60Hz Banding Filter Value (0x9E)
#[bitfield(u8, order=msb)]
#[derive(Copy, Clone)]
pub struct Bd60St {
    /// 60Hz banding value (bits 7:0)
    /// Effective only when [Com8::banding_filter] is set
    /// and [Com11::banding_filter] is [BandingFilter::Filter60Hz]
    #[bits(8)]
    band_60hz: u8,
}

/// HAECC1-6 - Histogram AEC/AGC Control (0x9F-0xA0, 0xA6-0xA9)
#[bitfield(u8, order=msb)]
#[derive(Copy, Clone)]
pub struct HAecCX {
    /// Histogram AEC value (bits 7:0)
    #[bits(8)]
    hist_aec: u8,
}

/// HAECC7 - Histogram AEC/AGC Control 7 (0xAA)
#[bitfield(u8, order=msb)]
#[derive(Copy, Clone)]
pub struct HAecC7 {
    /// AEC algorithm selection (bit 7)
    #[bits(1)]
    aec_algorithm: AecAlgorithm,

    /// Reserved (bits 6:0)
    #[bits(7)]
    reserved0: u8,
}

/// SCALING_PCLK_DELAY - Scaling Output Delay (0xA2)
/// Scaling pixel clock delay
#[bitfield(u8, order=msb)]
#[derive(Copy, Clone)]
pub struct ScalingPclkDelay {
    /// Reserved (bit 7)
    #[bits(1)]
    reserved0: u8,

    /// Scaling output delay (bits 6:0)
    #[bits(7)]
    pclk_delay: u8,
}

/// NT_CTRL - Night Mode Control (0xA4)
#[bitfield(u8, order=msb)]
#[derive(Copy, Clone)]
pub struct NtCtrl {
    /// Reserved (bits 7:4)
    #[bits(4)]
    reserved0: u8,

    /// Auto frame rate adjustment control (bit 3)
    #[bits(1)]
    auto_frame_rate_adj: FrameRateAdjustment,

    /// Reserved (bit 2)
    #[bits(1)]
    reserved1: u8,

    /// Auto frame rate adjustment switch point (bits 1:0)
    #[bits(2)]
    auto_frame_rate_adj_switch_point_raw: u8,
}

impl NtCtrl {
    pub fn auto_frame_rate_adj_switch_point(&self) -> Option<FrameRateAdjustmentPoint> {
        FrameRateAdjustmentPoint::from_bits(self.auto_frame_rate_adj_switch_point_raw())
    }

    pub fn set_auto_frame_rate_adj_switch_point(&mut self, point: FrameRateAdjustmentPoint) {
        self.set_auto_frame_rate_adj_switch_point_raw(point.into_bits());
    }
}

impl NtCtrlBuilder {
    pub fn with_auto_frame_rate_adj_switch_point(self, point: FrameRateAdjustmentPoint) -> Self {
        self.with_auto_frame_rate_adj_switch_point_raw(point.into_bits())
    }
}

/// BD50MAX - 50Hz Banding Maximum (0xA5)
/// BD60MAX - 60Hz Banding Maximum (0xAB)
#[bitfield(u8, order=msb)]
#[derive(Copy, Clone)]
pub struct BdMax {
    /// Banding step limit (bits 7:0)
    #[bits(8)]
    banding_step_limit: u8,
}

/// STR_OPT - Strobe Control (0xAC)
/// Strobe/flash control options
#[bitfield(u8, order=msb)]
#[derive(Copy, Clone)]
pub struct StrOpt {
    /// Strobe enable (bit 7)
    strobe: bool,

    /// R/G/B gain controlled by [StrX] for LED output frame (bit 6)
    gain_control: bool,

    /// Xenon mode option (bits 5:4)
    #[bits(2)]
    xenon_mode: XenonMode,

    /// Reserved (bits 3:2)
    #[bits(2)]
    reserved0: u8,

    /// Mode select (bits 1:0)
    #[bits(2)]
    strobe_mode_raw: u8,
}

impl StrOpt {
    pub fn strobe_mode(&self) -> StrobeMode {
        StrobeMode::from_bits(self.strobe_mode_raw())
    }

    pub fn set_strobe_mode(&mut self, mode: StrobeMode) {
        self.set_strobe_mode_raw(mode.into_bits());
    }
}

impl StrOptBuilder {
    pub fn with_strobe_mode(self, mode: StrobeMode) -> Self {
        self.with_strobe_mode_raw(mode.into_bits())
    }
}

/// STR_R - R Gain for LED Output Frame (0xAD)
/// STR_G - G Gain for LED Output Frame (0xAE)
/// STR_B - B Gain for LED Output Frame (0xAF)
#[bitfield(u8, order=msb)]
#[derive(Copy, Clone)]
pub struct StrX {
    /// Gain for LED Output Frame (bits 7:0)
    #[bits(8)]
    gain_led: u8,
}

/// ABLC1 - Automatic Black Level Calibration (0xB1)
#[bitfield(u8, order=msb)]
#[derive(Copy, Clone)]
pub struct Ablc1 {
    /// Reserved (bits 7:3)
    #[bits(5)]
    reserved0: u8,

    /// ABLC enable (bit 2)
    ablc_enable: bool,

    /// Reserved (bits 1:0)
    #[bits(2)]
    reserved1: u8,
}

/// THL_ST - ABLC Target (0xB3)
#[bitfield(u8, order=msb)]
#[derive(Copy, Clone)]
pub struct ThlSt {
    /// ABLC target (bits 7:0)
    #[bits(8)]
    ablc_target: u8,
}

/// THL_DLT - ABLC Stable Range (0xB5)
#[bitfield(u8, order=msb)]
#[derive(Copy, Clone)]
pub struct ThlDlt {
    /// ABLC stable range (bits 7:0)
    #[bits(8)]
    ablc_stable_range: u8,
}

/// AD_CHB - Blue Channel Black Level Compensation (0xBE)
/// AD_CHR - Red Channel Black Level Compensation (0xBF)
/// AD_CHGb - Gb Channel Black Level Compensation (0xC0)
/// AD_CHGr - Gr Channel Black Level Compensation (0xC1)
#[bitfield(u8, order=msb)]
#[derive(Copy, Clone)]
pub struct AdChX {
    /// Reserved (bit 7)
    #[bits(1)]
    reserved0: u8,

    /// Sign bit (bit 6)
    #[bits(1)]
    sign: bool,

    /// X channel black level compensation (bits 5:0)
    #[bits(6)]
    black_level_compensation: u8,
}

/// SATCTR - Saturation Control (0xC9)
#[bitfield(u8, order=msb)]
#[derive(Copy, Clone)]
pub struct SatCtr {
    /// UV saturation control min (bits 7:4)
    #[bits(4)]
    min: u8,

    /// UV saturation control result (bits 3:0)
    #[bits(4)]
    result: u8,
}

#[repr(u8)]
pub enum OutputDriveCapability {
    X1 = 0b00,
    X2 = 0b01,
    X3 = 0b10,
    X4 = 0b11,
}

impl OutputDriveCapability {
    pub const fn from_bits(bits: u8) -> Self {
        match bits {
            0b00 => Self::X1,
            0b01 => Self::X2,
            0b10 => Self::X3,
            0b11 => Self::X4,
            _ => unreachable!()
        }
    }

    pub const fn into_bits(self) -> u8 {
        self as u8
    }
}

#[repr(u8)]
pub enum TriState {
    On = 0b0,
    Off = 0b1,
}

impl TriState {
    pub const fn from_bits(bits: u8) -> Self {
        match bits {
            0b0 => Self::On,
            0b1 => Self::Off,
            _ => unreachable!()
        }
    }

    pub const fn into_bits(self) -> u8 {
        self as u8
    }
}

#[repr(u8)]
pub enum WindowSize {
    /// Full window
    FullWindow = 0b00,
    /// 1/2 window
    HalfWindow = 0b01,
    /// 1/4 window
    QuarterWindow = 0b10,
    /// Both 0b10 and 0b11 are marked as "1/4 window" in datasheet
    QuarterWindowToo = 0b11,
}

impl WindowSize {
    pub const fn from_bits(bits: u8) -> Self {
        match bits {
            0b00 => Self::FullWindow,
            0b01 => Self::HalfWindow,
            0b10 => Self::QuarterWindow,
            0b11 => Self::QuarterWindowToo,
            _ => unreachable!()
        }
    }

    pub const fn into_bits(self) -> u8 {
        self as u8
    }
}

#[repr(u8)]
pub enum ColorFormat {
    Yuv = 0b000,
    Rgb = 0b100,
    Bayer = 0b001,
    ProcessedBayer = 0b101,
}

impl ColorFormat {
    const fn from_bits(bits: u8) -> Self {
        match bits {
            0b000 => Self::Yuv,
            0b100 => Self::Rgb,
            0b001 => Self::Bayer,
            0b101 => Self::ProcessedBayer,
            _ => unreachable!()
        }
    }

    const fn into_bits(self) -> u8 {
        self as u8
    }
}

#[repr(u8)]
pub enum PredefinedResolution {
    QCif = 0b001,
    QVga = 0b010,
    Cif  = 0b100,
}

impl PredefinedResolution {
    const fn from_bits(bits: u8) -> Self {
        match bits {
            0b001 => Self::QCif,
            0b010 => Self::QVga,
            0b100 => Self::Cif,
            _ => unreachable!()
        }
    }

    const fn into_bits(self) -> u8 {
        self as u8
    }
}

#[repr(u8)]
pub enum PClkEdge {
    Falling = 0,
    Raising = 1,
}

impl PClkEdge {
    const fn from_bits(bits: u8) -> Self {
        match bits {
            0 => Self::Falling,
            1 => Self::Raising,
            _ => unreachable!()
        }
    }

    const fn into_bits(self) -> u8 {
        self as u8
    }
}

#[repr(u8)]
pub enum AdcRangeAdjustment {
    /// x1 range
    One = 0,
    /// x1.5 range
    OneAndAHalf = 1,
}

impl AdcRangeAdjustment {
    const fn from_bits(bits: u8) -> Self {
        match bits {
            0 => Self::One,
            1 => Self::OneAndAHalf,
            _ => unreachable!()
        }
    }

    const fn into_bits(self) -> u8 {
        self as u8
    }
}

/// ADC reference adjustment
#[repr(u8)]
pub enum AdcReferenceAdjustment {
    /// 0.8x
    ZeroDotEight = 0b000,
    /// 1x
    One = 0b100,
    /// 1.2x
    OneDotTwo = 0b111,
}

impl AdcReferenceAdjustment {
    const fn from_bits(bits: u8) -> Option<Self> {
        match bits {
            0b000 => Some(Self::ZeroDotEight),
            0b100 => Some(Self::One),
            0b111 => Some(Self::OneDotTwo),
            _ => None,
        }
    }

    const fn into_bits(self) -> u8 {
        self as u8
    }
}

#[repr(u8)]
pub enum BiasAdjustmentSign {
    /// Add bias
    Add = 0,
    /// Substract bias
    Substract = 1,
}

impl BiasAdjustmentSign {
    const fn from_bits(bits: u8) -> Self {
        match bits {
            0 => Self::Add,
            1 => Self::Substract,
            _ => unreachable!()
        }
    }

    const fn into_bits(self) -> u8 {
        self as u8
    }
}

/// Order of Luma and Chroma components in encoded color.
/// Note that U/V order depends on [Com13::uv_swap].
#[repr(u8)]
pub enum LumaChromaOrder {
    /// YUYV or YVYU
    LumaChroma = 0,
    /// UYVY or VYUY
    ChromaLuma = 1,
}

impl LumaChromaOrder {
    const fn from_bits(bits: u8) -> Self {
        match bits {
            0 => Self::LumaChroma,
            1 => Self::ChromaLuma,
            _ => unreachable!()
        }
    }

    const fn into_bits(self) -> u8 {
        self as u8
    }
}

/// Minimal frame rate
#[repr(u8)]
pub enum MinFrameRate {
    /// Same as normal mode frame rate
    Normal = 0b00,
    /// 1/2 of normal mode frame rate
    HalfOfNormal = 0b01,
    /// 1/4 of normal mode frame rate
    QuarterOfNormal = 0b10,
    /// 1/8 of normal mode frame rate
    EighthOfNormal = 0b11,
}

impl MinFrameRate {
    const fn from_bits(bits: u8) -> Self {
        match bits {
            0b00 => Self::Normal,
            0b01 => Self::HalfOfNormal,
            0b10 => Self::QuarterOfNormal,
            0b11 => Self::EighthOfNormal,
            _ => unreachable!()
        }
    }

    const fn into_bits(self) -> u8 {
        self as u8
    }
}

/// Banding filter
#[repr(u8)]
pub enum BandingFilter {
    /// Select [Bd60St] (0x9E) as Banding Filter Value
    Filter60Hz = 0,
    /// Select [Bd50St] (0x9D) as Banding Filter Value
    Filter50Hz = 1,
}

impl BandingFilter {
    const fn from_bits(bits: u8) -> Self {
        match bits {
            0 => Self::Filter60Hz,
            1 => Self::Filter50Hz,
            _ => unreachable!()
        }
    }

    const fn into_bits(self) -> u8 {
        self as u8
    }
}

/// HREF option
#[repr(u8)]
pub enum HRefOption {
    /// No HREF when VSYNC is low
    NoHRefWhenVSyncLow = 0,
    /// Always has HREF
    HRefAlways = 1,
}

impl HRefOption {
    const fn from_bits(bits: u8) -> Self {
        match bits {
            0 => Self::NoHRefWhenVSyncLow,
            1 => Self::HRefAlways,
            _ => unreachable!()
        }
    }

    const fn into_bits(self) -> u8 {
        self as u8
    }
}

/// PCLK divider
#[repr(u8)]
pub enum PClkDivider {
    DivBy1 = 0b000,
    DivBy2 = 0b001,
    DivBy4 = 0b010,
    DivBy8 = 0b011,
    DivBy16 = 0b100,
}

impl PClkDivider {
    const fn from_bits(bits: u8) -> Option<Self> {
        match bits {
            0b000 => Some(Self::DivBy1),
            0b001 => Some(Self::DivBy2),
            0b010 => Some(Self::DivBy4),
            0b011 => Some(Self::DivBy8),
            0b100 => Some(Self::DivBy16),
            _ => None
        }
    }

    const fn into_bits(self) -> u8 {
        self as u8
    }
}

/// Output data range
#[repr(u8)]
pub enum OutputRange {
    From10ToF0,
    From01ToFE,
    From00ToFF,
}

impl OutputRange {
    const fn from_bits(bits: u8) -> Self {
        match bits {
            0b00 | 0b01 => Self::From10ToF0,
            0b10 => Self::From01ToFE,
            0b11 => Self::From00ToFF,
            _ => unreachable!()
        }
    }

    const fn into_bits(self) -> u8 {
        match self {
            Self::From10ToF0 => 0b00,
            Self::From01ToFE => 0b10,
            Self::From00ToFF => 0b11,
        }
    }
}

/// RGB format
#[repr(u8)]
pub enum RgbFormat {
    /// Normal RGB output
    Normal,
    /// RGB 565, effective only when [Rgb444::rgb444_enable] is `false`
    Rgb565,
    /// RGB 555, effective only when [Rgb444::rgb444_enable] is `false`
    Rgb555,
}

impl RgbFormat {
    const fn from_bits(bits: u8) -> Self {
        match bits {
            0b00 | 0b10 => Self::Normal,
            0b01 => Self::Rgb565,
            0b11 => Self::Rgb555,
            _ => unreachable!()
        }
    }

    const fn into_bits(self) -> u8 {
        match self {
            Self::Normal => 0b00,
            Self::Rgb565 => 0b01,
            Self::Rgb555 => 0b11,
        }
    }
}

/// Lens componsation control
#[repr(u8)]
pub enum LensCompensationControl {
    // R, G, and B channel compensation coefficient is set by register [Lcc3]
    AllChannels = 0,
    // R, G, and B channel compensation coefficient is set by registers [Lcc6], [Lcc3], and [Lcc7], respectively
    PerChannel = 1,
}

impl LensCompensationControl {
    const fn from_bits(bits: u8) -> Self {
        match bits {
            0 => Self::AllChannels,
            1 => Self::PerChannel,
            _ => unreachable!()
        }
    }

    const fn into_bits(self) -> u8 {
        self as u8
    }
}

/// Fix Gain
#[repr(u8)]
pub enum FixGain {
    /// 1x
    X1    = 0b00,
    /// 1.25x
    X1_25 = 0b01,
    /// 1.5x
    X1_5  = 0b10,
    /// 1.75x
    X1_75 = 0b11,
}

impl FixGain {
    const fn from_bits(bits: u8) -> Self {
        match bits {
            0b00 => Self::X1,
            0b01 => Self::X1_25,
            0b10 => Self::X1_5,
            0b11 => Self::X1_75,
            _ => unreachable!()
        }
    }

    const fn into_bits(self) -> u8 {
        self as u8
    }
}

/// PLL control
#[repr(u8)]
pub enum PllControl {
    /// Bypass PLL
    BypassPll     = 0b00,
    /// Input clock x4
    InputClockX4  = 0b01,
    /// Input clock x6
    InputClockX6  = 0b10,
    /// Input clock x8
    InputClockX8  = 0b11,
}

impl PllControl {
    const fn from_bits(bits: u8) -> Self {
        match bits {
            0b00 => Self::BypassPll,
            0b01 => Self::InputClockX4,
            0b10 => Self::InputClockX6,
            0b11 => Self::InputClockX8,
            _ => unreachable!()
        }
    }

    const fn into_bits(self) -> u8 {
        self as u8
    }
}

/// Sampling calculation
#[repr(u8)]
pub enum Sampling {
    Truncation = 0,
    Rounding = 1,
}

impl Sampling {
    const fn from_bits(bits: u8) -> Self {
        match bits {
            0 => Self::Truncation,
            1 => Self::Rounding,
            _ => unreachable!()
        }
    }

    const fn into_bits(self) -> u8 {
        self as u8
    }
}

/// Average calculation
#[repr(u8)]
pub enum DownsamplingRate {
    NoDownsampling = 0b00,
    DownsampleBy2 = 0b01,
    DownsampleBy4 = 0b10,
    DownsampleBy8 = 0b11,
}

impl DownsamplingRate {
    const fn from_bits(bits: u8) -> Self {
        match bits {
            0b00 => Self::NoDownsampling,
            0b01 => Self::DownsampleBy2,
            0b10 => Self::DownsampleBy4,
            0b11 => Self::DownsampleBy8,
            _ => unreachable!()
        }
    }

    const fn into_bits(self) -> u8 {
        self as u8
    }
}

/// Digital gain control
#[repr(u8)]
pub enum DigitalGainControl {
    /// Digital gain control by [VRef::gain_high] and [Gain::gain_low]
    ByVRefGain = 0,
    /// Digital gain control by [Reg47]
    ByReg74 = 1,
}

impl DigitalGainControl {
    const fn from_bits(bits: u8) -> Self {
        match bits {
            0 => Self::ByVRefGain,
            1 => Self::ByReg74,
            _ => unreachable!()
        }
    }

    const fn into_bits(self) -> u8 {
        self as u8
    }
}

/// Digital gain
#[repr(u8)]
pub enum DigitalGain {
    /// Bypass
    Bypass = 0b00,
    /// 1x
    X1 = 0b01,
    /// 2x
    X2 = 0b10,
    /// 4x
    X4 = 0b11,
}

impl DigitalGain {
    const fn from_bits(bits: u8) -> Self {
        match bits {
            0b00 => Self::Bypass,
            0b01 => Self::X1,
            0b10 => Self::X2,
            0b11 => Self::X4,
            _ => unreachable!()
        }
    }

    const fn into_bits(self) -> u8 {
        self as u8
    }
}

/// RGB444 word format
#[repr(u8)]
pub enum Rgb444WordFormat {
    /// xR GB
    XrGb = 0,
    /// RG Bx
    RgBx = 1,
}

impl Rgb444WordFormat {
    const fn from_bits(bits: u8) -> Self {
        match bits {
            0 => Self::XrGb,
            1 => Self::RgBx,
            _ => unreachable!()
        }
    }

    const fn into_bits(self) -> u8 {
        self as u8
    }
}

/// Auto frame rate adjustment
#[repr(u8)]
pub enum FrameRateAdjustment {
    /// Double exposure time
    DoubleExposureTime = 0,
    /// Reduce frame rate by half
    ReduceFrameRateByHalf = 1,
}

impl FrameRateAdjustment {
    const fn from_bits(bits: u8) -> Self {
        match bits {
            0 => Self::DoubleExposureTime,
            1 => Self::ReduceFrameRateByHalf,
            _ => unreachable!()
        }
    }

    const fn into_bits(self) -> u8 {
        self as u8
    }
}

/// Frame rate adjustment switch point
#[repr(u8)]
pub enum FrameRateAdjustmentPoint {
    Gain2X = 0b00,
    Gain4X = 0b01,
    Gain8X = 0b10,
}

impl FrameRateAdjustmentPoint {
    const fn from_bits(bits: u8) -> Option<Self> {
        match bits {
            0b00 => Some(Self::Gain2X),
            0b01 => Some(Self::Gain4X),
            0b10 => Some(Self::Gain8X),
            _ => None
        }
    }

    const fn into_bits(self) -> u8 {
        self as u8
    }
}

/// AEC algorithm
#[repr(u8)]
pub enum AecAlgorithm {
    /// Average-based
    AverageBased = 0,
    /// Histogram-based
    HistogramBased = 1,
}

impl AecAlgorithm {
    const fn from_bits(bits: u8) -> Self {
        match bits {
            0 => Self::AverageBased,
            1 => Self::HistogramBased,
            _ => unreachable!()
        }
    }

    const fn into_bits(self) -> u8 {
        self as u8
    }
}

/// Xenon mode
#[repr(u8)]
pub enum XenonMode {
    OneRow = 0,
    TwoRows = 1,
    ThreeRows = 2,
    FourRows = 3,
}

impl XenonMode {
    const fn from_bits(bits: u8) -> Self {
        match bits {
            0 => Self::OneRow,
            1 => Self::TwoRows,
            2 => Self::ThreeRows,
            3 => Self::FourRows,
            _ => unreachable!()
        }
    }

    const fn into_bits(self) -> u8 {
        self as u8
    }
}

/// Strobe mode
#[repr(u8)]
pub enum StrobeMode {
    Xenon,
    Led1,
    Led2,
}

impl StrobeMode {
    const fn from_bits(bits: u8) -> Self {
        match bits {
            0b00 => Self::Xenon,
            0b01 => Self::Led1,
            0b10 | 0b11 => Self::Led2,
            _ => unreachable!()
        }
    }

    const fn into_bits(self) -> u8 {
        match self {
            Self::Xenon => 0b00,
            Self::Led1 => 0b01,
            Self::Led2 => 0b10,
        }
    }
}

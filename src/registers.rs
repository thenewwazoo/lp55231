//! I2C registers of the LP55231

/// ENABLE/ ENGINE CONTROL1
pub const CNTRL1: u8 = 0x00;
// TODO engine control modes
bitflags! {
    /// CNTRL1 register fields
    pub struct Cntrl1: u8 {
        /// Enables the chip
        const CHIP_EN = 1 << 6;
    }
}

/// ENGINE CONTROL2
///
/// Operation modes are defined in this register.
///
/// * Disabled: Engines can be configured to disabled mode each one separately.
/// * Load program: Writing to program memory is allowed only when the engine is in load program
/// operation mode and engine busy bit (reg 3A) is not set. Serial bus master should check the busy
/// bit before writing to program memory or allow at least 1ms delay after entering to load mode
/// before memory write, to ensure initalization. All the three engines are in hold while one or
/// more engines are in load program mode. PWM values are frozen, also. Program execution continues
/// when all the engines are out of load program mode. Load program mode resets the program counter
/// of the respective engine. Load program mode can be entered from the disabled mode only.
/// Entering load program mode from the run program mode is not allowed.
/// * Run Program: Run program mode executes the instructions stored in the program memory.
/// Execution register (ENG1_EXEC etc.) bits define how the program is executed (hold, step, free
/// run or execute once). Program start address can be programmed to the Program Counter (PC)
/// register. The Program Counter is reset to zero when the PC’s upper limit value is reached.
// TODO: "Entering load program mode from the run program mode is not allowed" constrain this using
// a type parameter to the Lp55231 struct
pub const CNTRL2: u8 = 0x01;

/// OUTPUT DIRECT/RATIOMETRIC MSB
///
/// A particular feature of the LP55231 is the ratiometric up/down dimming of the RGB-LEDs. In
/// other words, the LED driver PWM output varies in a ratiometric manner. By a ratiometric
/// approach the emitted color of an RGB–LED remains the same regardless of the initial magnitudes
/// of the R/G/B PWM outputs. For example, if the PWM output of the red LED output is doubled, the
/// output of green LED is doubled also.
pub const RATIO_MSB: u8 = 0x02;
/// OUTPUT DIRECT/RATIOMETRIC LSB
pub const RATIO_LSB: u8 = 0x03;
/// OUTPUT ON/OFF CONTROL MSB
pub const OUTPUT_ONOFF_MSB: u8 = 0x04;
/// OUTPUT ON/OFF CONTROL LSB
pub const OUTPUT_ONOFF_LSB: u8 = 0x05;

/// Per LED control channel for D1 - fader channel assig, log dimming enable, temperature compensation
///
/// This is the register used to assign the D1 output to the MASTER FADER group 1, 2, or 3, or none
/// of them. Also, this register sets the correction factor for the D1 output temperature
/// compensation and selects between linear and logarithmic PWM brightness adjustment. By using
/// logarithmic PWM-scale the visual effect looks like linear. When the logarithmic adjustment is
/// enabled, the chip handles internal PWM values with 12-bit resolution. This allows very
/// fine-grained PWM control at low PWM duty cycles.
pub const D_CTRL_BASE: u8 = 0x06;
bitflags! {
    /// D_CTRL_BASE register fields
    pub struct CtrlBase: u8 {
        /// No master fader set
        const MAPPING_NO_MASTER_FADER = 0b00 << 6;
        /// MASTER FADER1 controls output
        const MAPPING_MASTER_FADER_1 = 0b01 << 6;
        /// MASTER FADER1 controls output
        const MAPPING_MASTER_FADER_2 = 0b10 << 6;
        /// MASTER FADER1 controls output
        const MAPPING_MASTER_FADER_3 = 0b11 << 6;
        /// Logarithmic adjustment
        const LOG_EN = 1 << 5;
    }
}

impl CtrlBase {
    /// Reference temperature
    fn TEMP_COMP(v: u8) -> u8 {
        v & 0b11111
    }
}

/// Per LED control channel for D1 - fader channel assig, log dimming enable, temperature compensation
pub const D1_CTRL: u8 = 0x06;
/// Per LED control channel for D2 - fader channel assig, log dimming enable, temperature compensation
pub const D2_CTRL: u8 = 0x07;
/// Per LED control channel for D3 - fader channel assig, log dimming enable, temperature compensation
pub const D3_CTRL: u8 = 0x08;
/// Per LED control channel for D4 - fader channel assig, log dimming enable, temperature compensation
pub const D4_CTRL: u8 = 0x09;
/// Per LED control channel for D5 - fader channel assig, log dimming enable, temperature compensation
pub const D5_CTRL: u8 = 0x0a;
/// Per LED control channel for D6 - fader channel assig, log dimming enable, temperature compensation
pub const D6_CTRL: u8 = 0x0b;
/// Per LED control channel for D7 - fader channel assig, log dimming enable, temperature compensation
pub const D7_CTRL: u8 = 0x0c;
/// Per LED control channel for D8 - fader channel assig, log dimming enable, temperature compensation
pub const D8_CTRL: u8 = 0x0d;
/// Per LED control channel for D9 - fader channel assig, log dimming enable, temperature compensation
pub const D9_CTRL: u8 = 0x0e;

/// PWM control registers
///
/// This is the PWM duty cycle control for output. PWM register is effective during direct control
/// operation - direct PWM control is active after power up by default. Note: serial bus address
/// auto increment is not supported for register addresses from 16 to 1E.
pub const D_PWM_BASE: u8 = 0x16;

/// Direct D1 PWM control register
pub const D1_PWM: u8 = 0x16;
/// Direct D2 PWM control register
pub const D2_PWM: u8 = 0x17;
/// Direct D3 PWM control register
pub const D3_PWM: u8 = 0x18;
/// Direct D4 PWM control register
pub const D4_PWM: u8 = 0x19;
/// Direct D5 PWM control register
pub const D5_PWM: u8 = 0x1a;
/// Direct D6 PWM control register
pub const D6_PWM: u8 = 0x1b;
/// Direct D7 PWM control register
pub const D7_PWM: u8 = 0x1c;
/// Direct D8 PWM control register
pub const D8_PWM: u8 = 0x1d;
/// Direct D9 PWM control register
pub const D9_PWM: u8 = 0x1e;

/// Drive current register for D1
pub const D1_I_CTL: u8 = 0x26;
/// Drive current register for D2
pub const D2_I_CTL: u8 = 0x27;
/// Drive current register for D3
pub const D3_I_CTL: u8 = 0x28;
/// Drive current register for D4
pub const D4_I_CTL: u8 = 0x29;
/// Drive current register for D5
pub const D5_I_CTL: u8 = 0x2a;
/// Drive current register for D6
pub const D6_I_CTL: u8 = 0x2b;
/// Drive current register for D7
pub const D7_I_CTL: u8 = 0x2c;
/// Drive current register for D8
pub const D8_I_CTL: u8 = 0x2d;
/// Drive current register for D9
pub const D9_I_CTL: u8 = 0x2e;

/// MISC - This register contains miscellaneous control bits.
pub const MISC: u8 = 0x36;
bitflags! {
    /// MISC register fields
    pub struct Misc: u8 {
        /// Variable D source selection
        const VARIABLE_D_SEL = 1 << 7;
        /// The automatic increment feature of the serial bus address enables a quick memory write of successive registers within one transmission.
        const EN_AUTO_INCR = 1 << 6;
        /// Power save mode
        const POWERSAVE_EN = 1 << 5;
        /// Charge pump operation mode forced to bypass mode (1x).
        const CP_MODE_1x = 0b01 << 3;
        /// Charge pump operation mode forced to 1.5× mode; output voltage is boosted to 4.5 V
        const CP_MODE_1_5x = 0b10 << 3;
        /// Automatic mode selection
        const CP_MODE_AUTO = 0b11 << 3;
        /// Enables PWM powersave operation. Significant power savings can be achieved, for example, during ramp instruction.
        const PWM_PS_EN = 1 << 2;
        /// CLK_DET_EN
        const CLK_DET_EN = 1 << 1;
        /// INT_CLK_EN
        const INT_CLK_EN = 1 << 0;
    }
}

/// Program counter starting value for program execution engine 1
pub const PC1: u8 = 0x37;
/// Program counter starting value for program execution engine 2
pub const PC2: u8 = 0x38;
/// Program counter starting value for program execution engine 3
pub const PC3: u8 = 0x39;
/// STATUS/INTERRUPT
pub const STATUS_IRQ: u8 = 0x3A;
/// INT/GPO
pub const INT_GPIO: u8 = 0x3B;
/// These bits are used for storing a global 8-bit variable. Variable can be used to control program flow.
pub const GLOBAL_VAR: u8 = 0x3C;

/// RESET
pub const RESET: u8 = 0x3D;
bitflags! {
    /// RESET register fields
    pub struct Reset: u8 {
        /// Writing 11111111 into this register resets the LP55231. Internal registers are reset to the default values.
        const RESET_NOW = 0b11111111;
    }
}

/// TEMP ADC CONTROL
pub const TEMP_CTL: u8 = 0x3E;
/// These bits are used for storing an 8-bit temperature reading acquired from the internal temperature sensor.
pub const TEMP_READ: u8 = 0x3F;
/// These bits are used for storing an 8-bit temperature reading acquired from an external sensor, if such a sensor is used.
pub const TEMP_WRITE: u8 = 0x40;
/// LED TEST CONTROL
pub const TEST_CTL: u8 = 0x41;
/// LED TEST ADC
pub const TEST_ADC: u8 = 0x42;

/// These bits are used for Engine 1 local variable. Read-only register.
pub const ENGINE_A_VAR: u8 = 0x45;
/// These bits are used for Engine 2 local variable. Read-only register.
pub const ENGINE_B_VAR: u8 = 0x46;
/// These bits are used for Engine 3 local variable. Read-only register.
pub const ENGINE_C_VAR: u8 = 0x47;

/// An 8-bit register to control all the LED-drivers mapped to MASTER FADER1. Master fader allows
/// the user to control dimming of multiple LEDS with a single serial bus write.
pub const MASTER_FADE_1: u8 = 0x48;
/// An 8-bit register to control all the LED-drivers mapped to MASTER FADER2. Master fader allows
/// the user to control dimming of multiple LEDS with a single serial bus write.
pub const MASTER_FADE_2: u8 = 0x49;
/// An 8-bit register to control all the LED-drivers mapped to MASTER FADER3. Master fader allows
/// the user to control dimming of multiple LEDS with a single serial bus write.
pub const MASTER_FADE_3: u8 = 0x4A;

/// Program memory allocation for program execution engines is defined with PROG START ADDR registers.
pub const PROG1_START: u8 = 0x4C;
/// Engine 2 program start address.
pub const PROG2_START: u8 = 0x4D;
/// Engine 3 program start address.
pub const PROG3_START: u8 = 0x4E;
/// These bits select the program memory page. The program memory is divided into six pages of 16
/// instructions; thus, the total amount of the program memory is 96 instructions.
pub const PROG_PAGE_SEL: u8 = 0x4f;

/// PROGMEM page base - page selected by PROG_PAGE_SEL (0x4F)
pub const PROG_MEM_BASE: u8 = 0x50;
/// PROGMEM page end
pub const PROG_MEM_END: u8 = 0x6f;

/// Engine 1-to-LED mapping information, high byte
pub const ENG1_MAP_MSB: u8 = 0x70;
/// Engine 1-to-LED mapping information, low byte
pub const ENG1_MAP_LSB: u8 = 0x71;
/// Engine 2-to-LED mapping information, high byte
pub const ENG2_MAP_MSB: u8 = 0x72;
/// Engine 2-to-LED mapping information, low byte
pub const ENG2_MAP_LSB: u8 = 0x73;
/// Engine 3-to-LED mapping information, high byte
pub const ENG3_MAP_MSB: u8 = 0x74;
/// Engine 3-to-LED mapping information, low byte
pub const ENG3_MAP_LSB: u8 = 0x75;

/// With hysteresis and timer bits the user can optimize the charge pump performance to better meet the requirements of the application at hand. Some applications need to be optimized for efficiency and others need to be optimized for minimum EMI, for example.
pub const GAIN_CHANGE: u8 = 0x76;

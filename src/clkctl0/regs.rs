#[doc = "ADC0 fclk divider"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adc0fclkdiv(pub u32);
impl Adc0fclkdiv {
    #[doc = "Clock Divider Value Selection. . . 0: Divide by 1. ... 255: Divide by 256."]
    #[inline(always)]
    pub const fn div(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Clock Divider Value Selection. . . 0: Divide by 1. ... 255: Divide by 256."]
    #[inline(always)]
    pub fn set_div(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Resets the divider counter. Can be used to make sure a new divider value is used right away rather than completing the previous count."]
    #[inline(always)]
    pub const fn reset(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "Resets the divider counter. Can be used to make sure a new divider value is used right away rather than completing the previous count."]
    #[inline(always)]
    pub fn set_reset(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "Halts the divider counter. The intent is to allow the divider's clock source to be changed without the risk of a glitch at the output."]
    #[inline(always)]
    pub const fn halt(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Halts the divider counter. The intent is to allow the divider's clock source to be changed without the risk of a glitch at the output."]
    #[inline(always)]
    pub fn set_halt(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Divider status flag. Set when a change is made to the divider value, cleared when the change is complete."]
    #[inline(always)]
    pub const fn reqflag(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Divider status flag. Set when a change is made to the divider value, cleared when the change is complete."]
    #[inline(always)]
    pub fn set_reqflag(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Adc0fclkdiv {
    #[inline(always)]
    fn default() -> Adc0fclkdiv {
        Adc0fclkdiv(0)
    }
}
#[doc = "ADC0 fclk selection 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adc0fclksel0(pub u32);
impl Adc0fclksel0 {
    #[doc = "Clock Output Select 1st Stage. . ."]
    #[inline(always)]
    pub const fn sel(&self) -> super::vals::Adc0fclksel0Sel {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::Adc0fclksel0Sel::from_bits(val as u8)
    }
    #[doc = "Clock Output Select 1st Stage. . ."]
    #[inline(always)]
    pub fn set_sel(&mut self, val: super::vals::Adc0fclksel0Sel) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
}
impl Default for Adc0fclksel0 {
    #[inline(always)]
    fn default() -> Adc0fclksel0 {
        Adc0fclksel0(0)
    }
}
#[doc = "ADC0 fclk selection 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adc0fclksel1(pub u32);
impl Adc0fclksel1 {
    #[doc = "ADC Functional Clock Source Selection. . ."]
    #[inline(always)]
    pub const fn sel(&self) -> super::vals::Adc0fclksel1Sel {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::Adc0fclksel1Sel::from_bits(val as u8)
    }
    #[doc = "ADC Functional Clock Source Selection. . ."]
    #[inline(always)]
    pub fn set_sel(&mut self, val: super::vals::Adc0fclksel1Sel) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
}
impl Default for Adc0fclksel1 {
    #[inline(always)]
    fn default() -> Adc0fclksel1 {
        Adc0fclksel1(0)
    }
}
#[doc = "aux0 pll clk divider"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Aux0pllclkdiv(pub u32);
impl Aux0pllclkdiv {
    #[doc = "Clock Divider Value Selection. . . 0: Divide by 1. ... 255: Divide by 256."]
    #[inline(always)]
    pub const fn div(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Clock Divider Value Selection. . . 0: Divide by 1. ... 255: Divide by 256."]
    #[inline(always)]
    pub fn set_div(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Resets the divider counter. Can be used to make sure a new divider value is used right away rather than completing the previous count."]
    #[inline(always)]
    pub const fn reset(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "Resets the divider counter. Can be used to make sure a new divider value is used right away rather than completing the previous count."]
    #[inline(always)]
    pub fn set_reset(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "Halts the divider counter. The intent is to allow the divider's clock source to be changed without the risk of a glitch at the output."]
    #[inline(always)]
    pub const fn halt(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Halts the divider counter. The intent is to allow the divider's clock source to be changed without the risk of a glitch at the output."]
    #[inline(always)]
    pub fn set_halt(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Divider status flag. Set when a change is made to the divider value, cleared when the change is complete."]
    #[inline(always)]
    pub const fn reqflag(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Divider status flag. Set when a change is made to the divider value, cleared when the change is complete."]
    #[inline(always)]
    pub fn set_reqflag(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Aux0pllclkdiv {
    #[inline(always)]
    fn default() -> Aux0pllclkdiv {
        Aux0pllclkdiv(0)
    }
}
#[doc = "aux1 pll clk divider"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Aux1pllclkdiv(pub u32);
impl Aux1pllclkdiv {
    #[doc = "Clock Divider Value Selection. . . 0: Divide by 1. ... 255: Divide by 256."]
    #[inline(always)]
    pub const fn div(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Clock Divider Value Selection. . . 0: Divide by 1. ... 255: Divide by 256."]
    #[inline(always)]
    pub fn set_div(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Resets the divider counter. Can be used to make sure a new divider value is used right away rather than completing the previous count."]
    #[inline(always)]
    pub const fn reset(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "Resets the divider counter. Can be used to make sure a new divider value is used right away rather than completing the previous count."]
    #[inline(always)]
    pub fn set_reset(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "Halts the divider counter. The intent is to allow the divider's clock source to be changed without the risk of a glitch at the output."]
    #[inline(always)]
    pub const fn halt(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Halts the divider counter. The intent is to allow the divider's clock source to be changed without the risk of a glitch at the output."]
    #[inline(always)]
    pub fn set_halt(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Divider status flag. Set when a change is made to the divider value, cleared when the change is complete."]
    #[inline(always)]
    pub const fn reqflag(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Divider status flag. Set when a change is made to the divider value, cleared when the change is complete."]
    #[inline(always)]
    pub fn set_reqflag(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Aux1pllclkdiv {
    #[inline(always)]
    fn default() -> Aux1pllclkdiv {
        Aux1pllclkdiv(0)
    }
}
#[doc = "dsp pll clk divider"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dsppllclkdiv(pub u32);
impl Dsppllclkdiv {
    #[doc = "Clock Divider Value Selection. . . 0: Divide by 1. ... 255: Divide by 256."]
    #[inline(always)]
    pub const fn div(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Clock Divider Value Selection. . . 0: Divide by 1. ... 255: Divide by 256."]
    #[inline(always)]
    pub fn set_div(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Resets the divider counter. Can be used to make sure a new divider value is used right away rather than completing the previous count."]
    #[inline(always)]
    pub const fn reset(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "Resets the divider counter. Can be used to make sure a new divider value is used right away rather than completing the previous count."]
    #[inline(always)]
    pub fn set_reset(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "Halts the divider counter. The intent is to allow the divider's clock source to be changed without the risk of a glitch at the output."]
    #[inline(always)]
    pub const fn halt(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Halts the divider counter. The intent is to allow the divider's clock source to be changed without the risk of a glitch at the output."]
    #[inline(always)]
    pub fn set_halt(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Divider status flag. Set when a change is made to the divider value, cleared when the change is complete."]
    #[inline(always)]
    pub const fn reqflag(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Divider status flag. Set when a change is made to the divider value, cleared when the change is complete."]
    #[inline(always)]
    pub fn set_reqflag(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Dsppllclkdiv {
    #[inline(always)]
    fn default() -> Dsppllclkdiv {
        Dsppllclkdiv(0)
    }
}
#[doc = "FFRO control 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ffroctl0(pub u32);
impl Ffroctl0 {
    #[doc = "Trims temperature compensation of FFRO."]
    #[inline(always)]
    pub const fn trim_tempco(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "Trims temperature compensation of FFRO."]
    #[inline(always)]
    pub fn set_trim_tempco(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
    }
    #[doc = "Trims coarse frequency of FFRO."]
    #[inline(always)]
    pub const fn trim_coarse(&self) -> u8 {
        let val = (self.0 >> 5usize) & 0x3f;
        val as u8
    }
    #[doc = "Trims coarse frequency of FFRO."]
    #[inline(always)]
    pub fn set_trim_coarse(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 5usize)) | (((val as u32) & 0x3f) << 5usize);
    }
    #[doc = "Trims fine frequency of FFRO."]
    #[inline(always)]
    pub const fn trim_fine(&self) -> u8 {
        let val = (self.0 >> 11usize) & 0x7f;
        val as u8
    }
    #[doc = "Trims fine frequency of FFRO."]
    #[inline(always)]
    pub fn set_trim_fine(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 11usize)) | (((val as u32) & 0x7f) << 11usize);
    }
    #[doc = "Trims frequency range of FFRO."]
    #[inline(always)]
    pub const fn trim_range(&self) -> super::vals::TrimRange {
        let val = (self.0 >> 18usize) & 0x03;
        super::vals::TrimRange::from_bits(val as u8)
    }
    #[doc = "Trims frequency range of FFRO."]
    #[inline(always)]
    pub fn set_trim_range(&mut self, val: super::vals::TrimRange) {
        self.0 = (self.0 & !(0x03 << 18usize)) | (((val.to_bits() as u32) & 0x03) << 18usize);
    }
}
impl Default for Ffroctl0 {
    #[inline(always)]
    fn default() -> Ffroctl0 {
        Ffroctl0(0)
    }
}
#[doc = "FFRO control 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ffroctl1(pub u32);
impl Ffroctl1 {
    #[doc = "Update Safe Mode Control. In order to change any of the TRIM values, the user first needs to set the update safe mode bit, then proceed to change the respective TRIM values needed, followed by clearing the update safe mode bit."]
    #[inline(always)]
    pub const fn update(&self) -> super::vals::Update {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Update::from_bits(val as u8)
    }
    #[doc = "Update Safe Mode Control. In order to change any of the TRIM values, the user first needs to set the update safe mode bit, then proceed to change the respective TRIM values needed, followed by clearing the update safe mode bit."]
    #[inline(always)]
    pub fn set_update(&mut self, val: super::vals::Update) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Ffroctl1 {
    #[inline(always)]
    fn default() -> Ffroctl1 {
        Ffroctl1(0)
    }
}
#[doc = "FlexSPI FCLK divider"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Flexspifclkdiv(pub u32);
impl Flexspifclkdiv {
    #[doc = "Clock Divider Value Selection. . . 0: Divide by 1. ... 255: Divide by 256."]
    #[inline(always)]
    pub const fn div(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Clock Divider Value Selection. . . 0: Divide by 1. ... 255: Divide by 256."]
    #[inline(always)]
    pub fn set_div(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Resets the divider counter. Can be used to make sure a new divider value is used right away rather than completing the previous count."]
    #[inline(always)]
    pub const fn reset(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "Resets the divider counter. Can be used to make sure a new divider value is used right away rather than completing the previous count."]
    #[inline(always)]
    pub fn set_reset(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "Halts the divider counter. The intent is to allow the divider's clock source to be changed without the risk of a glitch at the output."]
    #[inline(always)]
    pub const fn halt(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Halts the divider counter. The intent is to allow the divider's clock source to be changed without the risk of a glitch at the output."]
    #[inline(always)]
    pub fn set_halt(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Divider status flag. Set when a change is made to the divider value, cleared when the change is complete."]
    #[inline(always)]
    pub const fn reqflag(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Divider status flag. Set when a change is made to the divider value, cleared when the change is complete."]
    #[inline(always)]
    pub fn set_reqflag(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Flexspifclkdiv {
    #[inline(always)]
    fn default() -> Flexspifclkdiv {
        Flexspifclkdiv(0)
    }
}
#[doc = "FlexSPI FCLK selection"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Flexspifclksel(pub u32);
impl Flexspifclksel {
    #[doc = "FlexSPI Functional Clock Source Selection. . ."]
    #[inline(always)]
    pub const fn sel(&self) -> super::vals::FlexspifclkselSel {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::FlexspifclkselSel::from_bits(val as u8)
    }
    #[doc = "FlexSPI Functional Clock Source Selection. . ."]
    #[inline(always)]
    pub fn set_sel(&mut self, val: super::vals::FlexspifclkselSel) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
}
impl Default for Flexspifclksel {
    #[inline(always)]
    fn default() -> Flexspifclksel {
        Flexspifclksel(0)
    }
}
#[doc = "low power oscillator control 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lposcctl0(pub u32);
impl Lposcctl0 {
    #[doc = "Clock ready flag status. LPOSC clock ready takes 64uS."]
    #[inline(always)]
    pub const fn clkrdy(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Clock ready flag status. LPOSC clock ready takes 64uS."]
    #[inline(always)]
    pub fn set_clkrdy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Lposcctl0 {
    #[inline(always)]
    fn default() -> Lposcctl0 {
        Lposcctl0(0)
    }
}
#[doc = "main clock selection A"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mainclksela(pub u32);
impl Mainclksela {
    #[doc = "Control Main 1st Stage Control Clock Source. . ."]
    #[inline(always)]
    pub const fn sel(&self) -> super::vals::MainclkselaSel {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::MainclkselaSel::from_bits(val as u8)
    }
    #[doc = "Control Main 1st Stage Control Clock Source. . ."]
    #[inline(always)]
    pub fn set_sel(&mut self, val: super::vals::MainclkselaSel) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
}
impl Default for Mainclksela {
    #[inline(always)]
    fn default() -> Mainclksela {
        Mainclksela(0)
    }
}
#[doc = "main clock selection B"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mainclkselb(pub u32);
impl Mainclkselb {
    #[doc = "Main Clock Source Selection. . ."]
    #[inline(always)]
    pub const fn sel(&self) -> super::vals::MainclkselbSel {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::MainclkselbSel::from_bits(val as u8)
    }
    #[doc = "Main Clock Source Selection. . ."]
    #[inline(always)]
    pub fn set_sel(&mut self, val: super::vals::MainclkselbSel) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
}
impl Default for Mainclkselb {
    #[inline(always)]
    fn default() -> Mainclkselb {
        Mainclkselb(0)
    }
}
#[doc = "main pll clk divider"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mainpllclkdiv(pub u32);
impl Mainpllclkdiv {
    #[doc = "Clock Divider Value Selection. . . 0: Divide by 1. ... 255: Divide by 256."]
    #[inline(always)]
    pub const fn div(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Clock Divider Value Selection. . . 0: Divide by 1. ... 255: Divide by 256."]
    #[inline(always)]
    pub fn set_div(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Resets the divider counter. Can be used to make sure a new divider value is used right away rather than completing the previous count."]
    #[inline(always)]
    pub const fn reset(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "Resets the divider counter. Can be used to make sure a new divider value is used right away rather than completing the previous count."]
    #[inline(always)]
    pub fn set_reset(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "Halts the divider counter. The intent is to allow the divider's clock source to be changed without the risk of a glitch at the output."]
    #[inline(always)]
    pub const fn halt(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Halts the divider counter. The intent is to allow the divider's clock source to be changed without the risk of a glitch at the output."]
    #[inline(always)]
    pub fn set_halt(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Divider status flag. Set when a change is made to the divider value, cleared when the change is complete."]
    #[inline(always)]
    pub const fn reqflag(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Divider status flag. Set when a change is made to the divider value, cleared when the change is complete."]
    #[inline(always)]
    pub fn set_reqflag(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Mainpllclkdiv {
    #[inline(always)]
    fn default() -> Mainpllclkdiv {
        Mainpllclkdiv(0)
    }
}
#[doc = "32k oscillator control0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Osc32khzctl0(pub u32);
impl Osc32khzctl0 {
    #[doc = "32KHz Enable."]
    #[inline(always)]
    pub const fn ena32khz(&self) -> super::vals::Ena32khz {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Ena32khz::from_bits(val as u8)
    }
    #[doc = "32KHz Enable."]
    #[inline(always)]
    pub fn set_ena32khz(&mut self, val: super::vals::Ena32khz) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for Osc32khzctl0 {
    #[inline(always)]
    fn default() -> Osc32khzctl0 {
        Osc32khzctl0(0)
    }
}
#[doc = "PFC divider register N"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pfcdiv(pub u32);
impl Pfcdiv {
    #[doc = "Clock Divider Value Selection. . . 0: Divide by 1. ... 255: Divide by 256."]
    #[inline(always)]
    pub const fn div(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Clock Divider Value Selection. . . 0: Divide by 1. ... 255: Divide by 256."]
    #[inline(always)]
    pub fn set_div(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Resets the divider counter. Can be used to make sure a new divider value is used right away rather than completing the previous count."]
    #[inline(always)]
    pub const fn reset(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "Resets the divider counter. Can be used to make sure a new divider value is used right away rather than completing the previous count."]
    #[inline(always)]
    pub fn set_reset(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "Halts the divider counter. The intent is to allow the divider clock source to be changed without the risk of a glitch at the output."]
    #[inline(always)]
    pub const fn halt(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Halts the divider counter. The intent is to allow the divider clock source to be changed without the risk of a glitch at the output."]
    #[inline(always)]
    pub fn set_halt(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Divider status flag. Set when a change is made to the divider value, cleared when the change is complete."]
    #[inline(always)]
    pub const fn reqflag(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Divider status flag. Set when a change is made to the divider value, cleared when the change is complete."]
    #[inline(always)]
    pub fn set_reqflag(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Pfcdiv {
    #[inline(always)]
    fn default() -> Pfcdiv {
        Pfcdiv(0)
    }
}
#[doc = "clock control register 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pscctl0(pub u32);
impl Pscctl0 {
    #[doc = "128KB ROM control"]
    #[inline(always)]
    pub const fn rom_ctl_128kb(&self) -> super::vals::RomCtl128kb {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::RomCtl128kb::from_bits(val as u8)
    }
    #[doc = "128KB ROM control"]
    #[inline(always)]
    pub fn set_rom_ctl_128kb(&mut self, val: super::vals::RomCtl128kb) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "powerquad clock control"]
    #[inline(always)]
    pub const fn powerquad_clk(&self) -> super::vals::Pscctl0PowerquadClk {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Pscctl0PowerquadClk::from_bits(val as u8)
    }
    #[doc = "powerquad clock control"]
    #[inline(always)]
    pub fn set_powerquad_clk(&mut self, val: super::vals::Pscctl0PowerquadClk) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "CAPSER clock control"]
    #[inline(always)]
    pub const fn casper_clk(&self) -> super::vals::Pscctl0CasperClk {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::Pscctl0CasperClk::from_bits(val as u8)
    }
    #[doc = "CAPSER clock control"]
    #[inline(always)]
    pub fn set_casper_clk(&mut self, val: super::vals::Pscctl0CasperClk) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "HASHCRYPT clock control"]
    #[inline(always)]
    pub const fn hashcrypt_clk(&self) -> super::vals::Pscctl0HashcryptClk {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::Pscctl0HashcryptClk::from_bits(val as u8)
    }
    #[doc = "HASHCRYPT clock control"]
    #[inline(always)]
    pub fn set_hashcrypt_clk(&mut self, val: super::vals::Pscctl0HashcryptClk) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "PUF clock control"]
    #[inline(always)]
    pub const fn puf_clk(&self) -> super::vals::Pscctl0PufClk {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::Pscctl0PufClk::from_bits(val as u8)
    }
    #[doc = "PUF clock control"]
    #[inline(always)]
    pub fn set_puf_clk(&mut self, val: super::vals::Pscctl0PufClk) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "RNG clock control"]
    #[inline(always)]
    pub const fn rng_clk(&self) -> super::vals::Pscctl0RngClk {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::Pscctl0RngClk::from_bits(val as u8)
    }
    #[doc = "RNG clock control"]
    #[inline(always)]
    pub fn set_rng_clk(&mut self, val: super::vals::Pscctl0RngClk) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "FLEXSPI clock control"]
    #[inline(always)]
    pub const fn flexspi_otfad_clk(&self) -> super::vals::Pscctl0FlexspiOtfadClk {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::Pscctl0FlexspiOtfadClk::from_bits(val as u8)
    }
    #[doc = "FLEXSPI clock control"]
    #[inline(always)]
    pub fn set_flexspi_otfad_clk(&mut self, val: super::vals::Pscctl0FlexspiOtfadClk) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "OTP clock control"]
    #[inline(always)]
    pub const fn otp_clk(&self) -> super::vals::Pscctl0OtpClk {
        let val = (self.0 >> 17usize) & 0x01;
        super::vals::Pscctl0OtpClk::from_bits(val as u8)
    }
    #[doc = "OTP clock control"]
    #[inline(always)]
    pub fn set_otp_clk(&mut self, val: super::vals::Pscctl0OtpClk) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
    #[doc = "USB PHY clock control"]
    #[inline(always)]
    pub const fn usbhs_phy_clk(&self) -> super::vals::Pscctl0UsbhsPhyClk {
        let val = (self.0 >> 20usize) & 0x01;
        super::vals::Pscctl0UsbhsPhyClk::from_bits(val as u8)
    }
    #[doc = "USB PHY clock control"]
    #[inline(always)]
    pub fn set_usbhs_phy_clk(&mut self, val: super::vals::Pscctl0UsbhsPhyClk) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
    #[doc = "USB DEVICE clock control"]
    #[inline(always)]
    pub const fn usbhs_device_clk(&self) -> super::vals::Pscctl0UsbhsDeviceClk {
        let val = (self.0 >> 21usize) & 0x01;
        super::vals::Pscctl0UsbhsDeviceClk::from_bits(val as u8)
    }
    #[doc = "USB DEVICE clock control"]
    #[inline(always)]
    pub fn set_usbhs_device_clk(&mut self, val: super::vals::Pscctl0UsbhsDeviceClk) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
    }
    #[doc = "USB HOST clock control"]
    #[inline(always)]
    pub const fn usbhs_host_clk(&self) -> super::vals::Pscctl0UsbhsHostClk {
        let val = (self.0 >> 22usize) & 0x01;
        super::vals::Pscctl0UsbhsHostClk::from_bits(val as u8)
    }
    #[doc = "USB HOST clock control"]
    #[inline(always)]
    pub fn set_usbhs_host_clk(&mut self, val: super::vals::Pscctl0UsbhsHostClk) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val.to_bits() as u32) & 0x01) << 22usize);
    }
    #[doc = "USBHS RAM clock control"]
    #[inline(always)]
    pub const fn usbhs_sram_clk(&self) -> super::vals::Pscctl0UsbhsSramClk {
        let val = (self.0 >> 23usize) & 0x01;
        super::vals::Pscctl0UsbhsSramClk::from_bits(val as u8)
    }
    #[doc = "USBHS RAM clock control"]
    #[inline(always)]
    pub fn set_usbhs_sram_clk(&mut self, val: super::vals::Pscctl0UsbhsSramClk) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "SCT clock control"]
    #[inline(always)]
    pub const fn sct_clk(&self) -> super::vals::Pscctl0SctClk {
        let val = (self.0 >> 24usize) & 0x01;
        super::vals::Pscctl0SctClk::from_bits(val as u8)
    }
    #[doc = "SCT clock control"]
    #[inline(always)]
    pub fn set_sct_clk(&mut self, val: super::vals::Pscctl0SctClk) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
}
impl Default for Pscctl0 {
    #[inline(always)]
    fn default() -> Pscctl0 {
        Pscctl0(0)
    }
}
#[doc = "clock clear register 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pscctl0Clr(pub u32);
impl Pscctl0Clr {
    #[doc = "ROM controller clock clear"]
    #[inline(always)]
    pub const fn rom_ctl_128kb_clk(&self) -> super::vals::Pscctl0ClrRomCtl128kbClk {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Pscctl0ClrRomCtl128kbClk::from_bits(val as u8)
    }
    #[doc = "ROM controller clock clear"]
    #[inline(always)]
    pub fn set_rom_ctl_128kb_clk(&mut self, val: super::vals::Pscctl0ClrRomCtl128kbClk) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "powerquad clock clear"]
    #[inline(always)]
    pub const fn powerquad_clk(&self) -> super::vals::Pscctl0ClrPowerquadClk {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Pscctl0ClrPowerquadClk::from_bits(val as u8)
    }
    #[doc = "powerquad clock clear"]
    #[inline(always)]
    pub fn set_powerquad_clk(&mut self, val: super::vals::Pscctl0ClrPowerquadClk) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "CAPSER clock clear"]
    #[inline(always)]
    pub const fn casper_clk(&self) -> super::vals::Pscctl0ClrCasperClk {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::Pscctl0ClrCasperClk::from_bits(val as u8)
    }
    #[doc = "CAPSER clock clear"]
    #[inline(always)]
    pub fn set_casper_clk(&mut self, val: super::vals::Pscctl0ClrCasperClk) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "HASHCRYPT clock clear"]
    #[inline(always)]
    pub const fn hashcrypt_clk(&self) -> super::vals::Pscctl0ClrHashcryptClk {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::Pscctl0ClrHashcryptClk::from_bits(val as u8)
    }
    #[doc = "HASHCRYPT clock clear"]
    #[inline(always)]
    pub fn set_hashcrypt_clk(&mut self, val: super::vals::Pscctl0ClrHashcryptClk) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "PUF clock clear"]
    #[inline(always)]
    pub const fn puf_clk(&self) -> super::vals::Pscctl0ClrPufClk {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::Pscctl0ClrPufClk::from_bits(val as u8)
    }
    #[doc = "PUF clock clear"]
    #[inline(always)]
    pub fn set_puf_clk(&mut self, val: super::vals::Pscctl0ClrPufClk) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "RNG clock clear"]
    #[inline(always)]
    pub const fn rng_clk(&self) -> super::vals::Pscctl0ClrRngClk {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::Pscctl0ClrRngClk::from_bits(val as u8)
    }
    #[doc = "RNG clock clear"]
    #[inline(always)]
    pub fn set_rng_clk(&mut self, val: super::vals::Pscctl0ClrRngClk) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "FLEXSPI clock clear"]
    #[inline(always)]
    pub const fn flexspi_otfad_clk(&self) -> super::vals::Pscctl0ClrFlexspiOtfadClk {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::Pscctl0ClrFlexspiOtfadClk::from_bits(val as u8)
    }
    #[doc = "FLEXSPI clock clear"]
    #[inline(always)]
    pub fn set_flexspi_otfad_clk(&mut self, val: super::vals::Pscctl0ClrFlexspiOtfadClk) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "OTP clock clear"]
    #[inline(always)]
    pub const fn otp_clk(&self) -> super::vals::Pscctl0ClrOtpClk {
        let val = (self.0 >> 17usize) & 0x01;
        super::vals::Pscctl0ClrOtpClk::from_bits(val as u8)
    }
    #[doc = "OTP clock clear"]
    #[inline(always)]
    pub fn set_otp_clk(&mut self, val: super::vals::Pscctl0ClrOtpClk) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
    #[doc = "USB PHY clock clear"]
    #[inline(always)]
    pub const fn usbhs_phy_clk(&self) -> super::vals::Pscctl0ClrUsbhsPhyClk {
        let val = (self.0 >> 20usize) & 0x01;
        super::vals::Pscctl0ClrUsbhsPhyClk::from_bits(val as u8)
    }
    #[doc = "USB PHY clock clear"]
    #[inline(always)]
    pub fn set_usbhs_phy_clk(&mut self, val: super::vals::Pscctl0ClrUsbhsPhyClk) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
    #[doc = "USB DEVICE clock clear"]
    #[inline(always)]
    pub const fn usbhs_device_clk(&self) -> super::vals::Pscctl0ClrUsbhsDeviceClk {
        let val = (self.0 >> 21usize) & 0x01;
        super::vals::Pscctl0ClrUsbhsDeviceClk::from_bits(val as u8)
    }
    #[doc = "USB DEVICE clock clear"]
    #[inline(always)]
    pub fn set_usbhs_device_clk(&mut self, val: super::vals::Pscctl0ClrUsbhsDeviceClk) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
    }
    #[doc = "USB HOST clock clear"]
    #[inline(always)]
    pub const fn usbhs_host_clk(&self) -> super::vals::Pscctl0ClrUsbhsHostClk {
        let val = (self.0 >> 22usize) & 0x01;
        super::vals::Pscctl0ClrUsbhsHostClk::from_bits(val as u8)
    }
    #[doc = "USB HOST clock clear"]
    #[inline(always)]
    pub fn set_usbhs_host_clk(&mut self, val: super::vals::Pscctl0ClrUsbhsHostClk) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val.to_bits() as u32) & 0x01) << 22usize);
    }
    #[doc = "USBHS RAM clock clear"]
    #[inline(always)]
    pub const fn usbhs_sram_clk(&self) -> super::vals::Pscctl0ClrUsbhsSramClk {
        let val = (self.0 >> 23usize) & 0x01;
        super::vals::Pscctl0ClrUsbhsSramClk::from_bits(val as u8)
    }
    #[doc = "USBHS RAM clock clear"]
    #[inline(always)]
    pub fn set_usbhs_sram_clk(&mut self, val: super::vals::Pscctl0ClrUsbhsSramClk) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "SCT clock clear"]
    #[inline(always)]
    pub const fn sct_clk(&self) -> super::vals::Pscctl0ClrSctClk {
        let val = (self.0 >> 24usize) & 0x01;
        super::vals::Pscctl0ClrSctClk::from_bits(val as u8)
    }
    #[doc = "SCT clock clear"]
    #[inline(always)]
    pub fn set_sct_clk(&mut self, val: super::vals::Pscctl0ClrSctClk) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
}
impl Default for Pscctl0Clr {
    #[inline(always)]
    fn default() -> Pscctl0Clr {
        Pscctl0Clr(0)
    }
}
#[doc = "clock set register 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pscctl0Set(pub u32);
impl Pscctl0Set {
    #[doc = "128KB ROM controller clock set"]
    #[inline(always)]
    pub const fn rom_ctl_128kb_clk(&self) -> super::vals::Pscctl0SetRomCtl128kbClk {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Pscctl0SetRomCtl128kbClk::from_bits(val as u8)
    }
    #[doc = "128KB ROM controller clock set"]
    #[inline(always)]
    pub fn set_rom_ctl_128kb_clk(&mut self, val: super::vals::Pscctl0SetRomCtl128kbClk) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "powerquad clock set"]
    #[inline(always)]
    pub const fn powerquad_clk(&self) -> super::vals::Pscctl0SetPowerquadClk {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Pscctl0SetPowerquadClk::from_bits(val as u8)
    }
    #[doc = "powerquad clock set"]
    #[inline(always)]
    pub fn set_powerquad_clk(&mut self, val: super::vals::Pscctl0SetPowerquadClk) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "CAPSER clock set"]
    #[inline(always)]
    pub const fn casper_clk(&self) -> super::vals::Pscctl0SetCasperClk {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::Pscctl0SetCasperClk::from_bits(val as u8)
    }
    #[doc = "CAPSER clock set"]
    #[inline(always)]
    pub fn set_casper_clk(&mut self, val: super::vals::Pscctl0SetCasperClk) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "HASHCRYPT clock set"]
    #[inline(always)]
    pub const fn hashcrypt_clk(&self) -> super::vals::Pscctl0SetHashcryptClk {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::Pscctl0SetHashcryptClk::from_bits(val as u8)
    }
    #[doc = "HASHCRYPT clock set"]
    #[inline(always)]
    pub fn set_hashcrypt_clk(&mut self, val: super::vals::Pscctl0SetHashcryptClk) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "PUF clock set"]
    #[inline(always)]
    pub const fn puf_clk(&self) -> super::vals::Pscctl0SetPufClk {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::Pscctl0SetPufClk::from_bits(val as u8)
    }
    #[doc = "PUF clock set"]
    #[inline(always)]
    pub fn set_puf_clk(&mut self, val: super::vals::Pscctl0SetPufClk) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "RNG clock set"]
    #[inline(always)]
    pub const fn rng_clk(&self) -> super::vals::Pscctl0SetRngClk {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::Pscctl0SetRngClk::from_bits(val as u8)
    }
    #[doc = "RNG clock set"]
    #[inline(always)]
    pub fn set_rng_clk(&mut self, val: super::vals::Pscctl0SetRngClk) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "FLEXSPI clock set"]
    #[inline(always)]
    pub const fn flexspi_otfad_clk(&self) -> super::vals::Pscctl0SetFlexspiOtfadClk {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::Pscctl0SetFlexspiOtfadClk::from_bits(val as u8)
    }
    #[doc = "FLEXSPI clock set"]
    #[inline(always)]
    pub fn set_flexspi_otfad_clk(&mut self, val: super::vals::Pscctl0SetFlexspiOtfadClk) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "OTP clock set"]
    #[inline(always)]
    pub const fn otp_clk(&self) -> super::vals::Pscctl0SetOtpClk {
        let val = (self.0 >> 17usize) & 0x01;
        super::vals::Pscctl0SetOtpClk::from_bits(val as u8)
    }
    #[doc = "OTP clock set"]
    #[inline(always)]
    pub fn set_otp_clk(&mut self, val: super::vals::Pscctl0SetOtpClk) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
    #[doc = "USB PHY clock set"]
    #[inline(always)]
    pub const fn usbhs_phy_clk(&self) -> super::vals::Pscctl0SetUsbhsPhyClk {
        let val = (self.0 >> 20usize) & 0x01;
        super::vals::Pscctl0SetUsbhsPhyClk::from_bits(val as u8)
    }
    #[doc = "USB PHY clock set"]
    #[inline(always)]
    pub fn set_usbhs_phy_clk(&mut self, val: super::vals::Pscctl0SetUsbhsPhyClk) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
    #[doc = "USB DEVICE clock set"]
    #[inline(always)]
    pub const fn usbhs_device_clk(&self) -> super::vals::Pscctl0SetUsbhsDeviceClk {
        let val = (self.0 >> 21usize) & 0x01;
        super::vals::Pscctl0SetUsbhsDeviceClk::from_bits(val as u8)
    }
    #[doc = "USB DEVICE clock set"]
    #[inline(always)]
    pub fn set_usbhs_device_clk(&mut self, val: super::vals::Pscctl0SetUsbhsDeviceClk) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
    }
    #[doc = "USB HOST clock set"]
    #[inline(always)]
    pub const fn usbhs_host_clk(&self) -> super::vals::Pscctl0SetUsbhsHostClk {
        let val = (self.0 >> 22usize) & 0x01;
        super::vals::Pscctl0SetUsbhsHostClk::from_bits(val as u8)
    }
    #[doc = "USB HOST clock set"]
    #[inline(always)]
    pub fn set_usbhs_host_clk(&mut self, val: super::vals::Pscctl0SetUsbhsHostClk) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val.to_bits() as u32) & 0x01) << 22usize);
    }
    #[doc = "USBHS RAM clock set"]
    #[inline(always)]
    pub const fn usbhs_sram_clk(&self) -> super::vals::Pscctl0SetUsbhsSramClk {
        let val = (self.0 >> 23usize) & 0x01;
        super::vals::Pscctl0SetUsbhsSramClk::from_bits(val as u8)
    }
    #[doc = "USBHS RAM clock set"]
    #[inline(always)]
    pub fn set_usbhs_sram_clk(&mut self, val: super::vals::Pscctl0SetUsbhsSramClk) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "SCT clock set"]
    #[inline(always)]
    pub const fn sct_clk(&self) -> super::vals::Pscctl0SetSctClk {
        let val = (self.0 >> 24usize) & 0x01;
        super::vals::Pscctl0SetSctClk::from_bits(val as u8)
    }
    #[doc = "SCT clock set"]
    #[inline(always)]
    pub fn set_sct_clk(&mut self, val: super::vals::Pscctl0SetSctClk) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
}
impl Default for Pscctl0Set {
    #[inline(always)]
    fn default() -> Pscctl0Set {
        Pscctl0Set(0)
    }
}
#[doc = "clock control register 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pscctl1(pub u32);
impl Pscctl1 {
    #[doc = "SDIO0 clock control"]
    #[inline(always)]
    pub const fn sdio0_clk(&self) -> super::vals::Pscctl1Sdio0Clk {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Pscctl1Sdio0Clk::from_bits(val as u8)
    }
    #[doc = "SDIO0 clock control"]
    #[inline(always)]
    pub fn set_sdio0_clk(&mut self, val: super::vals::Pscctl1Sdio0Clk) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "SDIO1 clock control"]
    #[inline(always)]
    pub const fn sdio1_clk(&self) -> super::vals::Pscctl1Sdio1Clk {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Pscctl1Sdio1Clk::from_bits(val as u8)
    }
    #[doc = "SDIO1 clock control"]
    #[inline(always)]
    pub fn set_sdio1_clk(&mut self, val: super::vals::Pscctl1Sdio1Clk) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Analog comparator clock control"]
    #[inline(always)]
    pub const fn acmp0_clk(&self) -> super::vals::Pscctl1Acmp0Clk {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::Pscctl1Acmp0Clk::from_bits(val as u8)
    }
    #[doc = "Analog comparator clock control"]
    #[inline(always)]
    pub fn set_acmp0_clk(&mut self, val: super::vals::Pscctl1Acmp0Clk) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
    #[doc = "ADC clock control"]
    #[inline(always)]
    pub const fn adc0_clk(&self) -> super::vals::Pscctl1Adc0Clk {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::Pscctl1Adc0Clk::from_bits(val as u8)
    }
    #[doc = "ADC clock control"]
    #[inline(always)]
    pub fn set_adc0_clk(&mut self, val: super::vals::Pscctl1Adc0Clk) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "SHSGPIO0 clock control"]
    #[inline(always)]
    pub const fn shsgpio0_clk(&self) -> super::vals::Pscctl1Shsgpio0Clk {
        let val = (self.0 >> 24usize) & 0x01;
        super::vals::Pscctl1Shsgpio0Clk::from_bits(val as u8)
    }
    #[doc = "SHSGPIO0 clock control"]
    #[inline(always)]
    pub fn set_shsgpio0_clk(&mut self, val: super::vals::Pscctl1Shsgpio0Clk) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
}
impl Default for Pscctl1 {
    #[inline(always)]
    fn default() -> Pscctl1 {
        Pscctl1(0)
    }
}
#[doc = "clock clear register 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pscctl1Clr(pub u32);
impl Pscctl1Clr {
    #[doc = "SDIO0 clock clear"]
    #[inline(always)]
    pub const fn sdio0_clk(&self) -> super::vals::Pscctl1ClrSdio0Clk {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Pscctl1ClrSdio0Clk::from_bits(val as u8)
    }
    #[doc = "SDIO0 clock clear"]
    #[inline(always)]
    pub fn set_sdio0_clk(&mut self, val: super::vals::Pscctl1ClrSdio0Clk) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "SDIO1 clock clear"]
    #[inline(always)]
    pub const fn sdio1_clk(&self) -> super::vals::Pscctl1ClrSdio1Clk {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Pscctl1ClrSdio1Clk::from_bits(val as u8)
    }
    #[doc = "SDIO1 clock clear"]
    #[inline(always)]
    pub fn set_sdio1_clk(&mut self, val: super::vals::Pscctl1ClrSdio1Clk) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Analog comparator clock clear"]
    #[inline(always)]
    pub const fn acmp0_clk(&self) -> super::vals::Pscctl1ClrAcmp0Clk {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::Pscctl1ClrAcmp0Clk::from_bits(val as u8)
    }
    #[doc = "Analog comparator clock clear"]
    #[inline(always)]
    pub fn set_acmp0_clk(&mut self, val: super::vals::Pscctl1ClrAcmp0Clk) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
    #[doc = "ADC clock clear"]
    #[inline(always)]
    pub const fn adc0_clk(&self) -> super::vals::Pscctl1ClrAdc0Clk {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::Pscctl1ClrAdc0Clk::from_bits(val as u8)
    }
    #[doc = "ADC clock clear"]
    #[inline(always)]
    pub fn set_adc0_clk(&mut self, val: super::vals::Pscctl1ClrAdc0Clk) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "SHSGPIO0 clock clear"]
    #[inline(always)]
    pub const fn shsgpio0_clk(&self) -> super::vals::Pscctl1ClrShsgpio0Clk {
        let val = (self.0 >> 24usize) & 0x01;
        super::vals::Pscctl1ClrShsgpio0Clk::from_bits(val as u8)
    }
    #[doc = "SHSGPIO0 clock clear"]
    #[inline(always)]
    pub fn set_shsgpio0_clk(&mut self, val: super::vals::Pscctl1ClrShsgpio0Clk) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
}
impl Default for Pscctl1Clr {
    #[inline(always)]
    fn default() -> Pscctl1Clr {
        Pscctl1Clr(0)
    }
}
#[doc = "clock set register 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pscctl1Set(pub u32);
impl Pscctl1Set {
    #[doc = "SDIO0 clock set"]
    #[inline(always)]
    pub const fn sdio0_clk(&self) -> super::vals::Pscctl1SetSdio0Clk {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Pscctl1SetSdio0Clk::from_bits(val as u8)
    }
    #[doc = "SDIO0 clock set"]
    #[inline(always)]
    pub fn set_sdio0_clk(&mut self, val: super::vals::Pscctl1SetSdio0Clk) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "SDIO1 clock set"]
    #[inline(always)]
    pub const fn sdio1_clk(&self) -> super::vals::Pscctl1SetSdio1Clk {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Pscctl1SetSdio1Clk::from_bits(val as u8)
    }
    #[doc = "SDIO1 clock set"]
    #[inline(always)]
    pub fn set_sdio1_clk(&mut self, val: super::vals::Pscctl1SetSdio1Clk) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Analog comparator clock set"]
    #[inline(always)]
    pub const fn acmp0_clk(&self) -> super::vals::Pscctl1SetAcmp0Clk {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::Pscctl1SetAcmp0Clk::from_bits(val as u8)
    }
    #[doc = "Analog comparator clock set"]
    #[inline(always)]
    pub fn set_acmp0_clk(&mut self, val: super::vals::Pscctl1SetAcmp0Clk) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
    #[doc = "ADC clock set"]
    #[inline(always)]
    pub const fn adc0_clk(&self) -> super::vals::Pscctl1SetAdc0Clk {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::Pscctl1SetAdc0Clk::from_bits(val as u8)
    }
    #[doc = "ADC clock set"]
    #[inline(always)]
    pub fn set_adc0_clk(&mut self, val: super::vals::Pscctl1SetAdc0Clk) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "SHSGPIO0 clock set"]
    #[inline(always)]
    pub const fn shsgpio0_clk(&self) -> super::vals::Pscctl1SetShsgpio0Clk {
        let val = (self.0 >> 24usize) & 0x01;
        super::vals::Pscctl1SetShsgpio0Clk::from_bits(val as u8)
    }
    #[doc = "SHSGPIO0 clock set"]
    #[inline(always)]
    pub fn set_shsgpio0_clk(&mut self, val: super::vals::Pscctl1SetShsgpio0Clk) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
}
impl Default for Pscctl1Set {
    #[inline(always)]
    fn default() -> Pscctl1Set {
        Pscctl1Set(0)
    }
}
#[doc = "clock control register 2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pscctl2(pub u32);
impl Pscctl2 {
    #[doc = "utick clock control"]
    #[inline(always)]
    pub const fn utick0_clk(&self) -> super::vals::Pscctl2Utick0Clk {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Pscctl2Utick0Clk::from_bits(val as u8)
    }
    #[doc = "utick clock control"]
    #[inline(always)]
    pub fn set_utick0_clk(&mut self, val: super::vals::Pscctl2Utick0Clk) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "wdt clock control"]
    #[inline(always)]
    pub const fn wwdt0_clk(&self) -> super::vals::Pscctl2Wwdt0Clk {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Pscctl2Wwdt0Clk::from_bits(val as u8)
    }
    #[doc = "wdt clock control"]
    #[inline(always)]
    pub fn set_wwdt0_clk(&mut self, val: super::vals::Pscctl2Wwdt0Clk) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
}
impl Default for Pscctl2 {
    #[inline(always)]
    fn default() -> Pscctl2 {
        Pscctl2(0)
    }
}
#[doc = "clock clear register 2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pscctl2Clr(pub u32);
impl Pscctl2Clr {
    #[doc = "utick clock clear"]
    #[inline(always)]
    pub const fn utick0_clk(&self) -> super::vals::Pscctl2ClrUtick0Clk {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Pscctl2ClrUtick0Clk::from_bits(val as u8)
    }
    #[doc = "utick clock clear"]
    #[inline(always)]
    pub fn set_utick0_clk(&mut self, val: super::vals::Pscctl2ClrUtick0Clk) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "wdt clock clear"]
    #[inline(always)]
    pub const fn wwdt0_clk(&self) -> super::vals::Pscctl2ClrWwdt0Clk {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Pscctl2ClrWwdt0Clk::from_bits(val as u8)
    }
    #[doc = "wdt clock clear"]
    #[inline(always)]
    pub fn set_wwdt0_clk(&mut self, val: super::vals::Pscctl2ClrWwdt0Clk) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
}
impl Default for Pscctl2Clr {
    #[inline(always)]
    fn default() -> Pscctl2Clr {
        Pscctl2Clr(0)
    }
}
#[doc = "clock set register 2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pscctl2Set(pub u32);
impl Pscctl2Set {
    #[doc = "utick clock set"]
    #[inline(always)]
    pub const fn utick0_clk(&self) -> super::vals::Pscctl2SetUtick0Clk {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Pscctl2SetUtick0Clk::from_bits(val as u8)
    }
    #[doc = "utick clock set"]
    #[inline(always)]
    pub fn set_utick0_clk(&mut self, val: super::vals::Pscctl2SetUtick0Clk) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "wdt clock set"]
    #[inline(always)]
    pub const fn wwdt0_clk(&self) -> super::vals::Pscctl2SetWwdt0Clk {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Pscctl2SetWwdt0Clk::from_bits(val as u8)
    }
    #[doc = "wdt clock set"]
    #[inline(always)]
    pub fn set_wwdt0_clk(&mut self, val: super::vals::Pscctl2SetWwdt0Clk) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
}
impl Default for Pscctl2Set {
    #[inline(always)]
    fn default() -> Pscctl2Set {
        Pscctl2Set(0)
    }
}
#[doc = "SCT fclk divider"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sctfclkdiv(pub u32);
impl Sctfclkdiv {
    #[doc = "Clock Divider Value Selection. . . 0: Divide by 1. ... 255: Divide by 256."]
    #[inline(always)]
    pub const fn div(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Clock Divider Value Selection. . . 0: Divide by 1. ... 255: Divide by 256."]
    #[inline(always)]
    pub fn set_div(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Resets the divider counter. Can be used to make sure a new divider value is used right away rather than completing the previous count."]
    #[inline(always)]
    pub const fn reset(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "Resets the divider counter. Can be used to make sure a new divider value is used right away rather than completing the previous count."]
    #[inline(always)]
    pub fn set_reset(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "Halts the divider counter. The intent is to allow the divider clock source to be changed without the risk of a glitch at the output."]
    #[inline(always)]
    pub const fn halt(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Halts the divider counter. The intent is to allow the divider clock source to be changed without the risk of a glitch at the output."]
    #[inline(always)]
    pub fn set_halt(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Divider status flag. Set when a change is made to the divider value, cleared when the change is complete."]
    #[inline(always)]
    pub const fn reqflag(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Divider status flag. Set when a change is made to the divider value, cleared when the change is complete."]
    #[inline(always)]
    pub fn set_reqflag(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Sctfclkdiv {
    #[inline(always)]
    fn default() -> Sctfclkdiv {
        Sctfclkdiv(0)
    }
}
#[doc = "SCT FCLK selection"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sctfclksel(pub u32);
impl Sctfclksel {
    #[doc = "SCT Functional Clock Source Selection. . ."]
    #[inline(always)]
    pub const fn sel(&self) -> super::vals::SctfclkselSel {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::SctfclkselSel::from_bits(val as u8)
    }
    #[doc = "SCT Functional Clock Source Selection. . ."]
    #[inline(always)]
    pub fn set_sel(&mut self, val: super::vals::SctfclkselSel) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
}
impl Default for Sctfclksel {
    #[inline(always)]
    fn default() -> Sctfclksel {
        Sctfclksel(0)
    }
}
#[doc = "SDIO0 FCLK divider"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sdio0fclkdiv(pub u32);
impl Sdio0fclkdiv {
    #[doc = "Clock Divider Value Selection. . . 0: Divide by 1. ... 255: Divide by 256."]
    #[inline(always)]
    pub const fn div(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Clock Divider Value Selection. . . 0: Divide by 1. ... 255: Divide by 256."]
    #[inline(always)]
    pub fn set_div(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Resets the divider counter. Can be used to make sure a new divider value is used right away rather than completing the previous count."]
    #[inline(always)]
    pub const fn reset(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "Resets the divider counter. Can be used to make sure a new divider value is used right away rather than completing the previous count."]
    #[inline(always)]
    pub fn set_reset(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "Halts the divider counter. The intent is to allow the divider clock source to be changed without the risk of a glitch at the output."]
    #[inline(always)]
    pub const fn halt(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Halts the divider counter. The intent is to allow the divider clock source to be changed without the risk of a glitch at the output."]
    #[inline(always)]
    pub fn set_halt(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Divider status flag. Set when a change is made to the divider value, cleared when the change is complete."]
    #[inline(always)]
    pub const fn reqflag(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Divider status flag. Set when a change is made to the divider value, cleared when the change is complete."]
    #[inline(always)]
    pub fn set_reqflag(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Sdio0fclkdiv {
    #[inline(always)]
    fn default() -> Sdio0fclkdiv {
        Sdio0fclkdiv(0)
    }
}
#[doc = "SDIO0 FCLK selection"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sdio0fclksel(pub u32);
impl Sdio0fclksel {
    #[doc = "SDIO0 Functional Clock Source Selection. ."]
    #[inline(always)]
    pub const fn sel(&self) -> super::vals::Sdio0fclkselSel {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::Sdio0fclkselSel::from_bits(val as u8)
    }
    #[doc = "SDIO0 Functional Clock Source Selection. ."]
    #[inline(always)]
    pub fn set_sel(&mut self, val: super::vals::Sdio0fclkselSel) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
}
impl Default for Sdio0fclksel {
    #[inline(always)]
    fn default() -> Sdio0fclksel {
        Sdio0fclksel(0)
    }
}
#[doc = "SDIO1 FCLK divider"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sdio1fclkdiv(pub u32);
impl Sdio1fclkdiv {
    #[doc = "Clock Divider Value Selection. . . 0: Divide by 1. ... 255: Divide by 256."]
    #[inline(always)]
    pub const fn div(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Clock Divider Value Selection. . . 0: Divide by 1. ... 255: Divide by 256."]
    #[inline(always)]
    pub fn set_div(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Resets the divider counter. Can be used to make sure a new divider value is used right away rather than completing the previous count."]
    #[inline(always)]
    pub const fn reset(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "Resets the divider counter. Can be used to make sure a new divider value is used right away rather than completing the previous count."]
    #[inline(always)]
    pub fn set_reset(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "Halts the divider counter. The intent is to allow the divider's clock source to be changed without the risk of a glitch at the output."]
    #[inline(always)]
    pub const fn halt(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Halts the divider counter. The intent is to allow the divider's clock source to be changed without the risk of a glitch at the output."]
    #[inline(always)]
    pub fn set_halt(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Divider status flag. Set when a change is made to the divider value, cleared when the change is complete."]
    #[inline(always)]
    pub const fn reqflag(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Divider status flag. Set when a change is made to the divider value, cleared when the change is complete."]
    #[inline(always)]
    pub fn set_reqflag(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Sdio1fclkdiv {
    #[inline(always)]
    fn default() -> Sdio1fclkdiv {
        Sdio1fclkdiv(0)
    }
}
#[doc = "SDIO1 FCLK selection"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sdio1fclksel(pub u32);
impl Sdio1fclksel {
    #[doc = "SDIO1 Functional Clock Source Selection. ."]
    #[inline(always)]
    pub const fn sel(&self) -> super::vals::Sdio1fclkselSel {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::Sdio1fclkselSel::from_bits(val as u8)
    }
    #[doc = "SDIO1 Functional Clock Source Selection. ."]
    #[inline(always)]
    pub fn set_sel(&mut self, val: super::vals::Sdio1fclkselSel) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
}
impl Default for Sdio1fclksel {
    #[inline(always)]
    fn default() -> Sdio1fclksel {
        Sdio1fclksel(0)
    }
}
#[doc = "system cpu AHB clock divider"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Syscpuahbclkdiv(pub u32);
impl Syscpuahbclkdiv {
    #[doc = "Clock Divider Value Selection. . . 0: Divide by 1. ... 255: Divide by 256."]
    #[inline(always)]
    pub const fn div(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Clock Divider Value Selection. . . 0: Divide by 1. ... 255: Divide by 256."]
    #[inline(always)]
    pub fn set_div(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Divider status flag. Set when a change is made to the divider value, cleared when the change is complete."]
    #[inline(always)]
    pub const fn reqflag(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Divider status flag. Set when a change is made to the divider value, cleared when the change is complete."]
    #[inline(always)]
    pub fn set_reqflag(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Syscpuahbclkdiv {
    #[inline(always)]
    fn default() -> Syscpuahbclkdiv {
        Syscpuahbclkdiv(0)
    }
}
#[doc = "system oscillator bypass"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sysoscbypass(pub u32);
impl Sysoscbypass {
    #[doc = "Extenal Clock Source Selection."]
    #[inline(always)]
    pub const fn sel(&self) -> super::vals::SysoscbypassSel {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::SysoscbypassSel::from_bits(val as u8)
    }
    #[doc = "Extenal Clock Source Selection."]
    #[inline(always)]
    pub fn set_sel(&mut self, val: super::vals::SysoscbypassSel) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
}
impl Default for Sysoscbypass {
    #[inline(always)]
    fn default() -> Sysoscbypass {
        Sysoscbypass(0)
    }
}
#[doc = "system oscillator control 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sysoscctl0(pub u32);
impl Sysoscctl0 {
    #[doc = "Enable signal for low power mode. . ."]
    #[inline(always)]
    pub const fn lp_enable(&self) -> super::vals::LpEnable {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::LpEnable::from_bits(val as u8)
    }
    #[doc = "Enable signal for low power mode. . ."]
    #[inline(always)]
    pub fn set_lp_enable(&mut self, val: super::vals::LpEnable) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Enable signal for external bypass clock. . ."]
    #[inline(always)]
    pub const fn bypass_enable(&self) -> super::vals::BypassEnable {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::BypassEnable::from_bits(val as u8)
    }
    #[doc = "Enable signal for external bypass clock. . ."]
    #[inline(always)]
    pub fn set_bypass_enable(&mut self, val: super::vals::BypassEnable) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
}
impl Default for Sysoscctl0 {
    #[inline(always)]
    fn default() -> Sysoscctl0 {
        Sysoscctl0(0)
    }
}
#[doc = "system pll0 clock selection"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Syspll0clksel(pub u32);
impl Syspll0clksel {
    #[doc = "System PLL Clock Source Selection. . ."]
    #[inline(always)]
    pub const fn sel(&self) -> super::vals::Syspll0clkselSel {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::Syspll0clkselSel::from_bits(val as u8)
    }
    #[doc = "System PLL Clock Source Selection. . ."]
    #[inline(always)]
    pub fn set_sel(&mut self, val: super::vals::Syspll0clkselSel) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
}
impl Default for Syspll0clksel {
    #[inline(always)]
    fn default() -> Syspll0clksel {
        Syspll0clksel(0)
    }
}
#[doc = "system pll0 control0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Syspll0ctl0(pub u32);
impl Syspll0ctl0 {
    #[doc = "SYSPLL0 BYPASS Mode"]
    #[inline(always)]
    pub const fn bypass(&self) -> super::vals::Bypass {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Bypass::from_bits(val as u8)
    }
    #[doc = "SYSPLL0 BYPASS Mode"]
    #[inline(always)]
    pub fn set_bypass(&mut self, val: super::vals::Bypass) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "SYSPLL0 Reset:"]
    #[inline(always)]
    pub const fn reset(&self) -> super::vals::Reset {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Reset::from_bits(val as u8)
    }
    #[doc = "SYSPLL0 Reset:"]
    #[inline(always)]
    pub fn set_reset(&mut self, val: super::vals::Reset) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Hold Ring Off Control: This bit is used to avoid multi wave within the VCO."]
    #[inline(always)]
    pub const fn holdringoff_ena(&self) -> super::vals::HoldringoffEna {
        let val = (self.0 >> 13usize) & 0x01;
        super::vals::HoldringoffEna::from_bits(val as u8)
    }
    #[doc = "Hold Ring Off Control: This bit is used to avoid multi wave within the VCO."]
    #[inline(always)]
    pub fn set_holdringoff_ena(&mut self, val: super::vals::HoldringoffEna) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "Multiplication Factor for FSYSPLL0_OUTPUT:"]
    #[inline(always)]
    pub const fn mult(&self) -> super::vals::Mult {
        let val = (self.0 >> 16usize) & 0xff;
        super::vals::Mult::from_bits(val as u8)
    }
    #[doc = "Multiplication Factor for FSYSPLL0_OUTPUT:"]
    #[inline(always)]
    pub fn set_mult(&mut self, val: super::vals::Mult) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val.to_bits() as u32) & 0xff) << 16usize);
    }
}
impl Default for Syspll0ctl0 {
    #[inline(always)]
    fn default() -> Syspll0ctl0 {
        Syspll0ctl0(0)
    }
}
#[doc = "system pll0 denom"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Syspll0denom(pub u32);
impl Syspll0denom {
    #[doc = "This field contains the denominator of the SYSPLL0 fractional loop divider."]
    #[inline(always)]
    pub const fn denom(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x3fff_ffff;
        val as u32
    }
    #[doc = "This field contains the denominator of the SYSPLL0 fractional loop divider."]
    #[inline(always)]
    pub fn set_denom(&mut self, val: u32) {
        self.0 = (self.0 & !(0x3fff_ffff << 0usize)) | (((val as u32) & 0x3fff_ffff) << 0usize);
    }
}
impl Default for Syspll0denom {
    #[inline(always)]
    fn default() -> Syspll0denom {
        Syspll0denom(0)
    }
}
#[doc = "system pll0 lock time"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Syspll0locktimediv2(pub u32);
impl Syspll0locktimediv2 {
    #[doc = "SYSPLL0 Lock Time Divide by 2: Programmed lock time is in uS (micro-seconds) and is programmed as half the actual lock time value."]
    #[inline(always)]
    pub const fn locktimediv2(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "SYSPLL0 Lock Time Divide by 2: Programmed lock time is in uS (micro-seconds) and is programmed as half the actual lock time value."]
    #[inline(always)]
    pub fn set_locktimediv2(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Syspll0locktimediv2 {
    #[inline(always)]
    fn default() -> Syspll0locktimediv2 {
        Syspll0locktimediv2(0)
    }
}
#[doc = "system pll0 number"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Syspll0num(pub u32);
impl Syspll0num {
    #[doc = "This field contains the numerator of the SYSPLL0 fractional loop divider."]
    #[inline(always)]
    pub const fn num(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x3fff_ffff;
        val as u32
    }
    #[doc = "This field contains the numerator of the SYSPLL0 fractional loop divider."]
    #[inline(always)]
    pub fn set_num(&mut self, val: u32) {
        self.0 = (self.0 & !(0x3fff_ffff << 0usize)) | (((val as u32) & 0x3fff_ffff) << 0usize);
    }
}
impl Default for Syspll0num {
    #[inline(always)]
    fn default() -> Syspll0num {
        Syspll0num(0)
    }
}
#[doc = "sys pll0 PFD"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Syspll0pfd(pub u32);
impl Syspll0pfd {
    #[doc = "PLL Fractional Divider 0: Controls the fractional divider value. Valid PFD values are decimal 12-35."]
    #[inline(always)]
    pub const fn pfd0(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    #[doc = "PLL Fractional Divider 0: Controls the fractional divider value. Valid PFD values are decimal 12-35."]
    #[inline(always)]
    pub fn set_pfd0(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
    }
    #[doc = "PFD0 Clock Ready Status Flag: Read as '1' clock ready. Cleared by writing a '1'."]
    #[inline(always)]
    pub const fn pfd0_clkrdy(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "PFD0 Clock Ready Status Flag: Read as '1' clock ready. Cleared by writing a '1'."]
    #[inline(always)]
    pub fn set_pfd0_clkrdy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "PFD0 Clock Gate: 0: PFD0 clock is not gated. 1: PFD0 clock is gated"]
    #[inline(always)]
    pub const fn pfd0_clkgate(&self) -> super::vals::Pfd0Clkgate {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Pfd0Clkgate::from_bits(val as u8)
    }
    #[doc = "PFD0 Clock Gate: 0: PFD0 clock is not gated. 1: PFD0 clock is gated"]
    #[inline(always)]
    pub fn set_pfd0_clkgate(&mut self, val: super::vals::Pfd0Clkgate) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "PLL Fractional Divider 1: Controls the fractional divider value. Valid PFD values are decimal 12-35."]
    #[inline(always)]
    pub const fn pfd1(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x3f;
        val as u8
    }
    #[doc = "PLL Fractional Divider 1: Controls the fractional divider value. Valid PFD values are decimal 12-35."]
    #[inline(always)]
    pub fn set_pfd1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 8usize)) | (((val as u32) & 0x3f) << 8usize);
    }
    #[doc = "PFD1 Clock Ready Status Flag: Read as '1' clock ready. Cleared by writing a '1'."]
    #[inline(always)]
    pub const fn pfd1_clkrdy(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "PFD1 Clock Ready Status Flag: Read as '1' clock ready. Cleared by writing a '1'."]
    #[inline(always)]
    pub fn set_pfd1_clkrdy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "PFD1 Clock Gate: 0: PFD1 clock is not gated. 1: PFD1 clock is gated."]
    #[inline(always)]
    pub const fn pfd1_clkgate(&self) -> super::vals::Pfd1Clkgate {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::Pfd1Clkgate::from_bits(val as u8)
    }
    #[doc = "PFD1 Clock Gate: 0: PFD1 clock is not gated. 1: PFD1 clock is gated."]
    #[inline(always)]
    pub fn set_pfd1_clkgate(&mut self, val: super::vals::Pfd1Clkgate) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
    #[doc = "PLL Fractional Divider 2: Controls the fractional divider value. Valid PFD values are decimal 12-35."]
    #[inline(always)]
    pub const fn pfd2(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x3f;
        val as u8
    }
    #[doc = "PLL Fractional Divider 2: Controls the fractional divider value. Valid PFD values are decimal 12-35."]
    #[inline(always)]
    pub fn set_pfd2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 16usize)) | (((val as u32) & 0x3f) << 16usize);
    }
    #[doc = "PFD2 Clock Ready Status Flag: Read as '1' clock ready. Cleared by writing a '1'."]
    #[inline(always)]
    pub const fn pfd2_clkrdy(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "PFD2 Clock Ready Status Flag: Read as '1' clock ready. Cleared by writing a '1'."]
    #[inline(always)]
    pub fn set_pfd2_clkrdy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "PFD2 Clock Gate: 0: PFD2 clock is not gated. 1: PFD2 clock is gated."]
    #[inline(always)]
    pub const fn pfd2_clkgate(&self) -> super::vals::Pfd2Clkgate {
        let val = (self.0 >> 23usize) & 0x01;
        super::vals::Pfd2Clkgate::from_bits(val as u8)
    }
    #[doc = "PFD2 Clock Gate: 0: PFD2 clock is not gated. 1: PFD2 clock is gated."]
    #[inline(always)]
    pub fn set_pfd2_clkgate(&mut self, val: super::vals::Pfd2Clkgate) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "PLL Fractional Divider 3: Controls the fractional divider value. Valid PFD values are decimal 12-35."]
    #[inline(always)]
    pub const fn pfd3(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x3f;
        val as u8
    }
    #[doc = "PLL Fractional Divider 3: Controls the fractional divider value. Valid PFD values are decimal 12-35."]
    #[inline(always)]
    pub fn set_pfd3(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 24usize)) | (((val as u32) & 0x3f) << 24usize);
    }
    #[doc = "PFD3 Clock Ready Status Flag: Read as '1' clock ready. Cleared by writing a '1'."]
    #[inline(always)]
    pub const fn pfd3_clkrdy(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "PFD3 Clock Ready Status Flag: Read as '1' clock ready. Cleared by writing a '1'."]
    #[inline(always)]
    pub fn set_pfd3_clkrdy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "PFD3 Clock Gate: 0: PFD3 clock is not gated. 1: PFD3 clock is gated."]
    #[inline(always)]
    pub const fn pfd3_clkgate(&self) -> super::vals::Pfd3Clkgate {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::Pfd3Clkgate::from_bits(val as u8)
    }
    #[doc = "PFD3 Clock Gate: 0: PFD3 clock is not gated. 1: PFD3 clock is gated."]
    #[inline(always)]
    pub fn set_pfd3_clkgate(&mut self, val: super::vals::Pfd3Clkgate) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Syspll0pfd {
    #[inline(always)]
    fn default() -> Syspll0pfd {
        Syspll0pfd(0)
    }
}
#[doc = "system tick fclk divider"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Systickfclkdiv(pub u32);
impl Systickfclkdiv {
    #[doc = "Clock Divider Value Selection. . . 0: Divide by 1. ... 255: Divide by 256."]
    #[inline(always)]
    pub const fn div(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Clock Divider Value Selection. . . 0: Divide by 1. ... 255: Divide by 256."]
    #[inline(always)]
    pub fn set_div(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Resets the divider counter. Can be used to make sure a new divider value is used right away rather than completing the previous count."]
    #[inline(always)]
    pub const fn reset(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "Resets the divider counter. Can be used to make sure a new divider value is used right away rather than completing the previous count."]
    #[inline(always)]
    pub fn set_reset(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "Halts the divider counter. The intent is to allow the divider's clock source to be changed without the risk of a glitch at the output."]
    #[inline(always)]
    pub const fn halt(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Halts the divider counter. The intent is to allow the divider's clock source to be changed without the risk of a glitch at the output."]
    #[inline(always)]
    pub fn set_halt(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Divider status flag. Set when a change is made to the divider value, cleared when the change is complete."]
    #[inline(always)]
    pub const fn reqflag(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Divider status flag. Set when a change is made to the divider value, cleared when the change is complete."]
    #[inline(always)]
    pub fn set_reqflag(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Systickfclkdiv {
    #[inline(always)]
    fn default() -> Systickfclkdiv {
        Systickfclkdiv(0)
    }
}
#[doc = "system tick fclk selection"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Systickfclksel(pub u32);
impl Systickfclksel {
    #[doc = "SYSTICK Functional Clock Source Selection. . ."]
    #[inline(always)]
    pub const fn sel(&self) -> super::vals::SystickfclkselSel {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::SystickfclkselSel::from_bits(val as u8)
    }
    #[doc = "SYSTICK Functional Clock Source Selection. . ."]
    #[inline(always)]
    pub fn set_sel(&mut self, val: super::vals::SystickfclkselSel) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
}
impl Default for Systickfclksel {
    #[inline(always)]
    fn default() -> Systickfclksel {
        Systickfclksel(0)
    }
}
#[doc = "USBHS Fclk divider"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Usbhsfclkdiv(pub u32);
impl Usbhsfclkdiv {
    #[doc = "Clock Divider Value Selection. . . 0: Divide by 1. ... 255: Divide by 256."]
    #[inline(always)]
    pub const fn div(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Clock Divider Value Selection. . . 0: Divide by 1. ... 255: Divide by 256."]
    #[inline(always)]
    pub fn set_div(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Resets the divider counter. Can be used to make sure a new divider value is used right away rather than completing the previous count."]
    #[inline(always)]
    pub const fn reset(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "Resets the divider counter. Can be used to make sure a new divider value is used right away rather than completing the previous count."]
    #[inline(always)]
    pub fn set_reset(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "Halts the divider counter. The intent is to allow the divider clock source to be changed without the risk of a glitch at the output."]
    #[inline(always)]
    pub const fn halt(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Halts the divider counter. The intent is to allow the divider clock source to be changed without the risk of a glitch at the output."]
    #[inline(always)]
    pub fn set_halt(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Divider status flag. Set when a change is made to the divider value, cleared when the change is complete."]
    #[inline(always)]
    pub const fn reqflag(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Divider status flag. Set when a change is made to the divider value, cleared when the change is complete."]
    #[inline(always)]
    pub fn set_reqflag(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Usbhsfclkdiv {
    #[inline(always)]
    fn default() -> Usbhsfclkdiv {
        Usbhsfclkdiv(0)
    }
}
#[doc = "USBHS Fclk selection"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Usbhsfclksel(pub u32);
impl Usbhsfclksel {
    #[doc = "USB HS Functional Clock Source Selection. . ."]
    #[inline(always)]
    pub const fn sel(&self) -> super::vals::UsbhsfclkselSel {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::UsbhsfclkselSel::from_bits(val as u8)
    }
    #[doc = "USB HS Functional Clock Source Selection. . ."]
    #[inline(always)]
    pub fn set_sel(&mut self, val: super::vals::UsbhsfclkselSel) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
}
impl Default for Usbhsfclksel {
    #[inline(always)]
    fn default() -> Usbhsfclksel {
        Usbhsfclksel(0)
    }
}
#[doc = "UTICK fclk selection"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Utickfclksel(pub u32);
impl Utickfclksel {
    #[doc = "uTICK Functional Clock Source Selection. ."]
    #[inline(always)]
    pub const fn sel(&self) -> super::vals::UtickfclkselSel {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::UtickfclkselSel::from_bits(val as u8)
    }
    #[doc = "uTICK Functional Clock Source Selection. ."]
    #[inline(always)]
    pub fn set_sel(&mut self, val: super::vals::UtickfclkselSel) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
}
impl Default for Utickfclksel {
    #[inline(always)]
    fn default() -> Utickfclksel {
        Utickfclksel(0)
    }
}
#[doc = "32k wake clock divider"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Wakeclk32khzdiv(pub u32);
impl Wakeclk32khzdiv {
    #[doc = "Halts the divider counter. The intent is to allow the divider's clock source to be changed without the risk of a glitch at the output."]
    #[inline(always)]
    pub const fn halt(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Halts the divider counter. The intent is to allow the divider's clock source to be changed without the risk of a glitch at the output."]
    #[inline(always)]
    pub fn set_halt(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
}
impl Default for Wakeclk32khzdiv {
    #[inline(always)]
    fn default() -> Wakeclk32khzdiv {
        Wakeclk32khzdiv(0)
    }
}
#[doc = "32k wake clock selection"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Wakeclk32khzsel(pub u32);
impl Wakeclk32khzsel {
    #[doc = "32KHz Wake Clock Low Power Functional Clock Source Selection. . ."]
    #[inline(always)]
    pub const fn sel(&self) -> super::vals::Wakeclk32khzselSel {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::Wakeclk32khzselSel::from_bits(val as u8)
    }
    #[doc = "32KHz Wake Clock Low Power Functional Clock Source Selection. . ."]
    #[inline(always)]
    pub fn set_sel(&mut self, val: super::vals::Wakeclk32khzselSel) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
}
impl Default for Wakeclk32khzsel {
    #[inline(always)]
    fn default() -> Wakeclk32khzsel {
        Wakeclk32khzsel(0)
    }
}
#[doc = "wdt clock selection"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Wdt0fclksel(pub u32);
impl Wdt0fclksel {
    #[doc = "WDT0 Functional Clock Source Selection. ."]
    #[inline(always)]
    pub const fn sel(&self) -> super::vals::Wdt0fclkselSel {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::Wdt0fclkselSel::from_bits(val as u8)
    }
    #[doc = "WDT0 Functional Clock Source Selection. ."]
    #[inline(always)]
    pub fn set_sel(&mut self, val: super::vals::Wdt0fclkselSel) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
}
impl Default for Wdt0fclksel {
    #[inline(always)]
    fn default() -> Wdt0fclksel {
        Wdt0fclksel(0)
    }
}

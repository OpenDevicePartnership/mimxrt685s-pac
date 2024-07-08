#[doc = "MRT Control register. This register controls the MRT modes."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctrl(pub u32);
impl Ctrl {
    #[doc = "Enable the TIMERn interrupt."]
    #[inline(always)]
    pub const fn inten(&self) -> super::vals::Inten {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Inten::from_bits(val as u8)
    }
    #[doc = "Enable the TIMERn interrupt."]
    #[inline(always)]
    pub fn set_inten(&mut self, val: super::vals::Inten) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Selects timer mode."]
    #[inline(always)]
    pub const fn mode(&self) -> super::vals::Mode {
        let val = (self.0 >> 1usize) & 0x03;
        super::vals::Mode::from_bits(val as u8)
    }
    #[doc = "Selects timer mode."]
    #[inline(always)]
    pub fn set_mode(&mut self, val: super::vals::Mode) {
        self.0 = (self.0 & !(0x03 << 1usize)) | (((val.to_bits() as u32) & 0x03) << 1usize);
    }
}
impl Default for Ctrl {
    #[inline(always)]
    fn default() -> Ctrl {
        Ctrl(0)
    }
}
#[doc = "Idle channel register. This register returns the number of the first idle channel."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IdleCh(pub u32);
impl IdleCh {
    #[doc = "Idle channel. Reading the CHAN bits, returns the lowest idle timer channel. The number is positioned such that it can be used as an offset from the MRT base address in order to access the registers for the allocated channel. If all timer channels are running, CHAN = 0xF. See text above for more details."]
    #[inline(always)]
    pub const fn chan(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Idle channel. Reading the CHAN bits, returns the lowest idle timer channel. The number is positioned such that it can be used as an offset from the MRT base address in order to access the registers for the allocated channel. If all timer channels are running, CHAN = 0xF. See text above for more details."]
    #[inline(always)]
    pub fn set_chan(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
}
impl Default for IdleCh {
    #[inline(always)]
    fn default() -> IdleCh {
        IdleCh(0)
    }
}
#[doc = "MRT Time interval value register. This value is loaded into the TIMER register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Intval(pub u32);
impl Intval {
    #[doc = "Time interval load value. This value is loaded into the TIMERn register and the MRT channel n starts counting down from IVALUE -1. If the timer is idle, writing a non-zero value to this bit field starts the timer immediately. If the timer is running, writing a zero to this bit field does the following: If LOAD = 1, the timer stops immediately. If LOAD = 0, the timer stops at the end of the time interval."]
    #[inline(always)]
    pub const fn ivalue(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "Time interval load value. This value is loaded into the TIMERn register and the MRT channel n starts counting down from IVALUE -1. If the timer is idle, writing a non-zero value to this bit field starts the timer immediately. If the timer is running, writing a zero to this bit field does the following: If LOAD = 1, the timer stops immediately. If LOAD = 0, the timer stops at the end of the time interval."]
    #[inline(always)]
    pub fn set_ivalue(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
    #[doc = "Determines how the timer interval value IVALUE -1 is loaded into the TIMERn register. This bit is write-only. Reading this bit always returns 0."]
    #[inline(always)]
    pub const fn load(&self) -> super::vals::Load {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::Load::from_bits(val as u8)
    }
    #[doc = "Determines how the timer interval value IVALUE -1 is loaded into the TIMERn register. This bit is write-only. Reading this bit always returns 0."]
    #[inline(always)]
    pub fn set_load(&mut self, val: super::vals::Load) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Intval {
    #[inline(always)]
    fn default() -> Intval {
        Intval(0)
    }
}
#[doc = "Global interrupt flag register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IrqFlag(pub u32);
impl IrqFlag {
    #[doc = "Monitors the interrupt flag of TIMER0."]
    #[inline(always)]
    pub const fn gflag0(&self) -> super::vals::Gflag0 {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Gflag0::from_bits(val as u8)
    }
    #[doc = "Monitors the interrupt flag of TIMER0."]
    #[inline(always)]
    pub fn set_gflag0(&mut self, val: super::vals::Gflag0) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Monitors the interrupt flag of TIMER1. See description of channel 0."]
    #[inline(always)]
    pub const fn gflag1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Monitors the interrupt flag of TIMER1. See description of channel 0."]
    #[inline(always)]
    pub fn set_gflag1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Monitors the interrupt flag of TIMER2. See description of channel 0."]
    #[inline(always)]
    pub const fn gflag2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Monitors the interrupt flag of TIMER2. See description of channel 0."]
    #[inline(always)]
    pub fn set_gflag2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Monitors the interrupt flag of TIMER3. See description of channel 0."]
    #[inline(always)]
    pub const fn gflag3(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Monitors the interrupt flag of TIMER3. See description of channel 0."]
    #[inline(always)]
    pub fn set_gflag3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
}
impl Default for IrqFlag {
    #[inline(always)]
    fn default() -> IrqFlag {
        IrqFlag(0)
    }
}
#[doc = "Module Configuration register. This register provides information about this particular MRT instance, and allows choosing an overall mode for the idle channel feature."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Modcfg(pub u32);
impl Modcfg {
    #[doc = "Identifies the number of channels in this MRT.(4 channels on this device.)"]
    #[inline(always)]
    pub const fn noc(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Identifies the number of channels in this MRT.(4 channels on this device.)"]
    #[inline(always)]
    pub fn set_noc(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Identifies the number of timer bits in this MRT. (24 bits wide on this device.)"]
    #[inline(always)]
    pub const fn nob(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x1f;
        val as u8
    }
    #[doc = "Identifies the number of timer bits in this MRT. (24 bits wide on this device.)"]
    #[inline(always)]
    pub fn set_nob(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 4usize)) | (((val as u32) & 0x1f) << 4usize);
    }
    #[doc = "Selects the operating mode for the INUSE flags and the IDLE_CH register."]
    #[inline(always)]
    pub const fn multitask(&self) -> super::vals::Multitask {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::Multitask::from_bits(val as u8)
    }
    #[doc = "Selects the operating mode for the INUSE flags and the IDLE_CH register."]
    #[inline(always)]
    pub fn set_multitask(&mut self, val: super::vals::Multitask) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Modcfg {
    #[inline(always)]
    fn default() -> Modcfg {
        Modcfg(0)
    }
}
#[doc = "MRT Status register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Stat(pub u32);
impl Stat {
    #[doc = "Monitors the interrupt flag."]
    #[inline(always)]
    pub const fn intflag(&self) -> super::vals::Intflag {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Intflag::from_bits(val as u8)
    }
    #[doc = "Monitors the interrupt flag."]
    #[inline(always)]
    pub fn set_intflag(&mut self, val: super::vals::Intflag) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Indicates the state of TIMERn. This bit is read-only."]
    #[inline(always)]
    pub const fn run(&self) -> super::vals::Run {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Run::from_bits(val as u8)
    }
    #[doc = "Indicates the state of TIMERn. This bit is read-only."]
    #[inline(always)]
    pub fn set_run(&mut self, val: super::vals::Run) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Channel In Use flag. Operating details depend on the MULTITASK bit in the MODCFG register, and affects the use of IDLE_CH. See Idle channel register for details of the two operating modes."]
    #[inline(always)]
    pub const fn inuse(&self) -> super::vals::Inuse {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Inuse::from_bits(val as u8)
    }
    #[doc = "Channel In Use flag. Operating details depend on the MULTITASK bit in the MODCFG register, and affects the use of IDLE_CH. See Idle channel register for details of the two operating modes."]
    #[inline(always)]
    pub fn set_inuse(&mut self, val: super::vals::Inuse) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
}
impl Default for Stat {
    #[inline(always)]
    fn default() -> Stat {
        Stat(0)
    }
}
#[doc = "MRT Timer register. This register reads the value of the down-counter."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Timer(pub u32);
impl Timer {
    #[doc = "Holds the current timer value of the down-counter. The initial value of the TIMERn register is loaded as IVALUE - 1 from the INTVALn register either at the end of the time interval or immediately in the following cases: INTVALn register is updated in the idle state. INTVALn register is updated with LOAD = 1. When the timer is in idle state, reading this bit fields returns -1 (0x00FF FFFF)."]
    #[inline(always)]
    pub const fn value(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "Holds the current timer value of the down-counter. The initial value of the TIMERn register is loaded as IVALUE - 1 from the INTVALn register either at the end of the time interval or immediately in the following cases: INTVALn register is updated in the idle state. INTVALn register is updated with LOAD = 1. When the timer is in idle state, reading this bit fields returns -1 (0x00FF FFFF)."]
    #[inline(always)]
    pub fn set_value(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Timer {
    #[inline(always)]
    fn default() -> Timer {
        Timer(0)
    }
}

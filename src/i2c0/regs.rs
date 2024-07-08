#[doc = "Configuration for shared functions."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfg(pub u32);
impl Cfg {
    #[doc = "Master Enable. When disabled, configurations settings for the Master function are not changed, but the Master function is internally reset."]
    #[inline(always)]
    pub const fn msten(&self) -> super::vals::Msten {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Msten::from_bits(val as u8)
    }
    #[doc = "Master Enable. When disabled, configurations settings for the Master function are not changed, but the Master function is internally reset."]
    #[inline(always)]
    pub fn set_msten(&mut self, val: super::vals::Msten) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Slave Enable. When disabled, configurations settings for the Slave function are not changed, but the Slave function is internally reset."]
    #[inline(always)]
    pub const fn slven(&self) -> super::vals::Slven {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Slven::from_bits(val as u8)
    }
    #[doc = "Slave Enable. When disabled, configurations settings for the Slave function are not changed, but the Slave function is internally reset."]
    #[inline(always)]
    pub fn set_slven(&mut self, val: super::vals::Slven) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Monitor Enable. When disabled, configurations settings for the Monitor function are not changed, but the Monitor function is internally reset."]
    #[inline(always)]
    pub const fn monen(&self) -> super::vals::Monen {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Monen::from_bits(val as u8)
    }
    #[doc = "Monitor Enable. When disabled, configurations settings for the Monitor function are not changed, but the Monitor function is internally reset."]
    #[inline(always)]
    pub fn set_monen(&mut self, val: super::vals::Monen) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "I2C bus Time-out Enable. When disabled, the time-out function is internally reset."]
    #[inline(always)]
    pub const fn timeouten(&self) -> super::vals::Timeouten {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Timeouten::from_bits(val as u8)
    }
    #[doc = "I2C bus Time-out Enable. When disabled, the time-out function is internally reset."]
    #[inline(always)]
    pub fn set_timeouten(&mut self, val: super::vals::Timeouten) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Monitor function Clock Stretching."]
    #[inline(always)]
    pub const fn monclkstr(&self) -> super::vals::Monclkstr {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Monclkstr::from_bits(val as u8)
    }
    #[doc = "Monitor function Clock Stretching."]
    #[inline(always)]
    pub fn set_monclkstr(&mut self, val: super::vals::Monclkstr) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "High-speed mode Capable enable. Since High Speed mode alters the way I2C pins drive and filter, as well as the timing for certain I2C signalling, enabling High-speed mode applies to all functions: Master, Slave, and Monitor."]
    #[inline(always)]
    pub const fn hscapable(&self) -> super::vals::Hscapable {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::Hscapable::from_bits(val as u8)
    }
    #[doc = "High-speed mode Capable enable. Since High Speed mode alters the way I2C pins drive and filter, as well as the timing for certain I2C signalling, enabling High-speed mode applies to all functions: Master, Slave, and Monitor."]
    #[inline(always)]
    pub fn set_hscapable(&mut self, val: super::vals::Hscapable) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
}
impl Default for Cfg {
    #[inline(always)]
    fn default() -> Cfg {
        Cfg(0)
    }
}
#[doc = "Clock pre-divider for the entire I2C interface. This determines what time increments are used for the MSTTIME register, and controls some timing of the Slave function."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Clkdiv(pub u32);
impl Clkdiv {
    #[doc = "This field controls how the Flexcomm clock (FCLK) is used by the I2C functions that need an internal clock in order to operate. 0x0000 = FCLK is used directly by the I2C. 0x0001 = FCLK is divided by 2 before use. 0x0002 = FCLK is divided by 3 before use. 0xFFFF = FCLK is divided by 65,536 before use."]
    #[inline(always)]
    pub const fn divval(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "This field controls how the Flexcomm clock (FCLK) is used by the I2C functions that need an internal clock in order to operate. 0x0000 = FCLK is used directly by the I2C. 0x0001 = FCLK is divided by 2 before use. 0x0002 = FCLK is divided by 3 before use. 0xFFFF = FCLK is divided by 65,536 before use."]
    #[inline(always)]
    pub fn set_divval(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Clkdiv {
    #[inline(always)]
    fn default() -> Clkdiv {
        Clkdiv(0)
    }
}
#[doc = "Peripheral identification register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Id(pub u32);
impl Id {
    #[doc = "Aperture: encoded as (aperture size/4K) -1, so 0x00 means a 4K aperture."]
    #[inline(always)]
    pub const fn aperture(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Aperture: encoded as (aperture size/4K) -1, so 0x00 means a 4K aperture."]
    #[inline(always)]
    pub fn set_aperture(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Minor revision of module implementation."]
    #[inline(always)]
    pub const fn minor_rev(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "Minor revision of module implementation."]
    #[inline(always)]
    pub fn set_minor_rev(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[doc = "Major revision of module implementation."]
    #[inline(always)]
    pub const fn major_rev(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x0f;
        val as u8
    }
    #[doc = "Major revision of module implementation."]
    #[inline(always)]
    pub fn set_major_rev(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
    }
    #[doc = "Module identifier for the selected function."]
    #[inline(always)]
    pub const fn id(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Module identifier for the selected function."]
    #[inline(always)]
    pub fn set_id(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Id {
    #[inline(always)]
    fn default() -> Id {
        Id(0)
    }
}
#[doc = "Interrupt Enable Clear register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Intenclr(pub u32);
impl Intenclr {
    #[doc = "Master Pending interrupt clear. Writing 1 to this bit clears the corresponding bit in the INTENSET register if implemented."]
    #[inline(always)]
    pub const fn mstpendingclr(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Master Pending interrupt clear. Writing 1 to this bit clears the corresponding bit in the INTENSET register if implemented."]
    #[inline(always)]
    pub fn set_mstpendingclr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Master Arbitration Loss interrupt clear."]
    #[inline(always)]
    pub const fn mstarblossclr(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Master Arbitration Loss interrupt clear."]
    #[inline(always)]
    pub fn set_mstarblossclr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Master Start/Stop Error interrupt clear."]
    #[inline(always)]
    pub const fn mstststperrclr(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Master Start/Stop Error interrupt clear."]
    #[inline(always)]
    pub fn set_mstststperrclr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Slave Pending interrupt clear."]
    #[inline(always)]
    pub const fn slvpendingclr(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Slave Pending interrupt clear."]
    #[inline(always)]
    pub fn set_slvpendingclr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Slave Not Stretching interrupt clear."]
    #[inline(always)]
    pub const fn slvnotstrclr(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Slave Not Stretching interrupt clear."]
    #[inline(always)]
    pub fn set_slvnotstrclr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Slave Deselect interrupt clear."]
    #[inline(always)]
    pub const fn slvdeselclr(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Slave Deselect interrupt clear."]
    #[inline(always)]
    pub fn set_slvdeselclr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Monitor data Ready interrupt clear."]
    #[inline(always)]
    pub const fn monrdyclr(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Monitor data Ready interrupt clear."]
    #[inline(always)]
    pub fn set_monrdyclr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Monitor Overrun interrupt clear."]
    #[inline(always)]
    pub const fn monovclr(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Monitor Overrun interrupt clear."]
    #[inline(always)]
    pub fn set_monovclr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Monitor Idle interrupt clear."]
    #[inline(always)]
    pub const fn monidleclr(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Monitor Idle interrupt clear."]
    #[inline(always)]
    pub fn set_monidleclr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Event time-out interrupt clear."]
    #[inline(always)]
    pub const fn eventtimeoutclr(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Event time-out interrupt clear."]
    #[inline(always)]
    pub fn set_eventtimeoutclr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "SCL time-out interrupt clear."]
    #[inline(always)]
    pub const fn scltimeoutclr(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "SCL time-out interrupt clear."]
    #[inline(always)]
    pub fn set_scltimeoutclr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
}
impl Default for Intenclr {
    #[inline(always)]
    fn default() -> Intenclr {
        Intenclr(0)
    }
}
#[doc = "Interrupt Enable Set and read register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Intenset(pub u32);
impl Intenset {
    #[doc = "Master Pending interrupt Enable."]
    #[inline(always)]
    pub const fn mstpendingen(&self) -> super::vals::Mstpendingen {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Mstpendingen::from_bits(val as u8)
    }
    #[doc = "Master Pending interrupt Enable."]
    #[inline(always)]
    pub fn set_mstpendingen(&mut self, val: super::vals::Mstpendingen) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Master Arbitration Loss interrupt Enable."]
    #[inline(always)]
    pub const fn mstarblossen(&self) -> super::vals::Mstarblossen {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Mstarblossen::from_bits(val as u8)
    }
    #[doc = "Master Arbitration Loss interrupt Enable."]
    #[inline(always)]
    pub fn set_mstarblossen(&mut self, val: super::vals::Mstarblossen) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "Master Start/Stop Error interrupt Enable."]
    #[inline(always)]
    pub const fn mstststperren(&self) -> super::vals::Mstststperren {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Mstststperren::from_bits(val as u8)
    }
    #[doc = "Master Start/Stop Error interrupt Enable."]
    #[inline(always)]
    pub fn set_mstststperren(&mut self, val: super::vals::Mstststperren) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Slave Pending interrupt Enable."]
    #[inline(always)]
    pub const fn slvpendingen(&self) -> super::vals::Slvpendingen {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Slvpendingen::from_bits(val as u8)
    }
    #[doc = "Slave Pending interrupt Enable."]
    #[inline(always)]
    pub fn set_slvpendingen(&mut self, val: super::vals::Slvpendingen) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Slave Not Stretching interrupt Enable."]
    #[inline(always)]
    pub const fn slvnotstren(&self) -> super::vals::Slvnotstren {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::Slvnotstren::from_bits(val as u8)
    }
    #[doc = "Slave Not Stretching interrupt Enable."]
    #[inline(always)]
    pub fn set_slvnotstren(&mut self, val: super::vals::Slvnotstren) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "Slave Deselect interrupt Enable."]
    #[inline(always)]
    pub const fn slvdeselen(&self) -> super::vals::Slvdeselen {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::Slvdeselen::from_bits(val as u8)
    }
    #[doc = "Slave Deselect interrupt Enable."]
    #[inline(always)]
    pub fn set_slvdeselen(&mut self, val: super::vals::Slvdeselen) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
    #[doc = "Monitor data Ready interrupt Enable."]
    #[inline(always)]
    pub const fn monrdyen(&self) -> super::vals::Monrdyen {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::Monrdyen::from_bits(val as u8)
    }
    #[doc = "Monitor data Ready interrupt Enable."]
    #[inline(always)]
    pub fn set_monrdyen(&mut self, val: super::vals::Monrdyen) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "Monitor Overrun interrupt Enable."]
    #[inline(always)]
    pub const fn monoven(&self) -> super::vals::Monoven {
        let val = (self.0 >> 17usize) & 0x01;
        super::vals::Monoven::from_bits(val as u8)
    }
    #[doc = "Monitor Overrun interrupt Enable."]
    #[inline(always)]
    pub fn set_monoven(&mut self, val: super::vals::Monoven) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
    #[doc = "Monitor Idle interrupt Enable."]
    #[inline(always)]
    pub const fn monidleen(&self) -> super::vals::Monidleen {
        let val = (self.0 >> 19usize) & 0x01;
        super::vals::Monidleen::from_bits(val as u8)
    }
    #[doc = "Monitor Idle interrupt Enable."]
    #[inline(always)]
    pub fn set_monidleen(&mut self, val: super::vals::Monidleen) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
    #[doc = "Event time-out interrupt Enable."]
    #[inline(always)]
    pub const fn eventtimeouten(&self) -> super::vals::Eventtimeouten {
        let val = (self.0 >> 24usize) & 0x01;
        super::vals::Eventtimeouten::from_bits(val as u8)
    }
    #[doc = "Event time-out interrupt Enable."]
    #[inline(always)]
    pub fn set_eventtimeouten(&mut self, val: super::vals::Eventtimeouten) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
    #[doc = "SCL time-out interrupt Enable."]
    #[inline(always)]
    pub const fn scltimeouten(&self) -> super::vals::Scltimeouten {
        let val = (self.0 >> 25usize) & 0x01;
        super::vals::Scltimeouten::from_bits(val as u8)
    }
    #[doc = "SCL time-out interrupt Enable."]
    #[inline(always)]
    pub fn set_scltimeouten(&mut self, val: super::vals::Scltimeouten) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val.to_bits() as u32) & 0x01) << 25usize);
    }
}
impl Default for Intenset {
    #[inline(always)]
    fn default() -> Intenset {
        Intenset(0)
    }
}
#[doc = "Interrupt Status register for Master, Slave, and Monitor functions."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Intstat(pub u32);
impl Intstat {
    #[doc = "Master Pending."]
    #[inline(always)]
    pub const fn mstpending(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Master Pending."]
    #[inline(always)]
    pub fn set_mstpending(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Master Arbitration Loss flag."]
    #[inline(always)]
    pub const fn mstarbloss(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Master Arbitration Loss flag."]
    #[inline(always)]
    pub fn set_mstarbloss(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Master Start/Stop Error flag."]
    #[inline(always)]
    pub const fn mstststperr(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Master Start/Stop Error flag."]
    #[inline(always)]
    pub fn set_mstststperr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Slave Pending."]
    #[inline(always)]
    pub const fn slvpending(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Slave Pending."]
    #[inline(always)]
    pub fn set_slvpending(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Slave Not Stretching status."]
    #[inline(always)]
    pub const fn slvnotstr(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Slave Not Stretching status."]
    #[inline(always)]
    pub fn set_slvnotstr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Slave Deselected flag."]
    #[inline(always)]
    pub const fn slvdesel(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Slave Deselected flag."]
    #[inline(always)]
    pub fn set_slvdesel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Monitor Ready."]
    #[inline(always)]
    pub const fn monrdy(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Monitor Ready."]
    #[inline(always)]
    pub fn set_monrdy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Monitor Overflow flag."]
    #[inline(always)]
    pub const fn monov(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Monitor Overflow flag."]
    #[inline(always)]
    pub fn set_monov(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Monitor Idle flag."]
    #[inline(always)]
    pub const fn monidle(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Monitor Idle flag."]
    #[inline(always)]
    pub fn set_monidle(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Event time-out Interrupt flag."]
    #[inline(always)]
    pub const fn eventtimeout(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Event time-out Interrupt flag."]
    #[inline(always)]
    pub fn set_eventtimeout(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "SCL time-out Interrupt flag."]
    #[inline(always)]
    pub const fn scltimeout(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "SCL time-out Interrupt flag."]
    #[inline(always)]
    pub fn set_scltimeout(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
}
impl Default for Intstat {
    #[inline(always)]
    fn default() -> Intstat {
        Intstat(0)
    }
}
#[doc = "Monitor receiver data register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Monrxdat(pub u32);
impl Monrxdat {
    #[doc = "Monitor function Receiver Data. This reflects every data byte that passes on the I2C pins."]
    #[inline(always)]
    pub const fn monrxdat(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Monitor function Receiver Data. This reflects every data byte that passes on the I2C pins."]
    #[inline(always)]
    pub fn set_monrxdat(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Monitor Received Start."]
    #[inline(always)]
    pub const fn monstart(&self) -> super::vals::Monstart {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Monstart::from_bits(val as u8)
    }
    #[doc = "Monitor Received Start."]
    #[inline(always)]
    pub fn set_monstart(&mut self, val: super::vals::Monstart) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Monitor Received Repeated Start."]
    #[inline(always)]
    pub const fn monrestart(&self) -> super::vals::Monrestart {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::Monrestart::from_bits(val as u8)
    }
    #[doc = "Monitor Received Repeated Start."]
    #[inline(always)]
    pub fn set_monrestart(&mut self, val: super::vals::Monrestart) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "Monitor Received NACK."]
    #[inline(always)]
    pub const fn monnack(&self) -> super::vals::Monnack {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::Monnack::from_bits(val as u8)
    }
    #[doc = "Monitor Received NACK."]
    #[inline(always)]
    pub fn set_monnack(&mut self, val: super::vals::Monnack) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
}
impl Default for Monrxdat {
    #[inline(always)]
    fn default() -> Monrxdat {
        Monrxdat(0)
    }
}
#[doc = "Master control register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mstctl(pub u32);
impl Mstctl {
    #[doc = "Master Continue. This bit is write-only."]
    #[inline(always)]
    pub const fn mstcontinue(&self) -> super::vals::Mstcontinue {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Mstcontinue::from_bits(val as u8)
    }
    #[doc = "Master Continue. This bit is write-only."]
    #[inline(always)]
    pub fn set_mstcontinue(&mut self, val: super::vals::Mstcontinue) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Master Start control. This bit is write-only."]
    #[inline(always)]
    pub const fn mststart(&self) -> super::vals::Mststart {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Mststart::from_bits(val as u8)
    }
    #[doc = "Master Start control. This bit is write-only."]
    #[inline(always)]
    pub fn set_mststart(&mut self, val: super::vals::Mststart) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Master Stop control. This bit is write-only."]
    #[inline(always)]
    pub const fn mststop(&self) -> super::vals::Mststop {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Mststop::from_bits(val as u8)
    }
    #[doc = "Master Stop control. This bit is write-only."]
    #[inline(always)]
    pub fn set_mststop(&mut self, val: super::vals::Mststop) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "Master DMA enable. Data operations of the I2C can be performed with DMA. Protocol type operations such as Start, address, Stop, and address match must always be done with software, typically via an interrupt. Address acknowledgement must also be done by software except when the I2C is configured to be HSCAPABLE (and address acknowledgement is handled entirely by hardware) or when Automatic Operation is enabled. When a DMA data transfer is complete, MSTDMA must be cleared prior to beginning the next operation, typically a Start or Stop.This bit is read/write."]
    #[inline(always)]
    pub const fn mstdma(&self) -> super::vals::Mstdma {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Mstdma::from_bits(val as u8)
    }
    #[doc = "Master DMA enable. Data operations of the I2C can be performed with DMA. Protocol type operations such as Start, address, Stop, and address match must always be done with software, typically via an interrupt. Address acknowledgement must also be done by software except when the I2C is configured to be HSCAPABLE (and address acknowledgement is handled entirely by hardware) or when Automatic Operation is enabled. When a DMA data transfer is complete, MSTDMA must be cleared prior to beginning the next operation, typically a Start or Stop.This bit is read/write."]
    #[inline(always)]
    pub fn set_mstdma(&mut self, val: super::vals::Mstdma) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
}
impl Default for Mstctl {
    #[inline(always)]
    fn default() -> Mstctl {
        Mstctl(0)
    }
}
#[doc = "Combined Master receiver and transmitter data register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mstdat(pub u32);
impl Mstdat {
    #[doc = "Master function data register. Read: read the most recently received data for the Master function. Write: transmit data using the Master function."]
    #[inline(always)]
    pub const fn data(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Master function data register. Read: read the most recently received data for the Master function. Write: transmit data using the Master function."]
    #[inline(always)]
    pub fn set_data(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Mstdat {
    #[inline(always)]
    fn default() -> Mstdat {
        Mstdat(0)
    }
}
#[doc = "Master timing configuration."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Msttime(pub u32);
impl Msttime {
    #[doc = "Master SCL Low time. Specifies the minimum low time that will be asserted by this master on SCL. Other devices on the bus (masters or slaves) could lengthen this time. This corresponds to the parameter t LOW in the I2C bus specification. I2C bus specification parameters tBUF and tSU;STA have the same values and are also controlled by MSTSCLLOW."]
    #[inline(always)]
    pub const fn mstscllow(&self) -> super::vals::Mstscllow {
        let val = (self.0 >> 0usize) & 0x07;
        super::vals::Mstscllow::from_bits(val as u8)
    }
    #[doc = "Master SCL Low time. Specifies the minimum low time that will be asserted by this master on SCL. Other devices on the bus (masters or slaves) could lengthen this time. This corresponds to the parameter t LOW in the I2C bus specification. I2C bus specification parameters tBUF and tSU;STA have the same values and are also controlled by MSTSCLLOW."]
    #[inline(always)]
    pub fn set_mstscllow(&mut self, val: super::vals::Mstscllow) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val.to_bits() as u32) & 0x07) << 0usize);
    }
    #[doc = "Master SCL High time. Specifies the minimum high time that will be asserted by this master on SCL. Other masters in a multi-master system could shorten this time. This corresponds to the parameter tHIGH in the I2C bus specification. I2C bus specification parameters tSU;STO and tHD;STA have the same values and are also controlled by MSTSCLHIGH."]
    #[inline(always)]
    pub const fn mstsclhigh(&self) -> super::vals::Mstsclhigh {
        let val = (self.0 >> 4usize) & 0x07;
        super::vals::Mstsclhigh::from_bits(val as u8)
    }
    #[doc = "Master SCL High time. Specifies the minimum high time that will be asserted by this master on SCL. Other masters in a multi-master system could shorten this time. This corresponds to the parameter tHIGH in the I2C bus specification. I2C bus specification parameters tSU;STO and tHD;STA have the same values and are also controlled by MSTSCLHIGH."]
    #[inline(always)]
    pub fn set_mstsclhigh(&mut self, val: super::vals::Mstsclhigh) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val.to_bits() as u32) & 0x07) << 4usize);
    }
}
impl Default for Msttime {
    #[inline(always)]
    fn default() -> Msttime {
        Msttime(0)
    }
}
#[doc = "Slave address register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Slvadr(pub u32);
impl Slvadr {
    #[doc = "Slave Address n Disable."]
    #[inline(always)]
    pub const fn sadisable(&self) -> super::vals::Sadisable {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Sadisable::from_bits(val as u8)
    }
    #[doc = "Slave Address n Disable."]
    #[inline(always)]
    pub fn set_sadisable(&mut self, val: super::vals::Sadisable) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Slave Address. Seven bit slave address that is compared to received addresses if enabled."]
    #[inline(always)]
    pub const fn slvadr(&self) -> u8 {
        let val = (self.0 >> 1usize) & 0x7f;
        val as u8
    }
    #[doc = "Slave Address. Seven bit slave address that is compared to received addresses if enabled."]
    #[inline(always)]
    pub fn set_slvadr(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 1usize)) | (((val as u32) & 0x7f) << 1usize);
    }
    #[doc = "Automatic NACK operation. Used in conjunction with AUTOACK and AUTOMATCHREAD, allows software to ignore I2C traffic while handling previous I2C data or other operations."]
    #[inline(always)]
    pub const fn autonack(&self) -> super::vals::Autonack {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::Autonack::from_bits(val as u8)
    }
    #[doc = "Automatic NACK operation. Used in conjunction with AUTOACK and AUTOMATCHREAD, allows software to ignore I2C traffic while handling previous I2C data or other operations."]
    #[inline(always)]
    pub fn set_autonack(&mut self, val: super::vals::Autonack) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
}
impl Default for Slvadr {
    #[inline(always)]
    fn default() -> Slvadr {
        Slvadr(0)
    }
}
#[doc = "Slave control register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Slvctl(pub u32);
impl Slvctl {
    #[doc = "Slave Continue."]
    #[inline(always)]
    pub const fn slvcontinue(&self) -> super::vals::Slvcontinue {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Slvcontinue::from_bits(val as u8)
    }
    #[doc = "Slave Continue."]
    #[inline(always)]
    pub fn set_slvcontinue(&mut self, val: super::vals::Slvcontinue) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Slave NACK."]
    #[inline(always)]
    pub const fn slvnack(&self) -> super::vals::Slvnack {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Slvnack::from_bits(val as u8)
    }
    #[doc = "Slave NACK."]
    #[inline(always)]
    pub fn set_slvnack(&mut self, val: super::vals::Slvnack) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "Slave DMA enable."]
    #[inline(always)]
    pub const fn slvdma(&self) -> super::vals::Slvdma {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Slvdma::from_bits(val as u8)
    }
    #[doc = "Slave DMA enable."]
    #[inline(always)]
    pub fn set_slvdma(&mut self, val: super::vals::Slvdma) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "Automatic Acknowledge.When this bit is set, it will cause an I2C header which matches SLVADR0 and the direction set by AUTOMATCHREAD to be ACKed immediately; this is used with DMA to allow processing of the data without intervention. If this bit is clear and a header matches SLVADR0, the behavior is controlled by AUTONACK in the SLVADR0 register: allowing NACK or interrupt."]
    #[inline(always)]
    pub const fn autoack(&self) -> super::vals::Autoack {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Autoack::from_bits(val as u8)
    }
    #[doc = "Automatic Acknowledge.When this bit is set, it will cause an I2C header which matches SLVADR0 and the direction set by AUTOMATCHREAD to be ACKed immediately; this is used with DMA to allow processing of the data without intervention. If this bit is clear and a header matches SLVADR0, the behavior is controlled by AUTONACK in the SLVADR0 register: allowing NACK or interrupt."]
    #[inline(always)]
    pub fn set_autoack(&mut self, val: super::vals::Autoack) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "When AUTOACK is set, this bit controls whether it matches a read or write request on the next header with an address matching SLVADR0. Since DMA needs to be configured to match the transfer direction, the direction needs to be specified. This bit allows a direction to be chosen for the next operation."]
    #[inline(always)]
    pub const fn automatchread(&self) -> super::vals::Automatchread {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::Automatchread::from_bits(val as u8)
    }
    #[doc = "When AUTOACK is set, this bit controls whether it matches a read or write request on the next header with an address matching SLVADR0. Since DMA needs to be configured to match the transfer direction, the direction needs to be specified. This bit allows a direction to be chosen for the next operation."]
    #[inline(always)]
    pub fn set_automatchread(&mut self, val: super::vals::Automatchread) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
}
impl Default for Slvctl {
    #[inline(always)]
    fn default() -> Slvctl {
        Slvctl(0)
    }
}
#[doc = "Combined Slave receiver and transmitter data register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Slvdat(pub u32);
impl Slvdat {
    #[doc = "Slave function data register. Read: read the most recently received data for the Slave function. Write: transmit data using the Slave function."]
    #[inline(always)]
    pub const fn data(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Slave function data register. Read: read the most recently received data for the Slave function. Write: transmit data using the Slave function."]
    #[inline(always)]
    pub fn set_data(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Slvdat {
    #[inline(always)]
    fn default() -> Slvdat {
        Slvdat(0)
    }
}
#[doc = "Slave Qualification for address 0."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Slvqual0(pub u32);
impl Slvqual0 {
    #[doc = "Qualify mode for slave address 0."]
    #[inline(always)]
    pub const fn qualmode0(&self) -> super::vals::Qualmode0 {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Qualmode0::from_bits(val as u8)
    }
    #[doc = "Qualify mode for slave address 0."]
    #[inline(always)]
    pub fn set_qualmode0(&mut self, val: super::vals::Qualmode0) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Slave address Qualifier for address 0. A value of 0 causes the address in SLVADR0 to be used as-is, assuming that it is enabled. If QUALMODE0 = 0, any bit in this field which is set to 1 will cause an automatic match of the corresponding bit of the received address when it is compared to the SLVADR0 register. If QUALMODE0 = 1, an address range is matched for address 0. This range extends from the value defined by SLVADR0 to the address defined by SLVQUAL0 (address matches when SLVADR0\\[7:1\\] <= received address <= SLVQUAL0\\[7:1\\])."]
    #[inline(always)]
    pub const fn slvqual0(&self) -> u8 {
        let val = (self.0 >> 1usize) & 0x7f;
        val as u8
    }
    #[doc = "Slave address Qualifier for address 0. A value of 0 causes the address in SLVADR0 to be used as-is, assuming that it is enabled. If QUALMODE0 = 0, any bit in this field which is set to 1 will cause an automatic match of the corresponding bit of the received address when it is compared to the SLVADR0 register. If QUALMODE0 = 1, an address range is matched for address 0. This range extends from the value defined by SLVADR0 to the address defined by SLVQUAL0 (address matches when SLVADR0\\[7:1\\] <= received address <= SLVQUAL0\\[7:1\\])."]
    #[inline(always)]
    pub fn set_slvqual0(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 1usize)) | (((val as u32) & 0x7f) << 1usize);
    }
}
impl Default for Slvqual0 {
    #[inline(always)]
    fn default() -> Slvqual0 {
        Slvqual0(0)
    }
}
#[doc = "Status register for Master, Slave, and Monitor functions."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Stat(pub u32);
impl Stat {
    #[doc = "Master Pending. Indicates that the Master is waiting to continue communication on the I2C-bus (pending) or is idle. When the master is pending, the MSTSTATE bits indicate what type of software service if any the master expects. This flag will cause an interrupt when set if, enabled via the INTENSET register. The MSTPENDING flag is not set when the DMA is handling an event (if the MSTDMA bit in the MSTCTL register is set). If the master is in the idle state, and no communication is needed, mask this interrupt."]
    #[inline(always)]
    pub const fn mstpending(&self) -> super::vals::Mstpending {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Mstpending::from_bits(val as u8)
    }
    #[doc = "Master Pending. Indicates that the Master is waiting to continue communication on the I2C-bus (pending) or is idle. When the master is pending, the MSTSTATE bits indicate what type of software service if any the master expects. This flag will cause an interrupt when set if, enabled via the INTENSET register. The MSTPENDING flag is not set when the DMA is handling an event (if the MSTDMA bit in the MSTCTL register is set). If the master is in the idle state, and no communication is needed, mask this interrupt."]
    #[inline(always)]
    pub fn set_mstpending(&mut self, val: super::vals::Mstpending) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Master State code. The master state code reflects the master state when the MSTPENDING bit is set, that is the master is pending or in the idle state. Each value of this field indicates a specific required service for the Master function. All other values are reserved. See Table 400 for details of state values and appropriate responses."]
    #[inline(always)]
    pub const fn mststate(&self) -> super::vals::Mststate {
        let val = (self.0 >> 1usize) & 0x07;
        super::vals::Mststate::from_bits(val as u8)
    }
    #[doc = "Master State code. The master state code reflects the master state when the MSTPENDING bit is set, that is the master is pending or in the idle state. Each value of this field indicates a specific required service for the Master function. All other values are reserved. See Table 400 for details of state values and appropriate responses."]
    #[inline(always)]
    pub fn set_mststate(&mut self, val: super::vals::Mststate) {
        self.0 = (self.0 & !(0x07 << 1usize)) | (((val.to_bits() as u32) & 0x07) << 1usize);
    }
    #[doc = "Master Arbitration Loss flag. This flag can be cleared by software writing a 1 to this bit. It is also cleared automatically a 1 is written to MSTCONTINUE."]
    #[inline(always)]
    pub const fn mstarbloss(&self) -> super::vals::Mstarbloss {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Mstarbloss::from_bits(val as u8)
    }
    #[doc = "Master Arbitration Loss flag. This flag can be cleared by software writing a 1 to this bit. It is also cleared automatically a 1 is written to MSTCONTINUE."]
    #[inline(always)]
    pub fn set_mstarbloss(&mut self, val: super::vals::Mstarbloss) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "Master Start/Stop Error flag. This flag can be cleared by software writing a 1 to this bit. It is also cleared automatically a 1 is written to MSTCONTINUE."]
    #[inline(always)]
    pub const fn mstststperr(&self) -> super::vals::Mstststperr {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Mstststperr::from_bits(val as u8)
    }
    #[doc = "Master Start/Stop Error flag. This flag can be cleared by software writing a 1 to this bit. It is also cleared automatically a 1 is written to MSTCONTINUE."]
    #[inline(always)]
    pub fn set_mstststperr(&mut self, val: super::vals::Mstststperr) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Slave Pending. Indicates that the Slave function is waiting to continue communication on the I2C-bus and needs software service. This flag will cause an interrupt when set if enabled via INTENSET. The SLVPENDING flag is not set when the DMA is handling an event (if the SLVDMA bit in the SLVCTL register is set). The SLVPENDING flag is read-only and is automatically cleared when a 1 is written to the SLVCONTINUE bit in the SLVCTL register. The point in time when SlvPending is set depends on whether the I2C interface is in HSCAPABLE mode. See Section 25.7.2.2.2. When the I2C interface is configured to be HSCAPABLE, HS master codes are detected automatically. Due to the requirements of the HS I2C specification, slave addresses must also be detected automatically, since the address must be acknowledged before the clock can be stretched."]
    #[inline(always)]
    pub const fn slvpending(&self) -> super::vals::Slvpending {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Slvpending::from_bits(val as u8)
    }
    #[doc = "Slave Pending. Indicates that the Slave function is waiting to continue communication on the I2C-bus and needs software service. This flag will cause an interrupt when set if enabled via INTENSET. The SLVPENDING flag is not set when the DMA is handling an event (if the SLVDMA bit in the SLVCTL register is set). The SLVPENDING flag is read-only and is automatically cleared when a 1 is written to the SLVCONTINUE bit in the SLVCTL register. The point in time when SlvPending is set depends on whether the I2C interface is in HSCAPABLE mode. See Section 25.7.2.2.2. When the I2C interface is configured to be HSCAPABLE, HS master codes are detected automatically. Due to the requirements of the HS I2C specification, slave addresses must also be detected automatically, since the address must be acknowledged before the clock can be stretched."]
    #[inline(always)]
    pub fn set_slvpending(&mut self, val: super::vals::Slvpending) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Slave State code. Each value of this field indicates a specific required service for the Slave function. All other values are reserved. See Table 401 for state values and actions. note that the occurrence of some states and how they are handled are affected by DMA mode and Automatic Operation modes."]
    #[inline(always)]
    pub const fn slvstate(&self) -> super::vals::Slvstate {
        let val = (self.0 >> 9usize) & 0x03;
        super::vals::Slvstate::from_bits(val as u8)
    }
    #[doc = "Slave State code. Each value of this field indicates a specific required service for the Slave function. All other values are reserved. See Table 401 for state values and actions. note that the occurrence of some states and how they are handled are affected by DMA mode and Automatic Operation modes."]
    #[inline(always)]
    pub fn set_slvstate(&mut self, val: super::vals::Slvstate) {
        self.0 = (self.0 & !(0x03 << 9usize)) | (((val.to_bits() as u32) & 0x03) << 9usize);
    }
    #[doc = "Slave Not Stretching. Indicates when the slave function is stretching the I2C clock. This is needed in order to gracefully invoke Deep Sleep or Power-down modes during slave operation. This read-only flag reflects the slave function status in real time."]
    #[inline(always)]
    pub const fn slvnotstr(&self) -> super::vals::Slvnotstr {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::Slvnotstr::from_bits(val as u8)
    }
    #[doc = "Slave Not Stretching. Indicates when the slave function is stretching the I2C clock. This is needed in order to gracefully invoke Deep Sleep or Power-down modes during slave operation. This read-only flag reflects the slave function status in real time."]
    #[inline(always)]
    pub fn set_slvnotstr(&mut self, val: super::vals::Slvnotstr) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "Slave address match Index. This field is valid when the I2C slave function has been selected by receiving an address that matches one of the slave addresses defined by any enabled slave address registers, and provides an identification of the address that was matched. It is possible that more than one address could be matched, but only one match can be reported here."]
    #[inline(always)]
    pub const fn slvidx(&self) -> super::vals::Slvidx {
        let val = (self.0 >> 12usize) & 0x03;
        super::vals::Slvidx::from_bits(val as u8)
    }
    #[doc = "Slave address match Index. This field is valid when the I2C slave function has been selected by receiving an address that matches one of the slave addresses defined by any enabled slave address registers, and provides an identification of the address that was matched. It is possible that more than one address could be matched, but only one match can be reported here."]
    #[inline(always)]
    pub fn set_slvidx(&mut self, val: super::vals::Slvidx) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
    #[doc = "Slave selected flag. SLVSEL is set after an address match when software tells the Slave function to acknowledge the address, or when the address has been automatically acknowledged. It is cleared when another address cycle presents an address that does not match an enabled address on the Slave function, when slave software decides to NACK a matched address, when there is a Stop detected on the bus, when the master NACKs slave data, and in some combinations of Automatic Operation. SLVSEL is not cleared if software NACKs data."]
    #[inline(always)]
    pub const fn slvsel(&self) -> super::vals::Slvsel {
        let val = (self.0 >> 14usize) & 0x01;
        super::vals::Slvsel::from_bits(val as u8)
    }
    #[doc = "Slave selected flag. SLVSEL is set after an address match when software tells the Slave function to acknowledge the address, or when the address has been automatically acknowledged. It is cleared when another address cycle presents an address that does not match an enabled address on the Slave function, when slave software decides to NACK a matched address, when there is a Stop detected on the bus, when the master NACKs slave data, and in some combinations of Automatic Operation. SLVSEL is not cleared if software NACKs data."]
    #[inline(always)]
    pub fn set_slvsel(&mut self, val: super::vals::Slvsel) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
    }
    #[doc = "Slave Deselected flag. This flag will cause an interrupt when set if enabled via INTENSET. This flag can be cleared by writing a 1 to this bit."]
    #[inline(always)]
    pub const fn slvdesel(&self) -> super::vals::Slvdesel {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::Slvdesel::from_bits(val as u8)
    }
    #[doc = "Slave Deselected flag. This flag will cause an interrupt when set if enabled via INTENSET. This flag can be cleared by writing a 1 to this bit."]
    #[inline(always)]
    pub fn set_slvdesel(&mut self, val: super::vals::Slvdesel) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
    #[doc = "Monitor Ready. This flag is cleared when the MONRXDAT register is read."]
    #[inline(always)]
    pub const fn monrdy(&self) -> super::vals::Monrdy {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::Monrdy::from_bits(val as u8)
    }
    #[doc = "Monitor Ready. This flag is cleared when the MONRXDAT register is read."]
    #[inline(always)]
    pub fn set_monrdy(&mut self, val: super::vals::Monrdy) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "Monitor Overflow flag."]
    #[inline(always)]
    pub const fn monov(&self) -> super::vals::Monov {
        let val = (self.0 >> 17usize) & 0x01;
        super::vals::Monov::from_bits(val as u8)
    }
    #[doc = "Monitor Overflow flag."]
    #[inline(always)]
    pub fn set_monov(&mut self, val: super::vals::Monov) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
    #[doc = "Monitor Active flag. Indicates when the Monitor function considers the I 2C bus to be active. Active is defined here as when some Master is on the bus: a bus Start has occurred more recently than a bus Stop."]
    #[inline(always)]
    pub const fn monactive(&self) -> super::vals::Monactive {
        let val = (self.0 >> 18usize) & 0x01;
        super::vals::Monactive::from_bits(val as u8)
    }
    #[doc = "Monitor Active flag. Indicates when the Monitor function considers the I 2C bus to be active. Active is defined here as when some Master is on the bus: a bus Start has occurred more recently than a bus Stop."]
    #[inline(always)]
    pub fn set_monactive(&mut self, val: super::vals::Monactive) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
    }
    #[doc = "Monitor Idle flag. This flag is set when the Monitor function sees the I2C bus change from active to inactive. This can be used by software to decide when to process data accumulated by the Monitor function. This flag will cause an interrupt when set if enabled via the INTENSET register. The flag can be cleared by writing a 1 to this bit."]
    #[inline(always)]
    pub const fn monidle(&self) -> super::vals::Monidle {
        let val = (self.0 >> 19usize) & 0x01;
        super::vals::Monidle::from_bits(val as u8)
    }
    #[doc = "Monitor Idle flag. This flag is set when the Monitor function sees the I2C bus change from active to inactive. This can be used by software to decide when to process data accumulated by the Monitor function. This flag will cause an interrupt when set if enabled via the INTENSET register. The flag can be cleared by writing a 1 to this bit."]
    #[inline(always)]
    pub fn set_monidle(&mut self, val: super::vals::Monidle) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
    #[doc = "Event Time-out Interrupt flag. Indicates when the time between events has been longer than the time specified by the TIMEOUT register. Events include Start, Stop, and clock edges. The flag is cleared by writing a 1 to this bit. No time-out is created when the I2C-bus is idle."]
    #[inline(always)]
    pub const fn eventtimeout(&self) -> super::vals::Eventtimeout {
        let val = (self.0 >> 24usize) & 0x01;
        super::vals::Eventtimeout::from_bits(val as u8)
    }
    #[doc = "Event Time-out Interrupt flag. Indicates when the time between events has been longer than the time specified by the TIMEOUT register. Events include Start, Stop, and clock edges. The flag is cleared by writing a 1 to this bit. No time-out is created when the I2C-bus is idle."]
    #[inline(always)]
    pub fn set_eventtimeout(&mut self, val: super::vals::Eventtimeout) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
    #[doc = "SCL Time-out Interrupt flag. Indicates when SCL has remained low longer than the time specific by the TIMEOUT register. The flag is cleared by writing a 1 to this bit."]
    #[inline(always)]
    pub const fn scltimeout(&self) -> super::vals::Scltimeout {
        let val = (self.0 >> 25usize) & 0x01;
        super::vals::Scltimeout::from_bits(val as u8)
    }
    #[doc = "SCL Time-out Interrupt flag. Indicates when SCL has remained low longer than the time specific by the TIMEOUT register. The flag is cleared by writing a 1 to this bit."]
    #[inline(always)]
    pub fn set_scltimeout(&mut self, val: super::vals::Scltimeout) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val.to_bits() as u32) & 0x01) << 25usize);
    }
}
impl Default for Stat {
    #[inline(always)]
    fn default() -> Stat {
        Stat(0)
    }
}
#[doc = "Time-out value register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Timeout(pub u32);
impl Timeout {
    #[doc = "Time-out time value, bottom four bits. These are hard-wired to 0xF. This gives a minimum time-out of 16 I2C function clocks and also a time-out resolution of 16 I2C function clocks."]
    #[inline(always)]
    pub const fn tomin(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Time-out time value, bottom four bits. These are hard-wired to 0xF. This gives a minimum time-out of 16 I2C function clocks and also a time-out resolution of 16 I2C function clocks."]
    #[inline(always)]
    pub fn set_tomin(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Time-out time value. Specifies the time-out interval value in increments of 16 I 2C function clocks, as defined by the CLKDIV register. To change this value while I2C is in operation, disable all time-outs, write a new value to TIMEOUT, then re-enable time-outs. 0x000 = A time-out will occur after 16 counts of the I2C function clock. 0x001 = A time-out will occur after 32 counts of the I2C function clock. 0xFFF = A time-out will occur after 65,536 counts of the I2C function clock."]
    #[inline(always)]
    pub const fn to(&self) -> u16 {
        let val = (self.0 >> 4usize) & 0x0fff;
        val as u16
    }
    #[doc = "Time-out time value. Specifies the time-out interval value in increments of 16 I 2C function clocks, as defined by the CLKDIV register. To change this value while I2C is in operation, disable all time-outs, write a new value to TIMEOUT, then re-enable time-outs. 0x000 = A time-out will occur after 16 counts of the I2C function clock. 0x001 = A time-out will occur after 32 counts of the I2C function clock. 0xFFF = A time-out will occur after 65,536 counts of the I2C function clock."]
    #[inline(always)]
    pub fn set_to(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 4usize)) | (((val as u32) & 0x0fff) << 4usize);
    }
}
impl Default for Timeout {
    #[inline(always)]
    fn default() -> Timeout {
        Timeout(0)
    }
}

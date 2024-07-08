#[doc = "CT32BIT N Counter Timer Capture Trigger Multiplexers M"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ct32bitCapSel(pub u32);
impl Ct32bitCapSel {
    #[doc = "Counter Timer m, Capture Port Input n 19:1 Mux Select. . ."]
    #[inline(always)]
    pub const fn capn_sel(&self) -> super::vals::CapnSel {
        let val = (self.0 >> 0usize) & 0x1f;
        super::vals::CapnSel::from_bits(val as u8)
    }
    #[doc = "Counter Timer m, Capture Port Input n 19:1 Mux Select. . ."]
    #[inline(always)]
    pub fn set_capn_sel(&mut self, val: super::vals::CapnSel) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val.to_bits() as u32) & 0x1f) << 0usize);
    }
}
impl Default for Ct32bitCapSel {
    #[inline(always)]
    fn default() -> Ct32bitCapSel {
        Ct32bitCapSel(0)
    }
}
#[doc = "DMAC0 input trigger enable 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dmac0ItrigEna0(pub u32);
impl Dmac0ItrigEna0 {
    #[doc = "DMAC0 input trigger inmux 0 enable"]
    #[inline(always)]
    pub const fn dmac0_itrig_inmux0(&self) -> super::vals::Dmac0ItrigEna0Dmac0ItrigInmux0 {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Dmac0ItrigEna0Dmac0ItrigInmux0::from_bits(val as u8)
    }
    #[doc = "DMAC0 input trigger inmux 0 enable"]
    #[inline(always)]
    pub fn set_dmac0_itrig_inmux0(&mut self, val: super::vals::Dmac0ItrigEna0Dmac0ItrigInmux0) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "DMAC0 input trigger inmux 1 enable"]
    #[inline(always)]
    pub const fn dmac0_itrig_inmux1(&self) -> super::vals::Dmac0ItrigEna0Dmac0ItrigInmux1 {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Dmac0ItrigEna0Dmac0ItrigInmux1::from_bits(val as u8)
    }
    #[doc = "DMAC0 input trigger inmux 1 enable"]
    #[inline(always)]
    pub fn set_dmac0_itrig_inmux1(&mut self, val: super::vals::Dmac0ItrigEna0Dmac0ItrigInmux1) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "DMAC0 input trigger inmux 2 enable"]
    #[inline(always)]
    pub const fn dmac0_itrig_inmux2(&self) -> super::vals::Dmac0ItrigInmux2 {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Dmac0ItrigInmux2::from_bits(val as u8)
    }
    #[doc = "DMAC0 input trigger inmux 2 enable"]
    #[inline(always)]
    pub fn set_dmac0_itrig_inmux2(&mut self, val: super::vals::Dmac0ItrigInmux2) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "DMAC0 input trigger inmux 3 enable"]
    #[inline(always)]
    pub const fn dmac0_itrig_inmux3(&self) -> super::vals::Dmac0ItrigEna0Dmac0ItrigInmux3 {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Dmac0ItrigEna0Dmac0ItrigInmux3::from_bits(val as u8)
    }
    #[doc = "DMAC0 input trigger inmux 3 enable"]
    #[inline(always)]
    pub fn set_dmac0_itrig_inmux3(&mut self, val: super::vals::Dmac0ItrigEna0Dmac0ItrigInmux3) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "DMAC0 input trigger inmux 4 enable"]
    #[inline(always)]
    pub const fn dmac0_itrig_inmux4(&self) -> super::vals::Dmac0ItrigEna0Dmac0ItrigInmux4 {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Dmac0ItrigEna0Dmac0ItrigInmux4::from_bits(val as u8)
    }
    #[doc = "DMAC0 input trigger inmux 4 enable"]
    #[inline(always)]
    pub fn set_dmac0_itrig_inmux4(&mut self, val: super::vals::Dmac0ItrigEna0Dmac0ItrigInmux4) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "DMAC0 input trigger inmux 5 enable"]
    #[inline(always)]
    pub const fn dmac0_itrig_inmux5(&self) -> super::vals::Dmac0ItrigEna0Dmac0ItrigInmux5 {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::Dmac0ItrigEna0Dmac0ItrigInmux5::from_bits(val as u8)
    }
    #[doc = "DMAC0 input trigger inmux 5 enable"]
    #[inline(always)]
    pub fn set_dmac0_itrig_inmux5(&mut self, val: super::vals::Dmac0ItrigEna0Dmac0ItrigInmux5) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "DMAC0 input trigger inmux 6 enable"]
    #[inline(always)]
    pub const fn dmac0_itrig_inmux6(&self) -> super::vals::Dmac0ItrigEna0Dmac0ItrigInmux6 {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Dmac0ItrigEna0Dmac0ItrigInmux6::from_bits(val as u8)
    }
    #[doc = "DMAC0 input trigger inmux 6 enable"]
    #[inline(always)]
    pub fn set_dmac0_itrig_inmux6(&mut self, val: super::vals::Dmac0ItrigEna0Dmac0ItrigInmux6) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "DMAC0 input trigger inmux 7 enable"]
    #[inline(always)]
    pub const fn dmac0_itrig_inmux7(&self) -> super::vals::Dmac0ItrigEna0Dmac0ItrigInmux7 {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Dmac0ItrigEna0Dmac0ItrigInmux7::from_bits(val as u8)
    }
    #[doc = "DMAC0 input trigger inmux 7 enable"]
    #[inline(always)]
    pub fn set_dmac0_itrig_inmux7(&mut self, val: super::vals::Dmac0ItrigEna0Dmac0ItrigInmux7) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "DMAC0 input trigger inmux 8 enable"]
    #[inline(always)]
    pub const fn dmac0_itrig_inmux8(&self) -> super::vals::Dmac0ItrigEna0Dmac0ItrigInmux8 {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Dmac0ItrigEna0Dmac0ItrigInmux8::from_bits(val as u8)
    }
    #[doc = "DMAC0 input trigger inmux 8 enable"]
    #[inline(always)]
    pub fn set_dmac0_itrig_inmux8(&mut self, val: super::vals::Dmac0ItrigEna0Dmac0ItrigInmux8) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "DMAC0 input trigger inmux 9 enable"]
    #[inline(always)]
    pub const fn dmac0_itrig_inmux9(&self) -> super::vals::Dmac0ItrigEna0Dmac0ItrigInmux9 {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::Dmac0ItrigEna0Dmac0ItrigInmux9::from_bits(val as u8)
    }
    #[doc = "DMAC0 input trigger inmux 9 enable"]
    #[inline(always)]
    pub fn set_dmac0_itrig_inmux9(&mut self, val: super::vals::Dmac0ItrigEna0Dmac0ItrigInmux9) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "DMAC0 input trigger inmux 10 enable"]
    #[inline(always)]
    pub const fn dmac0_itrig_inmux10(&self) -> super::vals::Dmac0ItrigEna0Dmac0ItrigInmux10 {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::Dmac0ItrigEna0Dmac0ItrigInmux10::from_bits(val as u8)
    }
    #[doc = "DMAC0 input trigger inmux 10 enable"]
    #[inline(always)]
    pub fn set_dmac0_itrig_inmux10(&mut self, val: super::vals::Dmac0ItrigEna0Dmac0ItrigInmux10) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "DMAC0 input trigger inmux 11 enable"]
    #[inline(always)]
    pub const fn dmac0_itrig_inmux11(&self) -> super::vals::Dmac0ItrigEna0Dmac0ItrigInmux11 {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::Dmac0ItrigEna0Dmac0ItrigInmux11::from_bits(val as u8)
    }
    #[doc = "DMAC0 input trigger inmux 11 enable"]
    #[inline(always)]
    pub fn set_dmac0_itrig_inmux11(&mut self, val: super::vals::Dmac0ItrigEna0Dmac0ItrigInmux11) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "DMAC0 input trigger inmux 12 enable"]
    #[inline(always)]
    pub const fn dmac0_itrig_inmux12(&self) -> super::vals::Dmac0ItrigEna0Dmac0ItrigInmux12 {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::Dmac0ItrigEna0Dmac0ItrigInmux12::from_bits(val as u8)
    }
    #[doc = "DMAC0 input trigger inmux 12 enable"]
    #[inline(always)]
    pub fn set_dmac0_itrig_inmux12(&mut self, val: super::vals::Dmac0ItrigEna0Dmac0ItrigInmux12) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "DMAC0 input trigger inmux 13 enable"]
    #[inline(always)]
    pub const fn dmac0_itrig_inmux13(&self) -> super::vals::Dmac0ItrigEna0Dmac0ItrigInmux13 {
        let val = (self.0 >> 13usize) & 0x01;
        super::vals::Dmac0ItrigEna0Dmac0ItrigInmux13::from_bits(val as u8)
    }
    #[doc = "DMAC0 input trigger inmux 13 enable"]
    #[inline(always)]
    pub fn set_dmac0_itrig_inmux13(&mut self, val: super::vals::Dmac0ItrigEna0Dmac0ItrigInmux13) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "DMAC0 input trigger inmux 14 enable"]
    #[inline(always)]
    pub const fn dmac0_itrig_inmux14(&self) -> super::vals::Dmac0ItrigEna0Dmac0ItrigInmux14 {
        let val = (self.0 >> 14usize) & 0x01;
        super::vals::Dmac0ItrigEna0Dmac0ItrigInmux14::from_bits(val as u8)
    }
    #[doc = "DMAC0 input trigger inmux 14 enable"]
    #[inline(always)]
    pub fn set_dmac0_itrig_inmux14(&mut self, val: super::vals::Dmac0ItrigEna0Dmac0ItrigInmux14) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
    }
    #[doc = "DMAC0 input trigger inmux 15 enable"]
    #[inline(always)]
    pub const fn dmac0_itrig_inmux15(&self) -> super::vals::Dmac0ItrigEna0Dmac0ItrigInmux15 {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::Dmac0ItrigEna0Dmac0ItrigInmux15::from_bits(val as u8)
    }
    #[doc = "DMAC0 input trigger inmux 15 enable"]
    #[inline(always)]
    pub fn set_dmac0_itrig_inmux15(&mut self, val: super::vals::Dmac0ItrigEna0Dmac0ItrigInmux15) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
    #[doc = "DMAC0 input trigger inmux 16 enable"]
    #[inline(always)]
    pub const fn dmac0_itrig_inmux16(&self) -> super::vals::Dmac0ItrigEna0Dmac0ItrigInmux16 {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::Dmac0ItrigEna0Dmac0ItrigInmux16::from_bits(val as u8)
    }
    #[doc = "DMAC0 input trigger inmux 16 enable"]
    #[inline(always)]
    pub fn set_dmac0_itrig_inmux16(&mut self, val: super::vals::Dmac0ItrigEna0Dmac0ItrigInmux16) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "DMAC0 input trigger inmux 17 enable"]
    #[inline(always)]
    pub const fn dmac0_itrig_inmux17(&self) -> super::vals::Dmac0ItrigEna0Dmac0ItrigInmux17 {
        let val = (self.0 >> 17usize) & 0x01;
        super::vals::Dmac0ItrigEna0Dmac0ItrigInmux17::from_bits(val as u8)
    }
    #[doc = "DMAC0 input trigger inmux 17 enable"]
    #[inline(always)]
    pub fn set_dmac0_itrig_inmux17(&mut self, val: super::vals::Dmac0ItrigEna0Dmac0ItrigInmux17) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
    #[doc = "DMAC0 input trigger inmux 18 enable"]
    #[inline(always)]
    pub const fn dmac0_itrig_inmux18(&self) -> super::vals::Dmac0ItrigEna0Dmac0ItrigInmux18 {
        let val = (self.0 >> 18usize) & 0x01;
        super::vals::Dmac0ItrigEna0Dmac0ItrigInmux18::from_bits(val as u8)
    }
    #[doc = "DMAC0 input trigger inmux 18 enable"]
    #[inline(always)]
    pub fn set_dmac0_itrig_inmux18(&mut self, val: super::vals::Dmac0ItrigEna0Dmac0ItrigInmux18) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
    }
    #[doc = "DMAC0 input trigger inmux 19 enable"]
    #[inline(always)]
    pub const fn dmac0_itrig_inmux19(&self) -> super::vals::Dmac0ItrigEna0Dmac0ItrigInmux19 {
        let val = (self.0 >> 19usize) & 0x01;
        super::vals::Dmac0ItrigEna0Dmac0ItrigInmux19::from_bits(val as u8)
    }
    #[doc = "DMAC0 input trigger inmux 19 enable"]
    #[inline(always)]
    pub fn set_dmac0_itrig_inmux19(&mut self, val: super::vals::Dmac0ItrigEna0Dmac0ItrigInmux19) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
    #[doc = "DMAC0 input trigger inmux 20 enable"]
    #[inline(always)]
    pub const fn dmac0_itrig_inmux20(&self) -> super::vals::Dmac0ItrigEna0Dmac0ItrigInmux20 {
        let val = (self.0 >> 20usize) & 0x01;
        super::vals::Dmac0ItrigEna0Dmac0ItrigInmux20::from_bits(val as u8)
    }
    #[doc = "DMAC0 input trigger inmux 20 enable"]
    #[inline(always)]
    pub fn set_dmac0_itrig_inmux20(&mut self, val: super::vals::Dmac0ItrigEna0Dmac0ItrigInmux20) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
    #[doc = "DMAC0 input trigger inmux 21 enable"]
    #[inline(always)]
    pub const fn dmac0_itrig_inmux21(&self) -> super::vals::Dmac0ItrigEna0Dmac0ItrigInmux21 {
        let val = (self.0 >> 21usize) & 0x01;
        super::vals::Dmac0ItrigEna0Dmac0ItrigInmux21::from_bits(val as u8)
    }
    #[doc = "DMAC0 input trigger inmux 21 enable"]
    #[inline(always)]
    pub fn set_dmac0_itrig_inmux21(&mut self, val: super::vals::Dmac0ItrigEna0Dmac0ItrigInmux21) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
    }
    #[doc = "DMAC0 input trigger inmux 22 enable"]
    #[inline(always)]
    pub const fn dmac0_itrig_inmux22(&self) -> super::vals::Dmac0ItrigEna0Dmac0ItrigInmux22 {
        let val = (self.0 >> 22usize) & 0x01;
        super::vals::Dmac0ItrigEna0Dmac0ItrigInmux22::from_bits(val as u8)
    }
    #[doc = "DMAC0 input trigger inmux 22 enable"]
    #[inline(always)]
    pub fn set_dmac0_itrig_inmux22(&mut self, val: super::vals::Dmac0ItrigEna0Dmac0ItrigInmux22) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val.to_bits() as u32) & 0x01) << 22usize);
    }
    #[doc = "DMAC0 input trigger inmux 23 enable"]
    #[inline(always)]
    pub const fn dmac0_itrig_inmux23(&self) -> super::vals::Dmac0ItrigEna0Dmac0ItrigInmux23 {
        let val = (self.0 >> 23usize) & 0x01;
        super::vals::Dmac0ItrigEna0Dmac0ItrigInmux23::from_bits(val as u8)
    }
    #[doc = "DMAC0 input trigger inmux 23 enable"]
    #[inline(always)]
    pub fn set_dmac0_itrig_inmux23(&mut self, val: super::vals::Dmac0ItrigEna0Dmac0ItrigInmux23) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "DMAC0 input trigger inmux 24 enable"]
    #[inline(always)]
    pub const fn dmac0_itrig_inmux24(&self) -> super::vals::Dmac0ItrigEna0Dmac0ItrigInmux24 {
        let val = (self.0 >> 24usize) & 0x01;
        super::vals::Dmac0ItrigEna0Dmac0ItrigInmux24::from_bits(val as u8)
    }
    #[doc = "DMAC0 input trigger inmux 24 enable"]
    #[inline(always)]
    pub fn set_dmac0_itrig_inmux24(&mut self, val: super::vals::Dmac0ItrigEna0Dmac0ItrigInmux24) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
    #[doc = "DMAC0 input trigger inmux 25 enable"]
    #[inline(always)]
    pub const fn dmac0_itrig_inmux25(&self) -> super::vals::Dmac0ItrigEna0Dmac0ItrigInmux25 {
        let val = (self.0 >> 25usize) & 0x01;
        super::vals::Dmac0ItrigEna0Dmac0ItrigInmux25::from_bits(val as u8)
    }
    #[doc = "DMAC0 input trigger inmux 25 enable"]
    #[inline(always)]
    pub fn set_dmac0_itrig_inmux25(&mut self, val: super::vals::Dmac0ItrigEna0Dmac0ItrigInmux25) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val.to_bits() as u32) & 0x01) << 25usize);
    }
    #[doc = "DMAC0 input trigger inmux 26 enable"]
    #[inline(always)]
    pub const fn dmac0_itrig_inmux26(&self) -> super::vals::Dmac0ItrigEna0Dmac0ItrigInmux26 {
        let val = (self.0 >> 26usize) & 0x01;
        super::vals::Dmac0ItrigEna0Dmac0ItrigInmux26::from_bits(val as u8)
    }
    #[doc = "DMAC0 input trigger inmux 26 enable"]
    #[inline(always)]
    pub fn set_dmac0_itrig_inmux26(&mut self, val: super::vals::Dmac0ItrigEna0Dmac0ItrigInmux26) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val.to_bits() as u32) & 0x01) << 26usize);
    }
    #[doc = "DMAC0 input trigger inmux 27 enable"]
    #[inline(always)]
    pub const fn dmac0_itrig_inmux27(&self) -> super::vals::Dmac0ItrigEna0Dmac0ItrigInmux27 {
        let val = (self.0 >> 27usize) & 0x01;
        super::vals::Dmac0ItrigEna0Dmac0ItrigInmux27::from_bits(val as u8)
    }
    #[doc = "DMAC0 input trigger inmux 27 enable"]
    #[inline(always)]
    pub fn set_dmac0_itrig_inmux27(&mut self, val: super::vals::Dmac0ItrigEna0Dmac0ItrigInmux27) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val.to_bits() as u32) & 0x01) << 27usize);
    }
    #[doc = "DMAC0 input trigger inmux 28 enable"]
    #[inline(always)]
    pub const fn dmac0_itrig_inmux28(&self) -> super::vals::Dmac0ItrigEna0Dmac0ItrigInmux28 {
        let val = (self.0 >> 28usize) & 0x01;
        super::vals::Dmac0ItrigEna0Dmac0ItrigInmux28::from_bits(val as u8)
    }
    #[doc = "DMAC0 input trigger inmux 28 enable"]
    #[inline(always)]
    pub fn set_dmac0_itrig_inmux28(&mut self, val: super::vals::Dmac0ItrigEna0Dmac0ItrigInmux28) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val.to_bits() as u32) & 0x01) << 28usize);
    }
    #[doc = "DMAC0 input trigger inmux 29 enable"]
    #[inline(always)]
    pub const fn dmac0_itrig_inmux29(&self) -> super::vals::Dmac0ItrigEna0Dmac0ItrigInmux29 {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::Dmac0ItrigEna0Dmac0ItrigInmux29::from_bits(val as u8)
    }
    #[doc = "DMAC0 input trigger inmux 29 enable"]
    #[inline(always)]
    pub fn set_dmac0_itrig_inmux29(&mut self, val: super::vals::Dmac0ItrigEna0Dmac0ItrigInmux29) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "DMAC0 input trigger inmux 30 enable"]
    #[inline(always)]
    pub const fn dmac0_itrig_inmux30(&self) -> super::vals::Dmac0ItrigEna0Dmac0ItrigInmux30 {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::Dmac0ItrigEna0Dmac0ItrigInmux30::from_bits(val as u8)
    }
    #[doc = "DMAC0 input trigger inmux 30 enable"]
    #[inline(always)]
    pub fn set_dmac0_itrig_inmux30(&mut self, val: super::vals::Dmac0ItrigEna0Dmac0ItrigInmux30) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "DMAC0 input trigger inmux 31 enable"]
    #[inline(always)]
    pub const fn dmac0_itrig_inmux31(&self) -> super::vals::Dmac0ItrigEna0Dmac0ItrigInmux31 {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::Dmac0ItrigEna0Dmac0ItrigInmux31::from_bits(val as u8)
    }
    #[doc = "DMAC0 input trigger inmux 31 enable"]
    #[inline(always)]
    pub fn set_dmac0_itrig_inmux31(&mut self, val: super::vals::Dmac0ItrigEna0Dmac0ItrigInmux31) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Dmac0ItrigEna0 {
    #[inline(always)]
    fn default() -> Dmac0ItrigEna0 {
        Dmac0ItrigEna0(0)
    }
}
#[doc = "DMAC0 input trigger enable clear 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dmac0ItrigEna0Clr(pub u32);
impl Dmac0ItrigEna0Clr {
    #[doc = "DMAC0 input trigger inmux 0 enable clear"]
    #[inline(always)]
    pub const fn dmac0_itrig_inmux0(&self) -> super::vals::Dmac0ItrigEna0ClrDmac0ItrigInmux0 {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Dmac0ItrigEna0ClrDmac0ItrigInmux0::from_bits(val as u8)
    }
    #[doc = "DMAC0 input trigger inmux 0 enable clear"]
    #[inline(always)]
    pub fn set_dmac0_itrig_inmux0(&mut self, val: super::vals::Dmac0ItrigEna0ClrDmac0ItrigInmux0) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "DMAC0 input trigger inmux 1 enable clear"]
    #[inline(always)]
    pub const fn dmac0_itrig_inmux1(&self) -> super::vals::Dmac0ItrigEna0ClrDmac0ItrigInmux1 {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Dmac0ItrigEna0ClrDmac0ItrigInmux1::from_bits(val as u8)
    }
    #[doc = "DMAC0 input trigger inmux 1 enable clear"]
    #[inline(always)]
    pub fn set_dmac0_itrig_inmux1(&mut self, val: super::vals::Dmac0ItrigEna0ClrDmac0ItrigInmux1) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "DMAC0 input trigger inmux 2 enable clear"]
    #[inline(always)]
    pub const fn dmac0_itrig_inmux2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "DMAC0 input trigger inmux 2 enable clear"]
    #[inline(always)]
    pub fn set_dmac0_itrig_inmux2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "DMAC0 input trigger inmux 3 enable clear"]
    #[inline(always)]
    pub const fn dmac0_itrig_inmux3(&self) -> super::vals::Dmac0ItrigEna0ClrDmac0ItrigInmux3 {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Dmac0ItrigEna0ClrDmac0ItrigInmux3::from_bits(val as u8)
    }
    #[doc = "DMAC0 input trigger inmux 3 enable clear"]
    #[inline(always)]
    pub fn set_dmac0_itrig_inmux3(&mut self, val: super::vals::Dmac0ItrigEna0ClrDmac0ItrigInmux3) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "DMAC0 input trigger inmux 4 enable clear"]
    #[inline(always)]
    pub const fn dmac0_itrig_inmux4(&self) -> super::vals::Dmac0ItrigEna0ClrDmac0ItrigInmux4 {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Dmac0ItrigEna0ClrDmac0ItrigInmux4::from_bits(val as u8)
    }
    #[doc = "DMAC0 input trigger inmux 4 enable clear"]
    #[inline(always)]
    pub fn set_dmac0_itrig_inmux4(&mut self, val: super::vals::Dmac0ItrigEna0ClrDmac0ItrigInmux4) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "DMAC0 input trigger inmux 5 enable clear"]
    #[inline(always)]
    pub const fn dmac0_itrig_inmux5(&self) -> super::vals::Dmac0ItrigEna0ClrDmac0ItrigInmux5 {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::Dmac0ItrigEna0ClrDmac0ItrigInmux5::from_bits(val as u8)
    }
    #[doc = "DMAC0 input trigger inmux 5 enable clear"]
    #[inline(always)]
    pub fn set_dmac0_itrig_inmux5(&mut self, val: super::vals::Dmac0ItrigEna0ClrDmac0ItrigInmux5) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "DMAC0 input trigger inmux 6 enable clear"]
    #[inline(always)]
    pub const fn dmac0_itrig_inmux6(&self) -> super::vals::Dmac0ItrigEna0ClrDmac0ItrigInmux6 {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Dmac0ItrigEna0ClrDmac0ItrigInmux6::from_bits(val as u8)
    }
    #[doc = "DMAC0 input trigger inmux 6 enable clear"]
    #[inline(always)]
    pub fn set_dmac0_itrig_inmux6(&mut self, val: super::vals::Dmac0ItrigEna0ClrDmac0ItrigInmux6) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "DMAC0 input trigger inmux 7 enable clear"]
    #[inline(always)]
    pub const fn dmac0_itrig_inmux7(&self) -> super::vals::Dmac0ItrigEna0ClrDmac0ItrigInmux7 {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Dmac0ItrigEna0ClrDmac0ItrigInmux7::from_bits(val as u8)
    }
    #[doc = "DMAC0 input trigger inmux 7 enable clear"]
    #[inline(always)]
    pub fn set_dmac0_itrig_inmux7(&mut self, val: super::vals::Dmac0ItrigEna0ClrDmac0ItrigInmux7) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "DMAC0 input trigger inmux 8 enable clear"]
    #[inline(always)]
    pub const fn dmac0_itrig_inmux8(&self) -> super::vals::Dmac0ItrigEna0ClrDmac0ItrigInmux8 {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Dmac0ItrigEna0ClrDmac0ItrigInmux8::from_bits(val as u8)
    }
    #[doc = "DMAC0 input trigger inmux 8 enable clear"]
    #[inline(always)]
    pub fn set_dmac0_itrig_inmux8(&mut self, val: super::vals::Dmac0ItrigEna0ClrDmac0ItrigInmux8) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "DMAC0 input trigger inmux 9 enable clear"]
    #[inline(always)]
    pub const fn dmac0_itrig_inmux9(&self) -> super::vals::Dmac0ItrigEna0ClrDmac0ItrigInmux9 {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::Dmac0ItrigEna0ClrDmac0ItrigInmux9::from_bits(val as u8)
    }
    #[doc = "DMAC0 input trigger inmux 9 enable clear"]
    #[inline(always)]
    pub fn set_dmac0_itrig_inmux9(&mut self, val: super::vals::Dmac0ItrigEna0ClrDmac0ItrigInmux9) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "DMAC0 input trigger inmux 10 enable clear"]
    #[inline(always)]
    pub const fn dmac0_itrig_inmux10(&self) -> super::vals::Dmac0ItrigEna0ClrDmac0ItrigInmux10 {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::Dmac0ItrigEna0ClrDmac0ItrigInmux10::from_bits(val as u8)
    }
    #[doc = "DMAC0 input trigger inmux 10 enable clear"]
    #[inline(always)]
    pub fn set_dmac0_itrig_inmux10(
        &mut self,
        val: super::vals::Dmac0ItrigEna0ClrDmac0ItrigInmux10,
    ) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "DMAC0 input trigger inmux 11 enable clear"]
    #[inline(always)]
    pub const fn dmac0_itrig_inmux11(&self) -> super::vals::Dmac0ItrigEna0ClrDmac0ItrigInmux11 {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::Dmac0ItrigEna0ClrDmac0ItrigInmux11::from_bits(val as u8)
    }
    #[doc = "DMAC0 input trigger inmux 11 enable clear"]
    #[inline(always)]
    pub fn set_dmac0_itrig_inmux11(
        &mut self,
        val: super::vals::Dmac0ItrigEna0ClrDmac0ItrigInmux11,
    ) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "DMAC0 input trigger inmux 12 enable clear"]
    #[inline(always)]
    pub const fn dmac0_itrig_inmux12(&self) -> super::vals::Dmac0ItrigEna0ClrDmac0ItrigInmux12 {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::Dmac0ItrigEna0ClrDmac0ItrigInmux12::from_bits(val as u8)
    }
    #[doc = "DMAC0 input trigger inmux 12 enable clear"]
    #[inline(always)]
    pub fn set_dmac0_itrig_inmux12(
        &mut self,
        val: super::vals::Dmac0ItrigEna0ClrDmac0ItrigInmux12,
    ) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "DMAC0 input trigger inmux 13 enable clear"]
    #[inline(always)]
    pub const fn dmac0_itrig_inmux13(&self) -> super::vals::Dmac0ItrigEna0ClrDmac0ItrigInmux13 {
        let val = (self.0 >> 13usize) & 0x01;
        super::vals::Dmac0ItrigEna0ClrDmac0ItrigInmux13::from_bits(val as u8)
    }
    #[doc = "DMAC0 input trigger inmux 13 enable clear"]
    #[inline(always)]
    pub fn set_dmac0_itrig_inmux13(
        &mut self,
        val: super::vals::Dmac0ItrigEna0ClrDmac0ItrigInmux13,
    ) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "DMAC0 input trigger inmux 14 enable clear"]
    #[inline(always)]
    pub const fn dmac0_itrig_inmux14(&self) -> super::vals::Dmac0ItrigEna0ClrDmac0ItrigInmux14 {
        let val = (self.0 >> 14usize) & 0x01;
        super::vals::Dmac0ItrigEna0ClrDmac0ItrigInmux14::from_bits(val as u8)
    }
    #[doc = "DMAC0 input trigger inmux 14 enable clear"]
    #[inline(always)]
    pub fn set_dmac0_itrig_inmux14(
        &mut self,
        val: super::vals::Dmac0ItrigEna0ClrDmac0ItrigInmux14,
    ) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
    }
    #[doc = "DMAC0 input trigger inmux 15 enable clear"]
    #[inline(always)]
    pub const fn dmac0_itrig_inmux15(&self) -> super::vals::Dmac0ItrigEna0ClrDmac0ItrigInmux15 {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::Dmac0ItrigEna0ClrDmac0ItrigInmux15::from_bits(val as u8)
    }
    #[doc = "DMAC0 input trigger inmux 15 enable clear"]
    #[inline(always)]
    pub fn set_dmac0_itrig_inmux15(
        &mut self,
        val: super::vals::Dmac0ItrigEna0ClrDmac0ItrigInmux15,
    ) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
    #[doc = "DMAC0 input trigger inmux 16 enable clear"]
    #[inline(always)]
    pub const fn dmac0_itrig_inmux16(&self) -> super::vals::Dmac0ItrigEna0ClrDmac0ItrigInmux16 {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::Dmac0ItrigEna0ClrDmac0ItrigInmux16::from_bits(val as u8)
    }
    #[doc = "DMAC0 input trigger inmux 16 enable clear"]
    #[inline(always)]
    pub fn set_dmac0_itrig_inmux16(
        &mut self,
        val: super::vals::Dmac0ItrigEna0ClrDmac0ItrigInmux16,
    ) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "DMAC0 input trigger inmux 17 enable clear"]
    #[inline(always)]
    pub const fn dmac0_itrig_inmux17(&self) -> super::vals::Dmac0ItrigEna0ClrDmac0ItrigInmux17 {
        let val = (self.0 >> 17usize) & 0x01;
        super::vals::Dmac0ItrigEna0ClrDmac0ItrigInmux17::from_bits(val as u8)
    }
    #[doc = "DMAC0 input trigger inmux 17 enable clear"]
    #[inline(always)]
    pub fn set_dmac0_itrig_inmux17(
        &mut self,
        val: super::vals::Dmac0ItrigEna0ClrDmac0ItrigInmux17,
    ) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
    #[doc = "DMAC0 input trigger inmux 18 enable clear"]
    #[inline(always)]
    pub const fn dmac0_itrig_inmux18(&self) -> super::vals::Dmac0ItrigEna0ClrDmac0ItrigInmux18 {
        let val = (self.0 >> 18usize) & 0x01;
        super::vals::Dmac0ItrigEna0ClrDmac0ItrigInmux18::from_bits(val as u8)
    }
    #[doc = "DMAC0 input trigger inmux 18 enable clear"]
    #[inline(always)]
    pub fn set_dmac0_itrig_inmux18(
        &mut self,
        val: super::vals::Dmac0ItrigEna0ClrDmac0ItrigInmux18,
    ) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
    }
    #[doc = "DMAC0 input trigger inmux 19 enable clear"]
    #[inline(always)]
    pub const fn dmac0_itrig_inmux19(&self) -> super::vals::Dmac0ItrigEna0ClrDmac0ItrigInmux19 {
        let val = (self.0 >> 19usize) & 0x01;
        super::vals::Dmac0ItrigEna0ClrDmac0ItrigInmux19::from_bits(val as u8)
    }
    #[doc = "DMAC0 input trigger inmux 19 enable clear"]
    #[inline(always)]
    pub fn set_dmac0_itrig_inmux19(
        &mut self,
        val: super::vals::Dmac0ItrigEna0ClrDmac0ItrigInmux19,
    ) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
    #[doc = "DMAC0 input trigger inmux 20 enable clear"]
    #[inline(always)]
    pub const fn dmac0_itrig_inmux20(&self) -> super::vals::Dmac0ItrigEna0ClrDmac0ItrigInmux20 {
        let val = (self.0 >> 20usize) & 0x01;
        super::vals::Dmac0ItrigEna0ClrDmac0ItrigInmux20::from_bits(val as u8)
    }
    #[doc = "DMAC0 input trigger inmux 20 enable clear"]
    #[inline(always)]
    pub fn set_dmac0_itrig_inmux20(
        &mut self,
        val: super::vals::Dmac0ItrigEna0ClrDmac0ItrigInmux20,
    ) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
    #[doc = "DMAC0 input trigger inmux 21 enable clear"]
    #[inline(always)]
    pub const fn dmac0_itrig_inmux21(&self) -> super::vals::Dmac0ItrigEna0ClrDmac0ItrigInmux21 {
        let val = (self.0 >> 21usize) & 0x01;
        super::vals::Dmac0ItrigEna0ClrDmac0ItrigInmux21::from_bits(val as u8)
    }
    #[doc = "DMAC0 input trigger inmux 21 enable clear"]
    #[inline(always)]
    pub fn set_dmac0_itrig_inmux21(
        &mut self,
        val: super::vals::Dmac0ItrigEna0ClrDmac0ItrigInmux21,
    ) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
    }
    #[doc = "DMAC0 input trigger inmux 22 enable clear"]
    #[inline(always)]
    pub const fn dmac0_itrig_inmux22(&self) -> super::vals::Dmac0ItrigEna0ClrDmac0ItrigInmux22 {
        let val = (self.0 >> 22usize) & 0x01;
        super::vals::Dmac0ItrigEna0ClrDmac0ItrigInmux22::from_bits(val as u8)
    }
    #[doc = "DMAC0 input trigger inmux 22 enable clear"]
    #[inline(always)]
    pub fn set_dmac0_itrig_inmux22(
        &mut self,
        val: super::vals::Dmac0ItrigEna0ClrDmac0ItrigInmux22,
    ) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val.to_bits() as u32) & 0x01) << 22usize);
    }
    #[doc = "DMAC0 input trigger inmux 23 enable clear"]
    #[inline(always)]
    pub const fn dmac0_itrig_inmux23(&self) -> super::vals::Dmac0ItrigEna0ClrDmac0ItrigInmux23 {
        let val = (self.0 >> 23usize) & 0x01;
        super::vals::Dmac0ItrigEna0ClrDmac0ItrigInmux23::from_bits(val as u8)
    }
    #[doc = "DMAC0 input trigger inmux 23 enable clear"]
    #[inline(always)]
    pub fn set_dmac0_itrig_inmux23(
        &mut self,
        val: super::vals::Dmac0ItrigEna0ClrDmac0ItrigInmux23,
    ) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "DMAC0 input trigger inmux 24 enable clear"]
    #[inline(always)]
    pub const fn dmac0_itrig_inmux24(&self) -> super::vals::Dmac0ItrigEna0ClrDmac0ItrigInmux24 {
        let val = (self.0 >> 24usize) & 0x01;
        super::vals::Dmac0ItrigEna0ClrDmac0ItrigInmux24::from_bits(val as u8)
    }
    #[doc = "DMAC0 input trigger inmux 24 enable clear"]
    #[inline(always)]
    pub fn set_dmac0_itrig_inmux24(
        &mut self,
        val: super::vals::Dmac0ItrigEna0ClrDmac0ItrigInmux24,
    ) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
    #[doc = "DMAC0 input trigger inmux 25 enable clear"]
    #[inline(always)]
    pub const fn dmac0_itrig_inmux25(&self) -> super::vals::Dmac0ItrigEna0ClrDmac0ItrigInmux25 {
        let val = (self.0 >> 25usize) & 0x01;
        super::vals::Dmac0ItrigEna0ClrDmac0ItrigInmux25::from_bits(val as u8)
    }
    #[doc = "DMAC0 input trigger inmux 25 enable clear"]
    #[inline(always)]
    pub fn set_dmac0_itrig_inmux25(
        &mut self,
        val: super::vals::Dmac0ItrigEna0ClrDmac0ItrigInmux25,
    ) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val.to_bits() as u32) & 0x01) << 25usize);
    }
    #[doc = "DMAC0 input trigger inmux 26 enable clear"]
    #[inline(always)]
    pub const fn dmac0_itrig_inmux26(&self) -> super::vals::Dmac0ItrigEna0ClrDmac0ItrigInmux26 {
        let val = (self.0 >> 26usize) & 0x01;
        super::vals::Dmac0ItrigEna0ClrDmac0ItrigInmux26::from_bits(val as u8)
    }
    #[doc = "DMAC0 input trigger inmux 26 enable clear"]
    #[inline(always)]
    pub fn set_dmac0_itrig_inmux26(
        &mut self,
        val: super::vals::Dmac0ItrigEna0ClrDmac0ItrigInmux26,
    ) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val.to_bits() as u32) & 0x01) << 26usize);
    }
    #[doc = "DMAC0 input trigger inmux 27 enable clear"]
    #[inline(always)]
    pub const fn dmac0_itrig_inmux27(&self) -> super::vals::Dmac0ItrigEna0ClrDmac0ItrigInmux27 {
        let val = (self.0 >> 27usize) & 0x01;
        super::vals::Dmac0ItrigEna0ClrDmac0ItrigInmux27::from_bits(val as u8)
    }
    #[doc = "DMAC0 input trigger inmux 27 enable clear"]
    #[inline(always)]
    pub fn set_dmac0_itrig_inmux27(
        &mut self,
        val: super::vals::Dmac0ItrigEna0ClrDmac0ItrigInmux27,
    ) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val.to_bits() as u32) & 0x01) << 27usize);
    }
    #[doc = "DMAC0 input trigger inmux 28 enable clear"]
    #[inline(always)]
    pub const fn dmac0_itrig_inmux28(&self) -> super::vals::Dmac0ItrigEna0ClrDmac0ItrigInmux28 {
        let val = (self.0 >> 28usize) & 0x01;
        super::vals::Dmac0ItrigEna0ClrDmac0ItrigInmux28::from_bits(val as u8)
    }
    #[doc = "DMAC0 input trigger inmux 28 enable clear"]
    #[inline(always)]
    pub fn set_dmac0_itrig_inmux28(
        &mut self,
        val: super::vals::Dmac0ItrigEna0ClrDmac0ItrigInmux28,
    ) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val.to_bits() as u32) & 0x01) << 28usize);
    }
    #[doc = "DMAC0 input trigger inmux 29 enable clear"]
    #[inline(always)]
    pub const fn dmac0_itrig_inmux29(&self) -> super::vals::Dmac0ItrigEna0ClrDmac0ItrigInmux29 {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::Dmac0ItrigEna0ClrDmac0ItrigInmux29::from_bits(val as u8)
    }
    #[doc = "DMAC0 input trigger inmux 29 enable clear"]
    #[inline(always)]
    pub fn set_dmac0_itrig_inmux29(
        &mut self,
        val: super::vals::Dmac0ItrigEna0ClrDmac0ItrigInmux29,
    ) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "DMAC0 input trigger inmux 30 enable clear"]
    #[inline(always)]
    pub const fn dmac0_itrig_inmux30(&self) -> super::vals::Dmac0ItrigEna0ClrDmac0ItrigInmux30 {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::Dmac0ItrigEna0ClrDmac0ItrigInmux30::from_bits(val as u8)
    }
    #[doc = "DMAC0 input trigger inmux 30 enable clear"]
    #[inline(always)]
    pub fn set_dmac0_itrig_inmux30(
        &mut self,
        val: super::vals::Dmac0ItrigEna0ClrDmac0ItrigInmux30,
    ) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "DMAC0 input trigger inmux 31 enable clear"]
    #[inline(always)]
    pub const fn dmac0_itrig_inmux31(&self) -> super::vals::Dmac0ItrigEna0ClrDmac0ItrigInmux31 {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::Dmac0ItrigEna0ClrDmac0ItrigInmux31::from_bits(val as u8)
    }
    #[doc = "DMAC0 input trigger inmux 31 enable clear"]
    #[inline(always)]
    pub fn set_dmac0_itrig_inmux31(
        &mut self,
        val: super::vals::Dmac0ItrigEna0ClrDmac0ItrigInmux31,
    ) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Dmac0ItrigEna0Clr {
    #[inline(always)]
    fn default() -> Dmac0ItrigEna0Clr {
        Dmac0ItrigEna0Clr(0)
    }
}
#[doc = "DMAC0 input trigger enable set 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dmac0ItrigEna0Set(pub u32);
impl Dmac0ItrigEna0Set {
    #[doc = "DMAC0 input trigger inmux 0 enable set"]
    #[inline(always)]
    pub const fn dmac0_itrig_inmux0(&self) -> super::vals::Dmac0ItrigEna0SetDmac0ItrigInmux0 {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Dmac0ItrigEna0SetDmac0ItrigInmux0::from_bits(val as u8)
    }
    #[doc = "DMAC0 input trigger inmux 0 enable set"]
    #[inline(always)]
    pub fn set_dmac0_itrig_inmux0(&mut self, val: super::vals::Dmac0ItrigEna0SetDmac0ItrigInmux0) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "DMAC0 input trigger inmux 1 enable set"]
    #[inline(always)]
    pub const fn dmac0_itrig_inmux1(&self) -> super::vals::Dmac0ItrigEna0SetDmac0ItrigInmux1 {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Dmac0ItrigEna0SetDmac0ItrigInmux1::from_bits(val as u8)
    }
    #[doc = "DMAC0 input trigger inmux 1 enable set"]
    #[inline(always)]
    pub fn set_dmac0_itrig_inmux1(&mut self, val: super::vals::Dmac0ItrigEna0SetDmac0ItrigInmux1) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "DMAC0 input trigger inmux 2 enable set"]
    #[inline(always)]
    pub const fn dmac0_itrig_inmux2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "DMAC0 input trigger inmux 2 enable set"]
    #[inline(always)]
    pub fn set_dmac0_itrig_inmux2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "DMAC0 input trigger inmux 3 enable set"]
    #[inline(always)]
    pub const fn dmac0_itrig_inmux3(&self) -> super::vals::Dmac0ItrigEna0SetDmac0ItrigInmux3 {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Dmac0ItrigEna0SetDmac0ItrigInmux3::from_bits(val as u8)
    }
    #[doc = "DMAC0 input trigger inmux 3 enable set"]
    #[inline(always)]
    pub fn set_dmac0_itrig_inmux3(&mut self, val: super::vals::Dmac0ItrigEna0SetDmac0ItrigInmux3) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "DMAC0 input trigger inmux 4 enable set"]
    #[inline(always)]
    pub const fn dmac0_itrig_inmux4(&self) -> super::vals::Dmac0ItrigEna0SetDmac0ItrigInmux4 {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Dmac0ItrigEna0SetDmac0ItrigInmux4::from_bits(val as u8)
    }
    #[doc = "DMAC0 input trigger inmux 4 enable set"]
    #[inline(always)]
    pub fn set_dmac0_itrig_inmux4(&mut self, val: super::vals::Dmac0ItrigEna0SetDmac0ItrigInmux4) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "DMAC0 input trigger inmux 5 enable set"]
    #[inline(always)]
    pub const fn dmac0_itrig_inmux5(&self) -> super::vals::Dmac0ItrigEna0SetDmac0ItrigInmux5 {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::Dmac0ItrigEna0SetDmac0ItrigInmux5::from_bits(val as u8)
    }
    #[doc = "DMAC0 input trigger inmux 5 enable set"]
    #[inline(always)]
    pub fn set_dmac0_itrig_inmux5(&mut self, val: super::vals::Dmac0ItrigEna0SetDmac0ItrigInmux5) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "DMAC0 input trigger inmux 6 enable set"]
    #[inline(always)]
    pub const fn dmac0_itrig_inmux6(&self) -> super::vals::Dmac0ItrigEna0SetDmac0ItrigInmux6 {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Dmac0ItrigEna0SetDmac0ItrigInmux6::from_bits(val as u8)
    }
    #[doc = "DMAC0 input trigger inmux 6 enable set"]
    #[inline(always)]
    pub fn set_dmac0_itrig_inmux6(&mut self, val: super::vals::Dmac0ItrigEna0SetDmac0ItrigInmux6) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "DMAC0 input trigger inmux 7 enable set"]
    #[inline(always)]
    pub const fn dmac0_itrig_inmux7(&self) -> super::vals::Dmac0ItrigEna0SetDmac0ItrigInmux7 {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Dmac0ItrigEna0SetDmac0ItrigInmux7::from_bits(val as u8)
    }
    #[doc = "DMAC0 input trigger inmux 7 enable set"]
    #[inline(always)]
    pub fn set_dmac0_itrig_inmux7(&mut self, val: super::vals::Dmac0ItrigEna0SetDmac0ItrigInmux7) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "DMAC0 input trigger inmux 8 enable set"]
    #[inline(always)]
    pub const fn dmac0_itrig_inmux8(&self) -> super::vals::Dmac0ItrigEna0SetDmac0ItrigInmux8 {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Dmac0ItrigEna0SetDmac0ItrigInmux8::from_bits(val as u8)
    }
    #[doc = "DMAC0 input trigger inmux 8 enable set"]
    #[inline(always)]
    pub fn set_dmac0_itrig_inmux8(&mut self, val: super::vals::Dmac0ItrigEna0SetDmac0ItrigInmux8) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "DMAC0 input trigger inmux 9 enable set"]
    #[inline(always)]
    pub const fn dmac0_itrig_inmux9(&self) -> super::vals::Dmac0ItrigEna0SetDmac0ItrigInmux9 {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::Dmac0ItrigEna0SetDmac0ItrigInmux9::from_bits(val as u8)
    }
    #[doc = "DMAC0 input trigger inmux 9 enable set"]
    #[inline(always)]
    pub fn set_dmac0_itrig_inmux9(&mut self, val: super::vals::Dmac0ItrigEna0SetDmac0ItrigInmux9) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "DMAC0 input trigger inmux 10 enable set"]
    #[inline(always)]
    pub const fn dmac0_itrig_inmux10(&self) -> super::vals::Dmac0ItrigEna0SetDmac0ItrigInmux10 {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::Dmac0ItrigEna0SetDmac0ItrigInmux10::from_bits(val as u8)
    }
    #[doc = "DMAC0 input trigger inmux 10 enable set"]
    #[inline(always)]
    pub fn set_dmac0_itrig_inmux10(
        &mut self,
        val: super::vals::Dmac0ItrigEna0SetDmac0ItrigInmux10,
    ) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "DMAC0 input trigger inmux 11 enable set"]
    #[inline(always)]
    pub const fn dmac0_itrig_inmux11(&self) -> super::vals::Dmac0ItrigEna0SetDmac0ItrigInmux11 {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::Dmac0ItrigEna0SetDmac0ItrigInmux11::from_bits(val as u8)
    }
    #[doc = "DMAC0 input trigger inmux 11 enable set"]
    #[inline(always)]
    pub fn set_dmac0_itrig_inmux11(
        &mut self,
        val: super::vals::Dmac0ItrigEna0SetDmac0ItrigInmux11,
    ) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "DMAC0 input trigger inmux 12 enable set"]
    #[inline(always)]
    pub const fn dmac0_itrig_inmux12(&self) -> super::vals::Dmac0ItrigEna0SetDmac0ItrigInmux12 {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::Dmac0ItrigEna0SetDmac0ItrigInmux12::from_bits(val as u8)
    }
    #[doc = "DMAC0 input trigger inmux 12 enable set"]
    #[inline(always)]
    pub fn set_dmac0_itrig_inmux12(
        &mut self,
        val: super::vals::Dmac0ItrigEna0SetDmac0ItrigInmux12,
    ) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "DMAC0 input trigger inmux 13 enable set"]
    #[inline(always)]
    pub const fn dmac0_itrig_inmux13(&self) -> super::vals::Dmac0ItrigEna0SetDmac0ItrigInmux13 {
        let val = (self.0 >> 13usize) & 0x01;
        super::vals::Dmac0ItrigEna0SetDmac0ItrigInmux13::from_bits(val as u8)
    }
    #[doc = "DMAC0 input trigger inmux 13 enable set"]
    #[inline(always)]
    pub fn set_dmac0_itrig_inmux13(
        &mut self,
        val: super::vals::Dmac0ItrigEna0SetDmac0ItrigInmux13,
    ) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "DMAC0 input trigger inmux 14 enable set"]
    #[inline(always)]
    pub const fn dmac0_itrig_inmux14(&self) -> super::vals::Dmac0ItrigEna0SetDmac0ItrigInmux14 {
        let val = (self.0 >> 14usize) & 0x01;
        super::vals::Dmac0ItrigEna0SetDmac0ItrigInmux14::from_bits(val as u8)
    }
    #[doc = "DMAC0 input trigger inmux 14 enable set"]
    #[inline(always)]
    pub fn set_dmac0_itrig_inmux14(
        &mut self,
        val: super::vals::Dmac0ItrigEna0SetDmac0ItrigInmux14,
    ) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
    }
    #[doc = "DMAC0 input trigger inmux 15 enable set"]
    #[inline(always)]
    pub const fn dmac0_itrig_inmux15(&self) -> super::vals::Dmac0ItrigEna0SetDmac0ItrigInmux15 {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::Dmac0ItrigEna0SetDmac0ItrigInmux15::from_bits(val as u8)
    }
    #[doc = "DMAC0 input trigger inmux 15 enable set"]
    #[inline(always)]
    pub fn set_dmac0_itrig_inmux15(
        &mut self,
        val: super::vals::Dmac0ItrigEna0SetDmac0ItrigInmux15,
    ) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
    #[doc = "DMAC0 input trigger inmux 16 enable set"]
    #[inline(always)]
    pub const fn dmac0_itrig_inmux16(&self) -> super::vals::Dmac0ItrigEna0SetDmac0ItrigInmux16 {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::Dmac0ItrigEna0SetDmac0ItrigInmux16::from_bits(val as u8)
    }
    #[doc = "DMAC0 input trigger inmux 16 enable set"]
    #[inline(always)]
    pub fn set_dmac0_itrig_inmux16(
        &mut self,
        val: super::vals::Dmac0ItrigEna0SetDmac0ItrigInmux16,
    ) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "DMAC0 input trigger inmux 17 enable set"]
    #[inline(always)]
    pub const fn dmac0_itrig_inmux17(&self) -> super::vals::Dmac0ItrigEna0SetDmac0ItrigInmux17 {
        let val = (self.0 >> 17usize) & 0x01;
        super::vals::Dmac0ItrigEna0SetDmac0ItrigInmux17::from_bits(val as u8)
    }
    #[doc = "DMAC0 input trigger inmux 17 enable set"]
    #[inline(always)]
    pub fn set_dmac0_itrig_inmux17(
        &mut self,
        val: super::vals::Dmac0ItrigEna0SetDmac0ItrigInmux17,
    ) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
    #[doc = "DMAC0 input trigger inmux 18 enable set"]
    #[inline(always)]
    pub const fn dmac0_itrig_inmux18(&self) -> super::vals::Dmac0ItrigEna0SetDmac0ItrigInmux18 {
        let val = (self.0 >> 18usize) & 0x01;
        super::vals::Dmac0ItrigEna0SetDmac0ItrigInmux18::from_bits(val as u8)
    }
    #[doc = "DMAC0 input trigger inmux 18 enable set"]
    #[inline(always)]
    pub fn set_dmac0_itrig_inmux18(
        &mut self,
        val: super::vals::Dmac0ItrigEna0SetDmac0ItrigInmux18,
    ) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
    }
    #[doc = "DMAC0 input trigger inmux 19 enable set"]
    #[inline(always)]
    pub const fn dmac0_itrig_inmux19(&self) -> super::vals::Dmac0ItrigEna0SetDmac0ItrigInmux19 {
        let val = (self.0 >> 19usize) & 0x01;
        super::vals::Dmac0ItrigEna0SetDmac0ItrigInmux19::from_bits(val as u8)
    }
    #[doc = "DMAC0 input trigger inmux 19 enable set"]
    #[inline(always)]
    pub fn set_dmac0_itrig_inmux19(
        &mut self,
        val: super::vals::Dmac0ItrigEna0SetDmac0ItrigInmux19,
    ) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
    #[doc = "DMAC0 input trigger inmux 20 enable set"]
    #[inline(always)]
    pub const fn dmac0_itrig_inmux20(&self) -> super::vals::Dmac0ItrigEna0SetDmac0ItrigInmux20 {
        let val = (self.0 >> 20usize) & 0x01;
        super::vals::Dmac0ItrigEna0SetDmac0ItrigInmux20::from_bits(val as u8)
    }
    #[doc = "DMAC0 input trigger inmux 20 enable set"]
    #[inline(always)]
    pub fn set_dmac0_itrig_inmux20(
        &mut self,
        val: super::vals::Dmac0ItrigEna0SetDmac0ItrigInmux20,
    ) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
    #[doc = "DMAC0 input trigger inmux 21 enable set"]
    #[inline(always)]
    pub const fn dmac0_itrig_inmux21(&self) -> super::vals::Dmac0ItrigEna0SetDmac0ItrigInmux21 {
        let val = (self.0 >> 21usize) & 0x01;
        super::vals::Dmac0ItrigEna0SetDmac0ItrigInmux21::from_bits(val as u8)
    }
    #[doc = "DMAC0 input trigger inmux 21 enable set"]
    #[inline(always)]
    pub fn set_dmac0_itrig_inmux21(
        &mut self,
        val: super::vals::Dmac0ItrigEna0SetDmac0ItrigInmux21,
    ) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
    }
    #[doc = "DMAC0 input trigger inmux 22 enable set"]
    #[inline(always)]
    pub const fn dmac0_itrig_inmux22(&self) -> super::vals::Dmac0ItrigEna0SetDmac0ItrigInmux22 {
        let val = (self.0 >> 22usize) & 0x01;
        super::vals::Dmac0ItrigEna0SetDmac0ItrigInmux22::from_bits(val as u8)
    }
    #[doc = "DMAC0 input trigger inmux 22 enable set"]
    #[inline(always)]
    pub fn set_dmac0_itrig_inmux22(
        &mut self,
        val: super::vals::Dmac0ItrigEna0SetDmac0ItrigInmux22,
    ) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val.to_bits() as u32) & 0x01) << 22usize);
    }
    #[doc = "DMAC0 input trigger inmux 23 enable set"]
    #[inline(always)]
    pub const fn dmac0_itrig_inmux23(&self) -> super::vals::Dmac0ItrigEna0SetDmac0ItrigInmux23 {
        let val = (self.0 >> 23usize) & 0x01;
        super::vals::Dmac0ItrigEna0SetDmac0ItrigInmux23::from_bits(val as u8)
    }
    #[doc = "DMAC0 input trigger inmux 23 enable set"]
    #[inline(always)]
    pub fn set_dmac0_itrig_inmux23(
        &mut self,
        val: super::vals::Dmac0ItrigEna0SetDmac0ItrigInmux23,
    ) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "DMAC0 input trigger inmux 24 enable set"]
    #[inline(always)]
    pub const fn dmac0_itrig_inmux24(&self) -> super::vals::Dmac0ItrigEna0SetDmac0ItrigInmux24 {
        let val = (self.0 >> 24usize) & 0x01;
        super::vals::Dmac0ItrigEna0SetDmac0ItrigInmux24::from_bits(val as u8)
    }
    #[doc = "DMAC0 input trigger inmux 24 enable set"]
    #[inline(always)]
    pub fn set_dmac0_itrig_inmux24(
        &mut self,
        val: super::vals::Dmac0ItrigEna0SetDmac0ItrigInmux24,
    ) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
    #[doc = "DMAC0 input trigger inmux 25 enable set"]
    #[inline(always)]
    pub const fn dmac0_itrig_inmux25(&self) -> super::vals::Dmac0ItrigEna0SetDmac0ItrigInmux25 {
        let val = (self.0 >> 25usize) & 0x01;
        super::vals::Dmac0ItrigEna0SetDmac0ItrigInmux25::from_bits(val as u8)
    }
    #[doc = "DMAC0 input trigger inmux 25 enable set"]
    #[inline(always)]
    pub fn set_dmac0_itrig_inmux25(
        &mut self,
        val: super::vals::Dmac0ItrigEna0SetDmac0ItrigInmux25,
    ) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val.to_bits() as u32) & 0x01) << 25usize);
    }
    #[doc = "DMAC0 input trigger inmux 26 enable set"]
    #[inline(always)]
    pub const fn dmac0_itrig_inmux26(&self) -> super::vals::Dmac0ItrigEna0SetDmac0ItrigInmux26 {
        let val = (self.0 >> 26usize) & 0x01;
        super::vals::Dmac0ItrigEna0SetDmac0ItrigInmux26::from_bits(val as u8)
    }
    #[doc = "DMAC0 input trigger inmux 26 enable set"]
    #[inline(always)]
    pub fn set_dmac0_itrig_inmux26(
        &mut self,
        val: super::vals::Dmac0ItrigEna0SetDmac0ItrigInmux26,
    ) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val.to_bits() as u32) & 0x01) << 26usize);
    }
    #[doc = "DMAC0 input trigger inmux 27 enable set"]
    #[inline(always)]
    pub const fn dmac0_itrig_inmux27(&self) -> super::vals::Dmac0ItrigEna0SetDmac0ItrigInmux27 {
        let val = (self.0 >> 27usize) & 0x01;
        super::vals::Dmac0ItrigEna0SetDmac0ItrigInmux27::from_bits(val as u8)
    }
    #[doc = "DMAC0 input trigger inmux 27 enable set"]
    #[inline(always)]
    pub fn set_dmac0_itrig_inmux27(
        &mut self,
        val: super::vals::Dmac0ItrigEna0SetDmac0ItrigInmux27,
    ) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val.to_bits() as u32) & 0x01) << 27usize);
    }
    #[doc = "DMAC0 input trigger inmux 28 enable set"]
    #[inline(always)]
    pub const fn dmac0_itrig_inmux28(&self) -> super::vals::Dmac0ItrigEna0SetDmac0ItrigInmux28 {
        let val = (self.0 >> 28usize) & 0x01;
        super::vals::Dmac0ItrigEna0SetDmac0ItrigInmux28::from_bits(val as u8)
    }
    #[doc = "DMAC0 input trigger inmux 28 enable set"]
    #[inline(always)]
    pub fn set_dmac0_itrig_inmux28(
        &mut self,
        val: super::vals::Dmac0ItrigEna0SetDmac0ItrigInmux28,
    ) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val.to_bits() as u32) & 0x01) << 28usize);
    }
    #[doc = "DMAC0 input trigger inmux 29 enable set"]
    #[inline(always)]
    pub const fn dmac0_itrig_inmux29(&self) -> super::vals::Dmac0ItrigEna0SetDmac0ItrigInmux29 {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::Dmac0ItrigEna0SetDmac0ItrigInmux29::from_bits(val as u8)
    }
    #[doc = "DMAC0 input trigger inmux 29 enable set"]
    #[inline(always)]
    pub fn set_dmac0_itrig_inmux29(
        &mut self,
        val: super::vals::Dmac0ItrigEna0SetDmac0ItrigInmux29,
    ) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "DMAC0 input trigger inmux 30 enable set"]
    #[inline(always)]
    pub const fn dmac0_itrig_inmux30(&self) -> super::vals::Dmac0ItrigEna0SetDmac0ItrigInmux30 {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::Dmac0ItrigEna0SetDmac0ItrigInmux30::from_bits(val as u8)
    }
    #[doc = "DMAC0 input trigger inmux 30 enable set"]
    #[inline(always)]
    pub fn set_dmac0_itrig_inmux30(
        &mut self,
        val: super::vals::Dmac0ItrigEna0SetDmac0ItrigInmux30,
    ) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "DMAC0 input trigger inmux 31 enable set"]
    #[inline(always)]
    pub const fn dmac0_itrig_inmux31(&self) -> super::vals::Dmac0ItrigEna0SetDmac0ItrigInmux31 {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::Dmac0ItrigEna0SetDmac0ItrigInmux31::from_bits(val as u8)
    }
    #[doc = "DMAC0 input trigger inmux 31 enable set"]
    #[inline(always)]
    pub fn set_dmac0_itrig_inmux31(
        &mut self,
        val: super::vals::Dmac0ItrigEna0SetDmac0ItrigInmux31,
    ) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Dmac0ItrigEna0Set {
    #[inline(always)]
    fn default() -> Dmac0ItrigEna0Set {
        Dmac0ItrigEna0Set(0)
    }
}
#[doc = "DMAC0 Input Trigger Multiplexers N"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dmac0ItrigSel(pub u32);
impl Dmac0ItrigSel {
    #[doc = "DMA Input Triggers(n) Selection. 22:1 Selection for each. . ."]
    #[inline(always)]
    pub const fn dma0_itrig_sel(&self) -> super::vals::Dma0ItrigSel {
        let val = (self.0 >> 0usize) & 0x1f;
        super::vals::Dma0ItrigSel::from_bits(val as u8)
    }
    #[doc = "DMA Input Triggers(n) Selection. 22:1 Selection for each. . ."]
    #[inline(always)]
    pub fn set_dma0_itrig_sel(&mut self, val: super::vals::Dma0ItrigSel) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val.to_bits() as u32) & 0x1f) << 0usize);
    }
}
impl Default for Dmac0ItrigSel {
    #[inline(always)]
    fn default() -> Dmac0ItrigSel {
        Dmac0ItrigSel(0)
    }
}
#[doc = "DMAC0 Output Trigger Multiplexers N"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dmac0OtrigSel(pub u32);
impl Dmac0OtrigSel {
    #[doc = "DMAC0 Output Triggers Select for A, B, C, D IE.,DMAC0_OTRIG_A, DMAC0_OTRIG_B, DMAC0_OTRIG_C, DMAC0_OTRIG_D DMA0 Output Triggers(n) Selection. 32:1 Selection for each. . ."]
    #[inline(always)]
    pub const fn dmac0_otrig_sel(&self) -> super::vals::Dmac0OtrigSel {
        let val = (self.0 >> 0usize) & 0x3f;
        super::vals::Dmac0OtrigSel::from_bits(val as u8)
    }
    #[doc = "DMAC0 Output Triggers Select for A, B, C, D IE.,DMAC0_OTRIG_A, DMAC0_OTRIG_B, DMAC0_OTRIG_C, DMAC0_OTRIG_D DMA0 Output Triggers(n) Selection. 32:1 Selection for each. . ."]
    #[inline(always)]
    pub fn set_dmac0_otrig_sel(&mut self, val: super::vals::Dmac0OtrigSel) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val.to_bits() as u32) & 0x3f) << 0usize);
    }
}
impl Default for Dmac0OtrigSel {
    #[inline(always)]
    fn default() -> Dmac0OtrigSel {
        Dmac0OtrigSel(0)
    }
}
#[doc = "DMAC0 request enable 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dmac0ReqEna0(pub u32);
impl Dmac0ReqEna0 {
    #[doc = "FLEXCOMM0 RX enable"]
    #[inline(always)]
    pub const fn flexcomm0_rx(&self) -> super::vals::Dmac0ReqEna0Flexcomm0Rx {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Dmac0ReqEna0Flexcomm0Rx::from_bits(val as u8)
    }
    #[doc = "FLEXCOMM0 RX enable"]
    #[inline(always)]
    pub fn set_flexcomm0_rx(&mut self, val: super::vals::Dmac0ReqEna0Flexcomm0Rx) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "FLEXCOMM0 TX enable"]
    #[inline(always)]
    pub const fn flexcomm0_tx(&self) -> super::vals::Dmac0ReqEna0Flexcomm0Tx {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Dmac0ReqEna0Flexcomm0Tx::from_bits(val as u8)
    }
    #[doc = "FLEXCOMM0 TX enable"]
    #[inline(always)]
    pub fn set_flexcomm0_tx(&mut self, val: super::vals::Dmac0ReqEna0Flexcomm0Tx) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "FLEXCOMM1 RX enable"]
    #[inline(always)]
    pub const fn flexcomm1_rx(&self) -> super::vals::Dmac0ReqEna0Flexcomm1Rx {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Dmac0ReqEna0Flexcomm1Rx::from_bits(val as u8)
    }
    #[doc = "FLEXCOMM1 RX enable"]
    #[inline(always)]
    pub fn set_flexcomm1_rx(&mut self, val: super::vals::Dmac0ReqEna0Flexcomm1Rx) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "FLEXCOMM1 TX enable"]
    #[inline(always)]
    pub const fn flexcomm1_tx(&self) -> super::vals::Dmac0ReqEna0Flexcomm1Tx {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Dmac0ReqEna0Flexcomm1Tx::from_bits(val as u8)
    }
    #[doc = "FLEXCOMM1 TX enable"]
    #[inline(always)]
    pub fn set_flexcomm1_tx(&mut self, val: super::vals::Dmac0ReqEna0Flexcomm1Tx) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "FLEXCOMM2 RX enable"]
    #[inline(always)]
    pub const fn flexcomm2_rx(&self) -> super::vals::Dmac0ReqEna0Flexcomm2Rx {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Dmac0ReqEna0Flexcomm2Rx::from_bits(val as u8)
    }
    #[doc = "FLEXCOMM2 RX enable"]
    #[inline(always)]
    pub fn set_flexcomm2_rx(&mut self, val: super::vals::Dmac0ReqEna0Flexcomm2Rx) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "FLEXCOMM2 TX enable"]
    #[inline(always)]
    pub const fn flexcomm2_tx(&self) -> super::vals::Dmac0ReqEna0Flexcomm2Tx {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::Dmac0ReqEna0Flexcomm2Tx::from_bits(val as u8)
    }
    #[doc = "FLEXCOMM2 TX enable"]
    #[inline(always)]
    pub fn set_flexcomm2_tx(&mut self, val: super::vals::Dmac0ReqEna0Flexcomm2Tx) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "FLEXCOMM3 RX enable"]
    #[inline(always)]
    pub const fn flexcomm3_rx(&self) -> super::vals::Dmac0ReqEna0Flexcomm3Rx {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Dmac0ReqEna0Flexcomm3Rx::from_bits(val as u8)
    }
    #[doc = "FLEXCOMM3 RX enable"]
    #[inline(always)]
    pub fn set_flexcomm3_rx(&mut self, val: super::vals::Dmac0ReqEna0Flexcomm3Rx) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "FLEXCOMM3 TX enable"]
    #[inline(always)]
    pub const fn flexcomm3_tx(&self) -> super::vals::Dmac0ReqEna0Flexcomm3Tx {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Dmac0ReqEna0Flexcomm3Tx::from_bits(val as u8)
    }
    #[doc = "FLEXCOMM3 TX enable"]
    #[inline(always)]
    pub fn set_flexcomm3_tx(&mut self, val: super::vals::Dmac0ReqEna0Flexcomm3Tx) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "FLEXCOMM4 RX enable"]
    #[inline(always)]
    pub const fn flexcomm4_rx(&self) -> super::vals::Dmac0ReqEna0Flexcomm4Rx {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Dmac0ReqEna0Flexcomm4Rx::from_bits(val as u8)
    }
    #[doc = "FLEXCOMM4 RX enable"]
    #[inline(always)]
    pub fn set_flexcomm4_rx(&mut self, val: super::vals::Dmac0ReqEna0Flexcomm4Rx) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "FLEXCOMM4 TX enable"]
    #[inline(always)]
    pub const fn flexcomm4_tx(&self) -> super::vals::Dmac0ReqEna0Flexcomm4Tx {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::Dmac0ReqEna0Flexcomm4Tx::from_bits(val as u8)
    }
    #[doc = "FLEXCOMM4 TX enable"]
    #[inline(always)]
    pub fn set_flexcomm4_tx(&mut self, val: super::vals::Dmac0ReqEna0Flexcomm4Tx) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "FLEXCOMM5 RX enable"]
    #[inline(always)]
    pub const fn flexcomm5_rx(&self) -> super::vals::Dmac0ReqEna0Flexcomm5Rx {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::Dmac0ReqEna0Flexcomm5Rx::from_bits(val as u8)
    }
    #[doc = "FLEXCOMM5 RX enable"]
    #[inline(always)]
    pub fn set_flexcomm5_rx(&mut self, val: super::vals::Dmac0ReqEna0Flexcomm5Rx) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "FLEXCOMM5 TX enable"]
    #[inline(always)]
    pub const fn flexcomm5_tx(&self) -> super::vals::Dmac0ReqEna0Flexcomm5Tx {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::Dmac0ReqEna0Flexcomm5Tx::from_bits(val as u8)
    }
    #[doc = "FLEXCOMM5 TX enable"]
    #[inline(always)]
    pub fn set_flexcomm5_tx(&mut self, val: super::vals::Dmac0ReqEna0Flexcomm5Tx) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "FLEXCOMM6 RX enable"]
    #[inline(always)]
    pub const fn flexcomm6_rx(&self) -> super::vals::Dmac0ReqEna0Flexcomm6Rx {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::Dmac0ReqEna0Flexcomm6Rx::from_bits(val as u8)
    }
    #[doc = "FLEXCOMM6 RX enable"]
    #[inline(always)]
    pub fn set_flexcomm6_rx(&mut self, val: super::vals::Dmac0ReqEna0Flexcomm6Rx) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "FLEXCOMM6 TX enable"]
    #[inline(always)]
    pub const fn flexcomm6_tx(&self) -> super::vals::Dmac0ReqEna0Flexcomm6Tx {
        let val = (self.0 >> 13usize) & 0x01;
        super::vals::Dmac0ReqEna0Flexcomm6Tx::from_bits(val as u8)
    }
    #[doc = "FLEXCOMM6 TX enable"]
    #[inline(always)]
    pub fn set_flexcomm6_tx(&mut self, val: super::vals::Dmac0ReqEna0Flexcomm6Tx) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "FLEXCOMM7 RX enable"]
    #[inline(always)]
    pub const fn flexcomm7_rx(&self) -> super::vals::Dmac0ReqEna0Flexcomm7Rx {
        let val = (self.0 >> 14usize) & 0x01;
        super::vals::Dmac0ReqEna0Flexcomm7Rx::from_bits(val as u8)
    }
    #[doc = "FLEXCOMM7 RX enable"]
    #[inline(always)]
    pub fn set_flexcomm7_rx(&mut self, val: super::vals::Dmac0ReqEna0Flexcomm7Rx) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
    }
    #[doc = "FLEXCOMM7 TX enable"]
    #[inline(always)]
    pub const fn flexcomm7_tx(&self) -> super::vals::Dmac0ReqEna0Flexcomm7Tx {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::Dmac0ReqEna0Flexcomm7Tx::from_bits(val as u8)
    }
    #[doc = "FLEXCOMM7 TX enable"]
    #[inline(always)]
    pub fn set_flexcomm7_tx(&mut self, val: super::vals::Dmac0ReqEna0Flexcomm7Tx) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
    #[doc = "DMIC0 channel 0 enable"]
    #[inline(always)]
    pub const fn dmic0ch0(&self) -> super::vals::Dmac0ReqEna0Dmic0ch0 {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::Dmac0ReqEna0Dmic0ch0::from_bits(val as u8)
    }
    #[doc = "DMIC0 channel 0 enable"]
    #[inline(always)]
    pub fn set_dmic0ch0(&mut self, val: super::vals::Dmac0ReqEna0Dmic0ch0) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "DMIC0 channel 1 enable"]
    #[inline(always)]
    pub const fn dmic0ch1(&self) -> super::vals::Dmac0ReqEna0Dmic0ch1 {
        let val = (self.0 >> 17usize) & 0x01;
        super::vals::Dmac0ReqEna0Dmic0ch1::from_bits(val as u8)
    }
    #[doc = "DMIC0 channel 1 enable"]
    #[inline(always)]
    pub fn set_dmic0ch1(&mut self, val: super::vals::Dmac0ReqEna0Dmic0ch1) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
    #[doc = "DMIC0 channel 2 enable"]
    #[inline(always)]
    pub const fn dmic0ch2(&self) -> super::vals::Dmac0ReqEna0Dmic0ch2 {
        let val = (self.0 >> 18usize) & 0x01;
        super::vals::Dmac0ReqEna0Dmic0ch2::from_bits(val as u8)
    }
    #[doc = "DMIC0 channel 2 enable"]
    #[inline(always)]
    pub fn set_dmic0ch2(&mut self, val: super::vals::Dmac0ReqEna0Dmic0ch2) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
    }
    #[doc = "DMIC0 channel 3 enable"]
    #[inline(always)]
    pub const fn dmic0ch3(&self) -> super::vals::Dmac0ReqEna0Dmic0ch3 {
        let val = (self.0 >> 19usize) & 0x01;
        super::vals::Dmac0ReqEna0Dmic0ch3::from_bits(val as u8)
    }
    #[doc = "DMIC0 channel 3 enable"]
    #[inline(always)]
    pub fn set_dmic0ch3(&mut self, val: super::vals::Dmac0ReqEna0Dmic0ch3) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
    #[doc = "DMIC0 channel 4 enable"]
    #[inline(always)]
    pub const fn dmic0ch4(&self) -> super::vals::Dmac0ReqEna0Dmic0ch4 {
        let val = (self.0 >> 20usize) & 0x01;
        super::vals::Dmac0ReqEna0Dmic0ch4::from_bits(val as u8)
    }
    #[doc = "DMIC0 channel 4 enable"]
    #[inline(always)]
    pub fn set_dmic0ch4(&mut self, val: super::vals::Dmac0ReqEna0Dmic0ch4) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
    #[doc = "DMIC0 channel 5 enable"]
    #[inline(always)]
    pub const fn dmic0ch5(&self) -> super::vals::Dmac0ReqEna0Dmic0ch5 {
        let val = (self.0 >> 21usize) & 0x01;
        super::vals::Dmac0ReqEna0Dmic0ch5::from_bits(val as u8)
    }
    #[doc = "DMIC0 channel 5 enable"]
    #[inline(always)]
    pub fn set_dmic0ch5(&mut self, val: super::vals::Dmac0ReqEna0Dmic0ch5) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
    }
    #[doc = "DMIC0 channel 6 enable"]
    #[inline(always)]
    pub const fn dmic0ch6(&self) -> super::vals::Dmac0ReqEna0Dmic0ch6 {
        let val = (self.0 >> 22usize) & 0x01;
        super::vals::Dmac0ReqEna0Dmic0ch6::from_bits(val as u8)
    }
    #[doc = "DMIC0 channel 6 enable"]
    #[inline(always)]
    pub fn set_dmic0ch6(&mut self, val: super::vals::Dmac0ReqEna0Dmic0ch6) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val.to_bits() as u32) & 0x01) << 22usize);
    }
    #[doc = "DMIC0 channel 7 enable"]
    #[inline(always)]
    pub const fn dmic0ch7(&self) -> super::vals::Dmac0ReqEna0Dmic0ch7 {
        let val = (self.0 >> 23usize) & 0x01;
        super::vals::Dmac0ReqEna0Dmic0ch7::from_bits(val as u8)
    }
    #[doc = "DMIC0 channel 7 enable"]
    #[inline(always)]
    pub fn set_dmic0ch7(&mut self, val: super::vals::Dmac0ReqEna0Dmic0ch7) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "I3C RX enable"]
    #[inline(always)]
    pub const fn i3c0_rx(&self) -> super::vals::Dmac0ReqEna0I3c0Rx {
        let val = (self.0 >> 24usize) & 0x01;
        super::vals::Dmac0ReqEna0I3c0Rx::from_bits(val as u8)
    }
    #[doc = "I3C RX enable"]
    #[inline(always)]
    pub fn set_i3c0_rx(&mut self, val: super::vals::Dmac0ReqEna0I3c0Rx) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
    #[doc = "I3C TX enable"]
    #[inline(always)]
    pub const fn i3c0_tx(&self) -> super::vals::Dmac0ReqEna0I3c0Tx {
        let val = (self.0 >> 25usize) & 0x01;
        super::vals::Dmac0ReqEna0I3c0Tx::from_bits(val as u8)
    }
    #[doc = "I3C TX enable"]
    #[inline(always)]
    pub fn set_i3c0_tx(&mut self, val: super::vals::Dmac0ReqEna0I3c0Tx) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val.to_bits() as u32) & 0x01) << 25usize);
    }
    #[doc = "FLEXCOMM14 RX enable"]
    #[inline(always)]
    pub const fn flexcomm14_rx(&self) -> super::vals::Dmac0ReqEna0Flexcomm14Rx {
        let val = (self.0 >> 26usize) & 0x01;
        super::vals::Dmac0ReqEna0Flexcomm14Rx::from_bits(val as u8)
    }
    #[doc = "FLEXCOMM14 RX enable"]
    #[inline(always)]
    pub fn set_flexcomm14_rx(&mut self, val: super::vals::Dmac0ReqEna0Flexcomm14Rx) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val.to_bits() as u32) & 0x01) << 26usize);
    }
    #[doc = "FLEXCOMM14 TX enable"]
    #[inline(always)]
    pub const fn flexcomm14_tx(&self) -> super::vals::Dmac0ReqEna0Flexcomm14Tx {
        let val = (self.0 >> 27usize) & 0x01;
        super::vals::Dmac0ReqEna0Flexcomm14Tx::from_bits(val as u8)
    }
    #[doc = "FLEXCOMM14 TX enable"]
    #[inline(always)]
    pub fn set_flexcomm14_tx(&mut self, val: super::vals::Dmac0ReqEna0Flexcomm14Tx) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val.to_bits() as u32) & 0x01) << 27usize);
    }
    #[doc = "hash enable"]
    #[inline(always)]
    pub const fn hashcrypt(&self) -> super::vals::Dmac0ReqEna0Hashcrypt {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::Dmac0ReqEna0Hashcrypt::from_bits(val as u8)
    }
    #[doc = "hash enable"]
    #[inline(always)]
    pub fn set_hashcrypt(&mut self, val: super::vals::Dmac0ReqEna0Hashcrypt) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
}
impl Default for Dmac0ReqEna0 {
    #[inline(always)]
    fn default() -> Dmac0ReqEna0 {
        Dmac0ReqEna0(0)
    }
}
#[doc = "DMAC0 request enable clear 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dmac0ReqEna0Clr(pub u32);
impl Dmac0ReqEna0Clr {
    #[doc = "FLEXCOMM0 RX enable clear"]
    #[inline(always)]
    pub const fn flexcomm0_rx(&self) -> super::vals::Dmac0ReqEna0ClrFlexcomm0Rx {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Dmac0ReqEna0ClrFlexcomm0Rx::from_bits(val as u8)
    }
    #[doc = "FLEXCOMM0 RX enable clear"]
    #[inline(always)]
    pub fn set_flexcomm0_rx(&mut self, val: super::vals::Dmac0ReqEna0ClrFlexcomm0Rx) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "FLEXCOMM0 TX enable clear"]
    #[inline(always)]
    pub const fn flexcomm0_tx(&self) -> super::vals::Dmac0ReqEna0ClrFlexcomm0Tx {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Dmac0ReqEna0ClrFlexcomm0Tx::from_bits(val as u8)
    }
    #[doc = "FLEXCOMM0 TX enable clear"]
    #[inline(always)]
    pub fn set_flexcomm0_tx(&mut self, val: super::vals::Dmac0ReqEna0ClrFlexcomm0Tx) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "FLEXCOMM1 RX enable clear"]
    #[inline(always)]
    pub const fn flexcomm1_rx(&self) -> super::vals::Dmac0ReqEna0ClrFlexcomm1Rx {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Dmac0ReqEna0ClrFlexcomm1Rx::from_bits(val as u8)
    }
    #[doc = "FLEXCOMM1 RX enable clear"]
    #[inline(always)]
    pub fn set_flexcomm1_rx(&mut self, val: super::vals::Dmac0ReqEna0ClrFlexcomm1Rx) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "FLEXCOMM1 TX enable clear"]
    #[inline(always)]
    pub const fn flexcomm1_tx(&self) -> super::vals::Dmac0ReqEna0ClrFlexcomm1Tx {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Dmac0ReqEna0ClrFlexcomm1Tx::from_bits(val as u8)
    }
    #[doc = "FLEXCOMM1 TX enable clear"]
    #[inline(always)]
    pub fn set_flexcomm1_tx(&mut self, val: super::vals::Dmac0ReqEna0ClrFlexcomm1Tx) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "FLEXCOMM2 RX enable clear"]
    #[inline(always)]
    pub const fn flexcomm2_rx(&self) -> super::vals::Dmac0ReqEna0ClrFlexcomm2Rx {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Dmac0ReqEna0ClrFlexcomm2Rx::from_bits(val as u8)
    }
    #[doc = "FLEXCOMM2 RX enable clear"]
    #[inline(always)]
    pub fn set_flexcomm2_rx(&mut self, val: super::vals::Dmac0ReqEna0ClrFlexcomm2Rx) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "FLEXCOMM2 TX enable clear"]
    #[inline(always)]
    pub const fn flexcomm2_tx(&self) -> super::vals::Dmac0ReqEna0ClrFlexcomm2Tx {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::Dmac0ReqEna0ClrFlexcomm2Tx::from_bits(val as u8)
    }
    #[doc = "FLEXCOMM2 TX enable clear"]
    #[inline(always)]
    pub fn set_flexcomm2_tx(&mut self, val: super::vals::Dmac0ReqEna0ClrFlexcomm2Tx) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "FLEXCOMM3 RX enable clear"]
    #[inline(always)]
    pub const fn flexcomm3_rx(&self) -> super::vals::Dmac0ReqEna0ClrFlexcomm3Rx {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Dmac0ReqEna0ClrFlexcomm3Rx::from_bits(val as u8)
    }
    #[doc = "FLEXCOMM3 RX enable clear"]
    #[inline(always)]
    pub fn set_flexcomm3_rx(&mut self, val: super::vals::Dmac0ReqEna0ClrFlexcomm3Rx) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "FLEXCOMM3 TX enable clear"]
    #[inline(always)]
    pub const fn flexcomm3_tx(&self) -> super::vals::Dmac0ReqEna0ClrFlexcomm3Tx {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Dmac0ReqEna0ClrFlexcomm3Tx::from_bits(val as u8)
    }
    #[doc = "FLEXCOMM3 TX enable clear"]
    #[inline(always)]
    pub fn set_flexcomm3_tx(&mut self, val: super::vals::Dmac0ReqEna0ClrFlexcomm3Tx) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "FLEXCOMM4 RX enable clear"]
    #[inline(always)]
    pub const fn flexcomm4_rx(&self) -> super::vals::Dmac0ReqEna0ClrFlexcomm4Rx {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Dmac0ReqEna0ClrFlexcomm4Rx::from_bits(val as u8)
    }
    #[doc = "FLEXCOMM4 RX enable clear"]
    #[inline(always)]
    pub fn set_flexcomm4_rx(&mut self, val: super::vals::Dmac0ReqEna0ClrFlexcomm4Rx) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "FLEXCOMM4 TX enable clear"]
    #[inline(always)]
    pub const fn flexcomm4_tx(&self) -> super::vals::Dmac0ReqEna0ClrFlexcomm4Tx {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::Dmac0ReqEna0ClrFlexcomm4Tx::from_bits(val as u8)
    }
    #[doc = "FLEXCOMM4 TX enable clear"]
    #[inline(always)]
    pub fn set_flexcomm4_tx(&mut self, val: super::vals::Dmac0ReqEna0ClrFlexcomm4Tx) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "FLEXCOMM5 RX enable clear"]
    #[inline(always)]
    pub const fn flexcomm5_rx(&self) -> super::vals::Dmac0ReqEna0ClrFlexcomm5Rx {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::Dmac0ReqEna0ClrFlexcomm5Rx::from_bits(val as u8)
    }
    #[doc = "FLEXCOMM5 RX enable clear"]
    #[inline(always)]
    pub fn set_flexcomm5_rx(&mut self, val: super::vals::Dmac0ReqEna0ClrFlexcomm5Rx) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "FLEXCOMM5 TX enable clear"]
    #[inline(always)]
    pub const fn flexcomm5_tx(&self) -> super::vals::Dmac0ReqEna0ClrFlexcomm5Tx {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::Dmac0ReqEna0ClrFlexcomm5Tx::from_bits(val as u8)
    }
    #[doc = "FLEXCOMM5 TX enable clear"]
    #[inline(always)]
    pub fn set_flexcomm5_tx(&mut self, val: super::vals::Dmac0ReqEna0ClrFlexcomm5Tx) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "FLEXCOMM6 RX enable clear"]
    #[inline(always)]
    pub const fn flexcomm6_rx(&self) -> super::vals::Dmac0ReqEna0ClrFlexcomm6Rx {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::Dmac0ReqEna0ClrFlexcomm6Rx::from_bits(val as u8)
    }
    #[doc = "FLEXCOMM6 RX enable clear"]
    #[inline(always)]
    pub fn set_flexcomm6_rx(&mut self, val: super::vals::Dmac0ReqEna0ClrFlexcomm6Rx) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "FLEXCOMM6 TX enable clear"]
    #[inline(always)]
    pub const fn flexcomm6_tx(&self) -> super::vals::Dmac0ReqEna0ClrFlexcomm6Tx {
        let val = (self.0 >> 13usize) & 0x01;
        super::vals::Dmac0ReqEna0ClrFlexcomm6Tx::from_bits(val as u8)
    }
    #[doc = "FLEXCOMM6 TX enable clear"]
    #[inline(always)]
    pub fn set_flexcomm6_tx(&mut self, val: super::vals::Dmac0ReqEna0ClrFlexcomm6Tx) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "FLEXCOMM7 RX enable clear"]
    #[inline(always)]
    pub const fn flexcomm7_rx(&self) -> super::vals::Dmac0ReqEna0ClrFlexcomm7Rx {
        let val = (self.0 >> 14usize) & 0x01;
        super::vals::Dmac0ReqEna0ClrFlexcomm7Rx::from_bits(val as u8)
    }
    #[doc = "FLEXCOMM7 RX enable clear"]
    #[inline(always)]
    pub fn set_flexcomm7_rx(&mut self, val: super::vals::Dmac0ReqEna0ClrFlexcomm7Rx) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
    }
    #[doc = "FLEXCOMM7 TX enable clear"]
    #[inline(always)]
    pub const fn flexcomm7_tx(&self) -> super::vals::Dmac0ReqEna0ClrFlexcomm7Tx {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::Dmac0ReqEna0ClrFlexcomm7Tx::from_bits(val as u8)
    }
    #[doc = "FLEXCOMM7 TX enable clear"]
    #[inline(always)]
    pub fn set_flexcomm7_tx(&mut self, val: super::vals::Dmac0ReqEna0ClrFlexcomm7Tx) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
    #[doc = "DMIC0 channel 0 enable clear"]
    #[inline(always)]
    pub const fn dmic0ch0(&self) -> super::vals::Dmac0ReqEna0ClrDmic0ch0 {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::Dmac0ReqEna0ClrDmic0ch0::from_bits(val as u8)
    }
    #[doc = "DMIC0 channel 0 enable clear"]
    #[inline(always)]
    pub fn set_dmic0ch0(&mut self, val: super::vals::Dmac0ReqEna0ClrDmic0ch0) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "DMIC0 channel 1 enable clear"]
    #[inline(always)]
    pub const fn dmic0ch1(&self) -> super::vals::Dmac0ReqEna0ClrDmic0ch1 {
        let val = (self.0 >> 17usize) & 0x01;
        super::vals::Dmac0ReqEna0ClrDmic0ch1::from_bits(val as u8)
    }
    #[doc = "DMIC0 channel 1 enable clear"]
    #[inline(always)]
    pub fn set_dmic0ch1(&mut self, val: super::vals::Dmac0ReqEna0ClrDmic0ch1) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
    #[doc = "DMIC0 channel 2 enable clear"]
    #[inline(always)]
    pub const fn dmic0ch2(&self) -> super::vals::Dmac0ReqEna0ClrDmic0ch2 {
        let val = (self.0 >> 18usize) & 0x01;
        super::vals::Dmac0ReqEna0ClrDmic0ch2::from_bits(val as u8)
    }
    #[doc = "DMIC0 channel 2 enable clear"]
    #[inline(always)]
    pub fn set_dmic0ch2(&mut self, val: super::vals::Dmac0ReqEna0ClrDmic0ch2) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
    }
    #[doc = "DMIC0 channel 3 enable clear"]
    #[inline(always)]
    pub const fn dmic0ch3(&self) -> super::vals::Dmac0ReqEna0ClrDmic0ch3 {
        let val = (self.0 >> 19usize) & 0x01;
        super::vals::Dmac0ReqEna0ClrDmic0ch3::from_bits(val as u8)
    }
    #[doc = "DMIC0 channel 3 enable clear"]
    #[inline(always)]
    pub fn set_dmic0ch3(&mut self, val: super::vals::Dmac0ReqEna0ClrDmic0ch3) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
    #[doc = "DMIC0 channel 4 enable clear"]
    #[inline(always)]
    pub const fn dmic0ch4(&self) -> super::vals::Dmac0ReqEna0ClrDmic0ch4 {
        let val = (self.0 >> 20usize) & 0x01;
        super::vals::Dmac0ReqEna0ClrDmic0ch4::from_bits(val as u8)
    }
    #[doc = "DMIC0 channel 4 enable clear"]
    #[inline(always)]
    pub fn set_dmic0ch4(&mut self, val: super::vals::Dmac0ReqEna0ClrDmic0ch4) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
    #[doc = "DMIC0 channel 5 enable clear"]
    #[inline(always)]
    pub const fn dmic0ch5(&self) -> super::vals::Dmac0ReqEna0ClrDmic0ch5 {
        let val = (self.0 >> 21usize) & 0x01;
        super::vals::Dmac0ReqEna0ClrDmic0ch5::from_bits(val as u8)
    }
    #[doc = "DMIC0 channel 5 enable clear"]
    #[inline(always)]
    pub fn set_dmic0ch5(&mut self, val: super::vals::Dmac0ReqEna0ClrDmic0ch5) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
    }
    #[doc = "DMIC0 channel 6 enable clear"]
    #[inline(always)]
    pub const fn dmic0ch6(&self) -> super::vals::Dmac0ReqEna0ClrDmic0ch6 {
        let val = (self.0 >> 22usize) & 0x01;
        super::vals::Dmac0ReqEna0ClrDmic0ch6::from_bits(val as u8)
    }
    #[doc = "DMIC0 channel 6 enable clear"]
    #[inline(always)]
    pub fn set_dmic0ch6(&mut self, val: super::vals::Dmac0ReqEna0ClrDmic0ch6) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val.to_bits() as u32) & 0x01) << 22usize);
    }
    #[doc = "DMIC0 channel 7 enable clear"]
    #[inline(always)]
    pub const fn dmic0ch7(&self) -> super::vals::Dmac0ReqEna0ClrDmic0ch7 {
        let val = (self.0 >> 23usize) & 0x01;
        super::vals::Dmac0ReqEna0ClrDmic0ch7::from_bits(val as u8)
    }
    #[doc = "DMIC0 channel 7 enable clear"]
    #[inline(always)]
    pub fn set_dmic0ch7(&mut self, val: super::vals::Dmac0ReqEna0ClrDmic0ch7) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "I3C RX enable clear"]
    #[inline(always)]
    pub const fn i3c0_rx(&self) -> super::vals::Dmac0ReqEna0ClrI3c0Rx {
        let val = (self.0 >> 24usize) & 0x01;
        super::vals::Dmac0ReqEna0ClrI3c0Rx::from_bits(val as u8)
    }
    #[doc = "I3C RX enable clear"]
    #[inline(always)]
    pub fn set_i3c0_rx(&mut self, val: super::vals::Dmac0ReqEna0ClrI3c0Rx) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
    #[doc = "I3C TX enable clear"]
    #[inline(always)]
    pub const fn i3c0_tx(&self) -> super::vals::Dmac0ReqEna0ClrI3c0Tx {
        let val = (self.0 >> 25usize) & 0x01;
        super::vals::Dmac0ReqEna0ClrI3c0Tx::from_bits(val as u8)
    }
    #[doc = "I3C TX enable clear"]
    #[inline(always)]
    pub fn set_i3c0_tx(&mut self, val: super::vals::Dmac0ReqEna0ClrI3c0Tx) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val.to_bits() as u32) & 0x01) << 25usize);
    }
    #[doc = "FLEXCOMM14 RX enable clear"]
    #[inline(always)]
    pub const fn flexcomm14_rx(&self) -> super::vals::Dmac0ReqEna0ClrFlexcomm14Rx {
        let val = (self.0 >> 26usize) & 0x01;
        super::vals::Dmac0ReqEna0ClrFlexcomm14Rx::from_bits(val as u8)
    }
    #[doc = "FLEXCOMM14 RX enable clear"]
    #[inline(always)]
    pub fn set_flexcomm14_rx(&mut self, val: super::vals::Dmac0ReqEna0ClrFlexcomm14Rx) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val.to_bits() as u32) & 0x01) << 26usize);
    }
    #[doc = "FLEXCOMM14 TX enable clear"]
    #[inline(always)]
    pub const fn flexcomm14_tx(&self) -> super::vals::Dmac0ReqEna0ClrFlexcomm14Tx {
        let val = (self.0 >> 27usize) & 0x01;
        super::vals::Dmac0ReqEna0ClrFlexcomm14Tx::from_bits(val as u8)
    }
    #[doc = "FLEXCOMM14 TX enable clear"]
    #[inline(always)]
    pub fn set_flexcomm14_tx(&mut self, val: super::vals::Dmac0ReqEna0ClrFlexcomm14Tx) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val.to_bits() as u32) & 0x01) << 27usize);
    }
    #[doc = "Hash enable clear"]
    #[inline(always)]
    pub const fn hashcrypt(&self) -> super::vals::Dmac0ReqEna0ClrHashcrypt {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::Dmac0ReqEna0ClrHashcrypt::from_bits(val as u8)
    }
    #[doc = "Hash enable clear"]
    #[inline(always)]
    pub fn set_hashcrypt(&mut self, val: super::vals::Dmac0ReqEna0ClrHashcrypt) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
}
impl Default for Dmac0ReqEna0Clr {
    #[inline(always)]
    fn default() -> Dmac0ReqEna0Clr {
        Dmac0ReqEna0Clr(0)
    }
}
#[doc = "DMAC0 request enable set 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dmac0ReqEna0Set(pub u32);
impl Dmac0ReqEna0Set {
    #[doc = "FLEXCOMM0 RX enable set"]
    #[inline(always)]
    pub const fn flexcomm0_rx(&self) -> super::vals::Dmac0ReqEna0SetFlexcomm0Rx {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Dmac0ReqEna0SetFlexcomm0Rx::from_bits(val as u8)
    }
    #[doc = "FLEXCOMM0 RX enable set"]
    #[inline(always)]
    pub fn set_flexcomm0_rx(&mut self, val: super::vals::Dmac0ReqEna0SetFlexcomm0Rx) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "FLEXCOMM0 TX enable set"]
    #[inline(always)]
    pub const fn flexcomm0_tx(&self) -> super::vals::Dmac0ReqEna0SetFlexcomm0Tx {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Dmac0ReqEna0SetFlexcomm0Tx::from_bits(val as u8)
    }
    #[doc = "FLEXCOMM0 TX enable set"]
    #[inline(always)]
    pub fn set_flexcomm0_tx(&mut self, val: super::vals::Dmac0ReqEna0SetFlexcomm0Tx) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "FLEXCOMM1 RX enable set"]
    #[inline(always)]
    pub const fn flexcomm1_rx(&self) -> super::vals::Dmac0ReqEna0SetFlexcomm1Rx {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Dmac0ReqEna0SetFlexcomm1Rx::from_bits(val as u8)
    }
    #[doc = "FLEXCOMM1 RX enable set"]
    #[inline(always)]
    pub fn set_flexcomm1_rx(&mut self, val: super::vals::Dmac0ReqEna0SetFlexcomm1Rx) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "FLEXCOMM1 TX enable set"]
    #[inline(always)]
    pub const fn flexcomm1_tx(&self) -> super::vals::Dmac0ReqEna0SetFlexcomm1Tx {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Dmac0ReqEna0SetFlexcomm1Tx::from_bits(val as u8)
    }
    #[doc = "FLEXCOMM1 TX enable set"]
    #[inline(always)]
    pub fn set_flexcomm1_tx(&mut self, val: super::vals::Dmac0ReqEna0SetFlexcomm1Tx) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "FLEXCOMM2 RX enable set"]
    #[inline(always)]
    pub const fn flexcomm2_rx(&self) -> super::vals::Dmac0ReqEna0SetFlexcomm2Rx {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Dmac0ReqEna0SetFlexcomm2Rx::from_bits(val as u8)
    }
    #[doc = "FLEXCOMM2 RX enable set"]
    #[inline(always)]
    pub fn set_flexcomm2_rx(&mut self, val: super::vals::Dmac0ReqEna0SetFlexcomm2Rx) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "FLEXCOMM2 TX enable set"]
    #[inline(always)]
    pub const fn flexcomm2_tx(&self) -> super::vals::Dmac0ReqEna0SetFlexcomm2Tx {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::Dmac0ReqEna0SetFlexcomm2Tx::from_bits(val as u8)
    }
    #[doc = "FLEXCOMM2 TX enable set"]
    #[inline(always)]
    pub fn set_flexcomm2_tx(&mut self, val: super::vals::Dmac0ReqEna0SetFlexcomm2Tx) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "FLEXCOMM3 RX enable set"]
    #[inline(always)]
    pub const fn flexcomm3_rx(&self) -> super::vals::Dmac0ReqEna0SetFlexcomm3Rx {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Dmac0ReqEna0SetFlexcomm3Rx::from_bits(val as u8)
    }
    #[doc = "FLEXCOMM3 RX enable set"]
    #[inline(always)]
    pub fn set_flexcomm3_rx(&mut self, val: super::vals::Dmac0ReqEna0SetFlexcomm3Rx) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "FLEXCOMM3 TX enable set"]
    #[inline(always)]
    pub const fn flexcomm3_tx(&self) -> super::vals::Dmac0ReqEna0SetFlexcomm3Tx {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Dmac0ReqEna0SetFlexcomm3Tx::from_bits(val as u8)
    }
    #[doc = "FLEXCOMM3 TX enable set"]
    #[inline(always)]
    pub fn set_flexcomm3_tx(&mut self, val: super::vals::Dmac0ReqEna0SetFlexcomm3Tx) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "FLEXCOMM4 RX enable set"]
    #[inline(always)]
    pub const fn flexcomm4_rx(&self) -> super::vals::Dmac0ReqEna0SetFlexcomm4Rx {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Dmac0ReqEna0SetFlexcomm4Rx::from_bits(val as u8)
    }
    #[doc = "FLEXCOMM4 RX enable set"]
    #[inline(always)]
    pub fn set_flexcomm4_rx(&mut self, val: super::vals::Dmac0ReqEna0SetFlexcomm4Rx) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "FLEXCOMM4 TX enable set"]
    #[inline(always)]
    pub const fn flexcomm4_tx(&self) -> super::vals::Dmac0ReqEna0SetFlexcomm4Tx {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::Dmac0ReqEna0SetFlexcomm4Tx::from_bits(val as u8)
    }
    #[doc = "FLEXCOMM4 TX enable set"]
    #[inline(always)]
    pub fn set_flexcomm4_tx(&mut self, val: super::vals::Dmac0ReqEna0SetFlexcomm4Tx) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "FLEXCOMM5 RX enable set"]
    #[inline(always)]
    pub const fn flexcomm5_rx(&self) -> super::vals::Dmac0ReqEna0SetFlexcomm5Rx {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::Dmac0ReqEna0SetFlexcomm5Rx::from_bits(val as u8)
    }
    #[doc = "FLEXCOMM5 RX enable set"]
    #[inline(always)]
    pub fn set_flexcomm5_rx(&mut self, val: super::vals::Dmac0ReqEna0SetFlexcomm5Rx) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "FLEXCOMM5 TX enable set"]
    #[inline(always)]
    pub const fn flexcomm5_tx(&self) -> super::vals::Dmac0ReqEna0SetFlexcomm5Tx {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::Dmac0ReqEna0SetFlexcomm5Tx::from_bits(val as u8)
    }
    #[doc = "FLEXCOMM5 TX enable set"]
    #[inline(always)]
    pub fn set_flexcomm5_tx(&mut self, val: super::vals::Dmac0ReqEna0SetFlexcomm5Tx) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "FLEXCOMM6 RX enable set"]
    #[inline(always)]
    pub const fn flexcomm6_rx(&self) -> super::vals::Dmac0ReqEna0SetFlexcomm6Rx {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::Dmac0ReqEna0SetFlexcomm6Rx::from_bits(val as u8)
    }
    #[doc = "FLEXCOMM6 RX enable set"]
    #[inline(always)]
    pub fn set_flexcomm6_rx(&mut self, val: super::vals::Dmac0ReqEna0SetFlexcomm6Rx) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "FLEXCOMM6 TX enable set"]
    #[inline(always)]
    pub const fn flexcomm6_tx(&self) -> super::vals::Dmac0ReqEna0SetFlexcomm6Tx {
        let val = (self.0 >> 13usize) & 0x01;
        super::vals::Dmac0ReqEna0SetFlexcomm6Tx::from_bits(val as u8)
    }
    #[doc = "FLEXCOMM6 TX enable set"]
    #[inline(always)]
    pub fn set_flexcomm6_tx(&mut self, val: super::vals::Dmac0ReqEna0SetFlexcomm6Tx) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "FLEXCOMM7 RX enable set"]
    #[inline(always)]
    pub const fn flexcomm7_rx(&self) -> super::vals::Dmac0ReqEna0SetFlexcomm7Rx {
        let val = (self.0 >> 14usize) & 0x01;
        super::vals::Dmac0ReqEna0SetFlexcomm7Rx::from_bits(val as u8)
    }
    #[doc = "FLEXCOMM7 RX enable set"]
    #[inline(always)]
    pub fn set_flexcomm7_rx(&mut self, val: super::vals::Dmac0ReqEna0SetFlexcomm7Rx) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
    }
    #[doc = "FLEXCOMM7 TX enable set"]
    #[inline(always)]
    pub const fn flexcomm7_tx(&self) -> super::vals::Dmac0ReqEna0SetFlexcomm7Tx {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::Dmac0ReqEna0SetFlexcomm7Tx::from_bits(val as u8)
    }
    #[doc = "FLEXCOMM7 TX enable set"]
    #[inline(always)]
    pub fn set_flexcomm7_tx(&mut self, val: super::vals::Dmac0ReqEna0SetFlexcomm7Tx) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
    #[doc = "DMIC0 channel 0 enable set"]
    #[inline(always)]
    pub const fn dmic0ch0(&self) -> super::vals::Dmac0ReqEna0SetDmic0ch0 {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::Dmac0ReqEna0SetDmic0ch0::from_bits(val as u8)
    }
    #[doc = "DMIC0 channel 0 enable set"]
    #[inline(always)]
    pub fn set_dmic0ch0(&mut self, val: super::vals::Dmac0ReqEna0SetDmic0ch0) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "DMIC0 channel 1 enable set"]
    #[inline(always)]
    pub const fn dmic0ch1(&self) -> super::vals::Dmac0ReqEna0SetDmic0ch1 {
        let val = (self.0 >> 17usize) & 0x01;
        super::vals::Dmac0ReqEna0SetDmic0ch1::from_bits(val as u8)
    }
    #[doc = "DMIC0 channel 1 enable set"]
    #[inline(always)]
    pub fn set_dmic0ch1(&mut self, val: super::vals::Dmac0ReqEna0SetDmic0ch1) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
    #[doc = "DMIC0 channel 2 enable set"]
    #[inline(always)]
    pub const fn dmic0ch2(&self) -> super::vals::Dmac0ReqEna0SetDmic0ch2 {
        let val = (self.0 >> 18usize) & 0x01;
        super::vals::Dmac0ReqEna0SetDmic0ch2::from_bits(val as u8)
    }
    #[doc = "DMIC0 channel 2 enable set"]
    #[inline(always)]
    pub fn set_dmic0ch2(&mut self, val: super::vals::Dmac0ReqEna0SetDmic0ch2) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
    }
    #[doc = "DMIC0 channel 3 enable set"]
    #[inline(always)]
    pub const fn dmic0ch3(&self) -> super::vals::Dmac0ReqEna0SetDmic0ch3 {
        let val = (self.0 >> 19usize) & 0x01;
        super::vals::Dmac0ReqEna0SetDmic0ch3::from_bits(val as u8)
    }
    #[doc = "DMIC0 channel 3 enable set"]
    #[inline(always)]
    pub fn set_dmic0ch3(&mut self, val: super::vals::Dmac0ReqEna0SetDmic0ch3) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
    #[doc = "DMIC0 channel 4 enable set"]
    #[inline(always)]
    pub const fn dmic0ch4(&self) -> super::vals::Dmac0ReqEna0SetDmic0ch4 {
        let val = (self.0 >> 20usize) & 0x01;
        super::vals::Dmac0ReqEna0SetDmic0ch4::from_bits(val as u8)
    }
    #[doc = "DMIC0 channel 4 enable set"]
    #[inline(always)]
    pub fn set_dmic0ch4(&mut self, val: super::vals::Dmac0ReqEna0SetDmic0ch4) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
    #[doc = "DMIC0 channel 5 enable set"]
    #[inline(always)]
    pub const fn dmic0ch5(&self) -> super::vals::Dmac0ReqEna0SetDmic0ch5 {
        let val = (self.0 >> 21usize) & 0x01;
        super::vals::Dmac0ReqEna0SetDmic0ch5::from_bits(val as u8)
    }
    #[doc = "DMIC0 channel 5 enable set"]
    #[inline(always)]
    pub fn set_dmic0ch5(&mut self, val: super::vals::Dmac0ReqEna0SetDmic0ch5) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
    }
    #[doc = "DMIC0 channel 6 enable set"]
    #[inline(always)]
    pub const fn dmic0ch6(&self) -> super::vals::Dmac0ReqEna0SetDmic0ch6 {
        let val = (self.0 >> 22usize) & 0x01;
        super::vals::Dmac0ReqEna0SetDmic0ch6::from_bits(val as u8)
    }
    #[doc = "DMIC0 channel 6 enable set"]
    #[inline(always)]
    pub fn set_dmic0ch6(&mut self, val: super::vals::Dmac0ReqEna0SetDmic0ch6) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val.to_bits() as u32) & 0x01) << 22usize);
    }
    #[doc = "DMIC0 channel 7 enable set"]
    #[inline(always)]
    pub const fn dmic0ch7(&self) -> super::vals::Dmac0ReqEna0SetDmic0ch7 {
        let val = (self.0 >> 23usize) & 0x01;
        super::vals::Dmac0ReqEna0SetDmic0ch7::from_bits(val as u8)
    }
    #[doc = "DMIC0 channel 7 enable set"]
    #[inline(always)]
    pub fn set_dmic0ch7(&mut self, val: super::vals::Dmac0ReqEna0SetDmic0ch7) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "I3C RX enable set"]
    #[inline(always)]
    pub const fn i3c0_rx(&self) -> super::vals::Dmac0ReqEna0SetI3c0Rx {
        let val = (self.0 >> 24usize) & 0x01;
        super::vals::Dmac0ReqEna0SetI3c0Rx::from_bits(val as u8)
    }
    #[doc = "I3C RX enable set"]
    #[inline(always)]
    pub fn set_i3c0_rx(&mut self, val: super::vals::Dmac0ReqEna0SetI3c0Rx) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
    #[doc = "I3C TX enable set"]
    #[inline(always)]
    pub const fn i3c0_tx(&self) -> super::vals::Dmac0ReqEna0SetI3c0Tx {
        let val = (self.0 >> 25usize) & 0x01;
        super::vals::Dmac0ReqEna0SetI3c0Tx::from_bits(val as u8)
    }
    #[doc = "I3C TX enable set"]
    #[inline(always)]
    pub fn set_i3c0_tx(&mut self, val: super::vals::Dmac0ReqEna0SetI3c0Tx) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val.to_bits() as u32) & 0x01) << 25usize);
    }
    #[doc = "FLEXCOMM14 TX enable set"]
    #[inline(always)]
    pub const fn flexcomm14_rx(&self) -> super::vals::Dmac0ReqEna0SetFlexcomm14Rx {
        let val = (self.0 >> 26usize) & 0x01;
        super::vals::Dmac0ReqEna0SetFlexcomm14Rx::from_bits(val as u8)
    }
    #[doc = "FLEXCOMM14 TX enable set"]
    #[inline(always)]
    pub fn set_flexcomm14_rx(&mut self, val: super::vals::Dmac0ReqEna0SetFlexcomm14Rx) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val.to_bits() as u32) & 0x01) << 26usize);
    }
    #[doc = "FLEXCOMM15 RX enable set"]
    #[inline(always)]
    pub const fn flexcomm14_tx(&self) -> super::vals::Dmac0ReqEna0SetFlexcomm14Tx {
        let val = (self.0 >> 27usize) & 0x01;
        super::vals::Dmac0ReqEna0SetFlexcomm14Tx::from_bits(val as u8)
    }
    #[doc = "FLEXCOMM15 RX enable set"]
    #[inline(always)]
    pub fn set_flexcomm14_tx(&mut self, val: super::vals::Dmac0ReqEna0SetFlexcomm14Tx) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val.to_bits() as u32) & 0x01) << 27usize);
    }
    #[doc = "Hash enable set"]
    #[inline(always)]
    pub const fn hashcrypt(&self) -> super::vals::Dmac0ReqEna0SetHashcrypt {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::Dmac0ReqEna0SetHashcrypt::from_bits(val as u8)
    }
    #[doc = "Hash enable set"]
    #[inline(always)]
    pub fn set_hashcrypt(&mut self, val: super::vals::Dmac0ReqEna0SetHashcrypt) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
}
impl Default for Dmac0ReqEna0Set {
    #[inline(always)]
    fn default() -> Dmac0ReqEna0Set {
        Dmac0ReqEna0Set(0)
    }
}
#[doc = "DMAC1 input trigger enable 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dmac1ItrigEna0(pub u32);
impl Dmac1ItrigEna0 {
    #[doc = "DMAC1 input trigger inmux 0 enable"]
    #[inline(always)]
    pub const fn dmac1_itrig_inmux0(&self) -> super::vals::Dmac1ItrigEna0Dmac1ItrigInmux0 {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Dmac1ItrigEna0Dmac1ItrigInmux0::from_bits(val as u8)
    }
    #[doc = "DMAC1 input trigger inmux 0 enable"]
    #[inline(always)]
    pub fn set_dmac1_itrig_inmux0(&mut self, val: super::vals::Dmac1ItrigEna0Dmac1ItrigInmux0) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "DMAC1 input trigger inmux 1 enable"]
    #[inline(always)]
    pub const fn dmac1_itrig_inmux1(&self) -> super::vals::Dmac1ItrigEna0Dmac1ItrigInmux1 {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Dmac1ItrigEna0Dmac1ItrigInmux1::from_bits(val as u8)
    }
    #[doc = "DMAC1 input trigger inmux 1 enable"]
    #[inline(always)]
    pub fn set_dmac1_itrig_inmux1(&mut self, val: super::vals::Dmac1ItrigEna0Dmac1ItrigInmux1) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "DMAC1 input trigger inmux 2 enable"]
    #[inline(always)]
    pub const fn dmac1_itrig_inmux2(&self) -> super::vals::Dmac1ItrigInmux2 {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Dmac1ItrigInmux2::from_bits(val as u8)
    }
    #[doc = "DMAC1 input trigger inmux 2 enable"]
    #[inline(always)]
    pub fn set_dmac1_itrig_inmux2(&mut self, val: super::vals::Dmac1ItrigInmux2) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "DMAC1 input trigger inmux 3 enable"]
    #[inline(always)]
    pub const fn dmac1_itrig_inmux3(&self) -> super::vals::Dmac1ItrigEna0Dmac1ItrigInmux3 {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Dmac1ItrigEna0Dmac1ItrigInmux3::from_bits(val as u8)
    }
    #[doc = "DMAC1 input trigger inmux 3 enable"]
    #[inline(always)]
    pub fn set_dmac1_itrig_inmux3(&mut self, val: super::vals::Dmac1ItrigEna0Dmac1ItrigInmux3) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "DMAC1 input trigger inmux 4 enable"]
    #[inline(always)]
    pub const fn dmac1_itrig_inmux4(&self) -> super::vals::Dmac1ItrigEna0Dmac1ItrigInmux4 {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Dmac1ItrigEna0Dmac1ItrigInmux4::from_bits(val as u8)
    }
    #[doc = "DMAC1 input trigger inmux 4 enable"]
    #[inline(always)]
    pub fn set_dmac1_itrig_inmux4(&mut self, val: super::vals::Dmac1ItrigEna0Dmac1ItrigInmux4) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "DMAC1 input trigger inmux 5 enable"]
    #[inline(always)]
    pub const fn dmac1_itrig_inmux5(&self) -> super::vals::Dmac1ItrigEna0Dmac1ItrigInmux5 {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::Dmac1ItrigEna0Dmac1ItrigInmux5::from_bits(val as u8)
    }
    #[doc = "DMAC1 input trigger inmux 5 enable"]
    #[inline(always)]
    pub fn set_dmac1_itrig_inmux5(&mut self, val: super::vals::Dmac1ItrigEna0Dmac1ItrigInmux5) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "DMAC1 input trigger inmux 6 enable"]
    #[inline(always)]
    pub const fn dmac1_itrig_inmux6(&self) -> super::vals::Dmac1ItrigEna0Dmac1ItrigInmux6 {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Dmac1ItrigEna0Dmac1ItrigInmux6::from_bits(val as u8)
    }
    #[doc = "DMAC1 input trigger inmux 6 enable"]
    #[inline(always)]
    pub fn set_dmac1_itrig_inmux6(&mut self, val: super::vals::Dmac1ItrigEna0Dmac1ItrigInmux6) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "DMAC1 input trigger inmux 7 enable"]
    #[inline(always)]
    pub const fn dmac1_itrig_inmux7(&self) -> super::vals::Dmac1ItrigEna0Dmac1ItrigInmux7 {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Dmac1ItrigEna0Dmac1ItrigInmux7::from_bits(val as u8)
    }
    #[doc = "DMAC1 input trigger inmux 7 enable"]
    #[inline(always)]
    pub fn set_dmac1_itrig_inmux7(&mut self, val: super::vals::Dmac1ItrigEna0Dmac1ItrigInmux7) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "DMAC1 input trigger inmux 8 enable"]
    #[inline(always)]
    pub const fn dmac1_itrig_inmux8(&self) -> super::vals::Dmac1ItrigEna0Dmac1ItrigInmux8 {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Dmac1ItrigEna0Dmac1ItrigInmux8::from_bits(val as u8)
    }
    #[doc = "DMAC1 input trigger inmux 8 enable"]
    #[inline(always)]
    pub fn set_dmac1_itrig_inmux8(&mut self, val: super::vals::Dmac1ItrigEna0Dmac1ItrigInmux8) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "DMAC1 input trigger inmux 9 enable"]
    #[inline(always)]
    pub const fn dmac1_itrig_inmux9(&self) -> super::vals::Dmac1ItrigEna0Dmac1ItrigInmux9 {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::Dmac1ItrigEna0Dmac1ItrigInmux9::from_bits(val as u8)
    }
    #[doc = "DMAC1 input trigger inmux 9 enable"]
    #[inline(always)]
    pub fn set_dmac1_itrig_inmux9(&mut self, val: super::vals::Dmac1ItrigEna0Dmac1ItrigInmux9) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "DMAC1 input trigger inmux 10 enable"]
    #[inline(always)]
    pub const fn dmac1_itrig_inmux10(&self) -> super::vals::Dmac1ItrigEna0Dmac1ItrigInmux10 {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::Dmac1ItrigEna0Dmac1ItrigInmux10::from_bits(val as u8)
    }
    #[doc = "DMAC1 input trigger inmux 10 enable"]
    #[inline(always)]
    pub fn set_dmac1_itrig_inmux10(&mut self, val: super::vals::Dmac1ItrigEna0Dmac1ItrigInmux10) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "DMAC1 input trigger inmux 11 enable"]
    #[inline(always)]
    pub const fn dmac1_itrig_inmux11(&self) -> super::vals::Dmac1ItrigEna0Dmac1ItrigInmux11 {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::Dmac1ItrigEna0Dmac1ItrigInmux11::from_bits(val as u8)
    }
    #[doc = "DMAC1 input trigger inmux 11 enable"]
    #[inline(always)]
    pub fn set_dmac1_itrig_inmux11(&mut self, val: super::vals::Dmac1ItrigEna0Dmac1ItrigInmux11) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "DMAC1 input trigger inmux 12 enable"]
    #[inline(always)]
    pub const fn dmac1_itrig_inmux12(&self) -> super::vals::Dmac1ItrigEna0Dmac1ItrigInmux12 {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::Dmac1ItrigEna0Dmac1ItrigInmux12::from_bits(val as u8)
    }
    #[doc = "DMAC1 input trigger inmux 12 enable"]
    #[inline(always)]
    pub fn set_dmac1_itrig_inmux12(&mut self, val: super::vals::Dmac1ItrigEna0Dmac1ItrigInmux12) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "DMAC1 input trigger inmux 13 enable"]
    #[inline(always)]
    pub const fn dmac1_itrig_inmux13(&self) -> super::vals::Dmac1ItrigEna0Dmac1ItrigInmux13 {
        let val = (self.0 >> 13usize) & 0x01;
        super::vals::Dmac1ItrigEna0Dmac1ItrigInmux13::from_bits(val as u8)
    }
    #[doc = "DMAC1 input trigger inmux 13 enable"]
    #[inline(always)]
    pub fn set_dmac1_itrig_inmux13(&mut self, val: super::vals::Dmac1ItrigEna0Dmac1ItrigInmux13) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "DMAC1 input trigger inmux 14 enable"]
    #[inline(always)]
    pub const fn dmac1_itrig_inmux14(&self) -> super::vals::Dmac1ItrigEna0Dmac1ItrigInmux14 {
        let val = (self.0 >> 14usize) & 0x01;
        super::vals::Dmac1ItrigEna0Dmac1ItrigInmux14::from_bits(val as u8)
    }
    #[doc = "DMAC1 input trigger inmux 14 enable"]
    #[inline(always)]
    pub fn set_dmac1_itrig_inmux14(&mut self, val: super::vals::Dmac1ItrigEna0Dmac1ItrigInmux14) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
    }
    #[doc = "DMAC1 input trigger inmux 15 enable"]
    #[inline(always)]
    pub const fn dmac1_itrig_inmux15(&self) -> super::vals::Dmac1ItrigEna0Dmac1ItrigInmux15 {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::Dmac1ItrigEna0Dmac1ItrigInmux15::from_bits(val as u8)
    }
    #[doc = "DMAC1 input trigger inmux 15 enable"]
    #[inline(always)]
    pub fn set_dmac1_itrig_inmux15(&mut self, val: super::vals::Dmac1ItrigEna0Dmac1ItrigInmux15) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
    #[doc = "DMAC1 input trigger inmux 16 enable"]
    #[inline(always)]
    pub const fn dmac1_itrig_inmux16(&self) -> super::vals::Dmac1ItrigEna0Dmac1ItrigInmux16 {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::Dmac1ItrigEna0Dmac1ItrigInmux16::from_bits(val as u8)
    }
    #[doc = "DMAC1 input trigger inmux 16 enable"]
    #[inline(always)]
    pub fn set_dmac1_itrig_inmux16(&mut self, val: super::vals::Dmac1ItrigEna0Dmac1ItrigInmux16) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "DMAC1 input trigger inmux 17 enable"]
    #[inline(always)]
    pub const fn dmac1_itrig_inmux17(&self) -> super::vals::Dmac1ItrigEna0Dmac1ItrigInmux17 {
        let val = (self.0 >> 17usize) & 0x01;
        super::vals::Dmac1ItrigEna0Dmac1ItrigInmux17::from_bits(val as u8)
    }
    #[doc = "DMAC1 input trigger inmux 17 enable"]
    #[inline(always)]
    pub fn set_dmac1_itrig_inmux17(&mut self, val: super::vals::Dmac1ItrigEna0Dmac1ItrigInmux17) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
    #[doc = "DMAC1 input trigger inmux 18 enable"]
    #[inline(always)]
    pub const fn dmac1_itrig_inmux18(&self) -> super::vals::Dmac1ItrigEna0Dmac1ItrigInmux18 {
        let val = (self.0 >> 18usize) & 0x01;
        super::vals::Dmac1ItrigEna0Dmac1ItrigInmux18::from_bits(val as u8)
    }
    #[doc = "DMAC1 input trigger inmux 18 enable"]
    #[inline(always)]
    pub fn set_dmac1_itrig_inmux18(&mut self, val: super::vals::Dmac1ItrigEna0Dmac1ItrigInmux18) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
    }
    #[doc = "DMAC1 input trigger inmux 19 enable"]
    #[inline(always)]
    pub const fn dmac1_itrig_inmux19(&self) -> super::vals::Dmac1ItrigEna0Dmac1ItrigInmux19 {
        let val = (self.0 >> 19usize) & 0x01;
        super::vals::Dmac1ItrigEna0Dmac1ItrigInmux19::from_bits(val as u8)
    }
    #[doc = "DMAC1 input trigger inmux 19 enable"]
    #[inline(always)]
    pub fn set_dmac1_itrig_inmux19(&mut self, val: super::vals::Dmac1ItrigEna0Dmac1ItrigInmux19) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
    #[doc = "DMAC1 input trigger inmux 20 enable"]
    #[inline(always)]
    pub const fn dmac1_itrig_inmux20(&self) -> super::vals::Dmac1ItrigEna0Dmac1ItrigInmux20 {
        let val = (self.0 >> 20usize) & 0x01;
        super::vals::Dmac1ItrigEna0Dmac1ItrigInmux20::from_bits(val as u8)
    }
    #[doc = "DMAC1 input trigger inmux 20 enable"]
    #[inline(always)]
    pub fn set_dmac1_itrig_inmux20(&mut self, val: super::vals::Dmac1ItrigEna0Dmac1ItrigInmux20) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
    #[doc = "DMAC1 input trigger inmux 21 enable"]
    #[inline(always)]
    pub const fn dmac1_itrig_inmux21(&self) -> super::vals::Dmac1ItrigEna0Dmac1ItrigInmux21 {
        let val = (self.0 >> 21usize) & 0x01;
        super::vals::Dmac1ItrigEna0Dmac1ItrigInmux21::from_bits(val as u8)
    }
    #[doc = "DMAC1 input trigger inmux 21 enable"]
    #[inline(always)]
    pub fn set_dmac1_itrig_inmux21(&mut self, val: super::vals::Dmac1ItrigEna0Dmac1ItrigInmux21) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
    }
    #[doc = "DMAC1 input trigger inmux 22 enable"]
    #[inline(always)]
    pub const fn dmac1_itrig_inmux22(&self) -> super::vals::Dmac1ItrigEna0Dmac1ItrigInmux22 {
        let val = (self.0 >> 22usize) & 0x01;
        super::vals::Dmac1ItrigEna0Dmac1ItrigInmux22::from_bits(val as u8)
    }
    #[doc = "DMAC1 input trigger inmux 22 enable"]
    #[inline(always)]
    pub fn set_dmac1_itrig_inmux22(&mut self, val: super::vals::Dmac1ItrigEna0Dmac1ItrigInmux22) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val.to_bits() as u32) & 0x01) << 22usize);
    }
    #[doc = "DMAC1 input trigger inmux 23 enable"]
    #[inline(always)]
    pub const fn dmac1_itrig_inmux23(&self) -> super::vals::Dmac1ItrigEna0Dmac1ItrigInmux23 {
        let val = (self.0 >> 23usize) & 0x01;
        super::vals::Dmac1ItrigEna0Dmac1ItrigInmux23::from_bits(val as u8)
    }
    #[doc = "DMAC1 input trigger inmux 23 enable"]
    #[inline(always)]
    pub fn set_dmac1_itrig_inmux23(&mut self, val: super::vals::Dmac1ItrigEna0Dmac1ItrigInmux23) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "DMAC1 input trigger inmux 24 enable"]
    #[inline(always)]
    pub const fn dmac1_itrig_inmux24(&self) -> super::vals::Dmac1ItrigEna0Dmac1ItrigInmux24 {
        let val = (self.0 >> 24usize) & 0x01;
        super::vals::Dmac1ItrigEna0Dmac1ItrigInmux24::from_bits(val as u8)
    }
    #[doc = "DMAC1 input trigger inmux 24 enable"]
    #[inline(always)]
    pub fn set_dmac1_itrig_inmux24(&mut self, val: super::vals::Dmac1ItrigEna0Dmac1ItrigInmux24) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
    #[doc = "DMAC1 input trigger inmux 25 enable"]
    #[inline(always)]
    pub const fn dmac1_itrig_inmux25(&self) -> super::vals::Dmac1ItrigEna0Dmac1ItrigInmux25 {
        let val = (self.0 >> 25usize) & 0x01;
        super::vals::Dmac1ItrigEna0Dmac1ItrigInmux25::from_bits(val as u8)
    }
    #[doc = "DMAC1 input trigger inmux 25 enable"]
    #[inline(always)]
    pub fn set_dmac1_itrig_inmux25(&mut self, val: super::vals::Dmac1ItrigEna0Dmac1ItrigInmux25) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val.to_bits() as u32) & 0x01) << 25usize);
    }
    #[doc = "DMAC1 input trigger inmux 25 enable"]
    #[inline(always)]
    pub const fn dmac1_itrig_inmux26(&self) -> super::vals::Dmac1ItrigEna0Dmac1ItrigInmux26 {
        let val = (self.0 >> 26usize) & 0x01;
        super::vals::Dmac1ItrigEna0Dmac1ItrigInmux26::from_bits(val as u8)
    }
    #[doc = "DMAC1 input trigger inmux 25 enable"]
    #[inline(always)]
    pub fn set_dmac1_itrig_inmux26(&mut self, val: super::vals::Dmac1ItrigEna0Dmac1ItrigInmux26) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val.to_bits() as u32) & 0x01) << 26usize);
    }
    #[doc = "DMAC1 input trigger inmux 25 enable"]
    #[inline(always)]
    pub const fn dmac1_itrig_inmux27(&self) -> super::vals::Dmac1ItrigEna0Dmac1ItrigInmux27 {
        let val = (self.0 >> 27usize) & 0x01;
        super::vals::Dmac1ItrigEna0Dmac1ItrigInmux27::from_bits(val as u8)
    }
    #[doc = "DMAC1 input trigger inmux 25 enable"]
    #[inline(always)]
    pub fn set_dmac1_itrig_inmux27(&mut self, val: super::vals::Dmac1ItrigEna0Dmac1ItrigInmux27) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val.to_bits() as u32) & 0x01) << 27usize);
    }
    #[doc = "DMAC1 input trigger inmux 25 enable"]
    #[inline(always)]
    pub const fn dmac1_itrig_inmux28(&self) -> super::vals::Dmac1ItrigEna0Dmac1ItrigInmux28 {
        let val = (self.0 >> 28usize) & 0x01;
        super::vals::Dmac1ItrigEna0Dmac1ItrigInmux28::from_bits(val as u8)
    }
    #[doc = "DMAC1 input trigger inmux 25 enable"]
    #[inline(always)]
    pub fn set_dmac1_itrig_inmux28(&mut self, val: super::vals::Dmac1ItrigEna0Dmac1ItrigInmux28) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val.to_bits() as u32) & 0x01) << 28usize);
    }
    #[doc = "DMAC1 input trigger inmux 25 enable"]
    #[inline(always)]
    pub const fn dmac1_itrig_inmux29(&self) -> super::vals::Dmac1ItrigEna0Dmac1ItrigInmux29 {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::Dmac1ItrigEna0Dmac1ItrigInmux29::from_bits(val as u8)
    }
    #[doc = "DMAC1 input trigger inmux 25 enable"]
    #[inline(always)]
    pub fn set_dmac1_itrig_inmux29(&mut self, val: super::vals::Dmac1ItrigEna0Dmac1ItrigInmux29) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "DMAC1 input trigger inmux 25 enable"]
    #[inline(always)]
    pub const fn dmac1_itrig_inmux30(&self) -> super::vals::Dmac1ItrigEna0Dmac1ItrigInmux30 {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::Dmac1ItrigEna0Dmac1ItrigInmux30::from_bits(val as u8)
    }
    #[doc = "DMAC1 input trigger inmux 25 enable"]
    #[inline(always)]
    pub fn set_dmac1_itrig_inmux30(&mut self, val: super::vals::Dmac1ItrigEna0Dmac1ItrigInmux30) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "DMAC1 input trigger inmux 25 enable"]
    #[inline(always)]
    pub const fn dmac1_itrig_inmux31(&self) -> super::vals::Dmac1ItrigEna0Dmac1ItrigInmux31 {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::Dmac1ItrigEna0Dmac1ItrigInmux31::from_bits(val as u8)
    }
    #[doc = "DMAC1 input trigger inmux 25 enable"]
    #[inline(always)]
    pub fn set_dmac1_itrig_inmux31(&mut self, val: super::vals::Dmac1ItrigEna0Dmac1ItrigInmux31) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Dmac1ItrigEna0 {
    #[inline(always)]
    fn default() -> Dmac1ItrigEna0 {
        Dmac1ItrigEna0(0)
    }
}
#[doc = "DMAC1 input trigger enable clear 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dmac1ItrigEna0Clr(pub u32);
impl Dmac1ItrigEna0Clr {
    #[doc = "DMAC1 input trigger inmux 0 enable clear"]
    #[inline(always)]
    pub const fn dmac1_itrig_inmux0(&self) -> super::vals::Dmac1ItrigEna0ClrDmac1ItrigInmux0 {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Dmac1ItrigEna0ClrDmac1ItrigInmux0::from_bits(val as u8)
    }
    #[doc = "DMAC1 input trigger inmux 0 enable clear"]
    #[inline(always)]
    pub fn set_dmac1_itrig_inmux0(&mut self, val: super::vals::Dmac1ItrigEna0ClrDmac1ItrigInmux0) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "DMAC1 input trigger inmux 1 enable clear"]
    #[inline(always)]
    pub const fn dmac1_itrig_inmux1(&self) -> super::vals::Dmac1ItrigEna0ClrDmac1ItrigInmux1 {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Dmac1ItrigEna0ClrDmac1ItrigInmux1::from_bits(val as u8)
    }
    #[doc = "DMAC1 input trigger inmux 1 enable clear"]
    #[inline(always)]
    pub fn set_dmac1_itrig_inmux1(&mut self, val: super::vals::Dmac1ItrigEna0ClrDmac1ItrigInmux1) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "DMAC1 input trigger inmux 2 enable clear"]
    #[inline(always)]
    pub const fn dmac1_itrig_inmux2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "DMAC1 input trigger inmux 2 enable clear"]
    #[inline(always)]
    pub fn set_dmac1_itrig_inmux2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "DMAC1 input trigger inmux 3 enable clear"]
    #[inline(always)]
    pub const fn dmac1_itrig_inmux3(&self) -> super::vals::Dmac1ItrigEna0ClrDmac1ItrigInmux3 {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Dmac1ItrigEna0ClrDmac1ItrigInmux3::from_bits(val as u8)
    }
    #[doc = "DMAC1 input trigger inmux 3 enable clear"]
    #[inline(always)]
    pub fn set_dmac1_itrig_inmux3(&mut self, val: super::vals::Dmac1ItrigEna0ClrDmac1ItrigInmux3) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "DMAC1 input trigger inmux 4 enable clear"]
    #[inline(always)]
    pub const fn dmac1_itrig_inmux4(&self) -> super::vals::Dmac1ItrigEna0ClrDmac1ItrigInmux4 {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Dmac1ItrigEna0ClrDmac1ItrigInmux4::from_bits(val as u8)
    }
    #[doc = "DMAC1 input trigger inmux 4 enable clear"]
    #[inline(always)]
    pub fn set_dmac1_itrig_inmux4(&mut self, val: super::vals::Dmac1ItrigEna0ClrDmac1ItrigInmux4) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "DMAC1 input trigger inmux 5 enable clear"]
    #[inline(always)]
    pub const fn dmac1_itrig_inmux5(&self) -> super::vals::Dmac1ItrigEna0ClrDmac1ItrigInmux5 {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::Dmac1ItrigEna0ClrDmac1ItrigInmux5::from_bits(val as u8)
    }
    #[doc = "DMAC1 input trigger inmux 5 enable clear"]
    #[inline(always)]
    pub fn set_dmac1_itrig_inmux5(&mut self, val: super::vals::Dmac1ItrigEna0ClrDmac1ItrigInmux5) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "DMAC1 input trigger inmux 6 enable clear"]
    #[inline(always)]
    pub const fn dmac1_itrig_inmux6(&self) -> super::vals::Dmac1ItrigEna0ClrDmac1ItrigInmux6 {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Dmac1ItrigEna0ClrDmac1ItrigInmux6::from_bits(val as u8)
    }
    #[doc = "DMAC1 input trigger inmux 6 enable clear"]
    #[inline(always)]
    pub fn set_dmac1_itrig_inmux6(&mut self, val: super::vals::Dmac1ItrigEna0ClrDmac1ItrigInmux6) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "DMAC1 input trigger inmux 7 enable clear"]
    #[inline(always)]
    pub const fn dmac1_itrig_inmux7(&self) -> super::vals::Dmac1ItrigEna0ClrDmac1ItrigInmux7 {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Dmac1ItrigEna0ClrDmac1ItrigInmux7::from_bits(val as u8)
    }
    #[doc = "DMAC1 input trigger inmux 7 enable clear"]
    #[inline(always)]
    pub fn set_dmac1_itrig_inmux7(&mut self, val: super::vals::Dmac1ItrigEna0ClrDmac1ItrigInmux7) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "DMAC1 input trigger inmux 8 enable clear"]
    #[inline(always)]
    pub const fn dmac1_itrig_inmux8(&self) -> super::vals::Dmac1ItrigEna0ClrDmac1ItrigInmux8 {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Dmac1ItrigEna0ClrDmac1ItrigInmux8::from_bits(val as u8)
    }
    #[doc = "DMAC1 input trigger inmux 8 enable clear"]
    #[inline(always)]
    pub fn set_dmac1_itrig_inmux8(&mut self, val: super::vals::Dmac1ItrigEna0ClrDmac1ItrigInmux8) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "DMAC1 input trigger inmux 9 enable clear"]
    #[inline(always)]
    pub const fn dmac1_itrig_inmux9(&self) -> super::vals::Dmac1ItrigEna0ClrDmac1ItrigInmux9 {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::Dmac1ItrigEna0ClrDmac1ItrigInmux9::from_bits(val as u8)
    }
    #[doc = "DMAC1 input trigger inmux 9 enable clear"]
    #[inline(always)]
    pub fn set_dmac1_itrig_inmux9(&mut self, val: super::vals::Dmac1ItrigEna0ClrDmac1ItrigInmux9) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "DMAC1 input trigger inmux 10 enable clear"]
    #[inline(always)]
    pub const fn dmac1_itrig_inmux10(&self) -> super::vals::Dmac1ItrigEna0ClrDmac1ItrigInmux10 {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::Dmac1ItrigEna0ClrDmac1ItrigInmux10::from_bits(val as u8)
    }
    #[doc = "DMAC1 input trigger inmux 10 enable clear"]
    #[inline(always)]
    pub fn set_dmac1_itrig_inmux10(
        &mut self,
        val: super::vals::Dmac1ItrigEna0ClrDmac1ItrigInmux10,
    ) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "DMAC1 input trigger inmux 11 enable clear"]
    #[inline(always)]
    pub const fn dmac1_itrig_inmux11(&self) -> super::vals::Dmac1ItrigEna0ClrDmac1ItrigInmux11 {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::Dmac1ItrigEna0ClrDmac1ItrigInmux11::from_bits(val as u8)
    }
    #[doc = "DMAC1 input trigger inmux 11 enable clear"]
    #[inline(always)]
    pub fn set_dmac1_itrig_inmux11(
        &mut self,
        val: super::vals::Dmac1ItrigEna0ClrDmac1ItrigInmux11,
    ) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "DMAC1 input trigger inmux 12 enable clear"]
    #[inline(always)]
    pub const fn dmac1_itrig_inmux12(&self) -> super::vals::Dmac1ItrigEna0ClrDmac1ItrigInmux12 {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::Dmac1ItrigEna0ClrDmac1ItrigInmux12::from_bits(val as u8)
    }
    #[doc = "DMAC1 input trigger inmux 12 enable clear"]
    #[inline(always)]
    pub fn set_dmac1_itrig_inmux12(
        &mut self,
        val: super::vals::Dmac1ItrigEna0ClrDmac1ItrigInmux12,
    ) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "DMAC1 input trigger inmux 13 enable clear"]
    #[inline(always)]
    pub const fn dmac1_itrig_inmux13(&self) -> super::vals::Dmac1ItrigEna0ClrDmac1ItrigInmux13 {
        let val = (self.0 >> 13usize) & 0x01;
        super::vals::Dmac1ItrigEna0ClrDmac1ItrigInmux13::from_bits(val as u8)
    }
    #[doc = "DMAC1 input trigger inmux 13 enable clear"]
    #[inline(always)]
    pub fn set_dmac1_itrig_inmux13(
        &mut self,
        val: super::vals::Dmac1ItrigEna0ClrDmac1ItrigInmux13,
    ) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "DMAC1 input trigger inmux 14 enable clear"]
    #[inline(always)]
    pub const fn dmac1_itrig_inmux14(&self) -> super::vals::Dmac1ItrigEna0ClrDmac1ItrigInmux14 {
        let val = (self.0 >> 14usize) & 0x01;
        super::vals::Dmac1ItrigEna0ClrDmac1ItrigInmux14::from_bits(val as u8)
    }
    #[doc = "DMAC1 input trigger inmux 14 enable clear"]
    #[inline(always)]
    pub fn set_dmac1_itrig_inmux14(
        &mut self,
        val: super::vals::Dmac1ItrigEna0ClrDmac1ItrigInmux14,
    ) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
    }
    #[doc = "DMAC1 input trigger inmux 15 enable clear"]
    #[inline(always)]
    pub const fn dmac1_itrig_inmux15(&self) -> super::vals::Dmac1ItrigEna0ClrDmac1ItrigInmux15 {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::Dmac1ItrigEna0ClrDmac1ItrigInmux15::from_bits(val as u8)
    }
    #[doc = "DMAC1 input trigger inmux 15 enable clear"]
    #[inline(always)]
    pub fn set_dmac1_itrig_inmux15(
        &mut self,
        val: super::vals::Dmac1ItrigEna0ClrDmac1ItrigInmux15,
    ) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
    #[doc = "DMAC1 input trigger inmux 16 enable clear"]
    #[inline(always)]
    pub const fn dmac1_itrig_inmux16(&self) -> super::vals::Dmac1ItrigEna0ClrDmac1ItrigInmux16 {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::Dmac1ItrigEna0ClrDmac1ItrigInmux16::from_bits(val as u8)
    }
    #[doc = "DMAC1 input trigger inmux 16 enable clear"]
    #[inline(always)]
    pub fn set_dmac1_itrig_inmux16(
        &mut self,
        val: super::vals::Dmac1ItrigEna0ClrDmac1ItrigInmux16,
    ) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "DMAC1 input trigger inmux 17 enable clear"]
    #[inline(always)]
    pub const fn dmac1_itrig_inmux17(&self) -> super::vals::Dmac1ItrigEna0ClrDmac1ItrigInmux17 {
        let val = (self.0 >> 17usize) & 0x01;
        super::vals::Dmac1ItrigEna0ClrDmac1ItrigInmux17::from_bits(val as u8)
    }
    #[doc = "DMAC1 input trigger inmux 17 enable clear"]
    #[inline(always)]
    pub fn set_dmac1_itrig_inmux17(
        &mut self,
        val: super::vals::Dmac1ItrigEna0ClrDmac1ItrigInmux17,
    ) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
    #[doc = "DMAC1 input trigger inmux 18 enable clear"]
    #[inline(always)]
    pub const fn dmac1_itrig_inmux18(&self) -> super::vals::Dmac1ItrigEna0ClrDmac1ItrigInmux18 {
        let val = (self.0 >> 18usize) & 0x01;
        super::vals::Dmac1ItrigEna0ClrDmac1ItrigInmux18::from_bits(val as u8)
    }
    #[doc = "DMAC1 input trigger inmux 18 enable clear"]
    #[inline(always)]
    pub fn set_dmac1_itrig_inmux18(
        &mut self,
        val: super::vals::Dmac1ItrigEna0ClrDmac1ItrigInmux18,
    ) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
    }
    #[doc = "DMAC1 input trigger inmux 19 enable clear"]
    #[inline(always)]
    pub const fn dmac1_itrig_inmux19(&self) -> super::vals::Dmac1ItrigEna0ClrDmac1ItrigInmux19 {
        let val = (self.0 >> 19usize) & 0x01;
        super::vals::Dmac1ItrigEna0ClrDmac1ItrigInmux19::from_bits(val as u8)
    }
    #[doc = "DMAC1 input trigger inmux 19 enable clear"]
    #[inline(always)]
    pub fn set_dmac1_itrig_inmux19(
        &mut self,
        val: super::vals::Dmac1ItrigEna0ClrDmac1ItrigInmux19,
    ) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
    #[doc = "DMAC1 input trigger inmux 20 enable clear"]
    #[inline(always)]
    pub const fn dmac1_itrig_inmux20(&self) -> super::vals::Dmac1ItrigEna0ClrDmac1ItrigInmux20 {
        let val = (self.0 >> 20usize) & 0x01;
        super::vals::Dmac1ItrigEna0ClrDmac1ItrigInmux20::from_bits(val as u8)
    }
    #[doc = "DMAC1 input trigger inmux 20 enable clear"]
    #[inline(always)]
    pub fn set_dmac1_itrig_inmux20(
        &mut self,
        val: super::vals::Dmac1ItrigEna0ClrDmac1ItrigInmux20,
    ) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
    #[doc = "DMAC1 input trigger inmux 21 enable clear"]
    #[inline(always)]
    pub const fn dmac1_itrig_inmux21(&self) -> super::vals::Dmac1ItrigEna0ClrDmac1ItrigInmux21 {
        let val = (self.0 >> 21usize) & 0x01;
        super::vals::Dmac1ItrigEna0ClrDmac1ItrigInmux21::from_bits(val as u8)
    }
    #[doc = "DMAC1 input trigger inmux 21 enable clear"]
    #[inline(always)]
    pub fn set_dmac1_itrig_inmux21(
        &mut self,
        val: super::vals::Dmac1ItrigEna0ClrDmac1ItrigInmux21,
    ) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
    }
    #[doc = "DMAC1 input trigger inmux 22 enable clear"]
    #[inline(always)]
    pub const fn dmac1_itrig_inmux22(&self) -> super::vals::Dmac1ItrigEna0ClrDmac1ItrigInmux22 {
        let val = (self.0 >> 22usize) & 0x01;
        super::vals::Dmac1ItrigEna0ClrDmac1ItrigInmux22::from_bits(val as u8)
    }
    #[doc = "DMAC1 input trigger inmux 22 enable clear"]
    #[inline(always)]
    pub fn set_dmac1_itrig_inmux22(
        &mut self,
        val: super::vals::Dmac1ItrigEna0ClrDmac1ItrigInmux22,
    ) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val.to_bits() as u32) & 0x01) << 22usize);
    }
    #[doc = "DMAC1 input trigger inmux 23 enable clear"]
    #[inline(always)]
    pub const fn dmac1_itrig_inmux23(&self) -> super::vals::Dmac1ItrigEna0ClrDmac1ItrigInmux23 {
        let val = (self.0 >> 23usize) & 0x01;
        super::vals::Dmac1ItrigEna0ClrDmac1ItrigInmux23::from_bits(val as u8)
    }
    #[doc = "DMAC1 input trigger inmux 23 enable clear"]
    #[inline(always)]
    pub fn set_dmac1_itrig_inmux23(
        &mut self,
        val: super::vals::Dmac1ItrigEna0ClrDmac1ItrigInmux23,
    ) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "DMAC1 input trigger inmux 24 enable clear"]
    #[inline(always)]
    pub const fn dmac1_itrig_inmux24(&self) -> super::vals::Dmac1ItrigEna0ClrDmac1ItrigInmux24 {
        let val = (self.0 >> 24usize) & 0x01;
        super::vals::Dmac1ItrigEna0ClrDmac1ItrigInmux24::from_bits(val as u8)
    }
    #[doc = "DMAC1 input trigger inmux 24 enable clear"]
    #[inline(always)]
    pub fn set_dmac1_itrig_inmux24(
        &mut self,
        val: super::vals::Dmac1ItrigEna0ClrDmac1ItrigInmux24,
    ) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
    #[doc = "DMAC1 input trigger inmux 25 enable clear"]
    #[inline(always)]
    pub const fn dmac1_itrig_inmux25(&self) -> super::vals::Dmac1ItrigEna0ClrDmac1ItrigInmux25 {
        let val = (self.0 >> 25usize) & 0x01;
        super::vals::Dmac1ItrigEna0ClrDmac1ItrigInmux25::from_bits(val as u8)
    }
    #[doc = "DMAC1 input trigger inmux 25 enable clear"]
    #[inline(always)]
    pub fn set_dmac1_itrig_inmux25(
        &mut self,
        val: super::vals::Dmac1ItrigEna0ClrDmac1ItrigInmux25,
    ) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val.to_bits() as u32) & 0x01) << 25usize);
    }
    #[doc = "DMAC1 input trigger inmux 25 enable clear"]
    #[inline(always)]
    pub const fn dmac1_itrig_inmux26(&self) -> super::vals::Dmac1ItrigEna0ClrDmac1ItrigInmux26 {
        let val = (self.0 >> 26usize) & 0x01;
        super::vals::Dmac1ItrigEna0ClrDmac1ItrigInmux26::from_bits(val as u8)
    }
    #[doc = "DMAC1 input trigger inmux 25 enable clear"]
    #[inline(always)]
    pub fn set_dmac1_itrig_inmux26(
        &mut self,
        val: super::vals::Dmac1ItrigEna0ClrDmac1ItrigInmux26,
    ) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val.to_bits() as u32) & 0x01) << 26usize);
    }
    #[doc = "DMAC1 input trigger inmux 25 enable clear"]
    #[inline(always)]
    pub const fn dmac1_itrig_inmux27(&self) -> super::vals::Dmac1ItrigEna0ClrDmac1ItrigInmux27 {
        let val = (self.0 >> 27usize) & 0x01;
        super::vals::Dmac1ItrigEna0ClrDmac1ItrigInmux27::from_bits(val as u8)
    }
    #[doc = "DMAC1 input trigger inmux 25 enable clear"]
    #[inline(always)]
    pub fn set_dmac1_itrig_inmux27(
        &mut self,
        val: super::vals::Dmac1ItrigEna0ClrDmac1ItrigInmux27,
    ) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val.to_bits() as u32) & 0x01) << 27usize);
    }
    #[doc = "DMAC1 input trigger inmux 25 enable clear"]
    #[inline(always)]
    pub const fn dmac1_itrig_inmux28(&self) -> super::vals::Dmac1ItrigEna0ClrDmac1ItrigInmux28 {
        let val = (self.0 >> 28usize) & 0x01;
        super::vals::Dmac1ItrigEna0ClrDmac1ItrigInmux28::from_bits(val as u8)
    }
    #[doc = "DMAC1 input trigger inmux 25 enable clear"]
    #[inline(always)]
    pub fn set_dmac1_itrig_inmux28(
        &mut self,
        val: super::vals::Dmac1ItrigEna0ClrDmac1ItrigInmux28,
    ) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val.to_bits() as u32) & 0x01) << 28usize);
    }
    #[doc = "DMAC1 input trigger inmux 25 enable clear"]
    #[inline(always)]
    pub const fn dmac1_itrig_inmux29(&self) -> super::vals::Dmac1ItrigEna0ClrDmac1ItrigInmux29 {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::Dmac1ItrigEna0ClrDmac1ItrigInmux29::from_bits(val as u8)
    }
    #[doc = "DMAC1 input trigger inmux 25 enable clear"]
    #[inline(always)]
    pub fn set_dmac1_itrig_inmux29(
        &mut self,
        val: super::vals::Dmac1ItrigEna0ClrDmac1ItrigInmux29,
    ) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "DMAC1 input trigger inmux 25 enable clear"]
    #[inline(always)]
    pub const fn dmac1_itrig_inmux30(&self) -> super::vals::Dmac1ItrigEna0ClrDmac1ItrigInmux30 {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::Dmac1ItrigEna0ClrDmac1ItrigInmux30::from_bits(val as u8)
    }
    #[doc = "DMAC1 input trigger inmux 25 enable clear"]
    #[inline(always)]
    pub fn set_dmac1_itrig_inmux30(
        &mut self,
        val: super::vals::Dmac1ItrigEna0ClrDmac1ItrigInmux30,
    ) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "DMAC1 input trigger inmux 25 enable clear"]
    #[inline(always)]
    pub const fn dmac1_itrig_inmux31(&self) -> super::vals::Dmac1ItrigEna0ClrDmac1ItrigInmux31 {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::Dmac1ItrigEna0ClrDmac1ItrigInmux31::from_bits(val as u8)
    }
    #[doc = "DMAC1 input trigger inmux 25 enable clear"]
    #[inline(always)]
    pub fn set_dmac1_itrig_inmux31(
        &mut self,
        val: super::vals::Dmac1ItrigEna0ClrDmac1ItrigInmux31,
    ) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Dmac1ItrigEna0Clr {
    #[inline(always)]
    fn default() -> Dmac1ItrigEna0Clr {
        Dmac1ItrigEna0Clr(0)
    }
}
#[doc = "DMAC1 input trigger enable set 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dmac1ItrigEna0Set(pub u32);
impl Dmac1ItrigEna0Set {
    #[doc = "DMAC1 input trigger inmux 0 enable set"]
    #[inline(always)]
    pub const fn dmac1_itrig_inmux0(&self) -> super::vals::Dmac1ItrigEna0SetDmac1ItrigInmux0 {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Dmac1ItrigEna0SetDmac1ItrigInmux0::from_bits(val as u8)
    }
    #[doc = "DMAC1 input trigger inmux 0 enable set"]
    #[inline(always)]
    pub fn set_dmac1_itrig_inmux0(&mut self, val: super::vals::Dmac1ItrigEna0SetDmac1ItrigInmux0) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "DMAC1 input trigger inmux 1 enable set"]
    #[inline(always)]
    pub const fn dmac1_itrig_inmux1(&self) -> super::vals::Dmac1ItrigEna0SetDmac1ItrigInmux1 {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Dmac1ItrigEna0SetDmac1ItrigInmux1::from_bits(val as u8)
    }
    #[doc = "DMAC1 input trigger inmux 1 enable set"]
    #[inline(always)]
    pub fn set_dmac1_itrig_inmux1(&mut self, val: super::vals::Dmac1ItrigEna0SetDmac1ItrigInmux1) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "DMAC1 input trigger inmux 2 enable set"]
    #[inline(always)]
    pub const fn dmac1_itrig_inmux2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "DMAC1 input trigger inmux 2 enable set"]
    #[inline(always)]
    pub fn set_dmac1_itrig_inmux2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "DMAC1 input trigger inmux 3 enable set"]
    #[inline(always)]
    pub const fn dmac1_itrig_inmux3(&self) -> super::vals::Dmac1ItrigEna0SetDmac1ItrigInmux3 {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Dmac1ItrigEna0SetDmac1ItrigInmux3::from_bits(val as u8)
    }
    #[doc = "DMAC1 input trigger inmux 3 enable set"]
    #[inline(always)]
    pub fn set_dmac1_itrig_inmux3(&mut self, val: super::vals::Dmac1ItrigEna0SetDmac1ItrigInmux3) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "DMAC1 input trigger inmux 4 enable set"]
    #[inline(always)]
    pub const fn dmac1_itrig_inmux4(&self) -> super::vals::Dmac1ItrigEna0SetDmac1ItrigInmux4 {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Dmac1ItrigEna0SetDmac1ItrigInmux4::from_bits(val as u8)
    }
    #[doc = "DMAC1 input trigger inmux 4 enable set"]
    #[inline(always)]
    pub fn set_dmac1_itrig_inmux4(&mut self, val: super::vals::Dmac1ItrigEna0SetDmac1ItrigInmux4) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "DMAC1 input trigger inmux 5 enable set"]
    #[inline(always)]
    pub const fn dmac1_itrig_inmux5(&self) -> super::vals::Dmac1ItrigEna0SetDmac1ItrigInmux5 {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::Dmac1ItrigEna0SetDmac1ItrigInmux5::from_bits(val as u8)
    }
    #[doc = "DMAC1 input trigger inmux 5 enable set"]
    #[inline(always)]
    pub fn set_dmac1_itrig_inmux5(&mut self, val: super::vals::Dmac1ItrigEna0SetDmac1ItrigInmux5) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "DMAC1 input trigger inmux 6 enable set"]
    #[inline(always)]
    pub const fn dmac1_itrig_inmux6(&self) -> super::vals::Dmac1ItrigEna0SetDmac1ItrigInmux6 {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Dmac1ItrigEna0SetDmac1ItrigInmux6::from_bits(val as u8)
    }
    #[doc = "DMAC1 input trigger inmux 6 enable set"]
    #[inline(always)]
    pub fn set_dmac1_itrig_inmux6(&mut self, val: super::vals::Dmac1ItrigEna0SetDmac1ItrigInmux6) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "DMAC1 input trigger inmux 7 enable set"]
    #[inline(always)]
    pub const fn dmac1_itrig_inmux7(&self) -> super::vals::Dmac1ItrigEna0SetDmac1ItrigInmux7 {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Dmac1ItrigEna0SetDmac1ItrigInmux7::from_bits(val as u8)
    }
    #[doc = "DMAC1 input trigger inmux 7 enable set"]
    #[inline(always)]
    pub fn set_dmac1_itrig_inmux7(&mut self, val: super::vals::Dmac1ItrigEna0SetDmac1ItrigInmux7) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "DMAC1 input trigger inmux 8 enable set"]
    #[inline(always)]
    pub const fn dmac1_itrig_inmux8(&self) -> super::vals::Dmac1ItrigEna0SetDmac1ItrigInmux8 {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Dmac1ItrigEna0SetDmac1ItrigInmux8::from_bits(val as u8)
    }
    #[doc = "DMAC1 input trigger inmux 8 enable set"]
    #[inline(always)]
    pub fn set_dmac1_itrig_inmux8(&mut self, val: super::vals::Dmac1ItrigEna0SetDmac1ItrigInmux8) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "DMAC1 input trigger inmux 9 enable set"]
    #[inline(always)]
    pub const fn dmac1_itrig_inmux9(&self) -> super::vals::Dmac1ItrigEna0SetDmac1ItrigInmux9 {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::Dmac1ItrigEna0SetDmac1ItrigInmux9::from_bits(val as u8)
    }
    #[doc = "DMAC1 input trigger inmux 9 enable set"]
    #[inline(always)]
    pub fn set_dmac1_itrig_inmux9(&mut self, val: super::vals::Dmac1ItrigEna0SetDmac1ItrigInmux9) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "DMAC1 input trigger inmux 10 enable set"]
    #[inline(always)]
    pub const fn dmac1_itrig_inmux10(&self) -> super::vals::Dmac1ItrigEna0SetDmac1ItrigInmux10 {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::Dmac1ItrigEna0SetDmac1ItrigInmux10::from_bits(val as u8)
    }
    #[doc = "DMAC1 input trigger inmux 10 enable set"]
    #[inline(always)]
    pub fn set_dmac1_itrig_inmux10(
        &mut self,
        val: super::vals::Dmac1ItrigEna0SetDmac1ItrigInmux10,
    ) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "DMAC1 input trigger inmux 11 enable set"]
    #[inline(always)]
    pub const fn dmac1_itrig_inmux11(&self) -> super::vals::Dmac1ItrigEna0SetDmac1ItrigInmux11 {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::Dmac1ItrigEna0SetDmac1ItrigInmux11::from_bits(val as u8)
    }
    #[doc = "DMAC1 input trigger inmux 11 enable set"]
    #[inline(always)]
    pub fn set_dmac1_itrig_inmux11(
        &mut self,
        val: super::vals::Dmac1ItrigEna0SetDmac1ItrigInmux11,
    ) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "DMAC1 input trigger inmux 12 enable set"]
    #[inline(always)]
    pub const fn dmac1_itrig_inmux12(&self) -> super::vals::Dmac1ItrigEna0SetDmac1ItrigInmux12 {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::Dmac1ItrigEna0SetDmac1ItrigInmux12::from_bits(val as u8)
    }
    #[doc = "DMAC1 input trigger inmux 12 enable set"]
    #[inline(always)]
    pub fn set_dmac1_itrig_inmux12(
        &mut self,
        val: super::vals::Dmac1ItrigEna0SetDmac1ItrigInmux12,
    ) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "DMAC1 input trigger inmux 13 enable set"]
    #[inline(always)]
    pub const fn dmac1_itrig_inmux13(&self) -> super::vals::Dmac1ItrigEna0SetDmac1ItrigInmux13 {
        let val = (self.0 >> 13usize) & 0x01;
        super::vals::Dmac1ItrigEna0SetDmac1ItrigInmux13::from_bits(val as u8)
    }
    #[doc = "DMAC1 input trigger inmux 13 enable set"]
    #[inline(always)]
    pub fn set_dmac1_itrig_inmux13(
        &mut self,
        val: super::vals::Dmac1ItrigEna0SetDmac1ItrigInmux13,
    ) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "DMAC1 input trigger inmux 14 enable set"]
    #[inline(always)]
    pub const fn dmac1_itrig_inmux14(&self) -> super::vals::Dmac1ItrigEna0SetDmac1ItrigInmux14 {
        let val = (self.0 >> 14usize) & 0x01;
        super::vals::Dmac1ItrigEna0SetDmac1ItrigInmux14::from_bits(val as u8)
    }
    #[doc = "DMAC1 input trigger inmux 14 enable set"]
    #[inline(always)]
    pub fn set_dmac1_itrig_inmux14(
        &mut self,
        val: super::vals::Dmac1ItrigEna0SetDmac1ItrigInmux14,
    ) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
    }
    #[doc = "DMAC1 input trigger inmux 15 enable set"]
    #[inline(always)]
    pub const fn dmac1_itrig_inmux15(&self) -> super::vals::Dmac1ItrigEna0SetDmac1ItrigInmux15 {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::Dmac1ItrigEna0SetDmac1ItrigInmux15::from_bits(val as u8)
    }
    #[doc = "DMAC1 input trigger inmux 15 enable set"]
    #[inline(always)]
    pub fn set_dmac1_itrig_inmux15(
        &mut self,
        val: super::vals::Dmac1ItrigEna0SetDmac1ItrigInmux15,
    ) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
    #[doc = "DMAC1 input trigger inmux 16 enable set"]
    #[inline(always)]
    pub const fn dmac1_itrig_inmux16(&self) -> super::vals::Dmac1ItrigEna0SetDmac1ItrigInmux16 {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::Dmac1ItrigEna0SetDmac1ItrigInmux16::from_bits(val as u8)
    }
    #[doc = "DMAC1 input trigger inmux 16 enable set"]
    #[inline(always)]
    pub fn set_dmac1_itrig_inmux16(
        &mut self,
        val: super::vals::Dmac1ItrigEna0SetDmac1ItrigInmux16,
    ) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "DMAC1 input trigger inmux 17 enable set"]
    #[inline(always)]
    pub const fn dmac1_itrig_inmux17(&self) -> super::vals::Dmac1ItrigEna0SetDmac1ItrigInmux17 {
        let val = (self.0 >> 17usize) & 0x01;
        super::vals::Dmac1ItrigEna0SetDmac1ItrigInmux17::from_bits(val as u8)
    }
    #[doc = "DMAC1 input trigger inmux 17 enable set"]
    #[inline(always)]
    pub fn set_dmac1_itrig_inmux17(
        &mut self,
        val: super::vals::Dmac1ItrigEna0SetDmac1ItrigInmux17,
    ) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
    #[doc = "DMAC1 input trigger inmux 18 enable set"]
    #[inline(always)]
    pub const fn dmac1_itrig_inmux18(&self) -> super::vals::Dmac1ItrigEna0SetDmac1ItrigInmux18 {
        let val = (self.0 >> 18usize) & 0x01;
        super::vals::Dmac1ItrigEna0SetDmac1ItrigInmux18::from_bits(val as u8)
    }
    #[doc = "DMAC1 input trigger inmux 18 enable set"]
    #[inline(always)]
    pub fn set_dmac1_itrig_inmux18(
        &mut self,
        val: super::vals::Dmac1ItrigEna0SetDmac1ItrigInmux18,
    ) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
    }
    #[doc = "DMAC1 input trigger inmux 19 enable set"]
    #[inline(always)]
    pub const fn dmac1_itrig_inmux19(&self) -> super::vals::Dmac1ItrigEna0SetDmac1ItrigInmux19 {
        let val = (self.0 >> 19usize) & 0x01;
        super::vals::Dmac1ItrigEna0SetDmac1ItrigInmux19::from_bits(val as u8)
    }
    #[doc = "DMAC1 input trigger inmux 19 enable set"]
    #[inline(always)]
    pub fn set_dmac1_itrig_inmux19(
        &mut self,
        val: super::vals::Dmac1ItrigEna0SetDmac1ItrigInmux19,
    ) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
    #[doc = "DMAC1 input trigger inmux 20 enable set"]
    #[inline(always)]
    pub const fn dmac1_itrig_inmux20(&self) -> super::vals::Dmac1ItrigEna0SetDmac1ItrigInmux20 {
        let val = (self.0 >> 20usize) & 0x01;
        super::vals::Dmac1ItrigEna0SetDmac1ItrigInmux20::from_bits(val as u8)
    }
    #[doc = "DMAC1 input trigger inmux 20 enable set"]
    #[inline(always)]
    pub fn set_dmac1_itrig_inmux20(
        &mut self,
        val: super::vals::Dmac1ItrigEna0SetDmac1ItrigInmux20,
    ) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
    #[doc = "DMAC1 input trigger inmux 21 enable set"]
    #[inline(always)]
    pub const fn dmac1_itrig_inmux21(&self) -> super::vals::Dmac1ItrigEna0SetDmac1ItrigInmux21 {
        let val = (self.0 >> 21usize) & 0x01;
        super::vals::Dmac1ItrigEna0SetDmac1ItrigInmux21::from_bits(val as u8)
    }
    #[doc = "DMAC1 input trigger inmux 21 enable set"]
    #[inline(always)]
    pub fn set_dmac1_itrig_inmux21(
        &mut self,
        val: super::vals::Dmac1ItrigEna0SetDmac1ItrigInmux21,
    ) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
    }
    #[doc = "DMAC1 input trigger inmux 22 enable set"]
    #[inline(always)]
    pub const fn dmac1_itrig_inmux22(&self) -> super::vals::Dmac1ItrigEna0SetDmac1ItrigInmux22 {
        let val = (self.0 >> 22usize) & 0x01;
        super::vals::Dmac1ItrigEna0SetDmac1ItrigInmux22::from_bits(val as u8)
    }
    #[doc = "DMAC1 input trigger inmux 22 enable set"]
    #[inline(always)]
    pub fn set_dmac1_itrig_inmux22(
        &mut self,
        val: super::vals::Dmac1ItrigEna0SetDmac1ItrigInmux22,
    ) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val.to_bits() as u32) & 0x01) << 22usize);
    }
    #[doc = "DMAC1 input trigger inmux 23 enable set"]
    #[inline(always)]
    pub const fn dmac1_itrig_inmux23(&self) -> super::vals::Dmac1ItrigEna0SetDmac1ItrigInmux23 {
        let val = (self.0 >> 23usize) & 0x01;
        super::vals::Dmac1ItrigEna0SetDmac1ItrigInmux23::from_bits(val as u8)
    }
    #[doc = "DMAC1 input trigger inmux 23 enable set"]
    #[inline(always)]
    pub fn set_dmac1_itrig_inmux23(
        &mut self,
        val: super::vals::Dmac1ItrigEna0SetDmac1ItrigInmux23,
    ) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "DMAC1 input trigger inmux 24 enable set"]
    #[inline(always)]
    pub const fn dmac1_itrig_inmux24(&self) -> super::vals::Dmac1ItrigEna0SetDmac1ItrigInmux24 {
        let val = (self.0 >> 24usize) & 0x01;
        super::vals::Dmac1ItrigEna0SetDmac1ItrigInmux24::from_bits(val as u8)
    }
    #[doc = "DMAC1 input trigger inmux 24 enable set"]
    #[inline(always)]
    pub fn set_dmac1_itrig_inmux24(
        &mut self,
        val: super::vals::Dmac1ItrigEna0SetDmac1ItrigInmux24,
    ) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
    #[doc = "DMAC1 input trigger inmux 25 enable set"]
    #[inline(always)]
    pub const fn dmac1_itrig_inmux25(&self) -> super::vals::Dmac1ItrigEna0SetDmac1ItrigInmux25 {
        let val = (self.0 >> 25usize) & 0x01;
        super::vals::Dmac1ItrigEna0SetDmac1ItrigInmux25::from_bits(val as u8)
    }
    #[doc = "DMAC1 input trigger inmux 25 enable set"]
    #[inline(always)]
    pub fn set_dmac1_itrig_inmux25(
        &mut self,
        val: super::vals::Dmac1ItrigEna0SetDmac1ItrigInmux25,
    ) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val.to_bits() as u32) & 0x01) << 25usize);
    }
    #[doc = "DMAC1 input trigger inmux 25 enable set"]
    #[inline(always)]
    pub const fn dmac1_itrig_inmux26(&self) -> super::vals::Dmac1ItrigEna0SetDmac1ItrigInmux26 {
        let val = (self.0 >> 26usize) & 0x01;
        super::vals::Dmac1ItrigEna0SetDmac1ItrigInmux26::from_bits(val as u8)
    }
    #[doc = "DMAC1 input trigger inmux 25 enable set"]
    #[inline(always)]
    pub fn set_dmac1_itrig_inmux26(
        &mut self,
        val: super::vals::Dmac1ItrigEna0SetDmac1ItrigInmux26,
    ) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val.to_bits() as u32) & 0x01) << 26usize);
    }
    #[doc = "DMAC1 input trigger inmux 25 enable set"]
    #[inline(always)]
    pub const fn dmac1_itrig_inmux27(&self) -> super::vals::Dmac1ItrigEna0SetDmac1ItrigInmux27 {
        let val = (self.0 >> 27usize) & 0x01;
        super::vals::Dmac1ItrigEna0SetDmac1ItrigInmux27::from_bits(val as u8)
    }
    #[doc = "DMAC1 input trigger inmux 25 enable set"]
    #[inline(always)]
    pub fn set_dmac1_itrig_inmux27(
        &mut self,
        val: super::vals::Dmac1ItrigEna0SetDmac1ItrigInmux27,
    ) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val.to_bits() as u32) & 0x01) << 27usize);
    }
    #[doc = "DMAC1 input trigger inmux 25 enable set"]
    #[inline(always)]
    pub const fn dmac1_itrig_inmux28(&self) -> super::vals::Dmac1ItrigEna0SetDmac1ItrigInmux28 {
        let val = (self.0 >> 28usize) & 0x01;
        super::vals::Dmac1ItrigEna0SetDmac1ItrigInmux28::from_bits(val as u8)
    }
    #[doc = "DMAC1 input trigger inmux 25 enable set"]
    #[inline(always)]
    pub fn set_dmac1_itrig_inmux28(
        &mut self,
        val: super::vals::Dmac1ItrigEna0SetDmac1ItrigInmux28,
    ) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val.to_bits() as u32) & 0x01) << 28usize);
    }
    #[doc = "DMAC1 input trigger inmux 25 enable set"]
    #[inline(always)]
    pub const fn dmac1_itrig_inmux29(&self) -> super::vals::Dmac1ItrigEna0SetDmac1ItrigInmux29 {
        let val = (self.0 >> 29usize) & 0x01;
        super::vals::Dmac1ItrigEna0SetDmac1ItrigInmux29::from_bits(val as u8)
    }
    #[doc = "DMAC1 input trigger inmux 25 enable set"]
    #[inline(always)]
    pub fn set_dmac1_itrig_inmux29(
        &mut self,
        val: super::vals::Dmac1ItrigEna0SetDmac1ItrigInmux29,
    ) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val.to_bits() as u32) & 0x01) << 29usize);
    }
    #[doc = "DMAC1 input trigger inmux 25 enable set"]
    #[inline(always)]
    pub const fn dmac1_itrig_inmux30(&self) -> super::vals::Dmac1ItrigEna0SetDmac1ItrigInmux30 {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::Dmac1ItrigEna0SetDmac1ItrigInmux30::from_bits(val as u8)
    }
    #[doc = "DMAC1 input trigger inmux 25 enable set"]
    #[inline(always)]
    pub fn set_dmac1_itrig_inmux30(
        &mut self,
        val: super::vals::Dmac1ItrigEna0SetDmac1ItrigInmux30,
    ) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
    #[doc = "DMAC1 input trigger inmux 25 enable set"]
    #[inline(always)]
    pub const fn dmac1_itrig_inmux31(&self) -> super::vals::Dmac1ItrigEna0SetDmac1ItrigInmux31 {
        let val = (self.0 >> 31usize) & 0x01;
        super::vals::Dmac1ItrigEna0SetDmac1ItrigInmux31::from_bits(val as u8)
    }
    #[doc = "DMAC1 input trigger inmux 25 enable set"]
    #[inline(always)]
    pub fn set_dmac1_itrig_inmux31(
        &mut self,
        val: super::vals::Dmac1ItrigEna0SetDmac1ItrigInmux31,
    ) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val.to_bits() as u32) & 0x01) << 31usize);
    }
}
impl Default for Dmac1ItrigEna0Set {
    #[inline(always)]
    fn default() -> Dmac1ItrigEna0Set {
        Dmac1ItrigEna0Set(0)
    }
}
#[doc = "DMAC1 Input Trigger Multiplexers N"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dmac1ItrigSel(pub u32);
impl Dmac1ItrigSel {
    #[doc = "DMA Input Triggers(n) Selection. 18:1 Selection for each. . ."]
    #[inline(always)]
    pub const fn dma1_itrig_sel(&self) -> super::vals::Dma1ItrigSel {
        let val = (self.0 >> 0usize) & 0x1f;
        super::vals::Dma1ItrigSel::from_bits(val as u8)
    }
    #[doc = "DMA Input Triggers(n) Selection. 18:1 Selection for each. . ."]
    #[inline(always)]
    pub fn set_dma1_itrig_sel(&mut self, val: super::vals::Dma1ItrigSel) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val.to_bits() as u32) & 0x1f) << 0usize);
    }
}
impl Default for Dmac1ItrigSel {
    #[inline(always)]
    fn default() -> Dmac1ItrigSel {
        Dmac1ItrigSel(0)
    }
}
#[doc = "DMAC1 Output Trigger Multiplexers N"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dmac1OtrigSel(pub u32);
impl Dmac1OtrigSel {
    #[doc = "DMA1 Output Triggers Select for A, B, C, D IE., DMA1_OTRIG_A, DMA1_OTRIG_B, DM1_OTRIG_C, DMA1_OTRIG_D DMA0 Output Triggers(n) Selection. 32:1 Selection for each. . ."]
    #[inline(always)]
    pub const fn dmac1_otrig_sel(&self) -> super::vals::Dmac1OtrigSel {
        let val = (self.0 >> 0usize) & 0x3f;
        super::vals::Dmac1OtrigSel::from_bits(val as u8)
    }
    #[doc = "DMA1 Output Triggers Select for A, B, C, D IE., DMA1_OTRIG_A, DMA1_OTRIG_B, DM1_OTRIG_C, DMA1_OTRIG_D DMA0 Output Triggers(n) Selection. 32:1 Selection for each. . ."]
    #[inline(always)]
    pub fn set_dmac1_otrig_sel(&mut self, val: super::vals::Dmac1OtrigSel) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val.to_bits() as u32) & 0x3f) << 0usize);
    }
}
impl Default for Dmac1OtrigSel {
    #[inline(always)]
    fn default() -> Dmac1OtrigSel {
        Dmac1OtrigSel(0)
    }
}
#[doc = "DMAC1 request enable 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dmac1ReqEna0(pub u32);
impl Dmac1ReqEna0 {
    #[doc = "FLEXCOMM0 RX enable"]
    #[inline(always)]
    pub const fn flexcomm0_rx(&self) -> super::vals::Dmac1ReqEna0Flexcomm0Rx {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Dmac1ReqEna0Flexcomm0Rx::from_bits(val as u8)
    }
    #[doc = "FLEXCOMM0 RX enable"]
    #[inline(always)]
    pub fn set_flexcomm0_rx(&mut self, val: super::vals::Dmac1ReqEna0Flexcomm0Rx) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "FLEXCOMM0 TX enable"]
    #[inline(always)]
    pub const fn flexcomm0_tx(&self) -> super::vals::Dmac1ReqEna0Flexcomm0Tx {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Dmac1ReqEna0Flexcomm0Tx::from_bits(val as u8)
    }
    #[doc = "FLEXCOMM0 TX enable"]
    #[inline(always)]
    pub fn set_flexcomm0_tx(&mut self, val: super::vals::Dmac1ReqEna0Flexcomm0Tx) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "FLEXCOMM1 RX enable"]
    #[inline(always)]
    pub const fn flexcomm1_rx(&self) -> super::vals::Dmac1ReqEna0Flexcomm1Rx {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Dmac1ReqEna0Flexcomm1Rx::from_bits(val as u8)
    }
    #[doc = "FLEXCOMM1 RX enable"]
    #[inline(always)]
    pub fn set_flexcomm1_rx(&mut self, val: super::vals::Dmac1ReqEna0Flexcomm1Rx) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "FLEXCOMM1 TX enable"]
    #[inline(always)]
    pub const fn flexcomm1_tx(&self) -> super::vals::Dmac1ReqEna0Flexcomm1Tx {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Dmac1ReqEna0Flexcomm1Tx::from_bits(val as u8)
    }
    #[doc = "FLEXCOMM1 TX enable"]
    #[inline(always)]
    pub fn set_flexcomm1_tx(&mut self, val: super::vals::Dmac1ReqEna0Flexcomm1Tx) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "FLEXCOMM2 RX enable"]
    #[inline(always)]
    pub const fn flexcomm2_rx(&self) -> super::vals::Dmac1ReqEna0Flexcomm2Rx {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Dmac1ReqEna0Flexcomm2Rx::from_bits(val as u8)
    }
    #[doc = "FLEXCOMM2 RX enable"]
    #[inline(always)]
    pub fn set_flexcomm2_rx(&mut self, val: super::vals::Dmac1ReqEna0Flexcomm2Rx) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "FLEXCOMM2 TX enable"]
    #[inline(always)]
    pub const fn flexcomm2_tx(&self) -> super::vals::Dmac1ReqEna0Flexcomm2Tx {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::Dmac1ReqEna0Flexcomm2Tx::from_bits(val as u8)
    }
    #[doc = "FLEXCOMM2 TX enable"]
    #[inline(always)]
    pub fn set_flexcomm2_tx(&mut self, val: super::vals::Dmac1ReqEna0Flexcomm2Tx) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "FLEXCOMM3 RX enable"]
    #[inline(always)]
    pub const fn flexcomm3_rx(&self) -> super::vals::Dmac1ReqEna0Flexcomm3Rx {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Dmac1ReqEna0Flexcomm3Rx::from_bits(val as u8)
    }
    #[doc = "FLEXCOMM3 RX enable"]
    #[inline(always)]
    pub fn set_flexcomm3_rx(&mut self, val: super::vals::Dmac1ReqEna0Flexcomm3Rx) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "FLEXCOMM3 TX enable"]
    #[inline(always)]
    pub const fn flexcomm3_tx(&self) -> super::vals::Dmac1ReqEna0Flexcomm3Tx {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Dmac1ReqEna0Flexcomm3Tx::from_bits(val as u8)
    }
    #[doc = "FLEXCOMM3 TX enable"]
    #[inline(always)]
    pub fn set_flexcomm3_tx(&mut self, val: super::vals::Dmac1ReqEna0Flexcomm3Tx) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "FLEXCOMM4 RX enable"]
    #[inline(always)]
    pub const fn flexcomm4_rx(&self) -> super::vals::Dmac1ReqEna0Flexcomm4Rx {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Dmac1ReqEna0Flexcomm4Rx::from_bits(val as u8)
    }
    #[doc = "FLEXCOMM4 RX enable"]
    #[inline(always)]
    pub fn set_flexcomm4_rx(&mut self, val: super::vals::Dmac1ReqEna0Flexcomm4Rx) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "FLEXCOMM4 TX enable"]
    #[inline(always)]
    pub const fn flexcomm4_tx(&self) -> super::vals::Dmac1ReqEna0Flexcomm4Tx {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::Dmac1ReqEna0Flexcomm4Tx::from_bits(val as u8)
    }
    #[doc = "FLEXCOMM4 TX enable"]
    #[inline(always)]
    pub fn set_flexcomm4_tx(&mut self, val: super::vals::Dmac1ReqEna0Flexcomm4Tx) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "FLEXCOMM5 RX enable"]
    #[inline(always)]
    pub const fn flexcomm5_rx(&self) -> super::vals::Dmac1ReqEna0Flexcomm5Rx {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::Dmac1ReqEna0Flexcomm5Rx::from_bits(val as u8)
    }
    #[doc = "FLEXCOMM5 RX enable"]
    #[inline(always)]
    pub fn set_flexcomm5_rx(&mut self, val: super::vals::Dmac1ReqEna0Flexcomm5Rx) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "FLEXCOMM5 TX enable"]
    #[inline(always)]
    pub const fn flexcomm5_tx(&self) -> super::vals::Dmac1ReqEna0Flexcomm5Tx {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::Dmac1ReqEna0Flexcomm5Tx::from_bits(val as u8)
    }
    #[doc = "FLEXCOMM5 TX enable"]
    #[inline(always)]
    pub fn set_flexcomm5_tx(&mut self, val: super::vals::Dmac1ReqEna0Flexcomm5Tx) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "FLEXCOMM6 RX enable"]
    #[inline(always)]
    pub const fn flexcomm6_rx(&self) -> super::vals::Dmac1ReqEna0Flexcomm6Rx {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::Dmac1ReqEna0Flexcomm6Rx::from_bits(val as u8)
    }
    #[doc = "FLEXCOMM6 RX enable"]
    #[inline(always)]
    pub fn set_flexcomm6_rx(&mut self, val: super::vals::Dmac1ReqEna0Flexcomm6Rx) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "FLEXCOMM6 TX enable"]
    #[inline(always)]
    pub const fn flexcomm6_tx(&self) -> super::vals::Dmac1ReqEna0Flexcomm6Tx {
        let val = (self.0 >> 13usize) & 0x01;
        super::vals::Dmac1ReqEna0Flexcomm6Tx::from_bits(val as u8)
    }
    #[doc = "FLEXCOMM6 TX enable"]
    #[inline(always)]
    pub fn set_flexcomm6_tx(&mut self, val: super::vals::Dmac1ReqEna0Flexcomm6Tx) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "FLEXCOMM7 RX enable"]
    #[inline(always)]
    pub const fn flexcomm7_rx(&self) -> super::vals::Dmac1ReqEna0Flexcomm7Rx {
        let val = (self.0 >> 14usize) & 0x01;
        super::vals::Dmac1ReqEna0Flexcomm7Rx::from_bits(val as u8)
    }
    #[doc = "FLEXCOMM7 RX enable"]
    #[inline(always)]
    pub fn set_flexcomm7_rx(&mut self, val: super::vals::Dmac1ReqEna0Flexcomm7Rx) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
    }
    #[doc = "FLEXCOMM7 TX enable"]
    #[inline(always)]
    pub const fn flexcomm7_tx(&self) -> super::vals::Dmac1ReqEna0Flexcomm7Tx {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::Dmac1ReqEna0Flexcomm7Tx::from_bits(val as u8)
    }
    #[doc = "FLEXCOMM7 TX enable"]
    #[inline(always)]
    pub fn set_flexcomm7_tx(&mut self, val: super::vals::Dmac1ReqEna0Flexcomm7Tx) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
    #[doc = "DMIC0 channel 0 enable"]
    #[inline(always)]
    pub const fn dmic0ch0(&self) -> super::vals::Dmac1ReqEna0Dmic0ch0 {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::Dmac1ReqEna0Dmic0ch0::from_bits(val as u8)
    }
    #[doc = "DMIC0 channel 0 enable"]
    #[inline(always)]
    pub fn set_dmic0ch0(&mut self, val: super::vals::Dmac1ReqEna0Dmic0ch0) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "DMIC0 channel 1 enable"]
    #[inline(always)]
    pub const fn dmic0ch1(&self) -> super::vals::Dmac1ReqEna0Dmic0ch1 {
        let val = (self.0 >> 17usize) & 0x01;
        super::vals::Dmac1ReqEna0Dmic0ch1::from_bits(val as u8)
    }
    #[doc = "DMIC0 channel 1 enable"]
    #[inline(always)]
    pub fn set_dmic0ch1(&mut self, val: super::vals::Dmac1ReqEna0Dmic0ch1) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
    #[doc = "DMIC0 channel 2 enable"]
    #[inline(always)]
    pub const fn dmic0ch2(&self) -> super::vals::Dmac1ReqEna0Dmic0ch2 {
        let val = (self.0 >> 18usize) & 0x01;
        super::vals::Dmac1ReqEna0Dmic0ch2::from_bits(val as u8)
    }
    #[doc = "DMIC0 channel 2 enable"]
    #[inline(always)]
    pub fn set_dmic0ch2(&mut self, val: super::vals::Dmac1ReqEna0Dmic0ch2) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
    }
    #[doc = "DMIC0 channel 3 enable"]
    #[inline(always)]
    pub const fn dmic0ch3(&self) -> super::vals::Dmac1ReqEna0Dmic0ch3 {
        let val = (self.0 >> 19usize) & 0x01;
        super::vals::Dmac1ReqEna0Dmic0ch3::from_bits(val as u8)
    }
    #[doc = "DMIC0 channel 3 enable"]
    #[inline(always)]
    pub fn set_dmic0ch3(&mut self, val: super::vals::Dmac1ReqEna0Dmic0ch3) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
    #[doc = "DMIC0 channel 4 enable"]
    #[inline(always)]
    pub const fn dmic0ch4(&self) -> super::vals::Dmac1ReqEna0Dmic0ch4 {
        let val = (self.0 >> 20usize) & 0x01;
        super::vals::Dmac1ReqEna0Dmic0ch4::from_bits(val as u8)
    }
    #[doc = "DMIC0 channel 4 enable"]
    #[inline(always)]
    pub fn set_dmic0ch4(&mut self, val: super::vals::Dmac1ReqEna0Dmic0ch4) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
    #[doc = "DMIC0 channel 5 enable"]
    #[inline(always)]
    pub const fn dmic0ch5(&self) -> super::vals::Dmac1ReqEna0Dmic0ch5 {
        let val = (self.0 >> 21usize) & 0x01;
        super::vals::Dmac1ReqEna0Dmic0ch5::from_bits(val as u8)
    }
    #[doc = "DMIC0 channel 5 enable"]
    #[inline(always)]
    pub fn set_dmic0ch5(&mut self, val: super::vals::Dmac1ReqEna0Dmic0ch5) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
    }
    #[doc = "DMIC0 channel 6 enable"]
    #[inline(always)]
    pub const fn dmic0ch6(&self) -> super::vals::Dmac1ReqEna0Dmic0ch6 {
        let val = (self.0 >> 22usize) & 0x01;
        super::vals::Dmac1ReqEna0Dmic0ch6::from_bits(val as u8)
    }
    #[doc = "DMIC0 channel 6 enable"]
    #[inline(always)]
    pub fn set_dmic0ch6(&mut self, val: super::vals::Dmac1ReqEna0Dmic0ch6) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val.to_bits() as u32) & 0x01) << 22usize);
    }
    #[doc = "DMIC0 channel 7 enable"]
    #[inline(always)]
    pub const fn dmic0ch7(&self) -> super::vals::Dmac1ReqEna0Dmic0ch7 {
        let val = (self.0 >> 23usize) & 0x01;
        super::vals::Dmac1ReqEna0Dmic0ch7::from_bits(val as u8)
    }
    #[doc = "DMIC0 channel 7 enable"]
    #[inline(always)]
    pub fn set_dmic0ch7(&mut self, val: super::vals::Dmac1ReqEna0Dmic0ch7) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "I3C RX enable"]
    #[inline(always)]
    pub const fn i3c0_rx(&self) -> super::vals::Dmac1ReqEna0I3c0Rx {
        let val = (self.0 >> 24usize) & 0x01;
        super::vals::Dmac1ReqEna0I3c0Rx::from_bits(val as u8)
    }
    #[doc = "I3C RX enable"]
    #[inline(always)]
    pub fn set_i3c0_rx(&mut self, val: super::vals::Dmac1ReqEna0I3c0Rx) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
    #[doc = "I3C TX enable"]
    #[inline(always)]
    pub const fn i3c0_tx(&self) -> super::vals::Dmac1ReqEna0I3c0Tx {
        let val = (self.0 >> 25usize) & 0x01;
        super::vals::Dmac1ReqEna0I3c0Tx::from_bits(val as u8)
    }
    #[doc = "I3C TX enable"]
    #[inline(always)]
    pub fn set_i3c0_tx(&mut self, val: super::vals::Dmac1ReqEna0I3c0Tx) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val.to_bits() as u32) & 0x01) << 25usize);
    }
    #[doc = "FLEXCOMM14 RX enable"]
    #[inline(always)]
    pub const fn flexcomm14_rx(&self) -> super::vals::Dmac1ReqEna0Flexcomm14Rx {
        let val = (self.0 >> 26usize) & 0x01;
        super::vals::Dmac1ReqEna0Flexcomm14Rx::from_bits(val as u8)
    }
    #[doc = "FLEXCOMM14 RX enable"]
    #[inline(always)]
    pub fn set_flexcomm14_rx(&mut self, val: super::vals::Dmac1ReqEna0Flexcomm14Rx) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val.to_bits() as u32) & 0x01) << 26usize);
    }
    #[doc = "FLEXCOMM14 TX enable"]
    #[inline(always)]
    pub const fn flexcomm14_tx(&self) -> super::vals::Dmac1ReqEna0Flexcomm14Tx {
        let val = (self.0 >> 27usize) & 0x01;
        super::vals::Dmac1ReqEna0Flexcomm14Tx::from_bits(val as u8)
    }
    #[doc = "FLEXCOMM14 TX enable"]
    #[inline(always)]
    pub fn set_flexcomm14_tx(&mut self, val: super::vals::Dmac1ReqEna0Flexcomm14Tx) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val.to_bits() as u32) & 0x01) << 27usize);
    }
    #[doc = "hash enable"]
    #[inline(always)]
    pub const fn hashcrypt(&self) -> super::vals::Dmac1ReqEna0Hashcrypt {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::Dmac1ReqEna0Hashcrypt::from_bits(val as u8)
    }
    #[doc = "hash enable"]
    #[inline(always)]
    pub fn set_hashcrypt(&mut self, val: super::vals::Dmac1ReqEna0Hashcrypt) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
}
impl Default for Dmac1ReqEna0 {
    #[inline(always)]
    fn default() -> Dmac1ReqEna0 {
        Dmac1ReqEna0(0)
    }
}
#[doc = "DMAC1 request enable clear 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dmac1ReqEna0Clr(pub u32);
impl Dmac1ReqEna0Clr {
    #[doc = "FLEXCOMM0 RX enable clear"]
    #[inline(always)]
    pub const fn flexcomm0_rx(&self) -> super::vals::Dmac1ReqEna0ClrFlexcomm0Rx {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Dmac1ReqEna0ClrFlexcomm0Rx::from_bits(val as u8)
    }
    #[doc = "FLEXCOMM0 RX enable clear"]
    #[inline(always)]
    pub fn set_flexcomm0_rx(&mut self, val: super::vals::Dmac1ReqEna0ClrFlexcomm0Rx) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "FLEXCOMM0 TX enable clear"]
    #[inline(always)]
    pub const fn flexcomm0_tx(&self) -> super::vals::Dmac1ReqEna0ClrFlexcomm0Tx {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Dmac1ReqEna0ClrFlexcomm0Tx::from_bits(val as u8)
    }
    #[doc = "FLEXCOMM0 TX enable clear"]
    #[inline(always)]
    pub fn set_flexcomm0_tx(&mut self, val: super::vals::Dmac1ReqEna0ClrFlexcomm0Tx) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "FLEXCOMM1 RX enable clear"]
    #[inline(always)]
    pub const fn flexcomm1_rx(&self) -> super::vals::Dmac1ReqEna0ClrFlexcomm1Rx {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Dmac1ReqEna0ClrFlexcomm1Rx::from_bits(val as u8)
    }
    #[doc = "FLEXCOMM1 RX enable clear"]
    #[inline(always)]
    pub fn set_flexcomm1_rx(&mut self, val: super::vals::Dmac1ReqEna0ClrFlexcomm1Rx) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "FLEXCOMM1 TX enable clear"]
    #[inline(always)]
    pub const fn flexcomm1_tx(&self) -> super::vals::Dmac1ReqEna0ClrFlexcomm1Tx {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Dmac1ReqEna0ClrFlexcomm1Tx::from_bits(val as u8)
    }
    #[doc = "FLEXCOMM1 TX enable clear"]
    #[inline(always)]
    pub fn set_flexcomm1_tx(&mut self, val: super::vals::Dmac1ReqEna0ClrFlexcomm1Tx) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "FLEXCOMM2 RX enable clear"]
    #[inline(always)]
    pub const fn flexcomm2_rx(&self) -> super::vals::Dmac1ReqEna0ClrFlexcomm2Rx {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Dmac1ReqEna0ClrFlexcomm2Rx::from_bits(val as u8)
    }
    #[doc = "FLEXCOMM2 RX enable clear"]
    #[inline(always)]
    pub fn set_flexcomm2_rx(&mut self, val: super::vals::Dmac1ReqEna0ClrFlexcomm2Rx) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "FLEXCOMM2 TX enable clear"]
    #[inline(always)]
    pub const fn flexcomm2_tx(&self) -> super::vals::Dmac1ReqEna0ClrFlexcomm2Tx {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::Dmac1ReqEna0ClrFlexcomm2Tx::from_bits(val as u8)
    }
    #[doc = "FLEXCOMM2 TX enable clear"]
    #[inline(always)]
    pub fn set_flexcomm2_tx(&mut self, val: super::vals::Dmac1ReqEna0ClrFlexcomm2Tx) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "FLEXCOMM3 RX enable clear"]
    #[inline(always)]
    pub const fn flexcomm3_rx(&self) -> super::vals::Dmac1ReqEna0ClrFlexcomm3Rx {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Dmac1ReqEna0ClrFlexcomm3Rx::from_bits(val as u8)
    }
    #[doc = "FLEXCOMM3 RX enable clear"]
    #[inline(always)]
    pub fn set_flexcomm3_rx(&mut self, val: super::vals::Dmac1ReqEna0ClrFlexcomm3Rx) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "FLEXCOMM3 TX enable clear"]
    #[inline(always)]
    pub const fn flexcomm3_tx(&self) -> super::vals::Dmac1ReqEna0ClrFlexcomm3Tx {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Dmac1ReqEna0ClrFlexcomm3Tx::from_bits(val as u8)
    }
    #[doc = "FLEXCOMM3 TX enable clear"]
    #[inline(always)]
    pub fn set_flexcomm3_tx(&mut self, val: super::vals::Dmac1ReqEna0ClrFlexcomm3Tx) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "FLEXCOMM4 RX enable clear"]
    #[inline(always)]
    pub const fn flexcomm4_rx(&self) -> super::vals::Dmac1ReqEna0ClrFlexcomm4Rx {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Dmac1ReqEna0ClrFlexcomm4Rx::from_bits(val as u8)
    }
    #[doc = "FLEXCOMM4 RX enable clear"]
    #[inline(always)]
    pub fn set_flexcomm4_rx(&mut self, val: super::vals::Dmac1ReqEna0ClrFlexcomm4Rx) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "FLEXCOMM4 TX enable clear"]
    #[inline(always)]
    pub const fn flexcomm4_tx(&self) -> super::vals::Dmac1ReqEna0ClrFlexcomm4Tx {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::Dmac1ReqEna0ClrFlexcomm4Tx::from_bits(val as u8)
    }
    #[doc = "FLEXCOMM4 TX enable clear"]
    #[inline(always)]
    pub fn set_flexcomm4_tx(&mut self, val: super::vals::Dmac1ReqEna0ClrFlexcomm4Tx) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "FLEXCOMM5 RX enable clear"]
    #[inline(always)]
    pub const fn flexcomm5_rx(&self) -> super::vals::Dmac1ReqEna0ClrFlexcomm5Rx {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::Dmac1ReqEna0ClrFlexcomm5Rx::from_bits(val as u8)
    }
    #[doc = "FLEXCOMM5 RX enable clear"]
    #[inline(always)]
    pub fn set_flexcomm5_rx(&mut self, val: super::vals::Dmac1ReqEna0ClrFlexcomm5Rx) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "FLEXCOMM5 TX enable clear"]
    #[inline(always)]
    pub const fn flexcomm5_tx(&self) -> super::vals::Dmac1ReqEna0ClrFlexcomm5Tx {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::Dmac1ReqEna0ClrFlexcomm5Tx::from_bits(val as u8)
    }
    #[doc = "FLEXCOMM5 TX enable clear"]
    #[inline(always)]
    pub fn set_flexcomm5_tx(&mut self, val: super::vals::Dmac1ReqEna0ClrFlexcomm5Tx) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "FLEXCOMM6 RX enable clear"]
    #[inline(always)]
    pub const fn flexcomm6_rx(&self) -> super::vals::Dmac1ReqEna0ClrFlexcomm6Rx {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::Dmac1ReqEna0ClrFlexcomm6Rx::from_bits(val as u8)
    }
    #[doc = "FLEXCOMM6 RX enable clear"]
    #[inline(always)]
    pub fn set_flexcomm6_rx(&mut self, val: super::vals::Dmac1ReqEna0ClrFlexcomm6Rx) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "FLEXCOMM6 TX enable clear"]
    #[inline(always)]
    pub const fn flexcomm6_tx(&self) -> super::vals::Dmac1ReqEna0ClrFlexcomm6Tx {
        let val = (self.0 >> 13usize) & 0x01;
        super::vals::Dmac1ReqEna0ClrFlexcomm6Tx::from_bits(val as u8)
    }
    #[doc = "FLEXCOMM6 TX enable clear"]
    #[inline(always)]
    pub fn set_flexcomm6_tx(&mut self, val: super::vals::Dmac1ReqEna0ClrFlexcomm6Tx) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "FLEXCOMM7 RX enable clear"]
    #[inline(always)]
    pub const fn flexcomm7_rx(&self) -> super::vals::Dmac1ReqEna0ClrFlexcomm7Rx {
        let val = (self.0 >> 14usize) & 0x01;
        super::vals::Dmac1ReqEna0ClrFlexcomm7Rx::from_bits(val as u8)
    }
    #[doc = "FLEXCOMM7 RX enable clear"]
    #[inline(always)]
    pub fn set_flexcomm7_rx(&mut self, val: super::vals::Dmac1ReqEna0ClrFlexcomm7Rx) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
    }
    #[doc = "FLEXCOMM7 TX enable clear"]
    #[inline(always)]
    pub const fn flexcomm7_tx(&self) -> super::vals::Dmac1ReqEna0ClrFlexcomm7Tx {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::Dmac1ReqEna0ClrFlexcomm7Tx::from_bits(val as u8)
    }
    #[doc = "FLEXCOMM7 TX enable clear"]
    #[inline(always)]
    pub fn set_flexcomm7_tx(&mut self, val: super::vals::Dmac1ReqEna0ClrFlexcomm7Tx) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
    #[doc = "DMIC0 channel 0 enable clear"]
    #[inline(always)]
    pub const fn dmic0ch0(&self) -> super::vals::Dmac1ReqEna0ClrDmic0ch0 {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::Dmac1ReqEna0ClrDmic0ch0::from_bits(val as u8)
    }
    #[doc = "DMIC0 channel 0 enable clear"]
    #[inline(always)]
    pub fn set_dmic0ch0(&mut self, val: super::vals::Dmac1ReqEna0ClrDmic0ch0) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "DMIC0 channel 1 enable clear"]
    #[inline(always)]
    pub const fn dmic0ch1(&self) -> super::vals::Dmac1ReqEna0ClrDmic0ch1 {
        let val = (self.0 >> 17usize) & 0x01;
        super::vals::Dmac1ReqEna0ClrDmic0ch1::from_bits(val as u8)
    }
    #[doc = "DMIC0 channel 1 enable clear"]
    #[inline(always)]
    pub fn set_dmic0ch1(&mut self, val: super::vals::Dmac1ReqEna0ClrDmic0ch1) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
    #[doc = "DMIC0 channel 2 enable clear"]
    #[inline(always)]
    pub const fn dmic0ch2(&self) -> super::vals::Dmac1ReqEna0ClrDmic0ch2 {
        let val = (self.0 >> 18usize) & 0x01;
        super::vals::Dmac1ReqEna0ClrDmic0ch2::from_bits(val as u8)
    }
    #[doc = "DMIC0 channel 2 enable clear"]
    #[inline(always)]
    pub fn set_dmic0ch2(&mut self, val: super::vals::Dmac1ReqEna0ClrDmic0ch2) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
    }
    #[doc = "DMIC0 channel 3 enable clear"]
    #[inline(always)]
    pub const fn dmic0ch3(&self) -> super::vals::Dmac1ReqEna0ClrDmic0ch3 {
        let val = (self.0 >> 19usize) & 0x01;
        super::vals::Dmac1ReqEna0ClrDmic0ch3::from_bits(val as u8)
    }
    #[doc = "DMIC0 channel 3 enable clear"]
    #[inline(always)]
    pub fn set_dmic0ch3(&mut self, val: super::vals::Dmac1ReqEna0ClrDmic0ch3) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
    #[doc = "DMIC0 channel 4 enable clear"]
    #[inline(always)]
    pub const fn dmic0ch4(&self) -> super::vals::Dmac1ReqEna0ClrDmic0ch4 {
        let val = (self.0 >> 20usize) & 0x01;
        super::vals::Dmac1ReqEna0ClrDmic0ch4::from_bits(val as u8)
    }
    #[doc = "DMIC0 channel 4 enable clear"]
    #[inline(always)]
    pub fn set_dmic0ch4(&mut self, val: super::vals::Dmac1ReqEna0ClrDmic0ch4) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
    #[doc = "DMIC0 channel 5 enable clear"]
    #[inline(always)]
    pub const fn dmic0ch5(&self) -> super::vals::Dmac1ReqEna0ClrDmic0ch5 {
        let val = (self.0 >> 21usize) & 0x01;
        super::vals::Dmac1ReqEna0ClrDmic0ch5::from_bits(val as u8)
    }
    #[doc = "DMIC0 channel 5 enable clear"]
    #[inline(always)]
    pub fn set_dmic0ch5(&mut self, val: super::vals::Dmac1ReqEna0ClrDmic0ch5) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
    }
    #[doc = "DMIC0 channel 6 enable clear"]
    #[inline(always)]
    pub const fn dmic0ch6(&self) -> super::vals::Dmac1ReqEna0ClrDmic0ch6 {
        let val = (self.0 >> 22usize) & 0x01;
        super::vals::Dmac1ReqEna0ClrDmic0ch6::from_bits(val as u8)
    }
    #[doc = "DMIC0 channel 6 enable clear"]
    #[inline(always)]
    pub fn set_dmic0ch6(&mut self, val: super::vals::Dmac1ReqEna0ClrDmic0ch6) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val.to_bits() as u32) & 0x01) << 22usize);
    }
    #[doc = "DMIC0 channel 7 enable clear"]
    #[inline(always)]
    pub const fn dmic0ch7(&self) -> super::vals::Dmac1ReqEna0ClrDmic0ch7 {
        let val = (self.0 >> 23usize) & 0x01;
        super::vals::Dmac1ReqEna0ClrDmic0ch7::from_bits(val as u8)
    }
    #[doc = "DMIC0 channel 7 enable clear"]
    #[inline(always)]
    pub fn set_dmic0ch7(&mut self, val: super::vals::Dmac1ReqEna0ClrDmic0ch7) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "I3C RX enable clear"]
    #[inline(always)]
    pub const fn i3c0_rx(&self) -> super::vals::Dmac1ReqEna0ClrI3c0Rx {
        let val = (self.0 >> 24usize) & 0x01;
        super::vals::Dmac1ReqEna0ClrI3c0Rx::from_bits(val as u8)
    }
    #[doc = "I3C RX enable clear"]
    #[inline(always)]
    pub fn set_i3c0_rx(&mut self, val: super::vals::Dmac1ReqEna0ClrI3c0Rx) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
    #[doc = "I3C TX enable clear"]
    #[inline(always)]
    pub const fn i3c0_tx(&self) -> super::vals::Dmac1ReqEna0ClrI3c0Tx {
        let val = (self.0 >> 25usize) & 0x01;
        super::vals::Dmac1ReqEna0ClrI3c0Tx::from_bits(val as u8)
    }
    #[doc = "I3C TX enable clear"]
    #[inline(always)]
    pub fn set_i3c0_tx(&mut self, val: super::vals::Dmac1ReqEna0ClrI3c0Tx) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val.to_bits() as u32) & 0x01) << 25usize);
    }
    #[doc = "FLEXCOMM14 RX enable clear"]
    #[inline(always)]
    pub const fn flexcomm14_rx(&self) -> super::vals::Dmac1ReqEna0ClrFlexcomm14Rx {
        let val = (self.0 >> 26usize) & 0x01;
        super::vals::Dmac1ReqEna0ClrFlexcomm14Rx::from_bits(val as u8)
    }
    #[doc = "FLEXCOMM14 RX enable clear"]
    #[inline(always)]
    pub fn set_flexcomm14_rx(&mut self, val: super::vals::Dmac1ReqEna0ClrFlexcomm14Rx) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val.to_bits() as u32) & 0x01) << 26usize);
    }
    #[doc = "FLEXCOMM14 TX enable clear"]
    #[inline(always)]
    pub const fn flexcomm14_tx(&self) -> super::vals::Dmac1ReqEna0ClrFlexcomm14Tx {
        let val = (self.0 >> 27usize) & 0x01;
        super::vals::Dmac1ReqEna0ClrFlexcomm14Tx::from_bits(val as u8)
    }
    #[doc = "FLEXCOMM14 TX enable clear"]
    #[inline(always)]
    pub fn set_flexcomm14_tx(&mut self, val: super::vals::Dmac1ReqEna0ClrFlexcomm14Tx) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val.to_bits() as u32) & 0x01) << 27usize);
    }
    #[doc = "Hash enable clear"]
    #[inline(always)]
    pub const fn hashcrypt(&self) -> super::vals::Dmac1ReqEna0ClrHashcrypt {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::Dmac1ReqEna0ClrHashcrypt::from_bits(val as u8)
    }
    #[doc = "Hash enable clear"]
    #[inline(always)]
    pub fn set_hashcrypt(&mut self, val: super::vals::Dmac1ReqEna0ClrHashcrypt) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
}
impl Default for Dmac1ReqEna0Clr {
    #[inline(always)]
    fn default() -> Dmac1ReqEna0Clr {
        Dmac1ReqEna0Clr(0)
    }
}
#[doc = "DMAC1 request enable set 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dmac1ReqEna0Set(pub u32);
impl Dmac1ReqEna0Set {
    #[doc = "FLEXCOMM0 RX enable set"]
    #[inline(always)]
    pub const fn flexcomm0_rx(&self) -> super::vals::Dmac1ReqEna0SetFlexcomm0Rx {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::Dmac1ReqEna0SetFlexcomm0Rx::from_bits(val as u8)
    }
    #[doc = "FLEXCOMM0 RX enable set"]
    #[inline(always)]
    pub fn set_flexcomm0_rx(&mut self, val: super::vals::Dmac1ReqEna0SetFlexcomm0Rx) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "FLEXCOMM0 TX enable set"]
    #[inline(always)]
    pub const fn flexcomm0_tx(&self) -> super::vals::Dmac1ReqEna0SetFlexcomm0Tx {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::Dmac1ReqEna0SetFlexcomm0Tx::from_bits(val as u8)
    }
    #[doc = "FLEXCOMM0 TX enable set"]
    #[inline(always)]
    pub fn set_flexcomm0_tx(&mut self, val: super::vals::Dmac1ReqEna0SetFlexcomm0Tx) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
    #[doc = "FLEXCOMM1 RX enable set"]
    #[inline(always)]
    pub const fn flexcomm1_rx(&self) -> super::vals::Dmac1ReqEna0SetFlexcomm1Rx {
        let val = (self.0 >> 2usize) & 0x01;
        super::vals::Dmac1ReqEna0SetFlexcomm1Rx::from_bits(val as u8)
    }
    #[doc = "FLEXCOMM1 RX enable set"]
    #[inline(always)]
    pub fn set_flexcomm1_rx(&mut self, val: super::vals::Dmac1ReqEna0SetFlexcomm1Rx) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val.to_bits() as u32) & 0x01) << 2usize);
    }
    #[doc = "FLEXCOMM1 TX enable set"]
    #[inline(always)]
    pub const fn flexcomm1_tx(&self) -> super::vals::Dmac1ReqEna0SetFlexcomm1Tx {
        let val = (self.0 >> 3usize) & 0x01;
        super::vals::Dmac1ReqEna0SetFlexcomm1Tx::from_bits(val as u8)
    }
    #[doc = "FLEXCOMM1 TX enable set"]
    #[inline(always)]
    pub fn set_flexcomm1_tx(&mut self, val: super::vals::Dmac1ReqEna0SetFlexcomm1Tx) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val.to_bits() as u32) & 0x01) << 3usize);
    }
    #[doc = "FLEXCOMM2 RX enable set"]
    #[inline(always)]
    pub const fn flexcomm2_rx(&self) -> super::vals::Dmac1ReqEna0SetFlexcomm2Rx {
        let val = (self.0 >> 4usize) & 0x01;
        super::vals::Dmac1ReqEna0SetFlexcomm2Rx::from_bits(val as u8)
    }
    #[doc = "FLEXCOMM2 RX enable set"]
    #[inline(always)]
    pub fn set_flexcomm2_rx(&mut self, val: super::vals::Dmac1ReqEna0SetFlexcomm2Rx) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val.to_bits() as u32) & 0x01) << 4usize);
    }
    #[doc = "FLEXCOMM2 TX enable set"]
    #[inline(always)]
    pub const fn flexcomm2_tx(&self) -> super::vals::Dmac1ReqEna0SetFlexcomm2Tx {
        let val = (self.0 >> 5usize) & 0x01;
        super::vals::Dmac1ReqEna0SetFlexcomm2Tx::from_bits(val as u8)
    }
    #[doc = "FLEXCOMM2 TX enable set"]
    #[inline(always)]
    pub fn set_flexcomm2_tx(&mut self, val: super::vals::Dmac1ReqEna0SetFlexcomm2Tx) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val.to_bits() as u32) & 0x01) << 5usize);
    }
    #[doc = "FLEXCOMM3 RX enable set"]
    #[inline(always)]
    pub const fn flexcomm3_rx(&self) -> super::vals::Dmac1ReqEna0SetFlexcomm3Rx {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Dmac1ReqEna0SetFlexcomm3Rx::from_bits(val as u8)
    }
    #[doc = "FLEXCOMM3 RX enable set"]
    #[inline(always)]
    pub fn set_flexcomm3_rx(&mut self, val: super::vals::Dmac1ReqEna0SetFlexcomm3Rx) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "FLEXCOMM3 TX enable set"]
    #[inline(always)]
    pub const fn flexcomm3_tx(&self) -> super::vals::Dmac1ReqEna0SetFlexcomm3Tx {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Dmac1ReqEna0SetFlexcomm3Tx::from_bits(val as u8)
    }
    #[doc = "FLEXCOMM3 TX enable set"]
    #[inline(always)]
    pub fn set_flexcomm3_tx(&mut self, val: super::vals::Dmac1ReqEna0SetFlexcomm3Tx) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "FLEXCOMM4 RX enable set"]
    #[inline(always)]
    pub const fn flexcomm4_rx(&self) -> super::vals::Dmac1ReqEna0SetFlexcomm4Rx {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Dmac1ReqEna0SetFlexcomm4Rx::from_bits(val as u8)
    }
    #[doc = "FLEXCOMM4 RX enable set"]
    #[inline(always)]
    pub fn set_flexcomm4_rx(&mut self, val: super::vals::Dmac1ReqEna0SetFlexcomm4Rx) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "FLEXCOMM4 TX enable set"]
    #[inline(always)]
    pub const fn flexcomm4_tx(&self) -> super::vals::Dmac1ReqEna0SetFlexcomm4Tx {
        let val = (self.0 >> 9usize) & 0x01;
        super::vals::Dmac1ReqEna0SetFlexcomm4Tx::from_bits(val as u8)
    }
    #[doc = "FLEXCOMM4 TX enable set"]
    #[inline(always)]
    pub fn set_flexcomm4_tx(&mut self, val: super::vals::Dmac1ReqEna0SetFlexcomm4Tx) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val.to_bits() as u32) & 0x01) << 9usize);
    }
    #[doc = "FLEXCOMM5 RX enable set"]
    #[inline(always)]
    pub const fn flexcomm5_rx(&self) -> super::vals::Dmac1ReqEna0SetFlexcomm5Rx {
        let val = (self.0 >> 10usize) & 0x01;
        super::vals::Dmac1ReqEna0SetFlexcomm5Rx::from_bits(val as u8)
    }
    #[doc = "FLEXCOMM5 RX enable set"]
    #[inline(always)]
    pub fn set_flexcomm5_rx(&mut self, val: super::vals::Dmac1ReqEna0SetFlexcomm5Rx) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val.to_bits() as u32) & 0x01) << 10usize);
    }
    #[doc = "FLEXCOMM5 TX enable set"]
    #[inline(always)]
    pub const fn flexcomm5_tx(&self) -> super::vals::Dmac1ReqEna0SetFlexcomm5Tx {
        let val = (self.0 >> 11usize) & 0x01;
        super::vals::Dmac1ReqEna0SetFlexcomm5Tx::from_bits(val as u8)
    }
    #[doc = "FLEXCOMM5 TX enable set"]
    #[inline(always)]
    pub fn set_flexcomm5_tx(&mut self, val: super::vals::Dmac1ReqEna0SetFlexcomm5Tx) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val.to_bits() as u32) & 0x01) << 11usize);
    }
    #[doc = "FLEXCOMM6 RX enable set"]
    #[inline(always)]
    pub const fn flexcomm6_rx(&self) -> super::vals::Dmac1ReqEna0SetFlexcomm6Rx {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::Dmac1ReqEna0SetFlexcomm6Rx::from_bits(val as u8)
    }
    #[doc = "FLEXCOMM6 RX enable set"]
    #[inline(always)]
    pub fn set_flexcomm6_rx(&mut self, val: super::vals::Dmac1ReqEna0SetFlexcomm6Rx) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "FLEXCOMM6 TX enable set"]
    #[inline(always)]
    pub const fn flexcomm6_tx(&self) -> super::vals::Dmac1ReqEna0SetFlexcomm6Tx {
        let val = (self.0 >> 13usize) & 0x01;
        super::vals::Dmac1ReqEna0SetFlexcomm6Tx::from_bits(val as u8)
    }
    #[doc = "FLEXCOMM6 TX enable set"]
    #[inline(always)]
    pub fn set_flexcomm6_tx(&mut self, val: super::vals::Dmac1ReqEna0SetFlexcomm6Tx) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val.to_bits() as u32) & 0x01) << 13usize);
    }
    #[doc = "FLEXCOMM7 RX enable set"]
    #[inline(always)]
    pub const fn flexcomm7_rx(&self) -> super::vals::Dmac1ReqEna0SetFlexcomm7Rx {
        let val = (self.0 >> 14usize) & 0x01;
        super::vals::Dmac1ReqEna0SetFlexcomm7Rx::from_bits(val as u8)
    }
    #[doc = "FLEXCOMM7 RX enable set"]
    #[inline(always)]
    pub fn set_flexcomm7_rx(&mut self, val: super::vals::Dmac1ReqEna0SetFlexcomm7Rx) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val.to_bits() as u32) & 0x01) << 14usize);
    }
    #[doc = "FLEXCOMM7 TX enable set"]
    #[inline(always)]
    pub const fn flexcomm7_tx(&self) -> super::vals::Dmac1ReqEna0SetFlexcomm7Tx {
        let val = (self.0 >> 15usize) & 0x01;
        super::vals::Dmac1ReqEna0SetFlexcomm7Tx::from_bits(val as u8)
    }
    #[doc = "FLEXCOMM7 TX enable set"]
    #[inline(always)]
    pub fn set_flexcomm7_tx(&mut self, val: super::vals::Dmac1ReqEna0SetFlexcomm7Tx) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val.to_bits() as u32) & 0x01) << 15usize);
    }
    #[doc = "DMIC0 channel 0 enable set"]
    #[inline(always)]
    pub const fn dmic0ch0(&self) -> super::vals::Dmac1ReqEna0SetDmic0ch0 {
        let val = (self.0 >> 16usize) & 0x01;
        super::vals::Dmac1ReqEna0SetDmic0ch0::from_bits(val as u8)
    }
    #[doc = "DMIC0 channel 0 enable set"]
    #[inline(always)]
    pub fn set_dmic0ch0(&mut self, val: super::vals::Dmac1ReqEna0SetDmic0ch0) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val.to_bits() as u32) & 0x01) << 16usize);
    }
    #[doc = "DMIC0 channel 1 enable set"]
    #[inline(always)]
    pub const fn dmic0ch1(&self) -> super::vals::Dmac1ReqEna0SetDmic0ch1 {
        let val = (self.0 >> 17usize) & 0x01;
        super::vals::Dmac1ReqEna0SetDmic0ch1::from_bits(val as u8)
    }
    #[doc = "DMIC0 channel 1 enable set"]
    #[inline(always)]
    pub fn set_dmic0ch1(&mut self, val: super::vals::Dmac1ReqEna0SetDmic0ch1) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val.to_bits() as u32) & 0x01) << 17usize);
    }
    #[doc = "DMIC0 channel 2 enable set"]
    #[inline(always)]
    pub const fn dmic0ch2(&self) -> super::vals::Dmac1ReqEna0SetDmic0ch2 {
        let val = (self.0 >> 18usize) & 0x01;
        super::vals::Dmac1ReqEna0SetDmic0ch2::from_bits(val as u8)
    }
    #[doc = "DMIC0 channel 2 enable set"]
    #[inline(always)]
    pub fn set_dmic0ch2(&mut self, val: super::vals::Dmac1ReqEna0SetDmic0ch2) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val.to_bits() as u32) & 0x01) << 18usize);
    }
    #[doc = "DMIC0 channel 3 enable set"]
    #[inline(always)]
    pub const fn dmic0ch3(&self) -> super::vals::Dmac1ReqEna0SetDmic0ch3 {
        let val = (self.0 >> 19usize) & 0x01;
        super::vals::Dmac1ReqEna0SetDmic0ch3::from_bits(val as u8)
    }
    #[doc = "DMIC0 channel 3 enable set"]
    #[inline(always)]
    pub fn set_dmic0ch3(&mut self, val: super::vals::Dmac1ReqEna0SetDmic0ch3) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val.to_bits() as u32) & 0x01) << 19usize);
    }
    #[doc = "DMIC0 channel 4 enable set"]
    #[inline(always)]
    pub const fn dmic0ch4(&self) -> super::vals::Dmac1ReqEna0SetDmic0ch4 {
        let val = (self.0 >> 20usize) & 0x01;
        super::vals::Dmac1ReqEna0SetDmic0ch4::from_bits(val as u8)
    }
    #[doc = "DMIC0 channel 4 enable set"]
    #[inline(always)]
    pub fn set_dmic0ch4(&mut self, val: super::vals::Dmac1ReqEna0SetDmic0ch4) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val.to_bits() as u32) & 0x01) << 20usize);
    }
    #[doc = "DMIC0 channel 5 enable set"]
    #[inline(always)]
    pub const fn dmic0ch5(&self) -> super::vals::Dmac1ReqEna0SetDmic0ch5 {
        let val = (self.0 >> 21usize) & 0x01;
        super::vals::Dmac1ReqEna0SetDmic0ch5::from_bits(val as u8)
    }
    #[doc = "DMIC0 channel 5 enable set"]
    #[inline(always)]
    pub fn set_dmic0ch5(&mut self, val: super::vals::Dmac1ReqEna0SetDmic0ch5) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val.to_bits() as u32) & 0x01) << 21usize);
    }
    #[doc = "DMIC0 channel 6 enable set"]
    #[inline(always)]
    pub const fn dmic0ch6(&self) -> super::vals::Dmac1ReqEna0SetDmic0ch6 {
        let val = (self.0 >> 22usize) & 0x01;
        super::vals::Dmac1ReqEna0SetDmic0ch6::from_bits(val as u8)
    }
    #[doc = "DMIC0 channel 6 enable set"]
    #[inline(always)]
    pub fn set_dmic0ch6(&mut self, val: super::vals::Dmac1ReqEna0SetDmic0ch6) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val.to_bits() as u32) & 0x01) << 22usize);
    }
    #[doc = "DMIC0 channel 7 enable set"]
    #[inline(always)]
    pub const fn dmic0ch7(&self) -> super::vals::Dmac1ReqEna0SetDmic0ch7 {
        let val = (self.0 >> 23usize) & 0x01;
        super::vals::Dmac1ReqEna0SetDmic0ch7::from_bits(val as u8)
    }
    #[doc = "DMIC0 channel 7 enable set"]
    #[inline(always)]
    pub fn set_dmic0ch7(&mut self, val: super::vals::Dmac1ReqEna0SetDmic0ch7) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val.to_bits() as u32) & 0x01) << 23usize);
    }
    #[doc = "I3C RX enable set"]
    #[inline(always)]
    pub const fn i3c0_rx(&self) -> super::vals::Dmac1ReqEna0SetI3c0Rx {
        let val = (self.0 >> 24usize) & 0x01;
        super::vals::Dmac1ReqEna0SetI3c0Rx::from_bits(val as u8)
    }
    #[doc = "I3C RX enable set"]
    #[inline(always)]
    pub fn set_i3c0_rx(&mut self, val: super::vals::Dmac1ReqEna0SetI3c0Rx) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val.to_bits() as u32) & 0x01) << 24usize);
    }
    #[doc = "I3C TX enable set"]
    #[inline(always)]
    pub const fn i3c0_tx(&self) -> super::vals::Dmac1ReqEna0SetI3c0Tx {
        let val = (self.0 >> 25usize) & 0x01;
        super::vals::Dmac1ReqEna0SetI3c0Tx::from_bits(val as u8)
    }
    #[doc = "I3C TX enable set"]
    #[inline(always)]
    pub fn set_i3c0_tx(&mut self, val: super::vals::Dmac1ReqEna0SetI3c0Tx) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val.to_bits() as u32) & 0x01) << 25usize);
    }
    #[doc = "FLEXCOMM14 TX enable set"]
    #[inline(always)]
    pub const fn flexcomm14_rx(&self) -> super::vals::Dmac1ReqEna0SetFlexcomm14Rx {
        let val = (self.0 >> 26usize) & 0x01;
        super::vals::Dmac1ReqEna0SetFlexcomm14Rx::from_bits(val as u8)
    }
    #[doc = "FLEXCOMM14 TX enable set"]
    #[inline(always)]
    pub fn set_flexcomm14_rx(&mut self, val: super::vals::Dmac1ReqEna0SetFlexcomm14Rx) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val.to_bits() as u32) & 0x01) << 26usize);
    }
    #[doc = "FLEXCOMM15 RX enable set"]
    #[inline(always)]
    pub const fn flexcomm14_tx(&self) -> super::vals::Dmac1ReqEna0SetFlexcomm14Tx {
        let val = (self.0 >> 27usize) & 0x01;
        super::vals::Dmac1ReqEna0SetFlexcomm14Tx::from_bits(val as u8)
    }
    #[doc = "FLEXCOMM15 RX enable set"]
    #[inline(always)]
    pub fn set_flexcomm14_tx(&mut self, val: super::vals::Dmac1ReqEna0SetFlexcomm14Tx) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val.to_bits() as u32) & 0x01) << 27usize);
    }
    #[doc = "Hash enable set"]
    #[inline(always)]
    pub const fn hashcrypt(&self) -> super::vals::Dmac1ReqEna0SetHashcrypt {
        let val = (self.0 >> 30usize) & 0x01;
        super::vals::Dmac1ReqEna0SetHashcrypt::from_bits(val as u8)
    }
    #[doc = "Hash enable set"]
    #[inline(always)]
    pub fn set_hashcrypt(&mut self, val: super::vals::Dmac1ReqEna0SetHashcrypt) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val.to_bits() as u32) & 0x01) << 30usize);
    }
}
impl Default for Dmac1ReqEna0Set {
    #[inline(always)]
    fn default() -> Dmac1ReqEna0Set {
        Dmac1ReqEna0Set(0)
    }
}
#[doc = "DSP Interrupt Input Multiplexers N"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DspIntSel(pub u32);
impl DspIntSel {
    #[doc = "DSP Input(n) Selection. 34:1 Selection for each. . ."]
    #[inline(always)]
    pub const fn dsp_int_sel(&self) -> super::vals::DspIntSel {
        let val = (self.0 >> 0usize) & 0x3f;
        super::vals::DspIntSel::from_bits(val as u8)
    }
    #[doc = "DSP Input(n) Selection. 34:1 Selection for each. . ."]
    #[inline(always)]
    pub fn set_dsp_int_sel(&mut self, val: super::vals::DspIntSel) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val.to_bits() as u32) & 0x3f) << 0usize);
    }
}
impl Default for DspIntSel {
    #[inline(always)]
    fn default() -> DspIntSel {
        DspIntSel(0)
    }
}
#[doc = "Frequency Measurement Input Channel Multiplexers"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FmeasureChSel(pub u32);
impl FmeasureChSel {
    #[doc = "Frequency Measure Channel n Selection 7:1 Mux Select. . ."]
    #[inline(always)]
    pub const fn fmeasure_sel(&self) -> super::vals::FmeasureSel {
        let val = (self.0 >> 0usize) & 0x1f;
        super::vals::FmeasureSel::from_bits(val as u8)
    }
    #[doc = "Frequency Measure Channel n Selection 7:1 Mux Select. . ."]
    #[inline(always)]
    pub fn set_fmeasure_sel(&mut self, val: super::vals::FmeasureSel) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val.to_bits() as u32) & 0x1f) << 0usize);
    }
}
impl Default for FmeasureChSel {
    #[inline(always)]
    fn default() -> FmeasureChSel {
        FmeasureChSel(0)
    }
}
#[doc = "GPIO Pin Input Multiplexer N"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PintSel(pub u32);
impl PintSel {
    #[doc = "Port Input (PIOx.y) 64 to 8 Mux Select. . . Pin number select for pin interrupt or pattern match engine input. (For PIOx_y: INTPIN = (x * 32) + y. PIO0_0 to PIO1_31 correspond to numbers 0 to 63."]
    #[inline(always)]
    pub const fn pint_sel(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Port Input (PIOx.y) 64 to 8 Mux Select. . . Pin number select for pin interrupt or pattern match engine input. (For PIOx_y: INTPIN = (x * 32) + y. PIO0_0 to PIO1_31 correspond to numbers 0 to 63."]
    #[inline(always)]
    pub fn set_pint_sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for PintSel {
    #[inline(always)]
    fn default() -> PintSel {
        PintSel(0)
    }
}
#[doc = "SCT Peripheral Input Multiplexers N"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sct0InSel(pub u32);
impl Sct0InSel {
    #[doc = "SCT0 Input(n) Selection. 24:1 Selection for each. . ."]
    #[inline(always)]
    pub const fn sct_in_sel(&self) -> super::vals::SctInSel {
        let val = (self.0 >> 0usize) & 0x1f;
        super::vals::SctInSel::from_bits(val as u8)
    }
    #[doc = "SCT0 Input(n) Selection. 24:1 Selection for each. . ."]
    #[inline(always)]
    pub fn set_sct_in_sel(&mut self, val: super::vals::SctInSel) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val.to_bits() as u32) & 0x1f) << 0usize);
    }
}
impl Default for Sct0InSel {
    #[inline(always)]
    fn default() -> Sct0InSel {
        Sct0InSel(0)
    }
}

#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Abbpair {
    #[doc = "Bank-pair 0 (1st)"]
    PAIR0 = 0x0,
    #[doc = "Bank-pair 1 (2nd)"]
    PAIR1 = 0x01,
}
impl Abbpair {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Abbpair {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Abbpair {
    #[inline(always)]
    fn from(val: u8) -> Abbpair {
        Abbpair::from_bits(val)
    }
}
impl From<Abbpair> for u8 {
    #[inline(always)]
    fn from(val: Abbpair) -> u8 {
        Abbpair::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Busy {
    #[doc = "Not busy - is idle"]
    IDLE = 0x0,
    #[doc = "Is busy"]
    BUSY = 0x01,
}
impl Busy {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Busy {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Busy {
    #[inline(always)]
    fn from(val: u8) -> Busy {
        Busy::from_bits(val)
    }
}
impl From<Busy> for u8 {
    #[inline(always)]
    fn from(val: Busy) -> u8 {
        Busy::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Carry {
    #[doc = "Carry was 0 or no carry"]
    NO_CARRY = 0x0,
    #[doc = "Carry was 1"]
    CARRY = 0x01,
}
impl Carry {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Carry {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Carry {
    #[inline(always)]
    fn from(val: u8) -> Carry {
        Carry::from_bits(val)
    }
}
impl From<Carry> for u8 {
    #[inline(always)]
    fn from(val: Carry) -> u8 {
        Carry::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Cdbpair {
    #[doc = "Bank-pair 0 (1st)"]
    PAIR0 = 0x0,
    #[doc = "Bank-pair 1 (2nd)"]
    PAIR1 = 0x01,
}
impl Cdbpair {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cdbpair {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cdbpair {
    #[inline(always)]
    fn from(val: u8) -> Cdbpair {
        Cdbpair::from_bits(val)
    }
}
impl From<Cdbpair> for u8 {
    #[inline(always)]
    fn from(val: Cdbpair) -> u8 {
        Cdbpair::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Cskip {
    #[doc = "No Skip"]
    NO_SKIP = 0x0,
    #[doc = "Skip if Carry is 1"]
    SKIP_IF_1 = 0x01,
    #[doc = "Skip if Carry is 0"]
    SKIP_IF_0 = 0x02,
    #[doc = "Set CTRLOFF to CDOFF and Skip"]
    SET_AND_SKIP = 0x03,
}
impl Cskip {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Cskip {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Cskip {
    #[inline(always)]
    fn from(val: u8) -> Cskip {
        Cskip::from_bits(val)
    }
}
impl From<Cskip> for u8 {
    #[inline(always)]
    fn from(val: Cskip) -> u8 {
        Cskip::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum IntenclrDone {
    #[doc = "If written 0, ignored"]
    IGNORED = 0x0,
    #[doc = "If written 1, do not Interrupt when done"]
    NO_INTERRUPT = 0x01,
}
impl IntenclrDone {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IntenclrDone {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IntenclrDone {
    #[inline(always)]
    fn from(val: u8) -> IntenclrDone {
        IntenclrDone::from_bits(val)
    }
}
impl From<IntenclrDone> for u8 {
    #[inline(always)]
    fn from(val: IntenclrDone) -> u8 {
        IntenclrDone::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum IntensetDone {
    #[doc = "Do not interrupt when done"]
    NO_INTERRUPT = 0x0,
    #[doc = "Interrupt when done"]
    INTERRUPT = 0x01,
}
impl IntensetDone {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IntensetDone {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IntensetDone {
    #[inline(always)]
    fn from(val: u8) -> IntensetDone {
        IntensetDone::from_bits(val)
    }
}
impl From<IntensetDone> for u8 {
    #[inline(always)]
    fn from(val: IntensetDone) -> u8 {
        IntensetDone::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum IntstatDone {
    #[doc = "Not caused by accelerator being done"]
    NOT_CAUSED = 0x0,
    #[doc = "Caused by accelerator being done"]
    CAUSED = 0x01,
}
impl IntstatDone {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> IntstatDone {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for IntstatDone {
    #[inline(always)]
    fn from(val: u8) -> IntstatDone {
        IntstatDone::from_bits(val)
    }
}
impl From<IntstatDone> for u8 {
    #[inline(always)]
    fn from(val: IntstatDone) -> u8 {
        IntstatDone::to_bits(val)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Key(pub u16);
impl Key {
    #[doc = "If set during write, will allow lock or unlock"]
    pub const KWY_VALUE: Self = Self(0x073d);
}
impl Key {
    pub const fn from_bits(val: u16) -> Key {
        Self(val & 0x1fff)
    }
    pub const fn to_bits(self) -> u16 {
        self.0
    }
}
impl From<u16> for Key {
    #[inline(always)]
    fn from(val: u16) -> Key {
        Key::from_bits(val)
    }
}
impl From<Key> for u16 {
    #[inline(always)]
    fn from(val: Key) -> u16 {
        Key::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Lock {
    #[doc = "unlock"]
    UNLOCK = 0x0,
    #[doc = "Lock to current security level"]
    LOCK = 0x01,
}
impl Lock {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Lock {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Lock {
    #[inline(always)]
    fn from(val: u8) -> Lock {
        Lock::from_bits(val)
    }
}
impl From<Lock> for u8 {
    #[inline(always)]
    fn from(val: Lock) -> u8 {
        Lock::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Resbpair {
    #[doc = "Bank-pair 0 (1st)"]
    PAIR0 = 0x0,
    #[doc = "Bank-pair 1 (2nd)"]
    PAIR1 = 0x01,
}
impl Resbpair {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Resbpair {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Resbpair {
    #[inline(always)]
    fn from(val: u8) -> Resbpair {
        Resbpair::from_bits(val)
    }
}
impl From<Resbpair> for u8 {
    #[inline(always)]
    fn from(val: Resbpair) -> u8 {
        Resbpair::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum StatusDone {
    #[doc = "Busy or just cleared"]
    BUSY = 0x0,
    #[doc = "Completed last operation"]
    COMPLETED = 0x01,
}
impl StatusDone {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> StatusDone {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for StatusDone {
    #[inline(always)]
    fn from(val: u8) -> StatusDone {
        StatusDone::from_bits(val)
    }
}
impl From<StatusDone> for u8 {
    #[inline(always)]
    fn from(val: StatusDone) -> u8 {
        StatusDone::to_bits(val)
    }
}

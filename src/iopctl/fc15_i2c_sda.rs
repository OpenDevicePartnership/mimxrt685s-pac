#[doc = "Register `FC15_I2C_SDA` reader"]
pub type R = crate::R<Fc15I2cSdaSpec>;
#[doc = "Register `FC15_I2C_SDA` writer"]
pub type W = crate::W<Fc15I2cSdaSpec>;
#[doc = "Function Selector. . .(FSELs Sources can be found in the next several pages.)\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Fsel {
    #[doc = "0: Function 0."]
    Function0 = 0,
    #[doc = "1: Function 1."]
    Function1 = 1,
    #[doc = "2: Function 2."]
    Function2 = 2,
    #[doc = "3: Function 3."]
    Function3 = 3,
    #[doc = "4: Function 4."]
    Function4 = 4,
    #[doc = "5: Function 5."]
    Function5 = 5,
    #[doc = "6: Function 6."]
    Function6 = 6,
    #[doc = "7: Function 7."]
    Function7 = 7,
    #[doc = "8: Function 8."]
    Function8 = 8,
    #[doc = "9: Function 9."]
    Function9 = 9,
    #[doc = "10: Function 10."]
    Function10 = 10,
    #[doc = "11: Function 11."]
    Function11 = 11,
    #[doc = "12: Function 12."]
    Function12 = 12,
    #[doc = "13: Function 13."]
    Function13 = 13,
    #[doc = "14: Function 14."]
    Function14 = 14,
    #[doc = "15: Function 15."]
    Function15 = 15,
}
impl From<Fsel> for u8 {
    #[inline(always)]
    fn from(variant: Fsel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Fsel {
    type Ux = u8;
}
impl crate::IsEnum for Fsel {}
#[doc = "Field `FSEL` reader - Function Selector. . .(FSELs Sources can be found in the next several pages.)"]
pub type FselR = crate::FieldReader<Fsel>;
impl FselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fsel {
        match self.bits {
            0 => Fsel::Function0,
            1 => Fsel::Function1,
            2 => Fsel::Function2,
            3 => Fsel::Function3,
            4 => Fsel::Function4,
            5 => Fsel::Function5,
            6 => Fsel::Function6,
            7 => Fsel::Function7,
            8 => Fsel::Function8,
            9 => Fsel::Function9,
            10 => Fsel::Function10,
            11 => Fsel::Function11,
            12 => Fsel::Function12,
            13 => Fsel::Function13,
            14 => Fsel::Function14,
            15 => Fsel::Function15,
            _ => unreachable!(),
        }
    }
    #[doc = "Function 0."]
    #[inline(always)]
    pub fn is_function_0(&self) -> bool {
        *self == Fsel::Function0
    }
    #[doc = "Function 1."]
    #[inline(always)]
    pub fn is_function_1(&self) -> bool {
        *self == Fsel::Function1
    }
    #[doc = "Function 2."]
    #[inline(always)]
    pub fn is_function_2(&self) -> bool {
        *self == Fsel::Function2
    }
    #[doc = "Function 3."]
    #[inline(always)]
    pub fn is_function_3(&self) -> bool {
        *self == Fsel::Function3
    }
    #[doc = "Function 4."]
    #[inline(always)]
    pub fn is_function_4(&self) -> bool {
        *self == Fsel::Function4
    }
    #[doc = "Function 5."]
    #[inline(always)]
    pub fn is_function_5(&self) -> bool {
        *self == Fsel::Function5
    }
    #[doc = "Function 6."]
    #[inline(always)]
    pub fn is_function_6(&self) -> bool {
        *self == Fsel::Function6
    }
    #[doc = "Function 7."]
    #[inline(always)]
    pub fn is_function_7(&self) -> bool {
        *self == Fsel::Function7
    }
    #[doc = "Function 8."]
    #[inline(always)]
    pub fn is_function_8(&self) -> bool {
        *self == Fsel::Function8
    }
    #[doc = "Function 9."]
    #[inline(always)]
    pub fn is_function_9(&self) -> bool {
        *self == Fsel::Function9
    }
    #[doc = "Function 10."]
    #[inline(always)]
    pub fn is_function_10(&self) -> bool {
        *self == Fsel::Function10
    }
    #[doc = "Function 11."]
    #[inline(always)]
    pub fn is_function_11(&self) -> bool {
        *self == Fsel::Function11
    }
    #[doc = "Function 12."]
    #[inline(always)]
    pub fn is_function_12(&self) -> bool {
        *self == Fsel::Function12
    }
    #[doc = "Function 13."]
    #[inline(always)]
    pub fn is_function_13(&self) -> bool {
        *self == Fsel::Function13
    }
    #[doc = "Function 14."]
    #[inline(always)]
    pub fn is_function_14(&self) -> bool {
        *self == Fsel::Function14
    }
    #[doc = "Function 15."]
    #[inline(always)]
    pub fn is_function_15(&self) -> bool {
        *self == Fsel::Function15
    }
}
#[doc = "Field `FSEL` writer - Function Selector. . .(FSELs Sources can be found in the next several pages.)"]
pub type FselW<'a, REG> = crate::FieldWriter<'a, REG, 4, Fsel, crate::Safe>;
impl<'a, REG> FselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Function 0."]
    #[inline(always)]
    pub fn function_0(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel::Function0)
    }
    #[doc = "Function 1."]
    #[inline(always)]
    pub fn function_1(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel::Function1)
    }
    #[doc = "Function 2."]
    #[inline(always)]
    pub fn function_2(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel::Function2)
    }
    #[doc = "Function 3."]
    #[inline(always)]
    pub fn function_3(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel::Function3)
    }
    #[doc = "Function 4."]
    #[inline(always)]
    pub fn function_4(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel::Function4)
    }
    #[doc = "Function 5."]
    #[inline(always)]
    pub fn function_5(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel::Function5)
    }
    #[doc = "Function 6."]
    #[inline(always)]
    pub fn function_6(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel::Function6)
    }
    #[doc = "Function 7."]
    #[inline(always)]
    pub fn function_7(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel::Function7)
    }
    #[doc = "Function 8."]
    #[inline(always)]
    pub fn function_8(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel::Function8)
    }
    #[doc = "Function 9."]
    #[inline(always)]
    pub fn function_9(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel::Function9)
    }
    #[doc = "Function 10."]
    #[inline(always)]
    pub fn function_10(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel::Function10)
    }
    #[doc = "Function 11."]
    #[inline(always)]
    pub fn function_11(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel::Function11)
    }
    #[doc = "Function 12."]
    #[inline(always)]
    pub fn function_12(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel::Function12)
    }
    #[doc = "Function 13."]
    #[inline(always)]
    pub fn function_13(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel::Function13)
    }
    #[doc = "Function 14."]
    #[inline(always)]
    pub fn function_14(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel::Function14)
    }
    #[doc = "Function 15."]
    #[inline(always)]
    pub fn function_15(self) -> &'a mut crate::W<REG> {
        self.variant(Fsel::Function15)
    }
}
#[doc = "Pullup / Pulldown Enable. . .\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pupdena {
    #[doc = "0: Disable."]
    Disabled = 0,
    #[doc = "1: Enable."]
    Enabled = 1,
}
impl From<Pupdena> for bool {
    #[inline(always)]
    fn from(variant: Pupdena) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PUPDENA` reader - Pullup / Pulldown Enable. . ."]
pub type PupdenaR = crate::BitReader<Pupdena>;
impl PupdenaR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pupdena {
        match self.bits {
            false => Pupdena::Disabled,
            true => Pupdena::Enabled,
        }
    }
    #[doc = "Disable."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Pupdena::Disabled
    }
    #[doc = "Enable."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Pupdena::Enabled
    }
}
#[doc = "Field `PUPDENA` writer - Pullup / Pulldown Enable. . ."]
pub type PupdenaW<'a, REG> = crate::BitWriter<'a, REG, Pupdena>;
impl<'a, REG> PupdenaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Pupdena::Disabled)
    }
    #[doc = "Enable."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Pupdena::Enabled)
    }
}
#[doc = "Pullup or Pulldown Selector. . .\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pupdsel {
    #[doc = "0: Pull-down."]
    PullDown = 0,
    #[doc = "1: Pull-up."]
    PullUp = 1,
}
impl From<Pupdsel> for bool {
    #[inline(always)]
    fn from(variant: Pupdsel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PUPDSEL` reader - Pullup or Pulldown Selector. . ."]
pub type PupdselR = crate::BitReader<Pupdsel>;
impl PupdselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pupdsel {
        match self.bits {
            false => Pupdsel::PullDown,
            true => Pupdsel::PullUp,
        }
    }
    #[doc = "Pull-down."]
    #[inline(always)]
    pub fn is_pull_down(&self) -> bool {
        *self == Pupdsel::PullDown
    }
    #[doc = "Pull-up."]
    #[inline(always)]
    pub fn is_pull_up(&self) -> bool {
        *self == Pupdsel::PullUp
    }
}
#[doc = "Field `PUPDSEL` writer - Pullup or Pulldown Selector. . ."]
pub type PupdselW<'a, REG> = crate::BitWriter<'a, REG, Pupdsel>;
impl<'a, REG> PupdselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Pull-down."]
    #[inline(always)]
    pub fn pull_down(self) -> &'a mut crate::W<REG> {
        self.variant(Pupdsel::PullDown)
    }
    #[doc = "Pull-up."]
    #[inline(always)]
    pub fn pull_up(self) -> &'a mut crate::W<REG> {
        self.variant(Pupdsel::PullUp)
    }
}
#[doc = "Input Buffer Enable. .\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ibena {
    #[doc = "0: Disable."]
    Disabled = 0,
    #[doc = "1: Enable."]
    Enabled = 1,
}
impl From<Ibena> for bool {
    #[inline(always)]
    fn from(variant: Ibena) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IBENA` reader - Input Buffer Enable. ."]
pub type IbenaR = crate::BitReader<Ibena>;
impl IbenaR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ibena {
        match self.bits {
            false => Ibena::Disabled,
            true => Ibena::Enabled,
        }
    }
    #[doc = "Disable."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Ibena::Disabled
    }
    #[doc = "Enable."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Ibena::Enabled
    }
}
#[doc = "Field `IBENA` writer - Input Buffer Enable. ."]
pub type IbenaW<'a, REG> = crate::BitWriter<'a, REG, Ibena>;
impl<'a, REG> IbenaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ibena::Disabled)
    }
    #[doc = "Enable."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ibena::Enabled)
    }
}
#[doc = "Slew Rate Control. . .\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Slewrate {
    #[doc = "0: Slew Rate is Normal."]
    Normal = 0,
    #[doc = "1: Slew Rate Slow."]
    Slow = 1,
}
impl From<Slewrate> for bool {
    #[inline(always)]
    fn from(variant: Slewrate) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SLEWRATE` reader - Slew Rate Control. . ."]
pub type SlewrateR = crate::BitReader<Slewrate>;
impl SlewrateR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Slewrate {
        match self.bits {
            false => Slewrate::Normal,
            true => Slewrate::Slow,
        }
    }
    #[doc = "Slew Rate is Normal."]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == Slewrate::Normal
    }
    #[doc = "Slew Rate Slow."]
    #[inline(always)]
    pub fn is_slow(&self) -> bool {
        *self == Slewrate::Slow
    }
}
#[doc = "Field `SLEWRATE` writer - Slew Rate Control. . ."]
pub type SlewrateW<'a, REG> = crate::BitWriter<'a, REG, Slewrate>;
impl<'a, REG> SlewrateW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Slew Rate is Normal."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(Slewrate::Normal)
    }
    #[doc = "Slew Rate Slow."]
    #[inline(always)]
    pub fn slow(self) -> &'a mut crate::W<REG> {
        self.variant(Slewrate::Slow)
    }
}
#[doc = "Drive Selector. . .\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fulldrive {
    #[doc = "0: Normal Drive."]
    NormalDrive = 0,
    #[doc = "1: Full Drive."]
    FullDrive = 1,
}
impl From<Fulldrive> for bool {
    #[inline(always)]
    fn from(variant: Fulldrive) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FULLDRIVE` reader - Drive Selector. . ."]
pub type FulldriveR = crate::BitReader<Fulldrive>;
impl FulldriveR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fulldrive {
        match self.bits {
            false => Fulldrive::NormalDrive,
            true => Fulldrive::FullDrive,
        }
    }
    #[doc = "Normal Drive."]
    #[inline(always)]
    pub fn is_normal_drive(&self) -> bool {
        *self == Fulldrive::NormalDrive
    }
    #[doc = "Full Drive."]
    #[inline(always)]
    pub fn is_full_drive(&self) -> bool {
        *self == Fulldrive::FullDrive
    }
}
#[doc = "Field `FULLDRIVE` writer - Drive Selector. . ."]
pub type FulldriveW<'a, REG> = crate::BitWriter<'a, REG, Fulldrive>;
impl<'a, REG> FulldriveW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal Drive."]
    #[inline(always)]
    pub fn normal_drive(self) -> &'a mut crate::W<REG> {
        self.variant(Fulldrive::NormalDrive)
    }
    #[doc = "Full Drive."]
    #[inline(always)]
    pub fn full_drive(self) -> &'a mut crate::W<REG> {
        self.variant(Fulldrive::FullDrive)
    }
}
#[doc = "Analog Mux Enable. . .\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Amena {
    #[doc = "0: Disable."]
    Disabled = 0,
    #[doc = "1: Enable."]
    Enabled = 1,
}
impl From<Amena> for bool {
    #[inline(always)]
    fn from(variant: Amena) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AMENA` reader - Analog Mux Enable. . ."]
pub type AmenaR = crate::BitReader<Amena>;
impl AmenaR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Amena {
        match self.bits {
            false => Amena::Disabled,
            true => Amena::Enabled,
        }
    }
    #[doc = "Disable."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Amena::Disabled
    }
    #[doc = "Enable."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Amena::Enabled
    }
}
#[doc = "Field `AMENA` writer - Analog Mux Enable. . ."]
pub type AmenaW<'a, REG> = crate::BitWriter<'a, REG, Amena>;
impl<'a, REG> AmenaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Amena::Disabled)
    }
    #[doc = "Enable."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Amena::Enabled)
    }
}
#[doc = "Pseudo Output Drain Enable. . .\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Odena {
    #[doc = "0: Disable."]
    Disabled = 0,
    #[doc = "1: Enable."]
    Enabled = 1,
}
impl From<Odena> for bool {
    #[inline(always)]
    fn from(variant: Odena) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ODENA` reader - Pseudo Output Drain Enable. . ."]
pub type OdenaR = crate::BitReader<Odena>;
impl OdenaR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Odena {
        match self.bits {
            false => Odena::Disabled,
            true => Odena::Enabled,
        }
    }
    #[doc = "Disable."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Odena::Disabled
    }
    #[doc = "Enable."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Odena::Enabled
    }
}
#[doc = "Field `ODENA` writer - Pseudo Output Drain Enable. . ."]
pub type OdenaW<'a, REG> = crate::BitWriter<'a, REG, Odena>;
impl<'a, REG> OdenaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Odena::Disabled)
    }
    #[doc = "Enable."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Odena::Enabled)
    }
}
#[doc = "Input Invert Enable. . .\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Iiena {
    #[doc = "0: Disable."]
    Disabled = 0,
    #[doc = "1: Enable."]
    Enabled = 1,
}
impl From<Iiena> for bool {
    #[inline(always)]
    fn from(variant: Iiena) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IIENA` reader - Input Invert Enable. . ."]
pub type IienaR = crate::BitReader<Iiena>;
impl IienaR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Iiena {
        match self.bits {
            false => Iiena::Disabled,
            true => Iiena::Enabled,
        }
    }
    #[doc = "Disable."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Iiena::Disabled
    }
    #[doc = "Enable."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Iiena::Enabled
    }
}
#[doc = "Field `IIENA` writer - Input Invert Enable. . ."]
pub type IienaW<'a, REG> = crate::BitWriter<'a, REG, Iiena>;
impl<'a, REG> IienaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Iiena::Disabled)
    }
    #[doc = "Enable."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Iiena::Enabled)
    }
}
impl R {
    #[doc = "Bits 0:3 - Function Selector. . .(FSELs Sources can be found in the next several pages.)"]
    #[inline(always)]
    pub fn fsel(&self) -> FselR {
        FselR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - Pullup / Pulldown Enable. . ."]
    #[inline(always)]
    pub fn pupdena(&self) -> PupdenaR {
        PupdenaR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Pullup or Pulldown Selector. . ."]
    #[inline(always)]
    pub fn pupdsel(&self) -> PupdselR {
        PupdselR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Input Buffer Enable. ."]
    #[inline(always)]
    pub fn ibena(&self) -> IbenaR {
        IbenaR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Slew Rate Control. . ."]
    #[inline(always)]
    pub fn slewrate(&self) -> SlewrateR {
        SlewrateR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Drive Selector. . ."]
    #[inline(always)]
    pub fn fulldrive(&self) -> FulldriveR {
        FulldriveR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Analog Mux Enable. . ."]
    #[inline(always)]
    pub fn amena(&self) -> AmenaR {
        AmenaR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Pseudo Output Drain Enable. . ."]
    #[inline(always)]
    pub fn odena(&self) -> OdenaR {
        OdenaR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Input Invert Enable. . ."]
    #[inline(always)]
    pub fn iiena(&self) -> IienaR {
        IienaR::new(((self.bits >> 11) & 1) != 0)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FC15_I2C_SDA")
            .field("fsel", &self.fsel())
            .field("pupdena", &self.pupdena())
            .field("pupdsel", &self.pupdsel())
            .field("ibena", &self.ibena())
            .field("slewrate", &self.slewrate())
            .field("fulldrive", &self.fulldrive())
            .field("amena", &self.amena())
            .field("odena", &self.odena())
            .field("iiena", &self.iiena())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:3 - Function Selector. . .(FSELs Sources can be found in the next several pages.)"]
    #[inline(always)]
    pub fn fsel(&mut self) -> FselW<Fc15I2cSdaSpec> {
        FselW::new(self, 0)
    }
    #[doc = "Bit 4 - Pullup / Pulldown Enable. . ."]
    #[inline(always)]
    pub fn pupdena(&mut self) -> PupdenaW<Fc15I2cSdaSpec> {
        PupdenaW::new(self, 4)
    }
    #[doc = "Bit 5 - Pullup or Pulldown Selector. . ."]
    #[inline(always)]
    pub fn pupdsel(&mut self) -> PupdselW<Fc15I2cSdaSpec> {
        PupdselW::new(self, 5)
    }
    #[doc = "Bit 6 - Input Buffer Enable. ."]
    #[inline(always)]
    pub fn ibena(&mut self) -> IbenaW<Fc15I2cSdaSpec> {
        IbenaW::new(self, 6)
    }
    #[doc = "Bit 7 - Slew Rate Control. . ."]
    #[inline(always)]
    pub fn slewrate(&mut self) -> SlewrateW<Fc15I2cSdaSpec> {
        SlewrateW::new(self, 7)
    }
    #[doc = "Bit 8 - Drive Selector. . ."]
    #[inline(always)]
    pub fn fulldrive(&mut self) -> FulldriveW<Fc15I2cSdaSpec> {
        FulldriveW::new(self, 8)
    }
    #[doc = "Bit 9 - Analog Mux Enable. . ."]
    #[inline(always)]
    pub fn amena(&mut self) -> AmenaW<Fc15I2cSdaSpec> {
        AmenaW::new(self, 9)
    }
    #[doc = "Bit 10 - Pseudo Output Drain Enable. . ."]
    #[inline(always)]
    pub fn odena(&mut self) -> OdenaW<Fc15I2cSdaSpec> {
        OdenaW::new(self, 10)
    }
    #[doc = "Bit 11 - Input Invert Enable. . ."]
    #[inline(always)]
    pub fn iiena(&mut self) -> IienaW<Fc15I2cSdaSpec> {
        IienaW::new(self, 11)
    }
}
#[doc = "Special Registers (No GPIO Function)\n\nYou can [`read`](crate::Reg::read) this register and get [`fc15_i2c_sda::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fc15_i2c_sda::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Fc15I2cSdaSpec;
impl crate::RegisterSpec for Fc15I2cSdaSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fc15_i2c_sda::R`](R) reader structure"]
impl crate::Readable for Fc15I2cSdaSpec {}
#[doc = "`write(|w| ..)` method takes [`fc15_i2c_sda::W`](W) writer structure"]
impl crate::Writable for Fc15I2cSdaSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FC15_I2C_SDA to value 0"]
impl crate::Resettable for Fc15I2cSdaSpec {
    const RESET_VALUE: u32 = 0;
}
#[doc = "Offset for FC15_I2C_SDA in IOPCTL"]
pub const FC15_I2C_SDA_IOPCTL_OFFSET: u32 = 0x404;

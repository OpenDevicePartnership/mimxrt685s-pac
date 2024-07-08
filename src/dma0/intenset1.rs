#[doc = "Register `INTENSET1` reader"]
pub type R = crate::R<Intenset1Spec>;
#[doc = "Register `INTENSET1` writer"]
pub type W = crate::W<Intenset1Spec>;
#[doc = "Interrupt Enable read and set for DMA channel 32.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Inten32 {
    #[doc = "0: The Interrupt for DMA channel 32 is disabled."]
    Disabled = 0,
    #[doc = "1: The Interrupt for DMA channel 32 is enabled."]
    Enabled = 1,
}
impl From<Inten32> for bool {
    #[inline(always)]
    fn from(variant: Inten32) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INTEN32` reader - Interrupt Enable read and set for DMA channel 32."]
pub type Inten32R = crate::BitReader<Inten32>;
impl Inten32R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Inten32 {
        match self.bits {
            false => Inten32::Disabled,
            true => Inten32::Enabled,
        }
    }
    #[doc = "The Interrupt for DMA channel 32 is disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Inten32::Disabled
    }
    #[doc = "The Interrupt for DMA channel 32 is enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Inten32::Enabled
    }
}
#[doc = "Field `INTEN32` writer - Interrupt Enable read and set for DMA channel 32."]
pub type Inten32W<'a, REG> = crate::BitWriter<'a, REG, Inten32>;
impl<'a, REG> Inten32W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The Interrupt for DMA channel 32 is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Inten32::Disabled)
    }
    #[doc = "The Interrupt for DMA channel 32 is enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Inten32::Enabled)
    }
}
#[doc = "Additional Interrupt Enable read and set bits for remaining DMA channels in the range 63 to 33. Any bits above the actually implemented channels are reserved.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum Inten63_33 {
    #[doc = "0: The Interrupt for the relevant DMA channel is disabled."]
    Disabled = 0,
    #[doc = "1: The Interrupt for the relevant DMA channel is enabled."]
    Enabled = 1,
}
impl From<Inten63_33> for u32 {
    #[inline(always)]
    fn from(variant: Inten63_33) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Inten63_33 {
    type Ux = u32;
}
impl crate::IsEnum for Inten63_33 {}
#[doc = "Field `INTEN63_33` reader - Additional Interrupt Enable read and set bits for remaining DMA channels in the range 63 to 33. Any bits above the actually implemented channels are reserved."]
pub type Inten63_33R = crate::FieldReader<Inten63_33>;
impl Inten63_33R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Inten63_33> {
        match self.bits {
            0 => Some(Inten63_33::Disabled),
            1 => Some(Inten63_33::Enabled),
            _ => None,
        }
    }
    #[doc = "The Interrupt for the relevant DMA channel is disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Inten63_33::Disabled
    }
    #[doc = "The Interrupt for the relevant DMA channel is enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Inten63_33::Enabled
    }
}
#[doc = "Field `INTEN63_33` writer - Additional Interrupt Enable read and set bits for remaining DMA channels in the range 63 to 33. Any bits above the actually implemented channels are reserved."]
pub type Inten63_33W<'a, REG> = crate::FieldWriter<'a, REG, 31, Inten63_33>;
impl<'a, REG> Inten63_33W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u32>,
{
    #[doc = "The Interrupt for the relevant DMA channel is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Inten63_33::Disabled)
    }
    #[doc = "The Interrupt for the relevant DMA channel is enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Inten63_33::Enabled)
    }
}
impl R {
    #[doc = "Bit 0 - Interrupt Enable read and set for DMA channel 32."]
    #[inline(always)]
    pub fn inten32(&self) -> Inten32R {
        Inten32R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:31 - Additional Interrupt Enable read and set bits for remaining DMA channels in the range 63 to 33. Any bits above the actually implemented channels are reserved."]
    #[inline(always)]
    pub fn inten63_33(&self) -> Inten63_33R {
        Inten63_33R::new((self.bits >> 1) & 0x7fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - Interrupt Enable read and set for DMA channel 32."]
    #[inline(always)]
    #[must_use]
    pub fn inten32(&mut self) -> Inten32W<Intenset1Spec> {
        Inten32W::new(self, 0)
    }
    #[doc = "Bits 1:31 - Additional Interrupt Enable read and set bits for remaining DMA channels in the range 63 to 33. Any bits above the actually implemented channels are reserved."]
    #[inline(always)]
    #[must_use]
    pub fn inten63_33(&mut self) -> Inten63_33W<Intenset1Spec> {
        Inten63_33W::new(self, 1)
    }
}
#[doc = "Interrupt Enable read and Set for all DMA channels.\n\nYou can [`read`](crate::Reg::read) this register and get [`intenset1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intenset1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Intenset1Spec;
impl crate::RegisterSpec for Intenset1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intenset1::R`](R) reader structure"]
impl crate::Readable for Intenset1Spec {}
#[doc = "`write(|w| ..)` method takes [`intenset1::W`](W) writer structure"]
impl crate::Writable for Intenset1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTENSET1 to value 0"]
impl crate::Resettable for Intenset1Spec {
    const RESET_VALUE: u32 = 0;
}

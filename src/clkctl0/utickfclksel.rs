#[doc = "Register `UTICKFCLKSEL` reader"]
pub type R = crate::R<UtickfclkselSpec>;
#[doc = "Register `UTICKFCLKSEL` writer"]
pub type W = crate::W<UtickfclkselSpec>;
#[doc = "uTICK Functional Clock Source Selection. .\n\nValue on reset: 7"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Sel {
    #[doc = "0: Low Power Oscillator Clock (LPOSC)."]
    Lposc = 0,
    #[doc = "7: None, this may be selected in order to reduce power when no output is needed."]
    None = 7,
}
impl From<Sel> for u8 {
    #[inline(always)]
    fn from(variant: Sel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Sel {
    type Ux = u8;
}
impl crate::IsEnum for Sel {}
#[doc = "Field `SEL` reader - uTICK Functional Clock Source Selection. ."]
pub type SelR = crate::FieldReader<Sel>;
impl SelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Sel> {
        match self.bits {
            0 => Some(Sel::Lposc),
            7 => Some(Sel::None),
            _ => None,
        }
    }
    #[doc = "Low Power Oscillator Clock (LPOSC)."]
    #[inline(always)]
    pub fn is_lposc(&self) -> bool {
        *self == Sel::Lposc
    }
    #[doc = "None, this may be selected in order to reduce power when no output is needed."]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == Sel::None
    }
}
#[doc = "Field `SEL` writer - uTICK Functional Clock Source Selection. ."]
pub type SelW<'a, REG> = crate::FieldWriter<'a, REG, 3, Sel>;
impl<'a, REG> SelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Low Power Oscillator Clock (LPOSC)."]
    #[inline(always)]
    pub fn lposc(self) -> &'a mut crate::W<REG> {
        self.variant(Sel::Lposc)
    }
    #[doc = "None, this may be selected in order to reduce power when no output is needed."]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(Sel::None)
    }
}
impl R {
    #[doc = "Bits 0:2 - uTICK Functional Clock Source Selection. ."]
    #[inline(always)]
    pub fn sel(&self) -> SelR {
        SelR::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - uTICK Functional Clock Source Selection. ."]
    #[inline(always)]
    #[must_use]
    pub fn sel(&mut self) -> SelW<UtickfclkselSpec> {
        SelW::new(self, 0)
    }
}
#[doc = "UTICK fclk selection\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`utickfclksel::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`utickfclksel::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UtickfclkselSpec;
impl crate::RegisterSpec for UtickfclkselSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`utickfclksel::R`](R) reader structure"]
impl crate::Readable for UtickfclkselSpec {}
#[doc = "`write(|w| ..)` method takes [`utickfclksel::W`](W) writer structure"]
impl crate::Writable for UtickfclkselSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets UTICKFCLKSEL to value 0x07"]
impl crate::Resettable for UtickfclkselSpec {
    const RESET_VALUE: u32 = 0x07;
}

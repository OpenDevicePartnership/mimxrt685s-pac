#[doc = "Register `FRG15CTL` reader"]
pub type R = crate::R<Frg15ctlSpec>;
#[doc = "Register `FRG15CTL` writer"]
pub type W = crate::W<Frg15ctlSpec>;
#[doc = "Field `DIV` reader - Denominator of the fractional divider. DIV is equal to the programmed value +1. Always set to 0xFF to use with the fractional baud rate generator."]
pub type DivR = crate::FieldReader;
#[doc = "Field `DIV` writer - Denominator of the fractional divider. DIV is equal to the programmed value +1. Always set to 0xFF to use with the fractional baud rate generator."]
pub type DivW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `MULT` reader - Numerator of the fractional divider. MULT is equal to the programmed value."]
pub type MultR = crate::FieldReader;
#[doc = "Field `MULT` writer - Numerator of the fractional divider. MULT is equal to the programmed value."]
pub type MultW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Denominator of the fractional divider. DIV is equal to the programmed value +1. Always set to 0xFF to use with the fractional baud rate generator."]
    #[inline(always)]
    pub fn div(&self) -> DivR {
        DivR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Numerator of the fractional divider. MULT is equal to the programmed value."]
    #[inline(always)]
    pub fn mult(&self) -> MultR {
        MultR::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Denominator of the fractional divider. DIV is equal to the programmed value +1. Always set to 0xFF to use with the fractional baud rate generator."]
    #[inline(always)]
    #[must_use]
    pub fn div(&mut self) -> DivW<Frg15ctlSpec> {
        DivW::new(self, 0)
    }
    #[doc = "Bits 8:15 - Numerator of the fractional divider. MULT is equal to the programmed value."]
    #[inline(always)]
    #[must_use]
    pub fn mult(&mut self) -> MultW<Frg15ctlSpec> {
        MultW::new(self, 8)
    }
}
#[doc = "FRG clock controller 15\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`frg15ctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`frg15ctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Frg15ctlSpec;
impl crate::RegisterSpec for Frg15ctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`frg15ctl::R`](R) reader structure"]
impl crate::Readable for Frg15ctlSpec {}
#[doc = "`write(|w| ..)` method takes [`frg15ctl::W`](W) writer structure"]
impl crate::Writable for Frg15ctlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FRG15CTL to value 0xff"]
impl crate::Resettable for Frg15ctlSpec {
    const RESET_VALUE: u32 = 0xff;
}

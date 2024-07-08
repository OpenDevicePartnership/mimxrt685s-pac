#[doc = "Register `RSTGT_W` writer"]
pub type W = crate::W<RstgtRstgtWSpec>;
#[doc = "Field `RSTGTN` writer - RSTGTN"]
pub type RstgtnW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `RSTGDP` writer - RSTGDP"]
pub type RstgdpW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl W {
    #[doc = "Bits 0:7 - RSTGTN"]
    #[inline(always)]
    #[must_use]
    pub fn rstgtn(&mut self) -> RstgtnW<RstgtRstgtWSpec> {
        RstgtnW::new(self, 0)
    }
    #[doc = "Bits 8:15 - RSTGDP"]
    #[inline(always)]
    #[must_use]
    pub fn rstgdp(&mut self) -> RstgdpW<RstgtRstgtWSpec> {
        RstgdpW::new(self, 8)
    }
}
#[doc = "Reset Gate Write\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rstgt_rstgt_w::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RstgtRstgtWSpec;
impl crate::RegisterSpec for RstgtRstgtWSpec {
    type Ux = u16;
}
#[doc = "`write(|w| ..)` method takes [`rstgt_rstgt_w::W`](W) writer structure"]
impl crate::Writable for RstgtRstgtWSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets RSTGT_W to value 0"]
impl crate::Resettable for RstgtRstgtWSpec {
    const RESET_VALUE: u16 = 0;
}

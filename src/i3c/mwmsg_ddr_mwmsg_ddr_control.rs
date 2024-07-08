#[doc = "Register `MWMSG_DDR_CONTROL` writer"]
pub type W = crate::W<MwmsgDdrMwmsgDdrControlSpec>;
#[doc = "Field `LEN` writer - Length of message"]
pub type LenW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `END` writer - End of message"]
pub type EndW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bits 0:9 - Length of message"]
    #[inline(always)]
    #[must_use]
    pub fn len(&mut self) -> LenW<MwmsgDdrMwmsgDdrControlSpec> {
        LenW::new(self, 0)
    }
    #[doc = "Bit 14 - End of message"]
    #[inline(always)]
    #[must_use]
    pub fn end(&mut self) -> EndW<MwmsgDdrMwmsgDdrControlSpec> {
        EndW::new(self, 14)
    }
}
#[doc = "Master Write Message in DDR mode\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mwmsg_ddr_mwmsg_ddr_control::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MwmsgDdrMwmsgDdrControlSpec;
impl crate::RegisterSpec for MwmsgDdrMwmsgDdrControlSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`mwmsg_ddr_mwmsg_ddr_control::W`](W) writer structure"]
impl crate::Writable for MwmsgDdrMwmsgDdrControlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MWMSG_DDR_CONTROL to value 0"]
impl crate::Resettable for MwmsgDdrMwmsgDdrControlSpec {
    const RESET_VALUE: u32 = 0;
}

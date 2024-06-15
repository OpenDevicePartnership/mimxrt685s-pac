#[doc = "Register `AUDIOPLL0LOCKTIMEDIV2` reader"]
pub type R = crate::R<Audiopll0locktimediv2Spec>;
#[doc = "Register `AUDIOPLL0LOCKTIMEDIV2` writer"]
pub type W = crate::W<Audiopll0locktimediv2Spec>;
#[doc = "Field `LOCKTIMEDIV2` reader - AUDIOPLL0 Lock Time Divide by 2: Programmed lock time is in uS (micro-seconds) and is programmed as half the actual lock time value."]
pub type Locktimediv2R = crate::FieldReader<u16>;
#[doc = "Field `LOCKTIMEDIV2` writer - AUDIOPLL0 Lock Time Divide by 2: Programmed lock time is in uS (micro-seconds) and is programmed as half the actual lock time value."]
pub type Locktimediv2W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - AUDIOPLL0 Lock Time Divide by 2: Programmed lock time is in uS (micro-seconds) and is programmed as half the actual lock time value."]
    #[inline(always)]
    pub fn locktimediv2(&self) -> Locktimediv2R {
        Locktimediv2R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - AUDIOPLL0 Lock Time Divide by 2: Programmed lock time is in uS (micro-seconds) and is programmed as half the actual lock time value."]
    #[inline(always)]
    #[must_use]
    pub fn locktimediv2(&mut self) -> Locktimediv2W<Audiopll0locktimediv2Spec> {
        Locktimediv2W::new(self, 0)
    }
}
#[doc = "audio pll0 lock time\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`audiopll0locktimediv2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`audiopll0locktimediv2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Audiopll0locktimediv2Spec;
impl crate::RegisterSpec for Audiopll0locktimediv2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`audiopll0locktimediv2::R`](R) reader structure"]
impl crate::Readable for Audiopll0locktimediv2Spec {}
#[doc = "`write(|w| ..)` method takes [`audiopll0locktimediv2::W`](W) writer structure"]
impl crate::Writable for Audiopll0locktimediv2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AUDIOPLL0LOCKTIMEDIV2 to value 0xcafe"]
impl crate::Resettable for Audiopll0locktimediv2Spec {
    const RESET_VALUE: u32 = 0xcafe;
}

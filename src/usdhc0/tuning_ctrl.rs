#[doc = "Register `TUNING_CTRL` reader"]
pub type R = crate::R<TuningCtrlSpec>;
#[doc = "Register `TUNING_CTRL` writer"]
pub type W = crate::W<TuningCtrlSpec>;
#[doc = "Field `TUNING_START_TAP` reader - TUNING_START_TAP"]
pub type TuningStartTapR = crate::FieldReader;
#[doc = "Field `TUNING_START_TAP` writer - TUNING_START_TAP"]
pub type TuningStartTapW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `TUNING_COUNTER` reader - TUNING_COUNTER"]
pub type TuningCounterR = crate::FieldReader;
#[doc = "Field `TUNING_COUNTER` writer - TUNING_COUNTER"]
pub type TuningCounterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `TUNING_STEP` reader - TUNING_STEP"]
pub type TuningStepR = crate::FieldReader;
#[doc = "Field `TUNING_STEP` writer - TUNING_STEP"]
pub type TuningStepW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `TUNING_WINDOW` reader - TUNING_WINDOW"]
pub type TuningWindowR = crate::FieldReader;
#[doc = "Field `TUNING_WINDOW` writer - TUNING_WINDOW"]
pub type TuningWindowW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `STD_TUNING_EN` reader - STD_TUNING_EN"]
pub type StdTuningEnR = crate::BitReader;
#[doc = "Field `STD_TUNING_EN` writer - STD_TUNING_EN"]
pub type StdTuningEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - TUNING_START_TAP"]
    #[inline(always)]
    pub fn tuning_start_tap(&self) -> TuningStartTapR {
        TuningStartTapR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - TUNING_COUNTER"]
    #[inline(always)]
    pub fn tuning_counter(&self) -> TuningCounterR {
        TuningCounterR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:18 - TUNING_STEP"]
    #[inline(always)]
    pub fn tuning_step(&self) -> TuningStepR {
        TuningStepR::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 20:22 - TUNING_WINDOW"]
    #[inline(always)]
    pub fn tuning_window(&self) -> TuningWindowR {
        TuningWindowR::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bit 24 - STD_TUNING_EN"]
    #[inline(always)]
    pub fn std_tuning_en(&self) -> StdTuningEnR {
        StdTuningEnR::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - TUNING_START_TAP"]
    #[inline(always)]
    #[must_use]
    pub fn tuning_start_tap(&mut self) -> TuningStartTapW<TuningCtrlSpec> {
        TuningStartTapW::new(self, 0)
    }
    #[doc = "Bits 8:15 - TUNING_COUNTER"]
    #[inline(always)]
    #[must_use]
    pub fn tuning_counter(&mut self) -> TuningCounterW<TuningCtrlSpec> {
        TuningCounterW::new(self, 8)
    }
    #[doc = "Bits 16:18 - TUNING_STEP"]
    #[inline(always)]
    #[must_use]
    pub fn tuning_step(&mut self) -> TuningStepW<TuningCtrlSpec> {
        TuningStepW::new(self, 16)
    }
    #[doc = "Bits 20:22 - TUNING_WINDOW"]
    #[inline(always)]
    #[must_use]
    pub fn tuning_window(&mut self) -> TuningWindowW<TuningCtrlSpec> {
        TuningWindowW::new(self, 20)
    }
    #[doc = "Bit 24 - STD_TUNING_EN"]
    #[inline(always)]
    #[must_use]
    pub fn std_tuning_en(&mut self) -> StdTuningEnW<TuningCtrlSpec> {
        StdTuningEnW::new(self, 24)
    }
}
#[doc = "Tuning Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tuning_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tuning_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TuningCtrlSpec;
impl crate::RegisterSpec for TuningCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tuning_ctrl::R`](R) reader structure"]
impl crate::Readable for TuningCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`tuning_ctrl::W`](W) writer structure"]
impl crate::Writable for TuningCtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TUNING_CTRL to value 0x0021_2800"]
impl crate::Resettable for TuningCtrlSpec {
    const RESET_VALUE: u32 = 0x0021_2800;
}

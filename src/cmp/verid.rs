#[doc = "Register `VERID` reader"]
pub type R = crate::R<VeridSpec>;
#[doc = "Field `FEATURE` reader - Feature Specification Number. This read only filed returns the feature set number."]
pub type FeatureR = crate::FieldReader<u16>;
#[doc = "Field `MINOR` reader - Minor Version Number. This read only field returns the minor version number for the module specification."]
pub type MinorR = crate::FieldReader;
#[doc = "Field `MAJOR` reader - Major Version Number. This read only field returns the major version number for the module specification."]
pub type MajorR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:15 - Feature Specification Number. This read only filed returns the feature set number."]
    #[inline(always)]
    pub fn feature(&self) -> FeatureR {
        FeatureR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:23 - Minor Version Number. This read only field returns the minor version number for the module specification."]
    #[inline(always)]
    pub fn minor(&self) -> MinorR {
        MinorR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Major Version Number. This read only field returns the major version number for the module specification."]
    #[inline(always)]
    pub fn major(&self) -> MajorR {
        MajorR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[cfg(feature = "debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("VERID")
            .field("feature", &self.feature())
            .field("minor", &self.minor())
            .field("major", &self.major())
            .finish()
    }
}
#[doc = "Version ID Register\n\nYou can [`read`](crate::Reg::read) this register and get [`verid::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VeridSpec;
impl crate::RegisterSpec for VeridSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`verid::R`](R) reader structure"]
impl crate::Readable for VeridSpec {}
#[doc = "`reset()` method sets VERID to value 0x0100_0000"]
impl crate::Resettable for VeridSpec {
    const RESET_VALUE: u32 = 0x0100_0000;
}

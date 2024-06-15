#[doc = "Register `VID2` reader"]
pub type R = crate::R<Vid2Spec>;
#[doc = "Shows the IP's Configuaration options for the TRNG.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ConfigOpt {
    #[doc = "0: TRNG_CONFIG_OPT for TRNG."]
    ConfigOpt0 = 0,
}
impl From<ConfigOpt> for u8 {
    #[inline(always)]
    fn from(variant: ConfigOpt) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ConfigOpt {
    type Ux = u8;
}
impl crate::IsEnum for ConfigOpt {}
#[doc = "Field `CONFIG_OPT` reader - Shows the IP's Configuaration options for the TRNG."]
pub type ConfigOptR = crate::FieldReader<ConfigOpt>;
impl ConfigOptR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<ConfigOpt> {
        match self.bits {
            0 => Some(ConfigOpt::ConfigOpt0),
            _ => None,
        }
    }
    #[doc = "TRNG_CONFIG_OPT for TRNG."]
    #[inline(always)]
    pub fn is_config_opt_0(&self) -> bool {
        *self == ConfigOpt::ConfigOpt0
    }
}
#[doc = "Shows the IP's ECO revision of the TRNG.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EcoRev {
    #[doc = "0: TRNG_ECO_REV for TRNG."]
    EcoRev0 = 0,
}
impl From<EcoRev> for u8 {
    #[inline(always)]
    fn from(variant: EcoRev) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for EcoRev {
    type Ux = u8;
}
impl crate::IsEnum for EcoRev {}
#[doc = "Field `ECO_REV` reader - Shows the IP's ECO revision of the TRNG."]
pub type EcoRevR = crate::FieldReader<EcoRev>;
impl EcoRevR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<EcoRev> {
        match self.bits {
            0 => Some(EcoRev::EcoRev0),
            _ => None,
        }
    }
    #[doc = "TRNG_ECO_REV for TRNG."]
    #[inline(always)]
    pub fn is_eco_rev_0(&self) -> bool {
        *self == EcoRev::EcoRev0
    }
}
#[doc = "Shows the integration options for the TRNG.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum IntgOpt {
    #[doc = "0: INTG_OPT for TRNG."]
    IntgOpt0 = 0,
}
impl From<IntgOpt> for u8 {
    #[inline(always)]
    fn from(variant: IntgOpt) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for IntgOpt {
    type Ux = u8;
}
impl crate::IsEnum for IntgOpt {}
#[doc = "Field `INTG_OPT` reader - Shows the integration options for the TRNG."]
pub type IntgOptR = crate::FieldReader<IntgOpt>;
impl IntgOptR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<IntgOpt> {
        match self.bits {
            0 => Some(IntgOpt::IntgOpt0),
            _ => None,
        }
    }
    #[doc = "INTG_OPT for TRNG."]
    #[inline(always)]
    pub fn is_intg_opt_0(&self) -> bool {
        *self == IntgOpt::IntgOpt0
    }
}
#[doc = "Shows the compile options for the TRNG.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Era {
    #[doc = "0: COMPILE_OPT for TRNG."]
    Era0 = 0,
}
impl From<Era> for u8 {
    #[inline(always)]
    fn from(variant: Era) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Era {
    type Ux = u8;
}
impl crate::IsEnum for Era {}
#[doc = "Field `ERA` reader - Shows the compile options for the TRNG."]
pub type EraR = crate::FieldReader<Era>;
impl EraR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Era> {
        match self.bits {
            0 => Some(Era::Era0),
            _ => None,
        }
    }
    #[doc = "COMPILE_OPT for TRNG."]
    #[inline(always)]
    pub fn is_era_0(&self) -> bool {
        *self == Era::Era0
    }
}
impl R {
    #[doc = "Bits 0:7 - Shows the IP's Configuaration options for the TRNG."]
    #[inline(always)]
    pub fn config_opt(&self) -> ConfigOptR {
        ConfigOptR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Shows the IP's ECO revision of the TRNG."]
    #[inline(always)]
    pub fn eco_rev(&self) -> EcoRevR {
        EcoRevR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Shows the integration options for the TRNG."]
    #[inline(always)]
    pub fn intg_opt(&self) -> IntgOptR {
        IntgOptR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Shows the compile options for the TRNG."]
    #[inline(always)]
    pub fn era(&self) -> EraR {
        EraR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[doc = "Version ID Register (LS)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vid2::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Vid2Spec;
impl crate::RegisterSpec for Vid2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vid2::R`](R) reader structure"]
impl crate::Readable for Vid2Spec {}
#[doc = "`reset()` method sets VID2 to value 0"]
impl crate::Resettable for Vid2Spec {
    const RESET_VALUE: u32 = 0;
}

#[doc = "Register `PDRUNCFG0_SET` writer"]
pub type W = crate::W<Pdruncfg0SetSpec>;
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PmicMode0 {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Sets the PDRUNCFG0 Bit"]
    SetPdruncfg0 = 1,
}
impl From<PmicMode0> for bool {
    #[inline(always)]
    fn from(variant: PmicMode0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PMIC_MODE0` writer - no description available"]
pub type PmicMode0W<'a, REG> = crate::BitWriter<'a, REG, PmicMode0>;
impl<'a, REG> PmicMode0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(PmicMode0::NoEffect)
    }
    #[doc = "Sets the PDRUNCFG0 Bit"]
    #[inline(always)]
    pub fn set_pdruncfg0(self) -> &'a mut crate::W<REG> {
        self.variant(PmicMode0::SetPdruncfg0)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PmicMode1 {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Sets the PDRUNCFG0 Bit"]
    SetPdruncfg0 = 1,
}
impl From<PmicMode1> for bool {
    #[inline(always)]
    fn from(variant: PmicMode1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PMIC_MODE1` writer - no description available"]
pub type PmicMode1W<'a, REG> = crate::BitWriter<'a, REG, PmicMode1>;
impl<'a, REG> PmicMode1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(PmicMode1::NoEffect)
    }
    #[doc = "Sets the PDRUNCFG0 Bit"]
    #[inline(always)]
    pub fn set_pdruncfg0(self) -> &'a mut crate::W<REG> {
        self.variant(PmicMode1::SetPdruncfg0)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VddcoreregLp {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Sets the PDRUNCFG0 Bit"]
    SetPdruncfg0 = 1,
}
impl From<VddcoreregLp> for bool {
    #[inline(always)]
    fn from(variant: VddcoreregLp) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VDDCOREREG_LP` writer - no description available"]
pub type VddcoreregLpW<'a, REG> = crate::BitWriter<'a, REG, VddcoreregLp>;
impl<'a, REG> VddcoreregLpW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(VddcoreregLp::NoEffect)
    }
    #[doc = "Sets the PDRUNCFG0 Bit"]
    #[inline(always)]
    pub fn set_pdruncfg0(self) -> &'a mut crate::W<REG> {
        self.variant(VddcoreregLp::SetPdruncfg0)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PmcrefLp {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Sets the PDRUNCFG0 Bit"]
    SetPdruncfg0 = 1,
}
impl From<PmcrefLp> for bool {
    #[inline(always)]
    fn from(variant: PmcrefLp) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PMCREF_LP` writer - no description available"]
pub type PmcrefLpW<'a, REG> = crate::BitWriter<'a, REG, PmcrefLp>;
impl<'a, REG> PmcrefLpW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(PmcrefLp::NoEffect)
    }
    #[doc = "Sets the PDRUNCFG0 Bit"]
    #[inline(always)]
    pub fn set_pdruncfg0(self) -> &'a mut crate::W<REG> {
        self.variant(PmcrefLp::SetPdruncfg0)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hvd1v8Pd {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Sets the PDRUNCFG0 Bit"]
    SetPdruncfg0 = 1,
}
impl From<Hvd1v8Pd> for bool {
    #[inline(always)]
    fn from(variant: Hvd1v8Pd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HVD1V8_PD` writer - no description available"]
pub type Hvd1v8PdW<'a, REG> = crate::BitWriter<'a, REG, Hvd1v8Pd>;
impl<'a, REG> Hvd1v8PdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Hvd1v8Pd::NoEffect)
    }
    #[doc = "Sets the PDRUNCFG0 Bit"]
    #[inline(always)]
    pub fn set_pdruncfg0(self) -> &'a mut crate::W<REG> {
        self.variant(Hvd1v8Pd::SetPdruncfg0)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PorcoreLp {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Sets the PDRUNCFG0 Bit"]
    SetPdruncfg0 = 1,
}
impl From<PorcoreLp> for bool {
    #[inline(always)]
    fn from(variant: PorcoreLp) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PORCORE_LP` writer - no description available"]
pub type PorcoreLpW<'a, REG> = crate::BitWriter<'a, REG, PorcoreLp>;
impl<'a, REG> PorcoreLpW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(PorcoreLp::NoEffect)
    }
    #[doc = "Sets the PDRUNCFG0 Bit"]
    #[inline(always)]
    pub fn set_pdruncfg0(self) -> &'a mut crate::W<REG> {
        self.variant(PorcoreLp::SetPdruncfg0)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LvdcoreLp {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Sets the PDRUNCFG0 Bit"]
    SetPdruncfg0 = 1,
}
impl From<LvdcoreLp> for bool {
    #[inline(always)]
    fn from(variant: LvdcoreLp) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LVDCORE_LP` writer - no description available"]
pub type LvdcoreLpW<'a, REG> = crate::BitWriter<'a, REG, LvdcoreLp>;
impl<'a, REG> LvdcoreLpW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(LvdcoreLp::NoEffect)
    }
    #[doc = "Sets the PDRUNCFG0 Bit"]
    #[inline(always)]
    pub fn set_pdruncfg0(self) -> &'a mut crate::W<REG> {
        self.variant(LvdcoreLp::SetPdruncfg0)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HvdcorePd {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Sets the PDRUNCFG0 Bit"]
    SetPdruncfg0 = 1,
}
impl From<HvdcorePd> for bool {
    #[inline(always)]
    fn from(variant: HvdcorePd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HVDCORE_PD` writer - no description available"]
pub type HvdcorePdW<'a, REG> = crate::BitWriter<'a, REG, HvdcorePd>;
impl<'a, REG> HvdcorePdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(HvdcorePd::NoEffect)
    }
    #[doc = "Sets the PDRUNCFG0 Bit"]
    #[inline(always)]
    pub fn set_pdruncfg0(self) -> &'a mut crate::W<REG> {
        self.variant(HvdcorePd::SetPdruncfg0)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RbbPd {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Sets the PDRUNCFG0 Bit"]
    SetPdruncfg0 = 1,
}
impl From<RbbPd> for bool {
    #[inline(always)]
    fn from(variant: RbbPd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RBB_PD` writer - no description available"]
pub type RbbPdW<'a, REG> = crate::BitWriter<'a, REG, RbbPd>;
impl<'a, REG> RbbPdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(RbbPd::NoEffect)
    }
    #[doc = "Sets the PDRUNCFG0 Bit"]
    #[inline(always)]
    pub fn set_pdruncfg0(self) -> &'a mut crate::W<REG> {
        self.variant(RbbPd::SetPdruncfg0)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FbbPd {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Sets the PDRUNCFG0 Bit"]
    SetPdruncfg0 = 1,
}
impl From<FbbPd> for bool {
    #[inline(always)]
    fn from(variant: FbbPd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FBB_PD` writer - no description available"]
pub type FbbPdW<'a, REG> = crate::BitWriter<'a, REG, FbbPd>;
impl<'a, REG> FbbPdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(FbbPd::NoEffect)
    }
    #[doc = "Sets the PDRUNCFG0 Bit"]
    #[inline(always)]
    pub fn set_pdruncfg0(self) -> &'a mut crate::W<REG> {
        self.variant(FbbPd::SetPdruncfg0)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SysxtalPd {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Sets the PDRUNCFG0 Bit"]
    SetPdruncfg0 = 1,
}
impl From<SysxtalPd> for bool {
    #[inline(always)]
    fn from(variant: SysxtalPd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SYSXTAL_PD` writer - no description available"]
pub type SysxtalPdW<'a, REG> = crate::BitWriter<'a, REG, SysxtalPd>;
impl<'a, REG> SysxtalPdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(SysxtalPd::NoEffect)
    }
    #[doc = "Sets the PDRUNCFG0 Bit"]
    #[inline(always)]
    pub fn set_pdruncfg0(self) -> &'a mut crate::W<REG> {
        self.variant(SysxtalPd::SetPdruncfg0)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LposcPd {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Sets the PDRUNCFG0 Bit"]
    SetPdruncfg0 = 1,
}
impl From<LposcPd> for bool {
    #[inline(always)]
    fn from(variant: LposcPd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LPOSC_PD` writer - no description available"]
pub type LposcPdW<'a, REG> = crate::BitWriter<'a, REG, LposcPd>;
impl<'a, REG> LposcPdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(LposcPd::NoEffect)
    }
    #[doc = "Sets the PDRUNCFG0 Bit"]
    #[inline(always)]
    pub fn set_pdruncfg0(self) -> &'a mut crate::W<REG> {
        self.variant(LposcPd::SetPdruncfg0)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SfroPd {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Sets the PDRUNCFG0 Bit"]
    SetPdruncfg0 = 1,
}
impl From<SfroPd> for bool {
    #[inline(always)]
    fn from(variant: SfroPd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SFRO_PD` writer - no description available"]
pub type SfroPdW<'a, REG> = crate::BitWriter<'a, REG, SfroPd>;
impl<'a, REG> SfroPdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(SfroPd::NoEffect)
    }
    #[doc = "Sets the PDRUNCFG0 Bit"]
    #[inline(always)]
    pub fn set_pdruncfg0(self) -> &'a mut crate::W<REG> {
        self.variant(SfroPd::SetPdruncfg0)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FfroPd {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Sets the PDRUNCFG0 Bit"]
    SetPdruncfg0 = 1,
}
impl From<FfroPd> for bool {
    #[inline(always)]
    fn from(variant: FfroPd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FFRO_PD` writer - no description available"]
pub type FfroPdW<'a, REG> = crate::BitWriter<'a, REG, FfroPd>;
impl<'a, REG> FfroPdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(FfroPd::NoEffect)
    }
    #[doc = "Sets the PDRUNCFG0 Bit"]
    #[inline(always)]
    pub fn set_pdruncfg0(self) -> &'a mut crate::W<REG> {
        self.variant(FfroPd::SetPdruncfg0)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SyspllldoPd {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Sets the PDRUNCFG0 Bit"]
    SetPdruncfg0 = 1,
}
impl From<SyspllldoPd> for bool {
    #[inline(always)]
    fn from(variant: SyspllldoPd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SYSPLLLDO_PD` writer - no description available"]
pub type SyspllldoPdW<'a, REG> = crate::BitWriter<'a, REG, SyspllldoPd>;
impl<'a, REG> SyspllldoPdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(SyspllldoPd::NoEffect)
    }
    #[doc = "Sets the PDRUNCFG0 Bit"]
    #[inline(always)]
    pub fn set_pdruncfg0(self) -> &'a mut crate::W<REG> {
        self.variant(SyspllldoPd::SetPdruncfg0)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SyspllanaPd {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Sets the PDRUNCFG0 Bit"]
    SetPdruncfg0 = 1,
}
impl From<SyspllanaPd> for bool {
    #[inline(always)]
    fn from(variant: SyspllanaPd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SYSPLLANA_PD` writer - no description available"]
pub type SyspllanaPdW<'a, REG> = crate::BitWriter<'a, REG, SyspllanaPd>;
impl<'a, REG> SyspllanaPdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(SyspllanaPd::NoEffect)
    }
    #[doc = "Sets the PDRUNCFG0 Bit"]
    #[inline(always)]
    pub fn set_pdruncfg0(self) -> &'a mut crate::W<REG> {
        self.variant(SyspllanaPd::SetPdruncfg0)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AudpllldoPd {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Sets the PDRUNCFG0 Bit"]
    SetPdruncfg0 = 1,
}
impl From<AudpllldoPd> for bool {
    #[inline(always)]
    fn from(variant: AudpllldoPd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AUDPLLLDO_PD` writer - no description available"]
pub type AudpllldoPdW<'a, REG> = crate::BitWriter<'a, REG, AudpllldoPd>;
impl<'a, REG> AudpllldoPdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(AudpllldoPd::NoEffect)
    }
    #[doc = "Sets the PDRUNCFG0 Bit"]
    #[inline(always)]
    pub fn set_pdruncfg0(self) -> &'a mut crate::W<REG> {
        self.variant(AudpllldoPd::SetPdruncfg0)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AudpllanaPd {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Sets the PDRUNCFG0 Bit"]
    SetPdruncfg0 = 1,
}
impl From<AudpllanaPd> for bool {
    #[inline(always)]
    fn from(variant: AudpllanaPd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AUDPLLANA_PD` writer - no description available"]
pub type AudpllanaPdW<'a, REG> = crate::BitWriter<'a, REG, AudpllanaPd>;
impl<'a, REG> AudpllanaPdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(AudpllanaPd::NoEffect)
    }
    #[doc = "Sets the PDRUNCFG0 Bit"]
    #[inline(always)]
    pub fn set_pdruncfg0(self) -> &'a mut crate::W<REG> {
        self.variant(AudpllanaPd::SetPdruncfg0)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AdcPd {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Sets the PDRUNCFG0 Bit"]
    SetPdruncfg0 = 1,
}
impl From<AdcPd> for bool {
    #[inline(always)]
    fn from(variant: AdcPd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADC_PD` writer - no description available"]
pub type AdcPdW<'a, REG> = crate::BitWriter<'a, REG, AdcPd>;
impl<'a, REG> AdcPdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(AdcPd::NoEffect)
    }
    #[doc = "Sets the PDRUNCFG0 Bit"]
    #[inline(always)]
    pub fn set_pdruncfg0(self) -> &'a mut crate::W<REG> {
        self.variant(AdcPd::SetPdruncfg0)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AdcLp {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Sets the PDRUNCFG0 Bit"]
    SetPdruncfg0 = 1,
}
impl From<AdcLp> for bool {
    #[inline(always)]
    fn from(variant: AdcLp) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADC_LP` writer - no description available"]
pub type AdcLpW<'a, REG> = crate::BitWriter<'a, REG, AdcLp>;
impl<'a, REG> AdcLpW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(AdcLp::NoEffect)
    }
    #[doc = "Sets the PDRUNCFG0 Bit"]
    #[inline(always)]
    pub fn set_pdruncfg0(self) -> &'a mut crate::W<REG> {
        self.variant(AdcLp::SetPdruncfg0)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AdctempsnsPd {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Sets the PDRUNCFG0 Bit"]
    SetPdruncfg0 = 1,
}
impl From<AdctempsnsPd> for bool {
    #[inline(always)]
    fn from(variant: AdctempsnsPd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADCTEMPSNS_PD` writer - no description available"]
pub type AdctempsnsPdW<'a, REG> = crate::BitWriter<'a, REG, AdctempsnsPd>;
impl<'a, REG> AdctempsnsPdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(AdctempsnsPd::NoEffect)
    }
    #[doc = "Sets the PDRUNCFG0 Bit"]
    #[inline(always)]
    pub fn set_pdruncfg0(self) -> &'a mut crate::W<REG> {
        self.variant(AdctempsnsPd::SetPdruncfg0)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AcmpPd {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Sets the PDRUNCFG0 Bit"]
    SetPdruncfg0 = 1,
}
impl From<AcmpPd> for bool {
    #[inline(always)]
    fn from(variant: AcmpPd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ACMP_PD` writer - no description available"]
pub type AcmpPdW<'a, REG> = crate::BitWriter<'a, REG, AcmpPd>;
impl<'a, REG> AcmpPdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(AcmpPd::NoEffect)
    }
    #[doc = "Sets the PDRUNCFG0 Bit"]
    #[inline(always)]
    pub fn set_pdruncfg0(self) -> &'a mut crate::W<REG> {
        self.variant(AcmpPd::SetPdruncfg0)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hspad0VdetLp {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Sets the PDRUNCFG0 Bit"]
    SetPdruncfg0 = 1,
}
impl From<Hspad0VdetLp> for bool {
    #[inline(always)]
    fn from(variant: Hspad0VdetLp) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HSPAD0_VDET_LP` writer - no description available"]
pub type Hspad0VdetLpW<'a, REG> = crate::BitWriter<'a, REG, Hspad0VdetLp>;
impl<'a, REG> Hspad0VdetLpW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Hspad0VdetLp::NoEffect)
    }
    #[doc = "Sets the PDRUNCFG0 Bit"]
    #[inline(always)]
    pub fn set_pdruncfg0(self) -> &'a mut crate::W<REG> {
        self.variant(Hspad0VdetLp::SetPdruncfg0)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hspad0RefPd {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Sets the PDRUNCFG0 Bit"]
    SetPdruncfg0 = 1,
}
impl From<Hspad0RefPd> for bool {
    #[inline(always)]
    fn from(variant: Hspad0RefPd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HSPAD0_REF_PD` writer - no description available"]
pub type Hspad0RefPdW<'a, REG> = crate::BitWriter<'a, REG, Hspad0RefPd>;
impl<'a, REG> Hspad0RefPdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Hspad0RefPd::NoEffect)
    }
    #[doc = "Sets the PDRUNCFG0 Bit"]
    #[inline(always)]
    pub fn set_pdruncfg0(self) -> &'a mut crate::W<REG> {
        self.variant(Hspad0RefPd::SetPdruncfg0)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hspad2VdetLp {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Sets the PDRUNCFG0 Bit"]
    SetPdruncfg0 = 1,
}
impl From<Hspad2VdetLp> for bool {
    #[inline(always)]
    fn from(variant: Hspad2VdetLp) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HSPAD2_VDET_LP` writer - no description available"]
pub type Hspad2VdetLpW<'a, REG> = crate::BitWriter<'a, REG, Hspad2VdetLp>;
impl<'a, REG> Hspad2VdetLpW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Hspad2VdetLp::NoEffect)
    }
    #[doc = "Sets the PDRUNCFG0 Bit"]
    #[inline(always)]
    pub fn set_pdruncfg0(self) -> &'a mut crate::W<REG> {
        self.variant(Hspad2VdetLp::SetPdruncfg0)
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hspad2RefPd {
    #[doc = "0: No effect"]
    NoEffect = 0,
    #[doc = "1: Sets the PDRUNCFG0 Bit"]
    SetPdruncfg0 = 1,
}
impl From<Hspad2RefPd> for bool {
    #[inline(always)]
    fn from(variant: Hspad2RefPd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HSPAD2_REF_PD` writer - no description available"]
pub type Hspad2RefPdW<'a, REG> = crate::BitWriter<'a, REG, Hspad2RefPd>;
impl<'a, REG> Hspad2RefPdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut crate::W<REG> {
        self.variant(Hspad2RefPd::NoEffect)
    }
    #[doc = "Sets the PDRUNCFG0 Bit"]
    #[inline(always)]
    pub fn set_pdruncfg0(self) -> &'a mut crate::W<REG> {
        self.variant(Hspad2RefPd::SetPdruncfg0)
    }
}
impl W {
    #[doc = "Bit 1 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn pmic_mode0(&mut self) -> PmicMode0W<Pdruncfg0SetSpec> {
        PmicMode0W::new(self, 1)
    }
    #[doc = "Bit 2 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn pmic_mode1(&mut self) -> PmicMode1W<Pdruncfg0SetSpec> {
        PmicMode1W::new(self, 2)
    }
    #[doc = "Bit 4 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn vddcorereg_lp(&mut self) -> VddcoreregLpW<Pdruncfg0SetSpec> {
        VddcoreregLpW::new(self, 4)
    }
    #[doc = "Bit 6 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn pmcref_lp(&mut self) -> PmcrefLpW<Pdruncfg0SetSpec> {
        PmcrefLpW::new(self, 6)
    }
    #[doc = "Bit 7 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn hvd1v8_pd(&mut self) -> Hvd1v8PdW<Pdruncfg0SetSpec> {
        Hvd1v8PdW::new(self, 7)
    }
    #[doc = "Bit 8 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn porcore_lp(&mut self) -> PorcoreLpW<Pdruncfg0SetSpec> {
        PorcoreLpW::new(self, 8)
    }
    #[doc = "Bit 9 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn lvdcore_lp(&mut self) -> LvdcoreLpW<Pdruncfg0SetSpec> {
        LvdcoreLpW::new(self, 9)
    }
    #[doc = "Bit 10 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn hvdcore_pd(&mut self) -> HvdcorePdW<Pdruncfg0SetSpec> {
        HvdcorePdW::new(self, 10)
    }
    #[doc = "Bit 11 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn rbb_pd(&mut self) -> RbbPdW<Pdruncfg0SetSpec> {
        RbbPdW::new(self, 11)
    }
    #[doc = "Bit 12 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn fbb_pd(&mut self) -> FbbPdW<Pdruncfg0SetSpec> {
        FbbPdW::new(self, 12)
    }
    #[doc = "Bit 13 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn sysxtal_pd(&mut self) -> SysxtalPdW<Pdruncfg0SetSpec> {
        SysxtalPdW::new(self, 13)
    }
    #[doc = "Bit 14 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn lposc_pd(&mut self) -> LposcPdW<Pdruncfg0SetSpec> {
        LposcPdW::new(self, 14)
    }
    #[doc = "Bit 15 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn sfro_pd(&mut self) -> SfroPdW<Pdruncfg0SetSpec> {
        SfroPdW::new(self, 15)
    }
    #[doc = "Bit 16 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn ffro_pd(&mut self) -> FfroPdW<Pdruncfg0SetSpec> {
        FfroPdW::new(self, 16)
    }
    #[doc = "Bit 17 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn syspllldo_pd(&mut self) -> SyspllldoPdW<Pdruncfg0SetSpec> {
        SyspllldoPdW::new(self, 17)
    }
    #[doc = "Bit 18 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn syspllana_pd(&mut self) -> SyspllanaPdW<Pdruncfg0SetSpec> {
        SyspllanaPdW::new(self, 18)
    }
    #[doc = "Bit 19 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn audpllldo_pd(&mut self) -> AudpllldoPdW<Pdruncfg0SetSpec> {
        AudpllldoPdW::new(self, 19)
    }
    #[doc = "Bit 20 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn audpllana_pd(&mut self) -> AudpllanaPdW<Pdruncfg0SetSpec> {
        AudpllanaPdW::new(self, 20)
    }
    #[doc = "Bit 21 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn adc_pd(&mut self) -> AdcPdW<Pdruncfg0SetSpec> {
        AdcPdW::new(self, 21)
    }
    #[doc = "Bit 22 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn adc_lp(&mut self) -> AdcLpW<Pdruncfg0SetSpec> {
        AdcLpW::new(self, 22)
    }
    #[doc = "Bit 23 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn adctempsns_pd(&mut self) -> AdctempsnsPdW<Pdruncfg0SetSpec> {
        AdctempsnsPdW::new(self, 23)
    }
    #[doc = "Bit 25 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn acmp_pd(&mut self) -> AcmpPdW<Pdruncfg0SetSpec> {
        AcmpPdW::new(self, 25)
    }
    #[doc = "Bit 26 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn hspad0_vdet_lp(&mut self) -> Hspad0VdetLpW<Pdruncfg0SetSpec> {
        Hspad0VdetLpW::new(self, 26)
    }
    #[doc = "Bit 27 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn hspad0_ref_pd(&mut self) -> Hspad0RefPdW<Pdruncfg0SetSpec> {
        Hspad0RefPdW::new(self, 27)
    }
    #[doc = "Bit 28 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn hspad2_vdet_lp(&mut self) -> Hspad2VdetLpW<Pdruncfg0SetSpec> {
        Hspad2VdetLpW::new(self, 28)
    }
    #[doc = "Bit 29 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn hspad2_ref_pd(&mut self) -> Hspad2RefPdW<Pdruncfg0SetSpec> {
        Hspad2RefPdW::new(self, 29)
    }
}
#[doc = "Run configuration 0 set\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pdruncfg0_set::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pdruncfg0SetSpec;
impl crate::RegisterSpec for Pdruncfg0SetSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`pdruncfg0_set::W`](W) writer structure"]
impl crate::Writable for Pdruncfg0SetSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PDRUNCFG0_SET to value 0"]
impl crate::Resettable for Pdruncfg0SetSpec {
    const RESET_VALUE: u32 = 0;
}

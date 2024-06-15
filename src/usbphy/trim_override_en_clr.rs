#[doc = "Register `TRIM_OVERRIDE_EN_CLR` reader"]
pub type R = crate::R<TrimOverrideEnClrSpec>;
#[doc = "Register `TRIM_OVERRIDE_EN_CLR` writer"]
pub type W = crate::W<TrimOverrideEnClrSpec>;
#[doc = "Field `TRIM_DIV_SEL_OVERRIDE` reader - Override enable for PLL_DIV_SEL, when set, the register value in PLL_SIC\\[1:0\\]
will be used."]
pub type TrimDivSelOverrideR = crate::BitReader;
#[doc = "Field `TRIM_DIV_SEL_OVERRIDE` writer - Override enable for PLL_DIV_SEL, when set, the register value in PLL_SIC\\[1:0\\]
will be used."]
pub type TrimDivSelOverrideW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRIM_ENV_TAIL_ADJ_VD_OVERRIDE` reader - Override enable for ENV_TAIL_ADJ, when set, the register value in DEBUG1\\[14:13\\]
will be used."]
pub type TrimEnvTailAdjVdOverrideR = crate::BitReader;
#[doc = "Field `TRIM_ENV_TAIL_ADJ_VD_OVERRIDE` writer - Override enable for ENV_TAIL_ADJ, when set, the register value in DEBUG1\\[14:13\\]
will be used."]
pub type TrimEnvTailAdjVdOverrideW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRIM_TX_D_CAL_OVERRIDE` reader - Override enable for TX_D_CAL, when set, the register value in TX\\[3:0\\]
will be used."]
pub type TrimTxDCalOverrideR = crate::BitReader;
#[doc = "Field `TRIM_TX_D_CAL_OVERRIDE` writer - Override enable for TX_D_CAL, when set, the register value in TX\\[3:0\\]
will be used."]
pub type TrimTxDCalOverrideW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRIM_TX_CAL45DP_OVERRIDE` reader - Override enable for TX_CAL45DP, when set, the register value in TX\\[19:16\\]
will be used."]
pub type TrimTxCal45dpOverrideR = crate::BitReader;
#[doc = "Field `TRIM_TX_CAL45DP_OVERRIDE` writer - Override enable for TX_CAL45DP, when set, the register value in TX\\[19:16\\]
will be used."]
pub type TrimTxCal45dpOverrideW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRIM_TX_CAL45DM_OVERRIDE` reader - Override enable for TX_CAL45DM, when set, the register value in TX\\[11:8\\]
will be used."]
pub type TrimTxCal45dmOverrideR = crate::BitReader;
#[doc = "Field `TRIM_TX_CAL45DM_OVERRIDE` writer - Override enable for TX_CAL45DM, when set, the register value in TX\\[11:8\\]
will be used."]
pub type TrimTxCal45dmOverrideW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRIM_REFBIAS_VBGADJ_OVERRIDE` reader - Override enable for bandgap adjustment"]
pub type TrimRefbiasVbgadjOverrideR = crate::BitReader;
#[doc = "Field `TRIM_REFBIAS_VBGADJ_OVERRIDE` writer - Override enable for bandgap adjustment"]
pub type TrimRefbiasVbgadjOverrideW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRIM_REFBIAS_TST_OVERRIDE` reader - Override enable for bias current control When this field is set, the register value in DEBUG1\\[22:21\\]
will be used"]
pub type TrimRefbiasTstOverrideR = crate::BitReader;
#[doc = "Field `TRIM_REFBIAS_TST_OVERRIDE` writer - Override enable for bias current control When this field is set, the register value in DEBUG1\\[22:21\\]
will be used"]
pub type TrimRefbiasTstOverrideW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRIM_USB2_REFBIAS_VBGADJ` reader - Adjustment bits for bandgap"]
pub type TrimUsb2RefbiasVbgadjR = crate::FieldReader;
#[doc = "Field `TRIM_USB2_REFBIAS_VBGADJ` writer - Adjustment bits for bandgap"]
pub type TrimUsb2RefbiasVbgadjW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `TRIM_USB2_REFBIAS_TST` reader - Bias current control for usb2_phy"]
pub type TrimUsb2RefbiasTstR = crate::FieldReader;
#[doc = "Field `TRIM_USB2_REFBIAS_TST` writer - Bias current control for usb2_phy"]
pub type TrimUsb2RefbiasTstW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TRIM_PLL_CTRL0_DIV_SEL` reader - IFR value of PLL_DIV_SEL."]
pub type TrimPllCtrl0DivSelR = crate::FieldReader;
#[doc = "Field `TRIM_PLL_CTRL0_DIV_SEL` writer - IFR value of PLL_DIV_SEL."]
pub type TrimPllCtrl0DivSelW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `TRIM_USB_REG_ENV_TAIL_ADJ_VD` reader - IFR value of ENV_TAIL_ADJ."]
pub type TrimUsbRegEnvTailAdjVdR = crate::FieldReader;
#[doc = "Field `TRIM_USB_REG_ENV_TAIL_ADJ_VD` writer - IFR value of ENV_TAIL_ADJ."]
pub type TrimUsbRegEnvTailAdjVdW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TRIM_USBPHY_TX_D_CAL` reader - IFR value of TX_D_CAL."]
pub type TrimUsbphyTxDCalR = crate::FieldReader;
#[doc = "Field `TRIM_USBPHY_TX_D_CAL` writer - IFR value of TX_D_CAL."]
pub type TrimUsbphyTxDCalW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `TRIM_USBPHY_TX_CAL45DP` reader - IFR value of TX_CAL45DP."]
pub type TrimUsbphyTxCal45dpR = crate::FieldReader;
#[doc = "Field `TRIM_USBPHY_TX_CAL45DP` writer - IFR value of TX_CAL45DP."]
pub type TrimUsbphyTxCal45dpW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `TRIM_USBPHY_TX_CAL45DM` reader - IFR value of TX_CAL45DM."]
pub type TrimUsbphyTxCal45dmR = crate::FieldReader;
#[doc = "Field `TRIM_USBPHY_TX_CAL45DM` writer - IFR value of TX_CAL45DM."]
pub type TrimUsbphyTxCal45dmW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bit 0 - Override enable for PLL_DIV_SEL, when set, the register value in PLL_SIC\\[1:0\\]
will be used."]
    #[inline(always)]
    pub fn trim_div_sel_override(&self) -> TrimDivSelOverrideR {
        TrimDivSelOverrideR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Override enable for ENV_TAIL_ADJ, when set, the register value in DEBUG1\\[14:13\\]
will be used."]
    #[inline(always)]
    pub fn trim_env_tail_adj_vd_override(&self) -> TrimEnvTailAdjVdOverrideR {
        TrimEnvTailAdjVdOverrideR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Override enable for TX_D_CAL, when set, the register value in TX\\[3:0\\]
will be used."]
    #[inline(always)]
    pub fn trim_tx_d_cal_override(&self) -> TrimTxDCalOverrideR {
        TrimTxDCalOverrideR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Override enable for TX_CAL45DP, when set, the register value in TX\\[19:16\\]
will be used."]
    #[inline(always)]
    pub fn trim_tx_cal45dp_override(&self) -> TrimTxCal45dpOverrideR {
        TrimTxCal45dpOverrideR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Override enable for TX_CAL45DM, when set, the register value in TX\\[11:8\\]
will be used."]
    #[inline(always)]
    pub fn trim_tx_cal45dm_override(&self) -> TrimTxCal45dmOverrideR {
        TrimTxCal45dmOverrideR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Override enable for bandgap adjustment"]
    #[inline(always)]
    pub fn trim_refbias_vbgadj_override(&self) -> TrimRefbiasVbgadjOverrideR {
        TrimRefbiasVbgadjOverrideR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Override enable for bias current control When this field is set, the register value in DEBUG1\\[22:21\\]
will be used"]
    #[inline(always)]
    pub fn trim_refbias_tst_override(&self) -> TrimRefbiasTstOverrideR {
        TrimRefbiasTstOverrideR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 10:12 - Adjustment bits for bandgap"]
    #[inline(always)]
    pub fn trim_usb2_refbias_vbgadj(&self) -> TrimUsb2RefbiasVbgadjR {
        TrimUsb2RefbiasVbgadjR::new(((self.bits >> 10) & 7) as u8)
    }
    #[doc = "Bits 13:14 - Bias current control for usb2_phy"]
    #[inline(always)]
    pub fn trim_usb2_refbias_tst(&self) -> TrimUsb2RefbiasTstR {
        TrimUsb2RefbiasTstR::new(((self.bits >> 13) & 3) as u8)
    }
    #[doc = "Bits 15:17 - IFR value of PLL_DIV_SEL."]
    #[inline(always)]
    pub fn trim_pll_ctrl0_div_sel(&self) -> TrimPllCtrl0DivSelR {
        TrimPllCtrl0DivSelR::new(((self.bits >> 15) & 7) as u8)
    }
    #[doc = "Bits 18:19 - IFR value of ENV_TAIL_ADJ."]
    #[inline(always)]
    pub fn trim_usb_reg_env_tail_adj_vd(&self) -> TrimUsbRegEnvTailAdjVdR {
        TrimUsbRegEnvTailAdjVdR::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:23 - IFR value of TX_D_CAL."]
    #[inline(always)]
    pub fn trim_usbphy_tx_d_cal(&self) -> TrimUsbphyTxDCalR {
        TrimUsbphyTxDCalR::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - IFR value of TX_CAL45DP."]
    #[inline(always)]
    pub fn trim_usbphy_tx_cal45dp(&self) -> TrimUsbphyTxCal45dpR {
        TrimUsbphyTxCal45dpR::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - IFR value of TX_CAL45DM."]
    #[inline(always)]
    pub fn trim_usbphy_tx_cal45dm(&self) -> TrimUsbphyTxCal45dmR {
        TrimUsbphyTxCal45dmR::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Override enable for PLL_DIV_SEL, when set, the register value in PLL_SIC\\[1:0\\]
will be used."]
    #[inline(always)]
    #[must_use]
    pub fn trim_div_sel_override(&mut self) -> TrimDivSelOverrideW<TrimOverrideEnClrSpec> {
        TrimDivSelOverrideW::new(self, 0)
    }
    #[doc = "Bit 1 - Override enable for ENV_TAIL_ADJ, when set, the register value in DEBUG1\\[14:13\\]
will be used."]
    #[inline(always)]
    #[must_use]
    pub fn trim_env_tail_adj_vd_override(
        &mut self,
    ) -> TrimEnvTailAdjVdOverrideW<TrimOverrideEnClrSpec> {
        TrimEnvTailAdjVdOverrideW::new(self, 1)
    }
    #[doc = "Bit 2 - Override enable for TX_D_CAL, when set, the register value in TX\\[3:0\\]
will be used."]
    #[inline(always)]
    #[must_use]
    pub fn trim_tx_d_cal_override(&mut self) -> TrimTxDCalOverrideW<TrimOverrideEnClrSpec> {
        TrimTxDCalOverrideW::new(self, 2)
    }
    #[doc = "Bit 3 - Override enable for TX_CAL45DP, when set, the register value in TX\\[19:16\\]
will be used."]
    #[inline(always)]
    #[must_use]
    pub fn trim_tx_cal45dp_override(&mut self) -> TrimTxCal45dpOverrideW<TrimOverrideEnClrSpec> {
        TrimTxCal45dpOverrideW::new(self, 3)
    }
    #[doc = "Bit 4 - Override enable for TX_CAL45DM, when set, the register value in TX\\[11:8\\]
will be used."]
    #[inline(always)]
    #[must_use]
    pub fn trim_tx_cal45dm_override(&mut self) -> TrimTxCal45dmOverrideW<TrimOverrideEnClrSpec> {
        TrimTxCal45dmOverrideW::new(self, 4)
    }
    #[doc = "Bit 5 - Override enable for bandgap adjustment"]
    #[inline(always)]
    #[must_use]
    pub fn trim_refbias_vbgadj_override(
        &mut self,
    ) -> TrimRefbiasVbgadjOverrideW<TrimOverrideEnClrSpec> {
        TrimRefbiasVbgadjOverrideW::new(self, 5)
    }
    #[doc = "Bit 6 - Override enable for bias current control When this field is set, the register value in DEBUG1\\[22:21\\]
will be used"]
    #[inline(always)]
    #[must_use]
    pub fn trim_refbias_tst_override(&mut self) -> TrimRefbiasTstOverrideW<TrimOverrideEnClrSpec> {
        TrimRefbiasTstOverrideW::new(self, 6)
    }
    #[doc = "Bits 10:12 - Adjustment bits for bandgap"]
    #[inline(always)]
    #[must_use]
    pub fn trim_usb2_refbias_vbgadj(&mut self) -> TrimUsb2RefbiasVbgadjW<TrimOverrideEnClrSpec> {
        TrimUsb2RefbiasVbgadjW::new(self, 10)
    }
    #[doc = "Bits 13:14 - Bias current control for usb2_phy"]
    #[inline(always)]
    #[must_use]
    pub fn trim_usb2_refbias_tst(&mut self) -> TrimUsb2RefbiasTstW<TrimOverrideEnClrSpec> {
        TrimUsb2RefbiasTstW::new(self, 13)
    }
    #[doc = "Bits 15:17 - IFR value of PLL_DIV_SEL."]
    #[inline(always)]
    #[must_use]
    pub fn trim_pll_ctrl0_div_sel(&mut self) -> TrimPllCtrl0DivSelW<TrimOverrideEnClrSpec> {
        TrimPllCtrl0DivSelW::new(self, 15)
    }
    #[doc = "Bits 18:19 - IFR value of ENV_TAIL_ADJ."]
    #[inline(always)]
    #[must_use]
    pub fn trim_usb_reg_env_tail_adj_vd(
        &mut self,
    ) -> TrimUsbRegEnvTailAdjVdW<TrimOverrideEnClrSpec> {
        TrimUsbRegEnvTailAdjVdW::new(self, 18)
    }
    #[doc = "Bits 20:23 - IFR value of TX_D_CAL."]
    #[inline(always)]
    #[must_use]
    pub fn trim_usbphy_tx_d_cal(&mut self) -> TrimUsbphyTxDCalW<TrimOverrideEnClrSpec> {
        TrimUsbphyTxDCalW::new(self, 20)
    }
    #[doc = "Bits 24:27 - IFR value of TX_CAL45DP."]
    #[inline(always)]
    #[must_use]
    pub fn trim_usbphy_tx_cal45dp(&mut self) -> TrimUsbphyTxCal45dpW<TrimOverrideEnClrSpec> {
        TrimUsbphyTxCal45dpW::new(self, 24)
    }
    #[doc = "Bits 28:31 - IFR value of TX_CAL45DM."]
    #[inline(always)]
    #[must_use]
    pub fn trim_usbphy_tx_cal45dm(&mut self) -> TrimUsbphyTxCal45dmW<TrimOverrideEnClrSpec> {
        TrimUsbphyTxCal45dmW::new(self, 28)
    }
}
#[doc = "USB PHY Trim Override Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`trim_override_en_clr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`trim_override_en_clr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TrimOverrideEnClrSpec;
impl crate::RegisterSpec for TrimOverrideEnClrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`trim_override_en_clr::R`](R) reader structure"]
impl crate::Readable for TrimOverrideEnClrSpec {}
#[doc = "`write(|w| ..)` method takes [`trim_override_en_clr::W`](W) writer structure"]
impl crate::Writable for TrimOverrideEnClrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TRIM_OVERRIDE_EN_CLR to value 0x7f"]
impl crate::Resettable for TrimOverrideEnClrSpec {
    const RESET_VALUE: u32 = 0x7f;
}

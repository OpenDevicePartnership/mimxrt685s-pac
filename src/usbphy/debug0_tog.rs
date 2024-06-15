#[doc = "Register `DEBUG0_TOG` reader"]
pub type R = crate::R<Debug0TogSpec>;
#[doc = "Register `DEBUG0_TOG` writer"]
pub type W = crate::W<Debug0TogSpec>;
#[doc = "Field `DEBUG_INTERFACE_HOLD` reader - Use holding registers to assist in timing for external UTMI interface."]
pub type DebugInterfaceHoldR = crate::BitReader;
#[doc = "Field `DEBUG_INTERFACE_HOLD` writer - Use holding registers to assist in timing for external UTMI interface."]
pub type DebugInterfaceHoldW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSTPULLDOWN` reader - This bit field selects whether to connect pulldown resistors on the USB_DP/USB_DM pins if the corresponding pulldown overdrive mode is enabled through DEBUG\\[5:4} Set bit 3 to value 1'b1 to connect the 15ohm pulldown on USB_DP line"]
pub type HstpulldownR = crate::FieldReader;
#[doc = "Field `HSTPULLDOWN` writer - This bit field selects whether to connect pulldown resistors on the USB_DP/USB_DM pins if the corresponding pulldown overdrive mode is enabled through DEBUG\\[5:4} Set bit 3 to value 1'b1 to connect the 15ohm pulldown on USB_DP line"]
pub type HstpulldownW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ENHSTPULLDOWN` reader - This bit field selects host pulldown overdrive mode"]
pub type EnhstpulldownR = crate::FieldReader;
#[doc = "Field `ENHSTPULLDOWN` writer - This bit field selects host pulldown overdrive mode"]
pub type EnhstpulldownW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TX2RXCOUNT` reader - Delay in between the end of transmit to the beginning of receive"]
pub type Tx2rxcountR = crate::FieldReader;
#[doc = "Field `TX2RXCOUNT` writer - Delay in between the end of transmit to the beginning of receive"]
pub type Tx2rxcountW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ENTX2RXCOUNT` reader - Set this bit to allow a countdown to transition in between TX and RX."]
pub type Entx2rxcountR = crate::BitReader;
#[doc = "Field `ENTX2RXCOUNT` writer - Set this bit to allow a countdown to transition in between TX and RX."]
pub type Entx2rxcountW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SQUELCHRESETCOUNT` reader - Delay in between the detection of squelch to the reset of high-speed RX."]
pub type SquelchresetcountR = crate::FieldReader;
#[doc = "Field `SQUELCHRESETCOUNT` writer - Delay in between the detection of squelch to the reset of high-speed RX."]
pub type SquelchresetcountW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `ENSQUELCHRESET` reader - Set bit to allow squelch to reset high-speed receive."]
pub type EnsquelchresetR = crate::BitReader;
#[doc = "Field `ENSQUELCHRESET` writer - Set bit to allow squelch to reset high-speed receive."]
pub type EnsquelchresetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SQUELCHRESETLENGTH` reader - Duration of RESET in terms of the number of 480-MHz cycles."]
pub type SquelchresetlengthR = crate::FieldReader;
#[doc = "Field `SQUELCHRESETLENGTH` writer - Duration of RESET in terms of the number of 480-MHz cycles."]
pub type SquelchresetlengthW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `HOST_RESUME_DEBUG` reader - Choose to trigger the host resume SE0 with HOST_FORCE_LS_SE0 = 0 or UTMI_SUSPEND = 1."]
pub type HostResumeDebugR = crate::BitReader;
#[doc = "Field `HOST_RESUME_DEBUG` writer - Choose to trigger the host resume SE0 with HOST_FORCE_LS_SE0 = 0 or UTMI_SUSPEND = 1."]
pub type HostResumeDebugW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLKGATE` reader - Gate Test Clocks"]
pub type ClkgateR = crate::BitReader;
#[doc = "Field `CLKGATE` writer - Gate Test Clocks"]
pub type ClkgateW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1 - Use holding registers to assist in timing for external UTMI interface."]
    #[inline(always)]
    pub fn debug_interface_hold(&self) -> DebugInterfaceHoldR {
        DebugInterfaceHoldR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - This bit field selects whether to connect pulldown resistors on the USB_DP/USB_DM pins if the corresponding pulldown overdrive mode is enabled through DEBUG\\[5:4} Set bit 3 to value 1'b1 to connect the 15ohm pulldown on USB_DP line"]
    #[inline(always)]
    pub fn hstpulldown(&self) -> HstpulldownR {
        HstpulldownR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - This bit field selects host pulldown overdrive mode"]
    #[inline(always)]
    pub fn enhstpulldown(&self) -> EnhstpulldownR {
        EnhstpulldownR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 8:11 - Delay in between the end of transmit to the beginning of receive"]
    #[inline(always)]
    pub fn tx2rxcount(&self) -> Tx2rxcountR {
        Tx2rxcountR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 12 - Set this bit to allow a countdown to transition in between TX and RX."]
    #[inline(always)]
    pub fn entx2rxcount(&self) -> Entx2rxcountR {
        Entx2rxcountR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 16:20 - Delay in between the detection of squelch to the reset of high-speed RX."]
    #[inline(always)]
    pub fn squelchresetcount(&self) -> SquelchresetcountR {
        SquelchresetcountR::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bit 24 - Set bit to allow squelch to reset high-speed receive."]
    #[inline(always)]
    pub fn ensquelchreset(&self) -> EnsquelchresetR {
        EnsquelchresetR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bits 25:28 - Duration of RESET in terms of the number of 480-MHz cycles."]
    #[inline(always)]
    pub fn squelchresetlength(&self) -> SquelchresetlengthR {
        SquelchresetlengthR::new(((self.bits >> 25) & 0x0f) as u8)
    }
    #[doc = "Bit 29 - Choose to trigger the host resume SE0 with HOST_FORCE_LS_SE0 = 0 or UTMI_SUSPEND = 1."]
    #[inline(always)]
    pub fn host_resume_debug(&self) -> HostResumeDebugR {
        HostResumeDebugR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Gate Test Clocks"]
    #[inline(always)]
    pub fn clkgate(&self) -> ClkgateR {
        ClkgateR::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Use holding registers to assist in timing for external UTMI interface."]
    #[inline(always)]
    #[must_use]
    pub fn debug_interface_hold(&mut self) -> DebugInterfaceHoldW<Debug0TogSpec> {
        DebugInterfaceHoldW::new(self, 1)
    }
    #[doc = "Bits 2:3 - This bit field selects whether to connect pulldown resistors on the USB_DP/USB_DM pins if the corresponding pulldown overdrive mode is enabled through DEBUG\\[5:4} Set bit 3 to value 1'b1 to connect the 15ohm pulldown on USB_DP line"]
    #[inline(always)]
    #[must_use]
    pub fn hstpulldown(&mut self) -> HstpulldownW<Debug0TogSpec> {
        HstpulldownW::new(self, 2)
    }
    #[doc = "Bits 4:5 - This bit field selects host pulldown overdrive mode"]
    #[inline(always)]
    #[must_use]
    pub fn enhstpulldown(&mut self) -> EnhstpulldownW<Debug0TogSpec> {
        EnhstpulldownW::new(self, 4)
    }
    #[doc = "Bits 8:11 - Delay in between the end of transmit to the beginning of receive"]
    #[inline(always)]
    #[must_use]
    pub fn tx2rxcount(&mut self) -> Tx2rxcountW<Debug0TogSpec> {
        Tx2rxcountW::new(self, 8)
    }
    #[doc = "Bit 12 - Set this bit to allow a countdown to transition in between TX and RX."]
    #[inline(always)]
    #[must_use]
    pub fn entx2rxcount(&mut self) -> Entx2rxcountW<Debug0TogSpec> {
        Entx2rxcountW::new(self, 12)
    }
    #[doc = "Bits 16:20 - Delay in between the detection of squelch to the reset of high-speed RX."]
    #[inline(always)]
    #[must_use]
    pub fn squelchresetcount(&mut self) -> SquelchresetcountW<Debug0TogSpec> {
        SquelchresetcountW::new(self, 16)
    }
    #[doc = "Bit 24 - Set bit to allow squelch to reset high-speed receive."]
    #[inline(always)]
    #[must_use]
    pub fn ensquelchreset(&mut self) -> EnsquelchresetW<Debug0TogSpec> {
        EnsquelchresetW::new(self, 24)
    }
    #[doc = "Bits 25:28 - Duration of RESET in terms of the number of 480-MHz cycles."]
    #[inline(always)]
    #[must_use]
    pub fn squelchresetlength(&mut self) -> SquelchresetlengthW<Debug0TogSpec> {
        SquelchresetlengthW::new(self, 25)
    }
    #[doc = "Bit 29 - Choose to trigger the host resume SE0 with HOST_FORCE_LS_SE0 = 0 or UTMI_SUSPEND = 1."]
    #[inline(always)]
    #[must_use]
    pub fn host_resume_debug(&mut self) -> HostResumeDebugW<Debug0TogSpec> {
        HostResumeDebugW::new(self, 29)
    }
    #[doc = "Bit 30 - Gate Test Clocks"]
    #[inline(always)]
    #[must_use]
    pub fn clkgate(&mut self) -> ClkgateW<Debug0TogSpec> {
        ClkgateW::new(self, 30)
    }
}
#[doc = "USB PHY Debug Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`debug0_tog::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`debug0_tog::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Debug0TogSpec;
impl crate::RegisterSpec for Debug0TogSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`debug0_tog::R`](R) reader structure"]
impl crate::Readable for Debug0TogSpec {}
#[doc = "`write(|w| ..)` method takes [`debug0_tog::W`](W) writer structure"]
impl crate::Writable for Debug0TogSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DEBUG0_TOG to value 0x7f18_0000"]
impl crate::Resettable for Debug0TogSpec {
    const RESET_VALUE: u32 = 0x7f18_0000;
}

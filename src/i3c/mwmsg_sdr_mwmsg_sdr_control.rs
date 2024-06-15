#[doc = "Register `MWMSG_SDR_CONTROL` writer"]
pub type W = crate::W<MwmsgSdrMwmsgSdrControlSpec>;
#[doc = "Direction\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dir {
    #[doc = "0: Write"]
    Write = 0,
    #[doc = "1: Read"]
    Read = 1,
}
impl From<Dir> for bool {
    #[inline(always)]
    fn from(variant: Dir) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIR` writer - Direction"]
pub type DirW<'a, REG> = crate::BitWriter<'a, REG, Dir>;
impl<'a, REG> DirW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Write"]
    #[inline(always)]
    pub fn write(self) -> &'a mut crate::W<REG> {
        self.variant(Dir::Write)
    }
    #[doc = "Read"]
    #[inline(always)]
    pub fn read(self) -> &'a mut crate::W<REG> {
        self.variant(Dir::Read)
    }
}
#[doc = "Field `ADDR` writer - Address to be written to"]
pub type AddrW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `END` writer - End of SDR message"]
pub type EndW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "I2C\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum I2c {
    #[doc = "0: I3C message"]
    I3cmessage = 0,
    #[doc = "1: I2C message"]
    I2cmessage = 1,
}
impl From<I2c> for bool {
    #[inline(always)]
    fn from(variant: I2c) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `I2C` writer - I2C"]
pub type I2cW<'a, REG> = crate::BitWriter<'a, REG, I2c>;
impl<'a, REG> I2cW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "I3C message"]
    #[inline(always)]
    pub fn i3cmessage(self) -> &'a mut crate::W<REG> {
        self.variant(I2c::I3cmessage)
    }
    #[doc = "I2C message"]
    #[inline(always)]
    pub fn i2cmessage(self) -> &'a mut crate::W<REG> {
        self.variant(I2c::I2cmessage)
    }
}
#[doc = "Field `LEN` writer - Length"]
pub type LenW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl W {
    #[doc = "Bit 0 - Direction"]
    #[inline(always)]
    #[must_use]
    pub fn dir(&mut self) -> DirW<MwmsgSdrMwmsgSdrControlSpec> {
        DirW::new(self, 0)
    }
    #[doc = "Bits 1:7 - Address to be written to"]
    #[inline(always)]
    #[must_use]
    pub fn addr(&mut self) -> AddrW<MwmsgSdrMwmsgSdrControlSpec> {
        AddrW::new(self, 1)
    }
    #[doc = "Bit 8 - End of SDR message"]
    #[inline(always)]
    #[must_use]
    pub fn end(&mut self) -> EndW<MwmsgSdrMwmsgSdrControlSpec> {
        EndW::new(self, 8)
    }
    #[doc = "Bit 10 - I2C"]
    #[inline(always)]
    #[must_use]
    pub fn i2c(&mut self) -> I2cW<MwmsgSdrMwmsgSdrControlSpec> {
        I2cW::new(self, 10)
    }
    #[doc = "Bits 11:15 - Length"]
    #[inline(always)]
    #[must_use]
    pub fn len(&mut self) -> LenW<MwmsgSdrMwmsgSdrControlSpec> {
        LenW::new(self, 11)
    }
}
#[doc = "Master Write Message in SDR mode\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mwmsg_sdr_mwmsg_sdr_control::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MwmsgSdrMwmsgSdrControlSpec;
impl crate::RegisterSpec for MwmsgSdrMwmsgSdrControlSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`mwmsg_sdr_mwmsg_sdr_control::W`](W) writer structure"]
impl crate::Writable for MwmsgSdrMwmsgSdrControlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MWMSG_SDR_CONTROL to value 0"]
impl crate::Resettable for MwmsgSdrMwmsgSdrControlSpec {
    const RESET_VALUE: u32 = 0;
}

#[doc = "Register `IDXBLK_H` reader"]
pub type R = crate::R<IdxblkHSpec>;
#[doc = "Register `IDXBLK_H` writer"]
pub type W = crate::W<IdxblkHSpec>;
#[doc = "Field `IDX8` reader - Index 8"]
pub type Idx8R = crate::FieldReader;
#[doc = "Field `IDX8` writer - Index 8"]
pub type Idx8W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `IDX9` reader - Index 9"]
pub type Idx9R = crate::FieldReader;
#[doc = "Field `IDX9` writer - Index 9"]
pub type Idx9W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `IDX10` reader - Index 10"]
pub type Idx10R = crate::FieldReader;
#[doc = "Field `IDX10` writer - Index 10"]
pub type Idx10W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `IDX11` reader - Index 11"]
pub type Idx11R = crate::FieldReader;
#[doc = "Field `IDX11` writer - Index 11"]
pub type Idx11W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `IDX12` reader - Index 12"]
pub type Idx12R = crate::FieldReader;
#[doc = "Field `IDX12` writer - Index 12"]
pub type Idx12W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `IDX13` reader - Index 13"]
pub type Idx13R = crate::FieldReader;
#[doc = "Field `IDX13` writer - Index 13"]
pub type Idx13W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `IDX14` reader - Index 14"]
pub type Idx14R = crate::FieldReader;
#[doc = "Field `IDX14` writer - Index 14"]
pub type Idx14W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `IDX15` reader - Index 15"]
pub type Idx15R = crate::FieldReader;
#[doc = "Field `IDX15` writer - Index 15"]
pub type Idx15W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `LOCK_IDX` reader - Lock Index"]
pub type LockIdxR = crate::FieldReader;
#[doc = "Field `LOCK_IDX` writer - Lock Index"]
pub type LockIdxW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - Index 8"]
    #[inline(always)]
    pub fn idx8(&self) -> Idx8R {
        Idx8R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Index 9"]
    #[inline(always)]
    pub fn idx9(&self) -> Idx9R {
        Idx9R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Index 10"]
    #[inline(always)]
    pub fn idx10(&self) -> Idx10R {
        Idx10R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Index 11"]
    #[inline(always)]
    pub fn idx11(&self) -> Idx11R {
        Idx11R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Index 12"]
    #[inline(always)]
    pub fn idx12(&self) -> Idx12R {
        Idx12R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Index 13"]
    #[inline(always)]
    pub fn idx13(&self) -> Idx13R {
        Idx13R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Index 14"]
    #[inline(always)]
    pub fn idx14(&self) -> Idx14R {
        Idx14R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - Index 15"]
    #[inline(always)]
    pub fn idx15(&self) -> Idx15R {
        Idx15R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 30:31 - Lock Index"]
    #[inline(always)]
    pub fn lock_idx(&self) -> LockIdxR {
        LockIdxR::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Index 8"]
    #[inline(always)]
    #[must_use]
    pub fn idx8(&mut self) -> Idx8W<IdxblkHSpec> {
        Idx8W::new(self, 0)
    }
    #[doc = "Bits 2:3 - Index 9"]
    #[inline(always)]
    #[must_use]
    pub fn idx9(&mut self) -> Idx9W<IdxblkHSpec> {
        Idx9W::new(self, 2)
    }
    #[doc = "Bits 4:5 - Index 10"]
    #[inline(always)]
    #[must_use]
    pub fn idx10(&mut self) -> Idx10W<IdxblkHSpec> {
        Idx10W::new(self, 4)
    }
    #[doc = "Bits 6:7 - Index 11"]
    #[inline(always)]
    #[must_use]
    pub fn idx11(&mut self) -> Idx11W<IdxblkHSpec> {
        Idx11W::new(self, 6)
    }
    #[doc = "Bits 8:9 - Index 12"]
    #[inline(always)]
    #[must_use]
    pub fn idx12(&mut self) -> Idx12W<IdxblkHSpec> {
        Idx12W::new(self, 8)
    }
    #[doc = "Bits 10:11 - Index 13"]
    #[inline(always)]
    #[must_use]
    pub fn idx13(&mut self) -> Idx13W<IdxblkHSpec> {
        Idx13W::new(self, 10)
    }
    #[doc = "Bits 12:13 - Index 14"]
    #[inline(always)]
    #[must_use]
    pub fn idx14(&mut self) -> Idx14W<IdxblkHSpec> {
        Idx14W::new(self, 12)
    }
    #[doc = "Bits 14:15 - Index 15"]
    #[inline(always)]
    #[must_use]
    pub fn idx15(&mut self) -> Idx15W<IdxblkHSpec> {
        Idx15W::new(self, 14)
    }
    #[doc = "Bits 30:31 - Lock Index"]
    #[inline(always)]
    #[must_use]
    pub fn lock_idx(&mut self) -> LockIdxW<IdxblkHSpec> {
        LockIdxW::new(self, 30)
    }
}
#[doc = "Index Block High\n\nYou can [`read`](crate::Reg::read) this register and get [`idxblk_h::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`idxblk_h::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IdxblkHSpec;
impl crate::RegisterSpec for IdxblkHSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`idxblk_h::R`](R) reader structure"]
impl crate::Readable for IdxblkHSpec {}
#[doc = "`write(|w| ..)` method takes [`idxblk_h::W`](W) writer structure"]
impl crate::Writable for IdxblkHSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IDXBLK_H to value 0x8000_aaaa"]
impl crate::Resettable for IdxblkHSpec {
    const RESET_VALUE: u32 = 0x8000_aaaa;
}

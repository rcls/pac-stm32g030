#[doc = "Register `AHBRSTR` reader"]
pub type R = crate::R<AHBRSTR_SPEC>;
#[doc = "Register `AHBRSTR` writer"]
pub type W = crate::W<AHBRSTR_SPEC>;
#[doc = "Field `DMA1RST` reader - DMA1 reset"]
pub type DMA1RST_R = crate::BitReader;
#[doc = "Field `DMA1RST` writer - DMA1 reset"]
pub type DMA1RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA2RST` reader - DMA1 reset"]
pub type DMA2RST_R = crate::BitReader;
#[doc = "Field `DMA2RST` writer - DMA1 reset"]
pub type DMA2RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLASHRST` reader - FLITF reset"]
pub type FLASHRST_R = crate::BitReader;
#[doc = "Field `FLASHRST` writer - FLITF reset"]
pub type FLASHRST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRCRST` reader - CRC reset"]
pub type CRCRST_R = crate::BitReader;
#[doc = "Field `CRCRST` writer - CRC reset"]
pub type CRCRST_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - DMA1 reset"]
    #[inline(always)]
    pub fn DMA1RST(&self) -> DMA1RST_R {
        DMA1RST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DMA1 reset"]
    #[inline(always)]
    pub fn DMA2RST(&self) -> DMA2RST_R {
        DMA2RST_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 8 - FLITF reset"]
    #[inline(always)]
    pub fn FLASHRST(&self) -> FLASHRST_R {
        FLASHRST_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 12 - CRC reset"]
    #[inline(always)]
    pub fn CRCRST(&self) -> CRCRST_R {
        CRCRST_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DMA1 reset"]
    #[inline(always)]
    pub fn DMA1RST(&mut self) -> DMA1RST_W<'_, AHBRSTR_SPEC> {
        DMA1RST_W::new(self, 0)
    }
    #[doc = "Bit 1 - DMA1 reset"]
    #[inline(always)]
    pub fn DMA2RST(&mut self) -> DMA2RST_W<'_, AHBRSTR_SPEC> {
        DMA2RST_W::new(self, 1)
    }
    #[doc = "Bit 8 - FLITF reset"]
    #[inline(always)]
    pub fn FLASHRST(&mut self) -> FLASHRST_W<'_, AHBRSTR_SPEC> {
        FLASHRST_W::new(self, 8)
    }
    #[doc = "Bit 12 - CRC reset"]
    #[inline(always)]
    pub fn CRCRST(&mut self) -> CRCRST_W<'_, AHBRSTR_SPEC> {
        CRCRST_W::new(self, 12)
    }
}
#[doc = "AHB peripheral reset register\n\nYou can [`read`](crate::Reg::read) this register and get [`ahbrstr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahbrstr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AHBRSTR_SPEC;
impl crate::RegisterSpec for AHBRSTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ahbrstr::R`](R) reader structure"]
impl crate::Readable for AHBRSTR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ahbrstr::W`](W) writer structure"]
impl crate::Writable for AHBRSTR_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets AHBRSTR to value 0"]
impl crate::Resettable for AHBRSTR_SPEC {}

#[doc = "Register `CR1` reader"]
pub type R = crate::R<CR1_SPEC>;
#[doc = "Register `CR1` writer"]
pub type W = crate::W<CR1_SPEC>;
#[doc = "Field `CEN` reader - Counter enable"]
pub type CEN_R = crate::BitReader;
#[doc = "Field `CEN` writer - Counter enable"]
pub type CEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UDIS` reader - Update disable"]
pub type UDIS_R = crate::BitReader;
#[doc = "Field `UDIS` writer - Update disable"]
pub type UDIS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `URS` reader - Update request source"]
pub type URS_R = crate::BitReader;
#[doc = "Field `URS` writer - Update request source"]
pub type URS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OPM` reader - One-pulse mode"]
pub type OPM_R = crate::BitReader;
#[doc = "Field `OPM` writer - One-pulse mode"]
pub type OPM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIR` reader - Direction"]
pub type DIR_R = crate::BitReader;
#[doc = "Field `DIR` writer - Direction"]
pub type DIR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMS` reader - Center-aligned mode selection"]
pub type CMS_R = crate::FieldReader;
#[doc = "Field `CMS` writer - Center-aligned mode selection"]
pub type CMS_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ARPE` reader - Auto-reload preload enable"]
pub type ARPE_R = crate::BitReader;
#[doc = "Field `ARPE` writer - Auto-reload preload enable"]
pub type ARPE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CKD` reader - Clock division"]
pub type CKD_R = crate::FieldReader;
#[doc = "Field `CKD` writer - Clock division"]
pub type CKD_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `UIFREMAP` reader - UIF status bit remapping"]
pub type UIFREMAP_R = crate::BitReader;
#[doc = "Field `UIFREMAP` writer - UIF status bit remapping"]
pub type UIFREMAP_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Counter enable"]
    #[inline(always)]
    pub fn CEN(&self) -> CEN_R {
        CEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Update disable"]
    #[inline(always)]
    pub fn UDIS(&self) -> UDIS_R {
        UDIS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Update request source"]
    #[inline(always)]
    pub fn URS(&self) -> URS_R {
        URS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - One-pulse mode"]
    #[inline(always)]
    pub fn OPM(&self) -> OPM_R {
        OPM_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Direction"]
    #[inline(always)]
    pub fn DIR(&self) -> DIR_R {
        DIR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:6 - Center-aligned mode selection"]
    #[inline(always)]
    pub fn CMS(&self) -> CMS_R {
        CMS_R::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 7 - Auto-reload preload enable"]
    #[inline(always)]
    pub fn ARPE(&self) -> ARPE_R {
        ARPE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - Clock division"]
    #[inline(always)]
    pub fn CKD(&self) -> CKD_R {
        CKD_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 11 - UIF status bit remapping"]
    #[inline(always)]
    pub fn UIFREMAP(&self) -> UIFREMAP_R {
        UIFREMAP_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Counter enable"]
    #[inline(always)]
    pub fn CEN(&mut self) -> CEN_W<'_, CR1_SPEC> {
        CEN_W::new(self, 0)
    }
    #[doc = "Bit 1 - Update disable"]
    #[inline(always)]
    pub fn UDIS(&mut self) -> UDIS_W<'_, CR1_SPEC> {
        UDIS_W::new(self, 1)
    }
    #[doc = "Bit 2 - Update request source"]
    #[inline(always)]
    pub fn URS(&mut self) -> URS_W<'_, CR1_SPEC> {
        URS_W::new(self, 2)
    }
    #[doc = "Bit 3 - One-pulse mode"]
    #[inline(always)]
    pub fn OPM(&mut self) -> OPM_W<'_, CR1_SPEC> {
        OPM_W::new(self, 3)
    }
    #[doc = "Bit 4 - Direction"]
    #[inline(always)]
    pub fn DIR(&mut self) -> DIR_W<'_, CR1_SPEC> {
        DIR_W::new(self, 4)
    }
    #[doc = "Bits 5:6 - Center-aligned mode selection"]
    #[inline(always)]
    pub fn CMS(&mut self) -> CMS_W<'_, CR1_SPEC> {
        CMS_W::new(self, 5)
    }
    #[doc = "Bit 7 - Auto-reload preload enable"]
    #[inline(always)]
    pub fn ARPE(&mut self) -> ARPE_W<'_, CR1_SPEC> {
        ARPE_W::new(self, 7)
    }
    #[doc = "Bits 8:9 - Clock division"]
    #[inline(always)]
    pub fn CKD(&mut self) -> CKD_W<'_, CR1_SPEC> {
        CKD_W::new(self, 8)
    }
    #[doc = "Bit 11 - UIF status bit remapping"]
    #[inline(always)]
    pub fn UIFREMAP(&mut self) -> UIFREMAP_W<'_, CR1_SPEC> {
        UIFREMAP_W::new(self, 11)
    }
}
#[doc = "control register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`cr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CR1_SPEC;
impl crate::RegisterSpec for CR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr1::R`](R) reader structure"]
impl crate::Readable for CR1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cr1::W`](W) writer structure"]
impl crate::Writable for CR1_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets CR1 to value 0"]
impl crate::Resettable for CR1_SPEC {}

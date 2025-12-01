#[doc = "Register `I2SCFGR` reader"]
pub type R = crate::R<I2SCFGR_SPEC>;
#[doc = "Register `I2SCFGR` writer"]
pub type W = crate::W<I2SCFGR_SPEC>;
#[doc = "Field `CHLEN` reader - Channel length (number of bits per audio channel)"]
pub type CHLEN_R = crate::BitReader;
#[doc = "Field `CHLEN` writer - Channel length (number of bits per audio channel)"]
pub type CHLEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DATLEN` reader - Data length to be transferred"]
pub type DATLEN_R = crate::FieldReader;
#[doc = "Field `DATLEN` writer - Data length to be transferred"]
pub type DATLEN_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CKPOL` reader - Inactive state clock polarity"]
pub type CKPOL_R = crate::BitReader;
#[doc = "Field `CKPOL` writer - Inactive state clock polarity"]
pub type CKPOL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2SSTD` reader - standard selection"]
pub type I2SSTD_R = crate::FieldReader;
#[doc = "Field `I2SSTD` writer - standard selection"]
pub type I2SSTD_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PCMSYNC` reader - PCM frame synchronization"]
pub type PCMSYNC_R = crate::BitReader;
#[doc = "Field `PCMSYNC` writer - PCM frame synchronization"]
pub type PCMSYNC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2SCFG` reader - I2S configuration mode"]
pub type I2SCFG_R = crate::FieldReader;
#[doc = "Field `I2SCFG` writer - I2S configuration mode"]
pub type I2SCFG_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SE2` reader - I2S enable"]
pub type SE2_R = crate::BitReader;
#[doc = "Field `SE2` writer - I2S enable"]
pub type SE2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2SMOD` reader - I2S mode selection"]
pub type I2SMOD_R = crate::BitReader;
#[doc = "Field `I2SMOD` writer - I2S mode selection"]
pub type I2SMOD_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Channel length (number of bits per audio channel)"]
    #[inline(always)]
    pub fn CHLEN(&self) -> CHLEN_R {
        CHLEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - Data length to be transferred"]
    #[inline(always)]
    pub fn DATLEN(&self) -> DATLEN_R {
        DATLEN_R::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bit 3 - Inactive state clock polarity"]
    #[inline(always)]
    pub fn CKPOL(&self) -> CKPOL_R {
        CKPOL_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - standard selection"]
    #[inline(always)]
    pub fn I2SSTD(&self) -> I2SSTD_R {
        I2SSTD_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 7 - PCM frame synchronization"]
    #[inline(always)]
    pub fn PCMSYNC(&self) -> PCMSYNC_R {
        PCMSYNC_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - I2S configuration mode"]
    #[inline(always)]
    pub fn I2SCFG(&self) -> I2SCFG_R {
        I2SCFG_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 10 - I2S enable"]
    #[inline(always)]
    pub fn SE2(&self) -> SE2_R {
        SE2_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - I2S mode selection"]
    #[inline(always)]
    pub fn I2SMOD(&self) -> I2SMOD_R {
        I2SMOD_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Channel length (number of bits per audio channel)"]
    #[inline(always)]
    pub fn CHLEN(&mut self) -> CHLEN_W<'_, I2SCFGR_SPEC> {
        CHLEN_W::new(self, 0)
    }
    #[doc = "Bits 1:2 - Data length to be transferred"]
    #[inline(always)]
    pub fn DATLEN(&mut self) -> DATLEN_W<'_, I2SCFGR_SPEC> {
        DATLEN_W::new(self, 1)
    }
    #[doc = "Bit 3 - Inactive state clock polarity"]
    #[inline(always)]
    pub fn CKPOL(&mut self) -> CKPOL_W<'_, I2SCFGR_SPEC> {
        CKPOL_W::new(self, 3)
    }
    #[doc = "Bits 4:5 - standard selection"]
    #[inline(always)]
    pub fn I2SSTD(&mut self) -> I2SSTD_W<'_, I2SCFGR_SPEC> {
        I2SSTD_W::new(self, 4)
    }
    #[doc = "Bit 7 - PCM frame synchronization"]
    #[inline(always)]
    pub fn PCMSYNC(&mut self) -> PCMSYNC_W<'_, I2SCFGR_SPEC> {
        PCMSYNC_W::new(self, 7)
    }
    #[doc = "Bits 8:9 - I2S configuration mode"]
    #[inline(always)]
    pub fn I2SCFG(&mut self) -> I2SCFG_W<'_, I2SCFGR_SPEC> {
        I2SCFG_W::new(self, 8)
    }
    #[doc = "Bit 10 - I2S enable"]
    #[inline(always)]
    pub fn SE2(&mut self) -> SE2_W<'_, I2SCFGR_SPEC> {
        SE2_W::new(self, 10)
    }
    #[doc = "Bit 11 - I2S mode selection"]
    #[inline(always)]
    pub fn I2SMOD(&mut self) -> I2SMOD_W<'_, I2SCFGR_SPEC> {
        I2SMOD_W::new(self, 11)
    }
}
#[doc = "configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`i2scfgr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2scfgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2SCFGR_SPEC;
impl crate::RegisterSpec for I2SCFGR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2scfgr::R`](R) reader structure"]
impl crate::Readable for I2SCFGR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`i2scfgr::W`](W) writer structure"]
impl crate::Writable for I2SCFGR_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets I2SCFGR to value 0"]
impl crate::Resettable for I2SCFGR_SPEC {}

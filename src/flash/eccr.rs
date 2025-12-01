#[doc = "Register `ECCR` reader"]
pub type R = crate::R<ECCR_SPEC>;
#[doc = "Register `ECCR` writer"]
pub type W = crate::W<ECCR_SPEC>;
#[doc = "Field `ADDR_ECC` reader - ECC fail address"]
pub type ADDR_ECC_R = crate::FieldReader<u16>;
#[doc = "Field `SYSF_ECC` reader - ECC fail for Corrected ECC Error or Double ECC Error in info block"]
pub type SYSF_ECC_R = crate::BitReader;
#[doc = "Field `ECCIE` reader - ECC correction interrupt enable"]
pub type ECCIE_R = crate::BitReader;
#[doc = "Field `ECCIE` writer - ECC correction interrupt enable"]
pub type ECCIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ECCC` reader - ECC correction"]
pub type ECCC_R = crate::BitReader;
#[doc = "Field `ECCC` writer - ECC correction"]
pub type ECCC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ECCD` reader - ECC detection"]
pub type ECCD_R = crate::BitReader;
#[doc = "Field `ECCD` writer - ECC detection"]
pub type ECCD_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:13 - ECC fail address"]
    #[inline(always)]
    pub fn ADDR_ECC(&self) -> ADDR_ECC_R {
        ADDR_ECC_R::new((self.bits & 0x3fff) as u16)
    }
    #[doc = "Bit 20 - ECC fail for Corrected ECC Error or Double ECC Error in info block"]
    #[inline(always)]
    pub fn SYSF_ECC(&self) -> SYSF_ECC_R {
        SYSF_ECC_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 24 - ECC correction interrupt enable"]
    #[inline(always)]
    pub fn ECCIE(&self) -> ECCIE_R {
        ECCIE_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 30 - ECC correction"]
    #[inline(always)]
    pub fn ECCC(&self) -> ECCC_R {
        ECCC_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - ECC detection"]
    #[inline(always)]
    pub fn ECCD(&self) -> ECCD_R {
        ECCD_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 24 - ECC correction interrupt enable"]
    #[inline(always)]
    pub fn ECCIE(&mut self) -> ECCIE_W<'_, ECCR_SPEC> {
        ECCIE_W::new(self, 24)
    }
    #[doc = "Bit 30 - ECC correction"]
    #[inline(always)]
    pub fn ECCC(&mut self) -> ECCC_W<'_, ECCR_SPEC> {
        ECCC_W::new(self, 30)
    }
    #[doc = "Bit 31 - ECC detection"]
    #[inline(always)]
    pub fn ECCD(&mut self) -> ECCD_W<'_, ECCR_SPEC> {
        ECCD_W::new(self, 31)
    }
}
#[doc = "Flash ECC register\n\nYou can [`read`](crate::Reg::read) this register and get [`eccr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eccr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ECCR_SPEC;
impl crate::RegisterSpec for ECCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`eccr::R`](R) reader structure"]
impl crate::Readable for ECCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`eccr::W`](W) writer structure"]
impl crate::Writable for ECCR_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets ECCR to value 0"]
impl crate::Resettable for ECCR_SPEC {}

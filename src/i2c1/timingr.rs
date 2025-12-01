#[doc = "Register `TIMINGR` reader"]
pub type R = crate::R<TIMINGR_SPEC>;
#[doc = "Register `TIMINGR` writer"]
pub type W = crate::W<TIMINGR_SPEC>;
#[doc = "Field `SCLL` reader - SCL low period (master mode)"]
pub type SCLL_R = crate::FieldReader;
#[doc = "Field `SCLL` writer - SCL low period (master mode)"]
pub type SCLL_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SCLH` reader - SCL high period (master mode)"]
pub type SCLH_R = crate::FieldReader;
#[doc = "Field `SCLH` writer - SCL high period (master mode)"]
pub type SCLH_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SDADEL` reader - Data hold time"]
pub type SDADEL_R = crate::FieldReader;
#[doc = "Field `SDADEL` writer - Data hold time"]
pub type SDADEL_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SCLDEL` reader - Data setup time"]
pub type SCLDEL_R = crate::FieldReader;
#[doc = "Field `SCLDEL` writer - Data setup time"]
pub type SCLDEL_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PRESC` reader - Timing prescaler"]
pub type PRESC_R = crate::FieldReader;
#[doc = "Field `PRESC` writer - Timing prescaler"]
pub type PRESC_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:7 - SCL low period (master mode)"]
    #[inline(always)]
    pub fn SCLL(&self) -> SCLL_R {
        SCLL_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - SCL high period (master mode)"]
    #[inline(always)]
    pub fn SCLH(&self) -> SCLH_R {
        SCLH_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:19 - Data hold time"]
    #[inline(always)]
    pub fn SDADEL(&self) -> SDADEL_R {
        SDADEL_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - Data setup time"]
    #[inline(always)]
    pub fn SCLDEL(&self) -> SCLDEL_R {
        SCLDEL_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - Timing prescaler"]
    #[inline(always)]
    pub fn PRESC(&self) -> PRESC_R {
        PRESC_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - SCL low period (master mode)"]
    #[inline(always)]
    pub fn SCLL(&mut self) -> SCLL_W<'_, TIMINGR_SPEC> {
        SCLL_W::new(self, 0)
    }
    #[doc = "Bits 8:15 - SCL high period (master mode)"]
    #[inline(always)]
    pub fn SCLH(&mut self) -> SCLH_W<'_, TIMINGR_SPEC> {
        SCLH_W::new(self, 8)
    }
    #[doc = "Bits 16:19 - Data hold time"]
    #[inline(always)]
    pub fn SDADEL(&mut self) -> SDADEL_W<'_, TIMINGR_SPEC> {
        SDADEL_W::new(self, 16)
    }
    #[doc = "Bits 20:23 - Data setup time"]
    #[inline(always)]
    pub fn SCLDEL(&mut self) -> SCLDEL_W<'_, TIMINGR_SPEC> {
        SCLDEL_W::new(self, 20)
    }
    #[doc = "Bits 28:31 - Timing prescaler"]
    #[inline(always)]
    pub fn PRESC(&mut self) -> PRESC_W<'_, TIMINGR_SPEC> {
        PRESC_W::new(self, 28)
    }
}
#[doc = "Timing register\n\nYou can [`read`](crate::Reg::read) this register and get [`timingr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timingr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TIMINGR_SPEC;
impl crate::RegisterSpec for TIMINGR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`timingr::R`](R) reader structure"]
impl crate::Readable for TIMINGR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`timingr::W`](W) writer structure"]
impl crate::Writable for TIMINGR_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets TIMINGR to value 0"]
impl crate::Resettable for TIMINGR_SPEC {}

#[doc = "Register `CICR` writer"]
pub type W = crate::W<CICR_SPEC>;
#[doc = "Field `LSIRDYC` writer - LSI ready interrupt clear"]
pub type LSIRDYC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LSERDYC` writer - LSE ready interrupt clear"]
pub type LSERDYC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSIRDYC` writer - HSI ready interrupt clear"]
pub type HSIRDYC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSERDYC` writer - HSE ready interrupt clear"]
pub type HSERDYC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PLLSYSRDYC` writer - PLL ready interrupt clear"]
pub type PLLSYSRDYC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSSC` writer - Clock security system interrupt clear"]
pub type CSSC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LSECSSC` writer - LSE Clock security system interrupt clear"]
pub type LSECSSC_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - LSI ready interrupt clear"]
    #[inline(always)]
    pub fn LSIRDYC(&mut self) -> LSIRDYC_W<'_, CICR_SPEC> {
        LSIRDYC_W::new(self, 0)
    }
    #[doc = "Bit 1 - LSE ready interrupt clear"]
    #[inline(always)]
    pub fn LSERDYC(&mut self) -> LSERDYC_W<'_, CICR_SPEC> {
        LSERDYC_W::new(self, 1)
    }
    #[doc = "Bit 3 - HSI ready interrupt clear"]
    #[inline(always)]
    pub fn HSIRDYC(&mut self) -> HSIRDYC_W<'_, CICR_SPEC> {
        HSIRDYC_W::new(self, 3)
    }
    #[doc = "Bit 4 - HSE ready interrupt clear"]
    #[inline(always)]
    pub fn HSERDYC(&mut self) -> HSERDYC_W<'_, CICR_SPEC> {
        HSERDYC_W::new(self, 4)
    }
    #[doc = "Bit 5 - PLL ready interrupt clear"]
    #[inline(always)]
    pub fn PLLSYSRDYC(&mut self) -> PLLSYSRDYC_W<'_, CICR_SPEC> {
        PLLSYSRDYC_W::new(self, 5)
    }
    #[doc = "Bit 8 - Clock security system interrupt clear"]
    #[inline(always)]
    pub fn CSSC(&mut self) -> CSSC_W<'_, CICR_SPEC> {
        CSSC_W::new(self, 8)
    }
    #[doc = "Bit 9 - LSE Clock security system interrupt clear"]
    #[inline(always)]
    pub fn LSECSSC(&mut self) -> LSECSSC_W<'_, CICR_SPEC> {
        LSECSSC_W::new(self, 9)
    }
}
#[doc = "Clock interrupt clear register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cicr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CICR_SPEC;
impl crate::RegisterSpec for CICR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`cicr::W`](W) writer structure"]
impl crate::Writable for CICR_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets CICR to value 0"]
impl crate::Resettable for CICR_SPEC {}

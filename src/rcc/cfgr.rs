#[doc = "Register `CFGR` reader"]
pub type R = crate::R<CFGR_SPEC>;
#[doc = "Register `CFGR` writer"]
pub type W = crate::W<CFGR_SPEC>;
#[doc = "Field `SW` reader - System clock switch"]
pub type SW_R = crate::FieldReader;
#[doc = "Field `SW` writer - System clock switch"]
pub type SW_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SWS` reader - System clock switch status"]
pub type SWS_R = crate::FieldReader;
#[doc = "Field `HPRE` reader - AHB prescaler"]
pub type HPRE_R = crate::FieldReader;
#[doc = "Field `HPRE` writer - AHB prescaler"]
pub type HPRE_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PPRE` reader - APB prescaler"]
pub type PPRE_R = crate::FieldReader;
#[doc = "Field `PPRE` writer - APB prescaler"]
pub type PPRE_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `MCO2SEL` reader - MCO2SEL"]
pub type MCO2SEL_R = crate::FieldReader;
#[doc = "Field `MCO2SEL` writer - MCO2SEL"]
pub type MCO2SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `MCO2PRE` reader - MCO2PRE"]
pub type MCO2PRE_R = crate::FieldReader;
#[doc = "Field `MCO2PRE` writer - MCO2PRE"]
pub type MCO2PRE_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `MCOSEL` reader - Microcontroller clock output"]
pub type MCOSEL_R = crate::FieldReader;
#[doc = "Field `MCOSEL` writer - Microcontroller clock output"]
pub type MCOSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `MCOPRE` reader - Microcontroller clock output prescaler"]
pub type MCOPRE_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:2 - System clock switch"]
    #[inline(always)]
    pub fn SW(&self) -> SW_R {
        SW_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:5 - System clock switch status"]
    #[inline(always)]
    pub fn SWS(&self) -> SWS_R {
        SWS_R::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bits 8:11 - AHB prescaler"]
    #[inline(always)]
    pub fn HPRE(&self) -> HPRE_R {
        HPRE_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:14 - APB prescaler"]
    #[inline(always)]
    pub fn PPRE(&self) -> PPRE_R {
        PPRE_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 16:19 - MCO2SEL"]
    #[inline(always)]
    pub fn MCO2SEL(&self) -> MCO2SEL_R {
        MCO2SEL_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - MCO2PRE"]
    #[inline(always)]
    pub fn MCO2PRE(&self) -> MCO2PRE_R {
        MCO2PRE_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Microcontroller clock output"]
    #[inline(always)]
    pub fn MCOSEL(&self) -> MCOSEL_R {
        MCOSEL_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - Microcontroller clock output prescaler"]
    #[inline(always)]
    pub fn MCOPRE(&self) -> MCOPRE_R {
        MCOPRE_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - System clock switch"]
    #[inline(always)]
    pub fn SW(&mut self) -> SW_W<'_, CFGR_SPEC> {
        SW_W::new(self, 0)
    }
    #[doc = "Bits 8:11 - AHB prescaler"]
    #[inline(always)]
    pub fn HPRE(&mut self) -> HPRE_W<'_, CFGR_SPEC> {
        HPRE_W::new(self, 8)
    }
    #[doc = "Bits 12:14 - APB prescaler"]
    #[inline(always)]
    pub fn PPRE(&mut self) -> PPRE_W<'_, CFGR_SPEC> {
        PPRE_W::new(self, 12)
    }
    #[doc = "Bits 16:19 - MCO2SEL"]
    #[inline(always)]
    pub fn MCO2SEL(&mut self) -> MCO2SEL_W<'_, CFGR_SPEC> {
        MCO2SEL_W::new(self, 16)
    }
    #[doc = "Bits 20:23 - MCO2PRE"]
    #[inline(always)]
    pub fn MCO2PRE(&mut self) -> MCO2PRE_W<'_, CFGR_SPEC> {
        MCO2PRE_W::new(self, 20)
    }
    #[doc = "Bits 24:27 - Microcontroller clock output"]
    #[inline(always)]
    pub fn MCOSEL(&mut self) -> MCOSEL_W<'_, CFGR_SPEC> {
        MCOSEL_W::new(self, 24)
    }
}
#[doc = "Clock configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`cfgr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CFGR_SPEC;
impl crate::RegisterSpec for CFGR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfgr::R`](R) reader structure"]
impl crate::Readable for CFGR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cfgr::W`](W) writer structure"]
impl crate::Writable for CFGR_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets CFGR to value 0"]
impl crate::Resettable for CFGR_SPEC {}

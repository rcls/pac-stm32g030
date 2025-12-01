#[doc = "Register `IOPRSTR` reader"]
pub type R = crate::R<IOPRSTR_SPEC>;
#[doc = "Register `IOPRSTR` writer"]
pub type W = crate::W<IOPRSTR_SPEC>;
#[doc = "Field `GPIOARST` reader - GPIOARST"]
pub type GPIOARST_R = crate::BitReader;
#[doc = "Field `GPIOARST` writer - GPIOARST"]
pub type GPIOARST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIOBRST` reader - GPIOBRST"]
pub type GPIOBRST_R = crate::BitReader;
#[doc = "Field `GPIOBRST` writer - GPIOBRST"]
pub type GPIOBRST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIOCRST` reader - GPIOCRST"]
pub type GPIOCRST_R = crate::BitReader;
#[doc = "Field `GPIOCRST` writer - GPIOCRST"]
pub type GPIOCRST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIODRST` reader - GPIODRST"]
pub type GPIODRST_R = crate::BitReader;
#[doc = "Field `GPIODRST` writer - GPIODRST"]
pub type GPIODRST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIOERST` reader - GPIOERST"]
pub type GPIOERST_R = crate::BitReader;
#[doc = "Field `GPIOERST` writer - GPIOERST"]
pub type GPIOERST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIOFRST` reader - GPIOFRST"]
pub type GPIOFRST_R = crate::BitReader;
#[doc = "Field `GPIOFRST` writer - GPIOFRST"]
pub type GPIOFRST_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - GPIOARST"]
    #[inline(always)]
    pub fn GPIOARST(&self) -> GPIOARST_R {
        GPIOARST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - GPIOBRST"]
    #[inline(always)]
    pub fn GPIOBRST(&self) -> GPIOBRST_R {
        GPIOBRST_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - GPIOCRST"]
    #[inline(always)]
    pub fn GPIOCRST(&self) -> GPIOCRST_R {
        GPIOCRST_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - GPIODRST"]
    #[inline(always)]
    pub fn GPIODRST(&self) -> GPIODRST_R {
        GPIODRST_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - GPIOERST"]
    #[inline(always)]
    pub fn GPIOERST(&self) -> GPIOERST_R {
        GPIOERST_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - GPIOFRST"]
    #[inline(always)]
    pub fn GPIOFRST(&self) -> GPIOFRST_R {
        GPIOFRST_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - GPIOARST"]
    #[inline(always)]
    pub fn GPIOARST(&mut self) -> GPIOARST_W<'_, IOPRSTR_SPEC> {
        GPIOARST_W::new(self, 0)
    }
    #[doc = "Bit 1 - GPIOBRST"]
    #[inline(always)]
    pub fn GPIOBRST(&mut self) -> GPIOBRST_W<'_, IOPRSTR_SPEC> {
        GPIOBRST_W::new(self, 1)
    }
    #[doc = "Bit 2 - GPIOCRST"]
    #[inline(always)]
    pub fn GPIOCRST(&mut self) -> GPIOCRST_W<'_, IOPRSTR_SPEC> {
        GPIOCRST_W::new(self, 2)
    }
    #[doc = "Bit 3 - GPIODRST"]
    #[inline(always)]
    pub fn GPIODRST(&mut self) -> GPIODRST_W<'_, IOPRSTR_SPEC> {
        GPIODRST_W::new(self, 3)
    }
    #[doc = "Bit 4 - GPIOERST"]
    #[inline(always)]
    pub fn GPIOERST(&mut self) -> GPIOERST_W<'_, IOPRSTR_SPEC> {
        GPIOERST_W::new(self, 4)
    }
    #[doc = "Bit 5 - GPIOFRST"]
    #[inline(always)]
    pub fn GPIOFRST(&mut self) -> GPIOFRST_W<'_, IOPRSTR_SPEC> {
        GPIOFRST_W::new(self, 5)
    }
}
#[doc = "I/O port reset register\n\nYou can [`read`](crate::Reg::read) this register and get [`ioprstr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ioprstr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IOPRSTR_SPEC;
impl crate::RegisterSpec for IOPRSTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ioprstr::R`](R) reader structure"]
impl crate::Readable for IOPRSTR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ioprstr::W`](W) writer structure"]
impl crate::Writable for IOPRSTR_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets IOPRSTR to value 0"]
impl crate::Resettable for IOPRSTR_SPEC {}

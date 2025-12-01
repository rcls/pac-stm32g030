#[doc = "Register `IOPENR` reader"]
pub type R = crate::R<IOPENR_SPEC>;
#[doc = "Register `IOPENR` writer"]
pub type W = crate::W<IOPENR_SPEC>;
#[doc = "Field `GPIOAEN` reader - I/O port A clock enable during Sleep mode"]
pub type GPIOAEN_R = crate::BitReader;
#[doc = "Field `GPIOAEN` writer - I/O port A clock enable during Sleep mode"]
pub type GPIOAEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIOBEN` reader - I/O port B clock enable during Sleep mode"]
pub type GPIOBEN_R = crate::BitReader;
#[doc = "Field `GPIOBEN` writer - I/O port B clock enable during Sleep mode"]
pub type GPIOBEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIOCEN` reader - I/O port C clock enable during Sleep mode"]
pub type GPIOCEN_R = crate::BitReader;
#[doc = "Field `GPIOCEN` writer - I/O port C clock enable during Sleep mode"]
pub type GPIOCEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIODEN` reader - I/O port D clock enable during Sleep mode"]
pub type GPIODEN_R = crate::BitReader;
#[doc = "Field `GPIODEN` writer - I/O port D clock enable during Sleep mode"]
pub type GPIODEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIOEEN` reader - I/O port E clock enable during Sleep mode"]
pub type GPIOEEN_R = crate::BitReader;
#[doc = "Field `GPIOEEN` writer - I/O port E clock enable during Sleep mode"]
pub type GPIOEEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIOFEN` reader - I/O port F clock enable during Sleep mode"]
pub type GPIOFEN_R = crate::BitReader;
#[doc = "Field `GPIOFEN` writer - I/O port F clock enable during Sleep mode"]
pub type GPIOFEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - I/O port A clock enable during Sleep mode"]
    #[inline(always)]
    pub fn GPIOAEN(&self) -> GPIOAEN_R {
        GPIOAEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - I/O port B clock enable during Sleep mode"]
    #[inline(always)]
    pub fn GPIOBEN(&self) -> GPIOBEN_R {
        GPIOBEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - I/O port C clock enable during Sleep mode"]
    #[inline(always)]
    pub fn GPIOCEN(&self) -> GPIOCEN_R {
        GPIOCEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - I/O port D clock enable during Sleep mode"]
    #[inline(always)]
    pub fn GPIODEN(&self) -> GPIODEN_R {
        GPIODEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - I/O port E clock enable during Sleep mode"]
    #[inline(always)]
    pub fn GPIOEEN(&self) -> GPIOEEN_R {
        GPIOEEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - I/O port F clock enable during Sleep mode"]
    #[inline(always)]
    pub fn GPIOFEN(&self) -> GPIOFEN_R {
        GPIOFEN_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - I/O port A clock enable during Sleep mode"]
    #[inline(always)]
    pub fn GPIOAEN(&mut self) -> GPIOAEN_W<'_, IOPENR_SPEC> {
        GPIOAEN_W::new(self, 0)
    }
    #[doc = "Bit 1 - I/O port B clock enable during Sleep mode"]
    #[inline(always)]
    pub fn GPIOBEN(&mut self) -> GPIOBEN_W<'_, IOPENR_SPEC> {
        GPIOBEN_W::new(self, 1)
    }
    #[doc = "Bit 2 - I/O port C clock enable during Sleep mode"]
    #[inline(always)]
    pub fn GPIOCEN(&mut self) -> GPIOCEN_W<'_, IOPENR_SPEC> {
        GPIOCEN_W::new(self, 2)
    }
    #[doc = "Bit 3 - I/O port D clock enable during Sleep mode"]
    #[inline(always)]
    pub fn GPIODEN(&mut self) -> GPIODEN_W<'_, IOPENR_SPEC> {
        GPIODEN_W::new(self, 3)
    }
    #[doc = "Bit 4 - I/O port E clock enable during Sleep mode"]
    #[inline(always)]
    pub fn GPIOEEN(&mut self) -> GPIOEEN_W<'_, IOPENR_SPEC> {
        GPIOEEN_W::new(self, 4)
    }
    #[doc = "Bit 5 - I/O port F clock enable during Sleep mode"]
    #[inline(always)]
    pub fn GPIOFEN(&mut self) -> GPIOFEN_W<'_, IOPENR_SPEC> {
        GPIOFEN_W::new(self, 5)
    }
}
#[doc = "GPIO clock enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`iopenr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iopenr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IOPENR_SPEC;
impl crate::RegisterSpec for IOPENR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`iopenr::R`](R) reader structure"]
impl crate::Readable for IOPENR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`iopenr::W`](W) writer structure"]
impl crate::Writable for IOPENR_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets IOPENR to value 0"]
impl crate::Resettable for IOPENR_SPEC {}

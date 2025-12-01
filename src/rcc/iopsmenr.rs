#[doc = "Register `IOPSMENR` reader"]
pub type R = crate::R<IOPSMENR_SPEC>;
#[doc = "Register `IOPSMENR` writer"]
pub type W = crate::W<IOPSMENR_SPEC>;
#[doc = "Field `GPIOASMEN` reader - I/O port A clock enable during Sleep mode"]
pub type GPIOASMEN_R = crate::BitReader;
#[doc = "Field `GPIOASMEN` writer - I/O port A clock enable during Sleep mode"]
pub type GPIOASMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIOBSMEN` reader - I/O port B clock enable during Sleep mode"]
pub type GPIOBSMEN_R = crate::BitReader;
#[doc = "Field `GPIOBSMEN` writer - I/O port B clock enable during Sleep mode"]
pub type GPIOBSMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIOCSMEN` reader - I/O port C clock enable during Sleep mode"]
pub type GPIOCSMEN_R = crate::BitReader;
#[doc = "Field `GPIOCSMEN` writer - I/O port C clock enable during Sleep mode"]
pub type GPIOCSMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIODSMEN` reader - I/O port D clock enable during Sleep mode"]
pub type GPIODSMEN_R = crate::BitReader;
#[doc = "Field `GPIODSMEN` writer - I/O port D clock enable during Sleep mode"]
pub type GPIODSMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIOESMEN` reader - I/O port E clock enable during Sleep mode"]
pub type GPIOESMEN_R = crate::BitReader;
#[doc = "Field `GPIOESMEN` writer - I/O port E clock enable during Sleep mode"]
pub type GPIOESMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIOFSMEN` reader - I/O port F clock enable during Sleep mode"]
pub type GPIOFSMEN_R = crate::BitReader;
#[doc = "Field `GPIOFSMEN` writer - I/O port F clock enable during Sleep mode"]
pub type GPIOFSMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - I/O port A clock enable during Sleep mode"]
    #[inline(always)]
    pub fn GPIOASMEN(&self) -> GPIOASMEN_R {
        GPIOASMEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - I/O port B clock enable during Sleep mode"]
    #[inline(always)]
    pub fn GPIOBSMEN(&self) -> GPIOBSMEN_R {
        GPIOBSMEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - I/O port C clock enable during Sleep mode"]
    #[inline(always)]
    pub fn GPIOCSMEN(&self) -> GPIOCSMEN_R {
        GPIOCSMEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - I/O port D clock enable during Sleep mode"]
    #[inline(always)]
    pub fn GPIODSMEN(&self) -> GPIODSMEN_R {
        GPIODSMEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - I/O port E clock enable during Sleep mode"]
    #[inline(always)]
    pub fn GPIOESMEN(&self) -> GPIOESMEN_R {
        GPIOESMEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - I/O port F clock enable during Sleep mode"]
    #[inline(always)]
    pub fn GPIOFSMEN(&self) -> GPIOFSMEN_R {
        GPIOFSMEN_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - I/O port A clock enable during Sleep mode"]
    #[inline(always)]
    pub fn GPIOASMEN(&mut self) -> GPIOASMEN_W<'_, IOPSMENR_SPEC> {
        GPIOASMEN_W::new(self, 0)
    }
    #[doc = "Bit 1 - I/O port B clock enable during Sleep mode"]
    #[inline(always)]
    pub fn GPIOBSMEN(&mut self) -> GPIOBSMEN_W<'_, IOPSMENR_SPEC> {
        GPIOBSMEN_W::new(self, 1)
    }
    #[doc = "Bit 2 - I/O port C clock enable during Sleep mode"]
    #[inline(always)]
    pub fn GPIOCSMEN(&mut self) -> GPIOCSMEN_W<'_, IOPSMENR_SPEC> {
        GPIOCSMEN_W::new(self, 2)
    }
    #[doc = "Bit 3 - I/O port D clock enable during Sleep mode"]
    #[inline(always)]
    pub fn GPIODSMEN(&mut self) -> GPIODSMEN_W<'_, IOPSMENR_SPEC> {
        GPIODSMEN_W::new(self, 3)
    }
    #[doc = "Bit 4 - I/O port E clock enable during Sleep mode"]
    #[inline(always)]
    pub fn GPIOESMEN(&mut self) -> GPIOESMEN_W<'_, IOPSMENR_SPEC> {
        GPIOESMEN_W::new(self, 4)
    }
    #[doc = "Bit 5 - I/O port F clock enable during Sleep mode"]
    #[inline(always)]
    pub fn GPIOFSMEN(&mut self) -> GPIOFSMEN_W<'_, IOPSMENR_SPEC> {
        GPIOFSMEN_W::new(self, 5)
    }
}
#[doc = "GPIO in Sleep mode clock enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`iopsmenr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iopsmenr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IOPSMENR_SPEC;
impl crate::RegisterSpec for IOPSMENR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`iopsmenr::R`](R) reader structure"]
impl crate::Readable for IOPSMENR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`iopsmenr::W`](W) writer structure"]
impl crate::Writable for IOPSMENR_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets IOPSMENR to value 0x3f"]
impl crate::Resettable for IOPSMENR_SPEC {
    const RESET_VALUE: u32 = 0x3f;
}

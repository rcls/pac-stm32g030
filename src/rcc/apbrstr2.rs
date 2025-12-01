#[doc = "Register `APBRSTR2` reader"]
pub type R = crate::R<APBRSTR2_SPEC>;
#[doc = "Register `APBRSTR2` writer"]
pub type W = crate::W<APBRSTR2_SPEC>;
#[doc = "Field `SYSCFGRST` reader - SYSCFG, COMP and VREFBUF reset"]
pub type SYSCFGRST_R = crate::BitReader;
#[doc = "Field `SYSCFGRST` writer - SYSCFG, COMP and VREFBUF reset"]
pub type SYSCFGRST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM1RST` reader - TIM1 timer reset"]
pub type TIM1RST_R = crate::BitReader;
#[doc = "Field `TIM1RST` writer - TIM1 timer reset"]
pub type TIM1RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI1RST` reader - SPI1 reset"]
pub type SPI1RST_R = crate::BitReader;
#[doc = "Field `SPI1RST` writer - SPI1 reset"]
pub type SPI1RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USART1RST` reader - USART1 reset"]
pub type USART1RST_R = crate::BitReader;
#[doc = "Field `USART1RST` writer - USART1 reset"]
pub type USART1RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM14RST` reader - TIM14 timer reset"]
pub type TIM14RST_R = crate::BitReader;
#[doc = "Field `TIM14RST` writer - TIM14 timer reset"]
pub type TIM14RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM15RST` reader - TIM15 timer reset"]
pub type TIM15RST_R = crate::BitReader;
#[doc = "Field `TIM15RST` writer - TIM15 timer reset"]
pub type TIM15RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM16RST` reader - TIM16 timer reset"]
pub type TIM16RST_R = crate::BitReader;
#[doc = "Field `TIM16RST` writer - TIM16 timer reset"]
pub type TIM16RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM17RST` reader - TIM17 timer reset"]
pub type TIM17RST_R = crate::BitReader;
#[doc = "Field `TIM17RST` writer - TIM17 timer reset"]
pub type TIM17RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADCRST` reader - ADC reset"]
pub type ADCRST_R = crate::BitReader;
#[doc = "Field `ADCRST` writer - ADC reset"]
pub type ADCRST_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SYSCFG, COMP and VREFBUF reset"]
    #[inline(always)]
    pub fn SYSCFGRST(&self) -> SYSCFGRST_R {
        SYSCFGRST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 11 - TIM1 timer reset"]
    #[inline(always)]
    pub fn TIM1RST(&self) -> TIM1RST_R {
        TIM1RST_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - SPI1 reset"]
    #[inline(always)]
    pub fn SPI1RST(&self) -> SPI1RST_R {
        SPI1RST_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 14 - USART1 reset"]
    #[inline(always)]
    pub fn USART1RST(&self) -> USART1RST_R {
        USART1RST_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - TIM14 timer reset"]
    #[inline(always)]
    pub fn TIM14RST(&self) -> TIM14RST_R {
        TIM14RST_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - TIM15 timer reset"]
    #[inline(always)]
    pub fn TIM15RST(&self) -> TIM15RST_R {
        TIM15RST_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - TIM16 timer reset"]
    #[inline(always)]
    pub fn TIM16RST(&self) -> TIM16RST_R {
        TIM16RST_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - TIM17 timer reset"]
    #[inline(always)]
    pub fn TIM17RST(&self) -> TIM17RST_R {
        TIM17RST_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 20 - ADC reset"]
    #[inline(always)]
    pub fn ADCRST(&self) -> ADCRST_R {
        ADCRST_R::new(((self.bits >> 20) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SYSCFG, COMP and VREFBUF reset"]
    #[inline(always)]
    pub fn SYSCFGRST(&mut self) -> SYSCFGRST_W<'_, APBRSTR2_SPEC> {
        SYSCFGRST_W::new(self, 0)
    }
    #[doc = "Bit 11 - TIM1 timer reset"]
    #[inline(always)]
    pub fn TIM1RST(&mut self) -> TIM1RST_W<'_, APBRSTR2_SPEC> {
        TIM1RST_W::new(self, 11)
    }
    #[doc = "Bit 12 - SPI1 reset"]
    #[inline(always)]
    pub fn SPI1RST(&mut self) -> SPI1RST_W<'_, APBRSTR2_SPEC> {
        SPI1RST_W::new(self, 12)
    }
    #[doc = "Bit 14 - USART1 reset"]
    #[inline(always)]
    pub fn USART1RST(&mut self) -> USART1RST_W<'_, APBRSTR2_SPEC> {
        USART1RST_W::new(self, 14)
    }
    #[doc = "Bit 15 - TIM14 timer reset"]
    #[inline(always)]
    pub fn TIM14RST(&mut self) -> TIM14RST_W<'_, APBRSTR2_SPEC> {
        TIM14RST_W::new(self, 15)
    }
    #[doc = "Bit 16 - TIM15 timer reset"]
    #[inline(always)]
    pub fn TIM15RST(&mut self) -> TIM15RST_W<'_, APBRSTR2_SPEC> {
        TIM15RST_W::new(self, 16)
    }
    #[doc = "Bit 17 - TIM16 timer reset"]
    #[inline(always)]
    pub fn TIM16RST(&mut self) -> TIM16RST_W<'_, APBRSTR2_SPEC> {
        TIM16RST_W::new(self, 17)
    }
    #[doc = "Bit 18 - TIM17 timer reset"]
    #[inline(always)]
    pub fn TIM17RST(&mut self) -> TIM17RST_W<'_, APBRSTR2_SPEC> {
        TIM17RST_W::new(self, 18)
    }
    #[doc = "Bit 20 - ADC reset"]
    #[inline(always)]
    pub fn ADCRST(&mut self) -> ADCRST_W<'_, APBRSTR2_SPEC> {
        ADCRST_W::new(self, 20)
    }
}
#[doc = "APB peripheral reset register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`apbrstr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apbrstr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct APBRSTR2_SPEC;
impl crate::RegisterSpec for APBRSTR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apbrstr2::R`](R) reader structure"]
impl crate::Readable for APBRSTR2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`apbrstr2::W`](W) writer structure"]
impl crate::Writable for APBRSTR2_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets APBRSTR2 to value 0"]
impl crate::Resettable for APBRSTR2_SPEC {}

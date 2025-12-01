#[doc = "Register `APBSMENR2` reader"]
pub type R = crate::R<APBSMENR2_SPEC>;
#[doc = "Register `APBSMENR2` writer"]
pub type W = crate::W<APBSMENR2_SPEC>;
#[doc = "Field `SYSCFGSMEN` reader - SYSCFG, COMP and VREFBUF clock enable during Sleep mode"]
pub type SYSCFGSMEN_R = crate::BitReader;
#[doc = "Field `SYSCFGSMEN` writer - SYSCFG, COMP and VREFBUF clock enable during Sleep mode"]
pub type SYSCFGSMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM1SMEN` reader - TIM1 timer clock enable during Sleep mode"]
pub type TIM1SMEN_R = crate::BitReader;
#[doc = "Field `TIM1SMEN` writer - TIM1 timer clock enable during Sleep mode"]
pub type TIM1SMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI1SMEN` reader - SPI1 clock enable during Sleep mode"]
pub type SPI1SMEN_R = crate::BitReader;
#[doc = "Field `SPI1SMEN` writer - SPI1 clock enable during Sleep mode"]
pub type SPI1SMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USART1SMEN` reader - USART1 clock enable during Sleep mode"]
pub type USART1SMEN_R = crate::BitReader;
#[doc = "Field `USART1SMEN` writer - USART1 clock enable during Sleep mode"]
pub type USART1SMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM14SMEN` reader - TIM14 timer clock enable during Sleep mode"]
pub type TIM14SMEN_R = crate::BitReader;
#[doc = "Field `TIM14SMEN` writer - TIM14 timer clock enable during Sleep mode"]
pub type TIM14SMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM15SMEN` reader - TIM15 timer clock enable during Sleep mode"]
pub type TIM15SMEN_R = crate::BitReader;
#[doc = "Field `TIM15SMEN` writer - TIM15 timer clock enable during Sleep mode"]
pub type TIM15SMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM16SMEN` reader - TIM16 timer clock enable during Sleep mode"]
pub type TIM16SMEN_R = crate::BitReader;
#[doc = "Field `TIM16SMEN` writer - TIM16 timer clock enable during Sleep mode"]
pub type TIM16SMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM17SMEN` reader - TIM16 timer clock enable during Sleep mode"]
pub type TIM17SMEN_R = crate::BitReader;
#[doc = "Field `TIM17SMEN` writer - TIM16 timer clock enable during Sleep mode"]
pub type TIM17SMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADCSMEN` reader - ADC clock enable during Sleep mode"]
pub type ADCSMEN_R = crate::BitReader;
#[doc = "Field `ADCSMEN` writer - ADC clock enable during Sleep mode"]
pub type ADCSMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SYSCFG, COMP and VREFBUF clock enable during Sleep mode"]
    #[inline(always)]
    pub fn SYSCFGSMEN(&self) -> SYSCFGSMEN_R {
        SYSCFGSMEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 11 - TIM1 timer clock enable during Sleep mode"]
    #[inline(always)]
    pub fn TIM1SMEN(&self) -> TIM1SMEN_R {
        TIM1SMEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - SPI1 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn SPI1SMEN(&self) -> SPI1SMEN_R {
        SPI1SMEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 14 - USART1 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn USART1SMEN(&self) -> USART1SMEN_R {
        USART1SMEN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - TIM14 timer clock enable during Sleep mode"]
    #[inline(always)]
    pub fn TIM14SMEN(&self) -> TIM14SMEN_R {
        TIM14SMEN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - TIM15 timer clock enable during Sleep mode"]
    #[inline(always)]
    pub fn TIM15SMEN(&self) -> TIM15SMEN_R {
        TIM15SMEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - TIM16 timer clock enable during Sleep mode"]
    #[inline(always)]
    pub fn TIM16SMEN(&self) -> TIM16SMEN_R {
        TIM16SMEN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - TIM16 timer clock enable during Sleep mode"]
    #[inline(always)]
    pub fn TIM17SMEN(&self) -> TIM17SMEN_R {
        TIM17SMEN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 20 - ADC clock enable during Sleep mode"]
    #[inline(always)]
    pub fn ADCSMEN(&self) -> ADCSMEN_R {
        ADCSMEN_R::new(((self.bits >> 20) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SYSCFG, COMP and VREFBUF clock enable during Sleep mode"]
    #[inline(always)]
    pub fn SYSCFGSMEN(&mut self) -> SYSCFGSMEN_W<'_, APBSMENR2_SPEC> {
        SYSCFGSMEN_W::new(self, 0)
    }
    #[doc = "Bit 11 - TIM1 timer clock enable during Sleep mode"]
    #[inline(always)]
    pub fn TIM1SMEN(&mut self) -> TIM1SMEN_W<'_, APBSMENR2_SPEC> {
        TIM1SMEN_W::new(self, 11)
    }
    #[doc = "Bit 12 - SPI1 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn SPI1SMEN(&mut self) -> SPI1SMEN_W<'_, APBSMENR2_SPEC> {
        SPI1SMEN_W::new(self, 12)
    }
    #[doc = "Bit 14 - USART1 clock enable during Sleep mode"]
    #[inline(always)]
    pub fn USART1SMEN(&mut self) -> USART1SMEN_W<'_, APBSMENR2_SPEC> {
        USART1SMEN_W::new(self, 14)
    }
    #[doc = "Bit 15 - TIM14 timer clock enable during Sleep mode"]
    #[inline(always)]
    pub fn TIM14SMEN(&mut self) -> TIM14SMEN_W<'_, APBSMENR2_SPEC> {
        TIM14SMEN_W::new(self, 15)
    }
    #[doc = "Bit 16 - TIM15 timer clock enable during Sleep mode"]
    #[inline(always)]
    pub fn TIM15SMEN(&mut self) -> TIM15SMEN_W<'_, APBSMENR2_SPEC> {
        TIM15SMEN_W::new(self, 16)
    }
    #[doc = "Bit 17 - TIM16 timer clock enable during Sleep mode"]
    #[inline(always)]
    pub fn TIM16SMEN(&mut self) -> TIM16SMEN_W<'_, APBSMENR2_SPEC> {
        TIM16SMEN_W::new(self, 17)
    }
    #[doc = "Bit 18 - TIM16 timer clock enable during Sleep mode"]
    #[inline(always)]
    pub fn TIM17SMEN(&mut self) -> TIM17SMEN_W<'_, APBSMENR2_SPEC> {
        TIM17SMEN_W::new(self, 18)
    }
    #[doc = "Bit 20 - ADC clock enable during Sleep mode"]
    #[inline(always)]
    pub fn ADCSMEN(&mut self) -> ADCSMEN_W<'_, APBSMENR2_SPEC> {
        ADCSMEN_W::new(self, 20)
    }
}
#[doc = "APB peripheral clock enable in Sleep mode register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`apbsmenr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apbsmenr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct APBSMENR2_SPEC;
impl crate::RegisterSpec for APBSMENR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apbsmenr2::R`](R) reader structure"]
impl crate::Readable for APBSMENR2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`apbsmenr2::W`](W) writer structure"]
impl crate::Writable for APBSMENR2_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets APBSMENR2 to value 0x0017_d801"]
impl crate::Resettable for APBSMENR2_SPEC {
    const RESET_VALUE: u32 = 0x0017_d801;
}

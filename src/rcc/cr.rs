#[doc = "Register `CR` reader"]
pub type R = crate::R<CR_SPEC>;
#[doc = "Register `CR` writer"]
pub type W = crate::W<CR_SPEC>;
#[doc = "Field `HSION` reader - HSI16 clock enable"]
pub type HSION_R = crate::BitReader;
#[doc = "Field `HSION` writer - HSI16 clock enable"]
pub type HSION_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSIKERON` reader - HSI16 always enable for peripheral kernels"]
pub type HSIKERON_R = crate::BitReader;
#[doc = "Field `HSIKERON` writer - HSI16 always enable for peripheral kernels"]
pub type HSIKERON_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSIRDY` reader - HSI16 clock ready flag"]
pub type HSIRDY_R = crate::BitReader;
#[doc = "Field `HSIRDY` writer - HSI16 clock ready flag"]
pub type HSIRDY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSIDIV` reader - HSI16 clock division factor"]
pub type HSIDIV_R = crate::FieldReader;
#[doc = "Field `HSIDIV` writer - HSI16 clock division factor"]
pub type HSIDIV_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `HSEON` reader - HSE clock enable"]
pub type HSEON_R = crate::BitReader;
#[doc = "Field `HSEON` writer - HSE clock enable"]
pub type HSEON_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSERDY` reader - HSE clock ready flag"]
pub type HSERDY_R = crate::BitReader;
#[doc = "Field `HSERDY` writer - HSE clock ready flag"]
pub type HSERDY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSEBYP` reader - HSE crystal oscillator bypass"]
pub type HSEBYP_R = crate::BitReader;
#[doc = "Field `HSEBYP` writer - HSE crystal oscillator bypass"]
pub type HSEBYP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSSON` reader - Clock security system enable"]
pub type CSSON_R = crate::BitReader;
#[doc = "Field `CSSON` writer - Clock security system enable"]
pub type CSSON_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PLLON` reader - PLL enable"]
pub type PLLON_R = crate::BitReader;
#[doc = "Field `PLLON` writer - PLL enable"]
pub type PLLON_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PLLRDY` reader - PLL clock ready flag"]
pub type PLLRDY_R = crate::BitReader;
#[doc = "Field `PLLRDY` writer - PLL clock ready flag"]
pub type PLLRDY_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 8 - HSI16 clock enable"]
    #[inline(always)]
    pub fn HSION(&self) -> HSION_R {
        HSION_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - HSI16 always enable for peripheral kernels"]
    #[inline(always)]
    pub fn HSIKERON(&self) -> HSIKERON_R {
        HSIKERON_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - HSI16 clock ready flag"]
    #[inline(always)]
    pub fn HSIRDY(&self) -> HSIRDY_R {
        HSIRDY_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 11:13 - HSI16 clock division factor"]
    #[inline(always)]
    pub fn HSIDIV(&self) -> HSIDIV_R {
        HSIDIV_R::new(((self.bits >> 11) & 7) as u8)
    }
    #[doc = "Bit 16 - HSE clock enable"]
    #[inline(always)]
    pub fn HSEON(&self) -> HSEON_R {
        HSEON_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - HSE clock ready flag"]
    #[inline(always)]
    pub fn HSERDY(&self) -> HSERDY_R {
        HSERDY_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - HSE crystal oscillator bypass"]
    #[inline(always)]
    pub fn HSEBYP(&self) -> HSEBYP_R {
        HSEBYP_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Clock security system enable"]
    #[inline(always)]
    pub fn CSSON(&self) -> CSSON_R {
        CSSON_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 24 - PLL enable"]
    #[inline(always)]
    pub fn PLLON(&self) -> PLLON_R {
        PLLON_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - PLL clock ready flag"]
    #[inline(always)]
    pub fn PLLRDY(&self) -> PLLRDY_R {
        PLLRDY_R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 8 - HSI16 clock enable"]
    #[inline(always)]
    pub fn HSION(&mut self) -> HSION_W<'_, CR_SPEC> {
        HSION_W::new(self, 8)
    }
    #[doc = "Bit 9 - HSI16 always enable for peripheral kernels"]
    #[inline(always)]
    pub fn HSIKERON(&mut self) -> HSIKERON_W<'_, CR_SPEC> {
        HSIKERON_W::new(self, 9)
    }
    #[doc = "Bit 10 - HSI16 clock ready flag"]
    #[inline(always)]
    pub fn HSIRDY(&mut self) -> HSIRDY_W<'_, CR_SPEC> {
        HSIRDY_W::new(self, 10)
    }
    #[doc = "Bits 11:13 - HSI16 clock division factor"]
    #[inline(always)]
    pub fn HSIDIV(&mut self) -> HSIDIV_W<'_, CR_SPEC> {
        HSIDIV_W::new(self, 11)
    }
    #[doc = "Bit 16 - HSE clock enable"]
    #[inline(always)]
    pub fn HSEON(&mut self) -> HSEON_W<'_, CR_SPEC> {
        HSEON_W::new(self, 16)
    }
    #[doc = "Bit 17 - HSE clock ready flag"]
    #[inline(always)]
    pub fn HSERDY(&mut self) -> HSERDY_W<'_, CR_SPEC> {
        HSERDY_W::new(self, 17)
    }
    #[doc = "Bit 18 - HSE crystal oscillator bypass"]
    #[inline(always)]
    pub fn HSEBYP(&mut self) -> HSEBYP_W<'_, CR_SPEC> {
        HSEBYP_W::new(self, 18)
    }
    #[doc = "Bit 19 - Clock security system enable"]
    #[inline(always)]
    pub fn CSSON(&mut self) -> CSSON_W<'_, CR_SPEC> {
        CSSON_W::new(self, 19)
    }
    #[doc = "Bit 24 - PLL enable"]
    #[inline(always)]
    pub fn PLLON(&mut self) -> PLLON_W<'_, CR_SPEC> {
        PLLON_W::new(self, 24)
    }
    #[doc = "Bit 25 - PLL clock ready flag"]
    #[inline(always)]
    pub fn PLLRDY(&mut self) -> PLLRDY_W<'_, CR_SPEC> {
        PLLRDY_W::new(self, 25)
    }
}
#[doc = "Clock control register\n\nYou can [`read`](crate::Reg::read) this register and get [`cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CR_SPEC;
impl crate::RegisterSpec for CR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr::R`](R) reader structure"]
impl crate::Readable for CR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cr::W`](W) writer structure"]
impl crate::Writable for CR_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets CR to value 0x63"]
impl crate::Resettable for CR_SPEC {
    const RESET_VALUE: u32 = 0x63;
}

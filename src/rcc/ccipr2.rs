#[doc = "Register `CCIPR2` reader"]
pub type R = crate::R<CCIPR2_SPEC>;
#[doc = "Register `CCIPR2` writer"]
pub type W = crate::W<CCIPR2_SPEC>;
#[doc = "Field `I2S1SEL` reader - 2S1SEL"]
pub type I2S1SEL_R = crate::FieldReader;
#[doc = "Field `I2S1SEL` writer - 2S1SEL"]
pub type I2S1SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `I2S2SEL` reader - I2S2SEL"]
pub type I2S2SEL_R = crate::FieldReader;
#[doc = "Field `I2S2SEL` writer - I2S2SEL"]
pub type I2S2SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `USBSEL` reader - USBSEL"]
pub type USBSEL_R = crate::FieldReader;
#[doc = "Field `USBSEL` writer - USBSEL"]
pub type USBSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - 2S1SEL"]
    #[inline(always)]
    pub fn I2S1SEL(&self) -> I2S1SEL_R {
        I2S1SEL_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - I2S2SEL"]
    #[inline(always)]
    pub fn I2S2SEL(&self) -> I2S2SEL_R {
        I2S2SEL_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 12:13 - USBSEL"]
    #[inline(always)]
    pub fn USBSEL(&self) -> USBSEL_R {
        USBSEL_R::new(((self.bits >> 12) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - 2S1SEL"]
    #[inline(always)]
    pub fn I2S1SEL(&mut self) -> I2S1SEL_W<'_, CCIPR2_SPEC> {
        I2S1SEL_W::new(self, 0)
    }
    #[doc = "Bits 2:3 - I2S2SEL"]
    #[inline(always)]
    pub fn I2S2SEL(&mut self) -> I2S2SEL_W<'_, CCIPR2_SPEC> {
        I2S2SEL_W::new(self, 2)
    }
    #[doc = "Bits 12:13 - USBSEL"]
    #[inline(always)]
    pub fn USBSEL(&mut self) -> USBSEL_W<'_, CCIPR2_SPEC> {
        USBSEL_W::new(self, 12)
    }
}
#[doc = "Peripherals independent clock configuration register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`ccipr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccipr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CCIPR2_SPEC;
impl crate::RegisterSpec for CCIPR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ccipr2::R`](R) reader structure"]
impl crate::Readable for CCIPR2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ccipr2::W`](W) writer structure"]
impl crate::Writable for CCIPR2_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets CCIPR2 to value 0"]
impl crate::Resettable for CCIPR2_SPEC {}

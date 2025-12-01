#[doc = "Register `I2SPR` reader"]
pub type R = crate::R<I2SPR_SPEC>;
#[doc = "Register `I2SPR` writer"]
pub type W = crate::W<I2SPR_SPEC>;
#[doc = "Field `I2SDIV` reader - linear prescaler"]
pub type I2SDIV_R = crate::FieldReader;
#[doc = "Field `I2SDIV` writer - linear prescaler"]
pub type I2SDIV_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `ODD` reader - Odd factor for the prescaler"]
pub type ODD_R = crate::BitReader;
#[doc = "Field `ODD` writer - Odd factor for the prescaler"]
pub type ODD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCKOE` reader - Master clock output enable"]
pub type MCKOE_R = crate::BitReader;
#[doc = "Field `MCKOE` writer - Master clock output enable"]
pub type MCKOE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - linear prescaler"]
    #[inline(always)]
    pub fn I2SDIV(&self) -> I2SDIV_R {
        I2SDIV_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - Odd factor for the prescaler"]
    #[inline(always)]
    pub fn ODD(&self) -> ODD_R {
        ODD_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Master clock output enable"]
    #[inline(always)]
    pub fn MCKOE(&self) -> MCKOE_R {
        MCKOE_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - linear prescaler"]
    #[inline(always)]
    pub fn I2SDIV(&mut self) -> I2SDIV_W<'_, I2SPR_SPEC> {
        I2SDIV_W::new(self, 0)
    }
    #[doc = "Bit 8 - Odd factor for the prescaler"]
    #[inline(always)]
    pub fn ODD(&mut self) -> ODD_W<'_, I2SPR_SPEC> {
        ODD_W::new(self, 8)
    }
    #[doc = "Bit 9 - Master clock output enable"]
    #[inline(always)]
    pub fn MCKOE(&mut self) -> MCKOE_W<'_, I2SPR_SPEC> {
        MCKOE_W::new(self, 9)
    }
}
#[doc = "prescaler register\n\nYou can [`read`](crate::Reg::read) this register and get [`i2spr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2spr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2SPR_SPEC;
impl crate::RegisterSpec for I2SPR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2spr::R`](R) reader structure"]
impl crate::Readable for I2SPR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`i2spr::W`](W) writer structure"]
impl crate::Writable for I2SPR_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets I2SPR to value 0"]
impl crate::Resettable for I2SPR_SPEC {}

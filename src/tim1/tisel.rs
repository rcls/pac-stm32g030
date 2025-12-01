#[doc = "Register `TISEL` reader"]
pub type R = crate::R<TISEL_SPEC>;
#[doc = "Register `TISEL` writer"]
pub type W = crate::W<TISEL_SPEC>;
#[doc = "Field `TI1SEL3_0` reader - selects TI1\\[0\\] to TI1\\[15\\] input"]
pub type TI1SEL3_0_R = crate::FieldReader;
#[doc = "Field `TI1SEL3_0` writer - selects TI1\\[0\\] to TI1\\[15\\] input"]
pub type TI1SEL3_0_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `TI2SEL3_0` reader - selects TI2\\[0\\] to TI2\\[15\\] input"]
pub type TI2SEL3_0_R = crate::FieldReader;
#[doc = "Field `TI2SEL3_0` writer - selects TI2\\[0\\] to TI2\\[15\\] input"]
pub type TI2SEL3_0_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `TI3SEL3_0` reader - selects TI3\\[0\\] to TI3\\[15\\] input"]
pub type TI3SEL3_0_R = crate::FieldReader;
#[doc = "Field `TI3SEL3_0` writer - selects TI3\\[0\\] to TI3\\[15\\] input"]
pub type TI3SEL3_0_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `TI4SEL3_0` reader - selects TI4\\[0\\] to TI4\\[15\\] input"]
pub type TI4SEL3_0_R = crate::FieldReader;
#[doc = "Field `TI4SEL3_0` writer - selects TI4\\[0\\] to TI4\\[15\\] input"]
pub type TI4SEL3_0_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - selects TI1\\[0\\] to TI1\\[15\\] input"]
    #[inline(always)]
    pub fn TI1SEL3_0(&self) -> TI1SEL3_0_R {
        TI1SEL3_0_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - selects TI2\\[0\\] to TI2\\[15\\] input"]
    #[inline(always)]
    pub fn TI2SEL3_0(&self) -> TI2SEL3_0_R {
        TI2SEL3_0_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - selects TI3\\[0\\] to TI3\\[15\\] input"]
    #[inline(always)]
    pub fn TI3SEL3_0(&self) -> TI3SEL3_0_R {
        TI3SEL3_0_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - selects TI4\\[0\\] to TI4\\[15\\] input"]
    #[inline(always)]
    pub fn TI4SEL3_0(&self) -> TI4SEL3_0_R {
        TI4SEL3_0_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - selects TI1\\[0\\] to TI1\\[15\\] input"]
    #[inline(always)]
    pub fn TI1SEL3_0(&mut self) -> TI1SEL3_0_W<'_, TISEL_SPEC> {
        TI1SEL3_0_W::new(self, 0)
    }
    #[doc = "Bits 8:11 - selects TI2\\[0\\] to TI2\\[15\\] input"]
    #[inline(always)]
    pub fn TI2SEL3_0(&mut self) -> TI2SEL3_0_W<'_, TISEL_SPEC> {
        TI2SEL3_0_W::new(self, 8)
    }
    #[doc = "Bits 16:19 - selects TI3\\[0\\] to TI3\\[15\\] input"]
    #[inline(always)]
    pub fn TI3SEL3_0(&mut self) -> TI3SEL3_0_W<'_, TISEL_SPEC> {
        TI3SEL3_0_W::new(self, 16)
    }
    #[doc = "Bits 24:27 - selects TI4\\[0\\] to TI4\\[15\\] input"]
    #[inline(always)]
    pub fn TI4SEL3_0(&mut self) -> TI4SEL3_0_W<'_, TISEL_SPEC> {
        TI4SEL3_0_W::new(self, 24)
    }
}
#[doc = "TIM1 timer input selection register\n\nYou can [`read`](crate::Reg::read) this register and get [`tisel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tisel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TISEL_SPEC;
impl crate::RegisterSpec for TISEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tisel::R`](R) reader structure"]
impl crate::Readable for TISEL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tisel::W`](W) writer structure"]
impl crate::Writable for TISEL_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets TISEL to value 0"]
impl crate::Resettable for TISEL_SPEC {}

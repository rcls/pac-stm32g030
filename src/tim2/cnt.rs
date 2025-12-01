#[doc = "Register `CNT` reader"]
pub type R = crate::R<CNT_SPEC>;
#[doc = "Register `CNT` writer"]
pub type W = crate::W<CNT_SPEC>;
#[doc = "Field `CNT_L` reader - Low counter value"]
pub type CNT_L_R = crate::FieldReader<u16>;
#[doc = "Field `CNT_L` writer - Low counter value"]
pub type CNT_L_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `CNT_H` reader - High counter value (TIM2 only)"]
pub type CNT_H_R = crate::FieldReader<u16>;
#[doc = "Field `CNT_H` writer - High counter value (TIM2 only)"]
pub type CNT_H_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Low counter value"]
    #[inline(always)]
    pub fn CNT_L(&self) -> CNT_L_R {
        CNT_L_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - High counter value (TIM2 only)"]
    #[inline(always)]
    pub fn CNT_H(&self) -> CNT_H_R {
        CNT_H_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Low counter value"]
    #[inline(always)]
    pub fn CNT_L(&mut self) -> CNT_L_W<'_, CNT_SPEC> {
        CNT_L_W::new(self, 0)
    }
    #[doc = "Bits 16:31 - High counter value (TIM2 only)"]
    #[inline(always)]
    pub fn CNT_H(&mut self) -> CNT_H_W<'_, CNT_SPEC> {
        CNT_H_W::new(self, 16)
    }
}
#[doc = "counter\n\nYou can [`read`](crate::Reg::read) this register and get [`cnt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cnt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CNT_SPEC;
impl crate::RegisterSpec for CNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cnt::R`](R) reader structure"]
impl crate::Readable for CNT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cnt::W`](W) writer structure"]
impl crate::Writable for CNT_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets CNT to value 0"]
impl crate::Resettable for CNT_SPEC {}

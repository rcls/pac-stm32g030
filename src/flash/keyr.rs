#[doc = "Register `KEYR` writer"]
pub type W = crate::W<KEYR_SPEC>;
#[doc = "Field `KEYR` writer - KEYR"]
pub type KEYR_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl W {
    #[doc = "Bits 0:31 - KEYR"]
    #[inline(always)]
    pub fn KEYR(&mut self) -> KEYR_W<'_, KEYR_SPEC> {
        KEYR_W::new(self, 0)
    }
}
#[doc = "Flash key register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`keyr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct KEYR_SPEC;
impl crate::RegisterSpec for KEYR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`keyr::W`](W) writer structure"]
impl crate::Writable for KEYR_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets KEYR to value 0"]
impl crate::Resettable for KEYR_SPEC {}

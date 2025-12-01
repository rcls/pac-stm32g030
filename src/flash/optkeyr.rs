#[doc = "Register `OPTKEYR` writer"]
pub type W = crate::W<OPTKEYR_SPEC>;
#[doc = "Field `OPTKEYR` writer - Option byte key"]
pub type OPTKEYR_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl W {
    #[doc = "Bits 0:31 - Option byte key"]
    #[inline(always)]
    pub fn OPTKEYR(&mut self) -> OPTKEYR_W<'_, OPTKEYR_SPEC> {
        OPTKEYR_W::new(self, 0)
    }
}
#[doc = "Option byte key register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`optkeyr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OPTKEYR_SPEC;
impl crate::RegisterSpec for OPTKEYR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`optkeyr::W`](W) writer structure"]
impl crate::Writable for OPTKEYR_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets OPTKEYR to value 0"]
impl crate::Resettable for OPTKEYR_SPEC {}

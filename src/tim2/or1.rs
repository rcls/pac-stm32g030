#[doc = "Register `OR1` reader"]
pub type R = crate::R<OR1_SPEC>;
#[doc = "Register `OR1` writer"]
pub type W = crate::W<OR1_SPEC>;
#[doc = "Field `IOCREF_CLR` reader - IOCREF_CLR"]
pub type IOCREF_CLR_R = crate::BitReader;
#[doc = "Field `IOCREF_CLR` writer - IOCREF_CLR"]
pub type IOCREF_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - IOCREF_CLR"]
    #[inline(always)]
    pub fn IOCREF_CLR(&self) -> IOCREF_CLR_R {
        IOCREF_CLR_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - IOCREF_CLR"]
    #[inline(always)]
    pub fn IOCREF_CLR(&mut self) -> IOCREF_CLR_W<'_, OR1_SPEC> {
        IOCREF_CLR_W::new(self, 0)
    }
}
#[doc = "TIM option register\n\nYou can [`read`](crate::Reg::read) this register and get [`or1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`or1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OR1_SPEC;
impl crate::RegisterSpec for OR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`or1::R`](R) reader structure"]
impl crate::Readable for OR1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`or1::W`](W) writer structure"]
impl crate::Writable for OR1_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets OR1 to value 0"]
impl crate::Resettable for OR1_SPEC {}

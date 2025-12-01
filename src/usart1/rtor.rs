#[doc = "Register `RTOR` reader"]
pub type R = crate::R<RTOR_SPEC>;
#[doc = "Register `RTOR` writer"]
pub type W = crate::W<RTOR_SPEC>;
#[doc = "Field `RTO` reader - Receiver timeout value"]
pub type RTO_R = crate::FieldReader<u32>;
#[doc = "Field `RTO` writer - Receiver timeout value"]
pub type RTO_W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
#[doc = "Field `BLEN` reader - Block Length"]
pub type BLEN_R = crate::FieldReader;
#[doc = "Field `BLEN` writer - Block Length"]
pub type BLEN_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:23 - Receiver timeout value"]
    #[inline(always)]
    pub fn RTO(&self) -> RTO_R {
        RTO_R::new(self.bits & 0x00ff_ffff)
    }
    #[doc = "Bits 24:31 - Block Length"]
    #[inline(always)]
    pub fn BLEN(&self) -> BLEN_R {
        BLEN_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:23 - Receiver timeout value"]
    #[inline(always)]
    pub fn RTO(&mut self) -> RTO_W<'_, RTOR_SPEC> {
        RTO_W::new(self, 0)
    }
    #[doc = "Bits 24:31 - Block Length"]
    #[inline(always)]
    pub fn BLEN(&mut self) -> BLEN_W<'_, RTOR_SPEC> {
        BLEN_W::new(self, 24)
    }
}
#[doc = "Receiver timeout register\n\nYou can [`read`](crate::Reg::read) this register and get [`rtor::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtor::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RTOR_SPEC;
impl crate::RegisterSpec for RTOR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rtor::R`](R) reader structure"]
impl crate::Readable for RTOR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rtor::W`](W) writer structure"]
impl crate::Writable for RTOR_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets RTOR to value 0"]
impl crate::Resettable for RTOR_SPEC {}

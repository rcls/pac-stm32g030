#[doc = "Register `WRP2AR` reader"]
pub type R = crate::R<WRP2AR_SPEC>;
#[doc = "Register `WRP2AR` writer"]
pub type W = crate::W<WRP2AR_SPEC>;
#[doc = "Field `WRP2A_STRT` reader - WRP2A_STRT"]
pub type WRP2A_STRT_R = crate::FieldReader;
#[doc = "Field `WRP2A_STRT` writer - WRP2A_STRT"]
pub type WRP2A_STRT_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `WRP2A_END` reader - WRP2A_END"]
pub type WRP2A_END_R = crate::FieldReader;
#[doc = "Field `WRP2A_END` writer - WRP2A_END"]
pub type WRP2A_END_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:6 - WRP2A_STRT"]
    #[inline(always)]
    pub fn WRP2A_STRT(&self) -> WRP2A_STRT_R {
        WRP2A_STRT_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 16:22 - WRP2A_END"]
    #[inline(always)]
    pub fn WRP2A_END(&self) -> WRP2A_END_R {
        WRP2A_END_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - WRP2A_STRT"]
    #[inline(always)]
    pub fn WRP2A_STRT(&mut self) -> WRP2A_STRT_W<'_, WRP2AR_SPEC> {
        WRP2A_STRT_W::new(self, 0)
    }
    #[doc = "Bits 16:22 - WRP2A_END"]
    #[inline(always)]
    pub fn WRP2A_END(&mut self) -> WRP2A_END_W<'_, WRP2AR_SPEC> {
        WRP2A_END_W::new(self, 16)
    }
}
#[doc = "FLASH WRP2 area A address register\n\nYou can [`read`](crate::Reg::read) this register and get [`wrp2ar::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wrp2ar::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WRP2AR_SPEC;
impl crate::RegisterSpec for WRP2AR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wrp2ar::R`](R) reader structure"]
impl crate::Readable for WRP2AR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`wrp2ar::W`](W) writer structure"]
impl crate::Writable for WRP2AR_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets WRP2AR to value 0"]
impl crate::Resettable for WRP2AR_SPEC {}

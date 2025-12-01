#[doc = "Register `RQR` writer"]
pub type W = crate::W<RQR_SPEC>;
#[doc = "Field `ABRRQ` writer - Auto baud rate request"]
pub type ABRRQ_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SBKRQ` writer - Send break request"]
pub type SBKRQ_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MMRQ` writer - Mute mode request"]
pub type MMRQ_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXFRQ` writer - Receive data flush request"]
pub type RXFRQ_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXFRQ` writer - Transmit data flush request"]
pub type TXFRQ_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Auto baud rate request"]
    #[inline(always)]
    pub fn ABRRQ(&mut self) -> ABRRQ_W<'_, RQR_SPEC> {
        ABRRQ_W::new(self, 0)
    }
    #[doc = "Bit 1 - Send break request"]
    #[inline(always)]
    pub fn SBKRQ(&mut self) -> SBKRQ_W<'_, RQR_SPEC> {
        SBKRQ_W::new(self, 1)
    }
    #[doc = "Bit 2 - Mute mode request"]
    #[inline(always)]
    pub fn MMRQ(&mut self) -> MMRQ_W<'_, RQR_SPEC> {
        MMRQ_W::new(self, 2)
    }
    #[doc = "Bit 3 - Receive data flush request"]
    #[inline(always)]
    pub fn RXFRQ(&mut self) -> RXFRQ_W<'_, RQR_SPEC> {
        RXFRQ_W::new(self, 3)
    }
    #[doc = "Bit 4 - Transmit data flush request"]
    #[inline(always)]
    pub fn TXFRQ(&mut self) -> TXFRQ_W<'_, RQR_SPEC> {
        TXFRQ_W::new(self, 4)
    }
}
#[doc = "Request register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rqr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RQR_SPEC;
impl crate::RegisterSpec for RQR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`rqr::W`](W) writer structure"]
impl crate::Writable for RQR_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets RQR to value 0"]
impl crate::Resettable for RQR_SPEC {}

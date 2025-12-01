#[doc = "Register `FPR1` reader"]
pub type R = crate::R<FPR1_SPEC>;
#[doc = "Register `FPR1` writer"]
pub type W = crate::W<FPR1_SPEC>;
#[doc = "Field `FPIF0` reader - Falling edge event pending for configurable line"]
pub type FPIF0_R = crate::BitReader;
#[doc = "Field `FPIF0` writer - Falling edge event pending for configurable line"]
pub type FPIF0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FPIF1` reader - Falling edge event pending for configurable line"]
pub type FPIF1_R = crate::BitReader;
#[doc = "Field `FPIF1` writer - Falling edge event pending for configurable line"]
pub type FPIF1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FPIF2` reader - Falling edge event pending for configurable line"]
pub type FPIF2_R = crate::BitReader;
#[doc = "Field `FPIF2` writer - Falling edge event pending for configurable line"]
pub type FPIF2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FPIF3` reader - Falling edge event pending for configurable line"]
pub type FPIF3_R = crate::BitReader;
#[doc = "Field `FPIF3` writer - Falling edge event pending for configurable line"]
pub type FPIF3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FPIF4` reader - Falling edge event pending for configurable line"]
pub type FPIF4_R = crate::BitReader;
#[doc = "Field `FPIF4` writer - Falling edge event pending for configurable line"]
pub type FPIF4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FPIF5` reader - Falling edge event pending for configurable line"]
pub type FPIF5_R = crate::BitReader;
#[doc = "Field `FPIF5` writer - Falling edge event pending for configurable line"]
pub type FPIF5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FPIF6` reader - Falling edge event pending for configurable line"]
pub type FPIF6_R = crate::BitReader;
#[doc = "Field `FPIF6` writer - Falling edge event pending for configurable line"]
pub type FPIF6_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FPIF7` reader - Falling edge event pending for configurable line"]
pub type FPIF7_R = crate::BitReader;
#[doc = "Field `FPIF7` writer - Falling edge event pending for configurable line"]
pub type FPIF7_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FPIF8` reader - Falling edge event pending for configurable line"]
pub type FPIF8_R = crate::BitReader;
#[doc = "Field `FPIF8` writer - Falling edge event pending for configurable line"]
pub type FPIF8_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FPIF9` reader - Falling edge event pending for configurable line"]
pub type FPIF9_R = crate::BitReader;
#[doc = "Field `FPIF9` writer - Falling edge event pending for configurable line"]
pub type FPIF9_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FPIF10` reader - Falling edge event pending for configurable line"]
pub type FPIF10_R = crate::BitReader;
#[doc = "Field `FPIF10` writer - Falling edge event pending for configurable line"]
pub type FPIF10_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FPIF11` reader - Falling edge event pending for configurable line"]
pub type FPIF11_R = crate::BitReader;
#[doc = "Field `FPIF11` writer - Falling edge event pending for configurable line"]
pub type FPIF11_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FPIF12` reader - Falling edge event pending for configurable line"]
pub type FPIF12_R = crate::BitReader;
#[doc = "Field `FPIF12` writer - Falling edge event pending for configurable line"]
pub type FPIF12_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FPIF13` reader - Falling edge event pending for configurable line"]
pub type FPIF13_R = crate::BitReader;
#[doc = "Field `FPIF13` writer - Falling edge event pending for configurable line"]
pub type FPIF13_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FPIF14` reader - Falling edge event pending for configurable line"]
pub type FPIF14_R = crate::BitReader;
#[doc = "Field `FPIF14` writer - Falling edge event pending for configurable line"]
pub type FPIF14_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FPIF15` reader - Falling edge event pending for configurable line"]
pub type FPIF15_R = crate::BitReader;
#[doc = "Field `FPIF15` writer - Falling edge event pending for configurable line"]
pub type FPIF15_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Falling edge event pending for configurable line"]
    #[inline(always)]
    pub fn FPIF0(&self) -> FPIF0_R {
        FPIF0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Falling edge event pending for configurable line"]
    #[inline(always)]
    pub fn FPIF1(&self) -> FPIF1_R {
        FPIF1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Falling edge event pending for configurable line"]
    #[inline(always)]
    pub fn FPIF2(&self) -> FPIF2_R {
        FPIF2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Falling edge event pending for configurable line"]
    #[inline(always)]
    pub fn FPIF3(&self) -> FPIF3_R {
        FPIF3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Falling edge event pending for configurable line"]
    #[inline(always)]
    pub fn FPIF4(&self) -> FPIF4_R {
        FPIF4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Falling edge event pending for configurable line"]
    #[inline(always)]
    pub fn FPIF5(&self) -> FPIF5_R {
        FPIF5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Falling edge event pending for configurable line"]
    #[inline(always)]
    pub fn FPIF6(&self) -> FPIF6_R {
        FPIF6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Falling edge event pending for configurable line"]
    #[inline(always)]
    pub fn FPIF7(&self) -> FPIF7_R {
        FPIF7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Falling edge event pending for configurable line"]
    #[inline(always)]
    pub fn FPIF8(&self) -> FPIF8_R {
        FPIF8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Falling edge event pending for configurable line"]
    #[inline(always)]
    pub fn FPIF9(&self) -> FPIF9_R {
        FPIF9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Falling edge event pending for configurable line"]
    #[inline(always)]
    pub fn FPIF10(&self) -> FPIF10_R {
        FPIF10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Falling edge event pending for configurable line"]
    #[inline(always)]
    pub fn FPIF11(&self) -> FPIF11_R {
        FPIF11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Falling edge event pending for configurable line"]
    #[inline(always)]
    pub fn FPIF12(&self) -> FPIF12_R {
        FPIF12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Falling edge event pending for configurable line"]
    #[inline(always)]
    pub fn FPIF13(&self) -> FPIF13_R {
        FPIF13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Falling edge event pending for configurable line"]
    #[inline(always)]
    pub fn FPIF14(&self) -> FPIF14_R {
        FPIF14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Falling edge event pending for configurable line"]
    #[inline(always)]
    pub fn FPIF15(&self) -> FPIF15_R {
        FPIF15_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Falling edge event pending for configurable line"]
    #[inline(always)]
    pub fn FPIF0(&mut self) -> FPIF0_W<'_, FPR1_SPEC> {
        FPIF0_W::new(self, 0)
    }
    #[doc = "Bit 1 - Falling edge event pending for configurable line"]
    #[inline(always)]
    pub fn FPIF1(&mut self) -> FPIF1_W<'_, FPR1_SPEC> {
        FPIF1_W::new(self, 1)
    }
    #[doc = "Bit 2 - Falling edge event pending for configurable line"]
    #[inline(always)]
    pub fn FPIF2(&mut self) -> FPIF2_W<'_, FPR1_SPEC> {
        FPIF2_W::new(self, 2)
    }
    #[doc = "Bit 3 - Falling edge event pending for configurable line"]
    #[inline(always)]
    pub fn FPIF3(&mut self) -> FPIF3_W<'_, FPR1_SPEC> {
        FPIF3_W::new(self, 3)
    }
    #[doc = "Bit 4 - Falling edge event pending for configurable line"]
    #[inline(always)]
    pub fn FPIF4(&mut self) -> FPIF4_W<'_, FPR1_SPEC> {
        FPIF4_W::new(self, 4)
    }
    #[doc = "Bit 5 - Falling edge event pending for configurable line"]
    #[inline(always)]
    pub fn FPIF5(&mut self) -> FPIF5_W<'_, FPR1_SPEC> {
        FPIF5_W::new(self, 5)
    }
    #[doc = "Bit 6 - Falling edge event pending for configurable line"]
    #[inline(always)]
    pub fn FPIF6(&mut self) -> FPIF6_W<'_, FPR1_SPEC> {
        FPIF6_W::new(self, 6)
    }
    #[doc = "Bit 7 - Falling edge event pending for configurable line"]
    #[inline(always)]
    pub fn FPIF7(&mut self) -> FPIF7_W<'_, FPR1_SPEC> {
        FPIF7_W::new(self, 7)
    }
    #[doc = "Bit 8 - Falling edge event pending for configurable line"]
    #[inline(always)]
    pub fn FPIF8(&mut self) -> FPIF8_W<'_, FPR1_SPEC> {
        FPIF8_W::new(self, 8)
    }
    #[doc = "Bit 9 - Falling edge event pending for configurable line"]
    #[inline(always)]
    pub fn FPIF9(&mut self) -> FPIF9_W<'_, FPR1_SPEC> {
        FPIF9_W::new(self, 9)
    }
    #[doc = "Bit 10 - Falling edge event pending for configurable line"]
    #[inline(always)]
    pub fn FPIF10(&mut self) -> FPIF10_W<'_, FPR1_SPEC> {
        FPIF10_W::new(self, 10)
    }
    #[doc = "Bit 11 - Falling edge event pending for configurable line"]
    #[inline(always)]
    pub fn FPIF11(&mut self) -> FPIF11_W<'_, FPR1_SPEC> {
        FPIF11_W::new(self, 11)
    }
    #[doc = "Bit 12 - Falling edge event pending for configurable line"]
    #[inline(always)]
    pub fn FPIF12(&mut self) -> FPIF12_W<'_, FPR1_SPEC> {
        FPIF12_W::new(self, 12)
    }
    #[doc = "Bit 13 - Falling edge event pending for configurable line"]
    #[inline(always)]
    pub fn FPIF13(&mut self) -> FPIF13_W<'_, FPR1_SPEC> {
        FPIF13_W::new(self, 13)
    }
    #[doc = "Bit 14 - Falling edge event pending for configurable line"]
    #[inline(always)]
    pub fn FPIF14(&mut self) -> FPIF14_W<'_, FPR1_SPEC> {
        FPIF14_W::new(self, 14)
    }
    #[doc = "Bit 15 - Falling edge event pending for configurable line"]
    #[inline(always)]
    pub fn FPIF15(&mut self) -> FPIF15_W<'_, FPR1_SPEC> {
        FPIF15_W::new(self, 15)
    }
}
#[doc = "EXTI falling edge pending register\n\nYou can [`read`](crate::Reg::read) this register and get [`fpr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fpr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FPR1_SPEC;
impl crate::RegisterSpec for FPR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fpr1::R`](R) reader structure"]
impl crate::Readable for FPR1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`fpr1::W`](W) writer structure"]
impl crate::Writable for FPR1_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets FPR1 to value 0"]
impl crate::Resettable for FPR1_SPEC {}

#[doc = "Register `PDCRA` reader"]
pub type R = crate::R<PDCRA_SPEC>;
#[doc = "Register `PDCRA` writer"]
pub type W = crate::W<PDCRA_SPEC>;
#[doc = "Field `PD0` reader - Port A pull-down bit y (y=0..15)"]
pub type PD0_R = crate::BitReader;
#[doc = "Field `PD0` writer - Port A pull-down bit y (y=0..15)"]
pub type PD0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PD1` reader - Port A pull-down bit y (y=0..15)"]
pub type PD1_R = crate::BitReader;
#[doc = "Field `PD1` writer - Port A pull-down bit y (y=0..15)"]
pub type PD1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PD2` reader - Port A pull-down bit y (y=0..15)"]
pub type PD2_R = crate::BitReader;
#[doc = "Field `PD2` writer - Port A pull-down bit y (y=0..15)"]
pub type PD2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PD3` reader - Port A pull-down bit y (y=0..15)"]
pub type PD3_R = crate::BitReader;
#[doc = "Field `PD3` writer - Port A pull-down bit y (y=0..15)"]
pub type PD3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PD4` reader - Port A pull-down bit y (y=0..15)"]
pub type PD4_R = crate::BitReader;
#[doc = "Field `PD4` writer - Port A pull-down bit y (y=0..15)"]
pub type PD4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PD5` reader - Port A pull-down bit y (y=0..15)"]
pub type PD5_R = crate::BitReader;
#[doc = "Field `PD5` writer - Port A pull-down bit y (y=0..15)"]
pub type PD5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PD6` reader - Port A pull-down bit y (y=0..15)"]
pub type PD6_R = crate::BitReader;
#[doc = "Field `PD6` writer - Port A pull-down bit y (y=0..15)"]
pub type PD6_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PD7` reader - Port A pull-down bit y (y=0..15)"]
pub type PD7_R = crate::BitReader;
#[doc = "Field `PD7` writer - Port A pull-down bit y (y=0..15)"]
pub type PD7_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PD8` reader - Port A pull-down bit y (y=0..15)"]
pub type PD8_R = crate::BitReader;
#[doc = "Field `PD8` writer - Port A pull-down bit y (y=0..15)"]
pub type PD8_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PD9` reader - Port A pull-down bit y (y=0..15)"]
pub type PD9_R = crate::BitReader;
#[doc = "Field `PD9` writer - Port A pull-down bit y (y=0..15)"]
pub type PD9_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PD10` reader - Port A pull-down bit y (y=0..15)"]
pub type PD10_R = crate::BitReader;
#[doc = "Field `PD10` writer - Port A pull-down bit y (y=0..15)"]
pub type PD10_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PD11` reader - Port A pull-down bit y (y=0..15)"]
pub type PD11_R = crate::BitReader;
#[doc = "Field `PD11` writer - Port A pull-down bit y (y=0..15)"]
pub type PD11_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PD12` reader - Port A pull-down bit y (y=0..15)"]
pub type PD12_R = crate::BitReader;
#[doc = "Field `PD12` writer - Port A pull-down bit y (y=0..15)"]
pub type PD12_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PD13` reader - Port A pull-down bit y (y=0..15)"]
pub type PD13_R = crate::BitReader;
#[doc = "Field `PD13` writer - Port A pull-down bit y (y=0..15)"]
pub type PD13_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PD14` reader - Port A pull-down bit y (y=0..15)"]
pub type PD14_R = crate::BitReader;
#[doc = "Field `PD14` writer - Port A pull-down bit y (y=0..15)"]
pub type PD14_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PD15` reader - Port A pull-down bit y (y=0..15)"]
pub type PD15_R = crate::BitReader;
#[doc = "Field `PD15` writer - Port A pull-down bit y (y=0..15)"]
pub type PD15_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Port A pull-down bit y (y=0..15)"]
    #[inline(always)]
    pub fn PD0(&self) -> PD0_R {
        PD0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Port A pull-down bit y (y=0..15)"]
    #[inline(always)]
    pub fn PD1(&self) -> PD1_R {
        PD1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Port A pull-down bit y (y=0..15)"]
    #[inline(always)]
    pub fn PD2(&self) -> PD2_R {
        PD2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Port A pull-down bit y (y=0..15)"]
    #[inline(always)]
    pub fn PD3(&self) -> PD3_R {
        PD3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Port A pull-down bit y (y=0..15)"]
    #[inline(always)]
    pub fn PD4(&self) -> PD4_R {
        PD4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Port A pull-down bit y (y=0..15)"]
    #[inline(always)]
    pub fn PD5(&self) -> PD5_R {
        PD5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Port A pull-down bit y (y=0..15)"]
    #[inline(always)]
    pub fn PD6(&self) -> PD6_R {
        PD6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Port A pull-down bit y (y=0..15)"]
    #[inline(always)]
    pub fn PD7(&self) -> PD7_R {
        PD7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Port A pull-down bit y (y=0..15)"]
    #[inline(always)]
    pub fn PD8(&self) -> PD8_R {
        PD8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Port A pull-down bit y (y=0..15)"]
    #[inline(always)]
    pub fn PD9(&self) -> PD9_R {
        PD9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Port A pull-down bit y (y=0..15)"]
    #[inline(always)]
    pub fn PD10(&self) -> PD10_R {
        PD10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Port A pull-down bit y (y=0..15)"]
    #[inline(always)]
    pub fn PD11(&self) -> PD11_R {
        PD11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Port A pull-down bit y (y=0..15)"]
    #[inline(always)]
    pub fn PD12(&self) -> PD12_R {
        PD12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Port A pull-down bit y (y=0..15)"]
    #[inline(always)]
    pub fn PD13(&self) -> PD13_R {
        PD13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Port A pull-down bit y (y=0..15)"]
    #[inline(always)]
    pub fn PD14(&self) -> PD14_R {
        PD14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Port A pull-down bit y (y=0..15)"]
    #[inline(always)]
    pub fn PD15(&self) -> PD15_R {
        PD15_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Port A pull-down bit y (y=0..15)"]
    #[inline(always)]
    pub fn PD0(&mut self) -> PD0_W<'_, PDCRA_SPEC> {
        PD0_W::new(self, 0)
    }
    #[doc = "Bit 1 - Port A pull-down bit y (y=0..15)"]
    #[inline(always)]
    pub fn PD1(&mut self) -> PD1_W<'_, PDCRA_SPEC> {
        PD1_W::new(self, 1)
    }
    #[doc = "Bit 2 - Port A pull-down bit y (y=0..15)"]
    #[inline(always)]
    pub fn PD2(&mut self) -> PD2_W<'_, PDCRA_SPEC> {
        PD2_W::new(self, 2)
    }
    #[doc = "Bit 3 - Port A pull-down bit y (y=0..15)"]
    #[inline(always)]
    pub fn PD3(&mut self) -> PD3_W<'_, PDCRA_SPEC> {
        PD3_W::new(self, 3)
    }
    #[doc = "Bit 4 - Port A pull-down bit y (y=0..15)"]
    #[inline(always)]
    pub fn PD4(&mut self) -> PD4_W<'_, PDCRA_SPEC> {
        PD4_W::new(self, 4)
    }
    #[doc = "Bit 5 - Port A pull-down bit y (y=0..15)"]
    #[inline(always)]
    pub fn PD5(&mut self) -> PD5_W<'_, PDCRA_SPEC> {
        PD5_W::new(self, 5)
    }
    #[doc = "Bit 6 - Port A pull-down bit y (y=0..15)"]
    #[inline(always)]
    pub fn PD6(&mut self) -> PD6_W<'_, PDCRA_SPEC> {
        PD6_W::new(self, 6)
    }
    #[doc = "Bit 7 - Port A pull-down bit y (y=0..15)"]
    #[inline(always)]
    pub fn PD7(&mut self) -> PD7_W<'_, PDCRA_SPEC> {
        PD7_W::new(self, 7)
    }
    #[doc = "Bit 8 - Port A pull-down bit y (y=0..15)"]
    #[inline(always)]
    pub fn PD8(&mut self) -> PD8_W<'_, PDCRA_SPEC> {
        PD8_W::new(self, 8)
    }
    #[doc = "Bit 9 - Port A pull-down bit y (y=0..15)"]
    #[inline(always)]
    pub fn PD9(&mut self) -> PD9_W<'_, PDCRA_SPEC> {
        PD9_W::new(self, 9)
    }
    #[doc = "Bit 10 - Port A pull-down bit y (y=0..15)"]
    #[inline(always)]
    pub fn PD10(&mut self) -> PD10_W<'_, PDCRA_SPEC> {
        PD10_W::new(self, 10)
    }
    #[doc = "Bit 11 - Port A pull-down bit y (y=0..15)"]
    #[inline(always)]
    pub fn PD11(&mut self) -> PD11_W<'_, PDCRA_SPEC> {
        PD11_W::new(self, 11)
    }
    #[doc = "Bit 12 - Port A pull-down bit y (y=0..15)"]
    #[inline(always)]
    pub fn PD12(&mut self) -> PD12_W<'_, PDCRA_SPEC> {
        PD12_W::new(self, 12)
    }
    #[doc = "Bit 13 - Port A pull-down bit y (y=0..15)"]
    #[inline(always)]
    pub fn PD13(&mut self) -> PD13_W<'_, PDCRA_SPEC> {
        PD13_W::new(self, 13)
    }
    #[doc = "Bit 14 - Port A pull-down bit y (y=0..15)"]
    #[inline(always)]
    pub fn PD14(&mut self) -> PD14_W<'_, PDCRA_SPEC> {
        PD14_W::new(self, 14)
    }
    #[doc = "Bit 15 - Port A pull-down bit y (y=0..15)"]
    #[inline(always)]
    pub fn PD15(&mut self) -> PD15_W<'_, PDCRA_SPEC> {
        PD15_W::new(self, 15)
    }
}
#[doc = "Power Port A pull-down control register\n\nYou can [`read`](crate::Reg::read) this register and get [`pdcra::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pdcra::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PDCRA_SPEC;
impl crate::RegisterSpec for PDCRA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pdcra::R`](R) reader structure"]
impl crate::Readable for PDCRA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pdcra::W`](W) writer structure"]
impl crate::Writable for PDCRA_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets PDCRA to value 0"]
impl crate::Resettable for PDCRA_SPEC {}

#[doc = "Register `RTSR1` reader"]
pub type R = crate::R<RTSR1_SPEC>;
#[doc = "Register `RTSR1` writer"]
pub type W = crate::W<RTSR1_SPEC>;
#[doc = "Field `RT0` reader - Rising trigger event configuration bit of Configurable Event line"]
pub type RT0_R = crate::BitReader;
#[doc = "Field `RT0` writer - Rising trigger event configuration bit of Configurable Event line"]
pub type RT0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RT1` reader - Rising trigger event configuration bit of Configurable Event line"]
pub type RT1_R = crate::BitReader;
#[doc = "Field `RT1` writer - Rising trigger event configuration bit of Configurable Event line"]
pub type RT1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RT2` reader - Rising trigger event configuration bit of Configurable Event line"]
pub type RT2_R = crate::BitReader;
#[doc = "Field `RT2` writer - Rising trigger event configuration bit of Configurable Event line"]
pub type RT2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RT3` reader - Rising trigger event configuration bit of Configurable Event line"]
pub type RT3_R = crate::BitReader;
#[doc = "Field `RT3` writer - Rising trigger event configuration bit of Configurable Event line"]
pub type RT3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RT4` reader - Rising trigger event configuration bit of Configurable Event line"]
pub type RT4_R = crate::BitReader;
#[doc = "Field `RT4` writer - Rising trigger event configuration bit of Configurable Event line"]
pub type RT4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RT5` reader - Rising trigger event configuration bit of Configurable Event line"]
pub type RT5_R = crate::BitReader;
#[doc = "Field `RT5` writer - Rising trigger event configuration bit of Configurable Event line"]
pub type RT5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RT6` reader - Rising trigger event configuration bit of Configurable Event line"]
pub type RT6_R = crate::BitReader;
#[doc = "Field `RT6` writer - Rising trigger event configuration bit of Configurable Event line"]
pub type RT6_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RT7` reader - Rising trigger event configuration bit of Configurable Event line"]
pub type RT7_R = crate::BitReader;
#[doc = "Field `RT7` writer - Rising trigger event configuration bit of Configurable Event line"]
pub type RT7_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RT8` reader - Rising trigger event configuration bit of Configurable Event line"]
pub type RT8_R = crate::BitReader;
#[doc = "Field `RT8` writer - Rising trigger event configuration bit of Configurable Event line"]
pub type RT8_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RT9` reader - Rising trigger event configuration bit of Configurable Event line"]
pub type RT9_R = crate::BitReader;
#[doc = "Field `RT9` writer - Rising trigger event configuration bit of Configurable Event line"]
pub type RT9_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RT10` reader - Rising trigger event configuration bit of Configurable Event line"]
pub type RT10_R = crate::BitReader;
#[doc = "Field `RT10` writer - Rising trigger event configuration bit of Configurable Event line"]
pub type RT10_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RT11` reader - Rising trigger event configuration bit of Configurable Event line"]
pub type RT11_R = crate::BitReader;
#[doc = "Field `RT11` writer - Rising trigger event configuration bit of Configurable Event line"]
pub type RT11_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RT12` reader - Rising trigger event configuration bit of Configurable Event line"]
pub type RT12_R = crate::BitReader;
#[doc = "Field `RT12` writer - Rising trigger event configuration bit of Configurable Event line"]
pub type RT12_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RT13` reader - Rising trigger event configuration bit of Configurable Event line"]
pub type RT13_R = crate::BitReader;
#[doc = "Field `RT13` writer - Rising trigger event configuration bit of Configurable Event line"]
pub type RT13_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RT14` reader - Rising trigger event configuration bit of Configurable Event line"]
pub type RT14_R = crate::BitReader;
#[doc = "Field `RT14` writer - Rising trigger event configuration bit of Configurable Event line"]
pub type RT14_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RT15` reader - Rising trigger event configuration bit of Configurable Event line"]
pub type RT15_R = crate::BitReader;
#[doc = "Field `RT15` writer - Rising trigger event configuration bit of Configurable Event line"]
pub type RT15_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Rising trigger event configuration bit of Configurable Event line"]
    #[inline(always)]
    pub fn RT0(&self) -> RT0_R {
        RT0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Rising trigger event configuration bit of Configurable Event line"]
    #[inline(always)]
    pub fn RT1(&self) -> RT1_R {
        RT1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Rising trigger event configuration bit of Configurable Event line"]
    #[inline(always)]
    pub fn RT2(&self) -> RT2_R {
        RT2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Rising trigger event configuration bit of Configurable Event line"]
    #[inline(always)]
    pub fn RT3(&self) -> RT3_R {
        RT3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Rising trigger event configuration bit of Configurable Event line"]
    #[inline(always)]
    pub fn RT4(&self) -> RT4_R {
        RT4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Rising trigger event configuration bit of Configurable Event line"]
    #[inline(always)]
    pub fn RT5(&self) -> RT5_R {
        RT5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Rising trigger event configuration bit of Configurable Event line"]
    #[inline(always)]
    pub fn RT6(&self) -> RT6_R {
        RT6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Rising trigger event configuration bit of Configurable Event line"]
    #[inline(always)]
    pub fn RT7(&self) -> RT7_R {
        RT7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Rising trigger event configuration bit of Configurable Event line"]
    #[inline(always)]
    pub fn RT8(&self) -> RT8_R {
        RT8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Rising trigger event configuration bit of Configurable Event line"]
    #[inline(always)]
    pub fn RT9(&self) -> RT9_R {
        RT9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Rising trigger event configuration bit of Configurable Event line"]
    #[inline(always)]
    pub fn RT10(&self) -> RT10_R {
        RT10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Rising trigger event configuration bit of Configurable Event line"]
    #[inline(always)]
    pub fn RT11(&self) -> RT11_R {
        RT11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Rising trigger event configuration bit of Configurable Event line"]
    #[inline(always)]
    pub fn RT12(&self) -> RT12_R {
        RT12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Rising trigger event configuration bit of Configurable Event line"]
    #[inline(always)]
    pub fn RT13(&self) -> RT13_R {
        RT13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Rising trigger event configuration bit of Configurable Event line"]
    #[inline(always)]
    pub fn RT14(&self) -> RT14_R {
        RT14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Rising trigger event configuration bit of Configurable Event line"]
    #[inline(always)]
    pub fn RT15(&self) -> RT15_R {
        RT15_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Rising trigger event configuration bit of Configurable Event line"]
    #[inline(always)]
    pub fn RT0(&mut self) -> RT0_W<'_, RTSR1_SPEC> {
        RT0_W::new(self, 0)
    }
    #[doc = "Bit 1 - Rising trigger event configuration bit of Configurable Event line"]
    #[inline(always)]
    pub fn RT1(&mut self) -> RT1_W<'_, RTSR1_SPEC> {
        RT1_W::new(self, 1)
    }
    #[doc = "Bit 2 - Rising trigger event configuration bit of Configurable Event line"]
    #[inline(always)]
    pub fn RT2(&mut self) -> RT2_W<'_, RTSR1_SPEC> {
        RT2_W::new(self, 2)
    }
    #[doc = "Bit 3 - Rising trigger event configuration bit of Configurable Event line"]
    #[inline(always)]
    pub fn RT3(&mut self) -> RT3_W<'_, RTSR1_SPEC> {
        RT3_W::new(self, 3)
    }
    #[doc = "Bit 4 - Rising trigger event configuration bit of Configurable Event line"]
    #[inline(always)]
    pub fn RT4(&mut self) -> RT4_W<'_, RTSR1_SPEC> {
        RT4_W::new(self, 4)
    }
    #[doc = "Bit 5 - Rising trigger event configuration bit of Configurable Event line"]
    #[inline(always)]
    pub fn RT5(&mut self) -> RT5_W<'_, RTSR1_SPEC> {
        RT5_W::new(self, 5)
    }
    #[doc = "Bit 6 - Rising trigger event configuration bit of Configurable Event line"]
    #[inline(always)]
    pub fn RT6(&mut self) -> RT6_W<'_, RTSR1_SPEC> {
        RT6_W::new(self, 6)
    }
    #[doc = "Bit 7 - Rising trigger event configuration bit of Configurable Event line"]
    #[inline(always)]
    pub fn RT7(&mut self) -> RT7_W<'_, RTSR1_SPEC> {
        RT7_W::new(self, 7)
    }
    #[doc = "Bit 8 - Rising trigger event configuration bit of Configurable Event line"]
    #[inline(always)]
    pub fn RT8(&mut self) -> RT8_W<'_, RTSR1_SPEC> {
        RT8_W::new(self, 8)
    }
    #[doc = "Bit 9 - Rising trigger event configuration bit of Configurable Event line"]
    #[inline(always)]
    pub fn RT9(&mut self) -> RT9_W<'_, RTSR1_SPEC> {
        RT9_W::new(self, 9)
    }
    #[doc = "Bit 10 - Rising trigger event configuration bit of Configurable Event line"]
    #[inline(always)]
    pub fn RT10(&mut self) -> RT10_W<'_, RTSR1_SPEC> {
        RT10_W::new(self, 10)
    }
    #[doc = "Bit 11 - Rising trigger event configuration bit of Configurable Event line"]
    #[inline(always)]
    pub fn RT11(&mut self) -> RT11_W<'_, RTSR1_SPEC> {
        RT11_W::new(self, 11)
    }
    #[doc = "Bit 12 - Rising trigger event configuration bit of Configurable Event line"]
    #[inline(always)]
    pub fn RT12(&mut self) -> RT12_W<'_, RTSR1_SPEC> {
        RT12_W::new(self, 12)
    }
    #[doc = "Bit 13 - Rising trigger event configuration bit of Configurable Event line"]
    #[inline(always)]
    pub fn RT13(&mut self) -> RT13_W<'_, RTSR1_SPEC> {
        RT13_W::new(self, 13)
    }
    #[doc = "Bit 14 - Rising trigger event configuration bit of Configurable Event line"]
    #[inline(always)]
    pub fn RT14(&mut self) -> RT14_W<'_, RTSR1_SPEC> {
        RT14_W::new(self, 14)
    }
    #[doc = "Bit 15 - Rising trigger event configuration bit of Configurable Event line"]
    #[inline(always)]
    pub fn RT15(&mut self) -> RT15_W<'_, RTSR1_SPEC> {
        RT15_W::new(self, 15)
    }
}
#[doc = "EXTI rising trigger selection register\n\nYou can [`read`](crate::Reg::read) this register and get [`rtsr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtsr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RTSR1_SPEC;
impl crate::RegisterSpec for RTSR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rtsr1::R`](R) reader structure"]
impl crate::Readable for RTSR1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rtsr1::W`](W) writer structure"]
impl crate::Writable for RTSR1_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets RTSR1 to value 0"]
impl crate::Resettable for RTSR1_SPEC {}

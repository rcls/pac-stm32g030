#[doc = "Register `SWIER1` reader"]
pub type R = crate::R<SWIER1_SPEC>;
#[doc = "Register `SWIER1` writer"]
pub type W = crate::W<SWIER1_SPEC>;
#[doc = "Field `SWI0` reader - Software rising edge event trigger on line"]
pub type SWI0_R = crate::BitReader;
#[doc = "Field `SWI0` writer - Software rising edge event trigger on line"]
pub type SWI0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWI1` reader - Software rising edge event trigger on line"]
pub type SWI1_R = crate::BitReader;
#[doc = "Field `SWI1` writer - Software rising edge event trigger on line"]
pub type SWI1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWI2` reader - Software rising edge event trigger on line"]
pub type SWI2_R = crate::BitReader;
#[doc = "Field `SWI2` writer - Software rising edge event trigger on line"]
pub type SWI2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWI3` reader - Software rising edge event trigger on line"]
pub type SWI3_R = crate::BitReader;
#[doc = "Field `SWI3` writer - Software rising edge event trigger on line"]
pub type SWI3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWI4` reader - Software rising edge event trigger on line"]
pub type SWI4_R = crate::BitReader;
#[doc = "Field `SWI4` writer - Software rising edge event trigger on line"]
pub type SWI4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWI5` reader - Software rising edge event trigger on line"]
pub type SWI5_R = crate::BitReader;
#[doc = "Field `SWI5` writer - Software rising edge event trigger on line"]
pub type SWI5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWI6` reader - Software rising edge event trigger on line"]
pub type SWI6_R = crate::BitReader;
#[doc = "Field `SWI6` writer - Software rising edge event trigger on line"]
pub type SWI6_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWI7` reader - Software rising edge event trigger on line"]
pub type SWI7_R = crate::BitReader;
#[doc = "Field `SWI7` writer - Software rising edge event trigger on line"]
pub type SWI7_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWI8` reader - Software rising edge event trigger on line"]
pub type SWI8_R = crate::BitReader;
#[doc = "Field `SWI8` writer - Software rising edge event trigger on line"]
pub type SWI8_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWI9` reader - Software rising edge event trigger on line"]
pub type SWI9_R = crate::BitReader;
#[doc = "Field `SWI9` writer - Software rising edge event trigger on line"]
pub type SWI9_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWI10` reader - Software rising edge event trigger on line"]
pub type SWI10_R = crate::BitReader;
#[doc = "Field `SWI10` writer - Software rising edge event trigger on line"]
pub type SWI10_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWI11` reader - Software rising edge event trigger on line"]
pub type SWI11_R = crate::BitReader;
#[doc = "Field `SWI11` writer - Software rising edge event trigger on line"]
pub type SWI11_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWI12` reader - Software rising edge event trigger on line"]
pub type SWI12_R = crate::BitReader;
#[doc = "Field `SWI12` writer - Software rising edge event trigger on line"]
pub type SWI12_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWI13` reader - Software rising edge event trigger on line"]
pub type SWI13_R = crate::BitReader;
#[doc = "Field `SWI13` writer - Software rising edge event trigger on line"]
pub type SWI13_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWI14` reader - Software rising edge event trigger on line"]
pub type SWI14_R = crate::BitReader;
#[doc = "Field `SWI14` writer - Software rising edge event trigger on line"]
pub type SWI14_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWI15` reader - Software rising edge event trigger on line"]
pub type SWI15_R = crate::BitReader;
#[doc = "Field `SWI15` writer - Software rising edge event trigger on line"]
pub type SWI15_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Software rising edge event trigger on line"]
    #[inline(always)]
    pub fn SWI0(&self) -> SWI0_R {
        SWI0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Software rising edge event trigger on line"]
    #[inline(always)]
    pub fn SWI1(&self) -> SWI1_R {
        SWI1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Software rising edge event trigger on line"]
    #[inline(always)]
    pub fn SWI2(&self) -> SWI2_R {
        SWI2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Software rising edge event trigger on line"]
    #[inline(always)]
    pub fn SWI3(&self) -> SWI3_R {
        SWI3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Software rising edge event trigger on line"]
    #[inline(always)]
    pub fn SWI4(&self) -> SWI4_R {
        SWI4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Software rising edge event trigger on line"]
    #[inline(always)]
    pub fn SWI5(&self) -> SWI5_R {
        SWI5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Software rising edge event trigger on line"]
    #[inline(always)]
    pub fn SWI6(&self) -> SWI6_R {
        SWI6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Software rising edge event trigger on line"]
    #[inline(always)]
    pub fn SWI7(&self) -> SWI7_R {
        SWI7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Software rising edge event trigger on line"]
    #[inline(always)]
    pub fn SWI8(&self) -> SWI8_R {
        SWI8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Software rising edge event trigger on line"]
    #[inline(always)]
    pub fn SWI9(&self) -> SWI9_R {
        SWI9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Software rising edge event trigger on line"]
    #[inline(always)]
    pub fn SWI10(&self) -> SWI10_R {
        SWI10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Software rising edge event trigger on line"]
    #[inline(always)]
    pub fn SWI11(&self) -> SWI11_R {
        SWI11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Software rising edge event trigger on line"]
    #[inline(always)]
    pub fn SWI12(&self) -> SWI12_R {
        SWI12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Software rising edge event trigger on line"]
    #[inline(always)]
    pub fn SWI13(&self) -> SWI13_R {
        SWI13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Software rising edge event trigger on line"]
    #[inline(always)]
    pub fn SWI14(&self) -> SWI14_R {
        SWI14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Software rising edge event trigger on line"]
    #[inline(always)]
    pub fn SWI15(&self) -> SWI15_R {
        SWI15_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Software rising edge event trigger on line"]
    #[inline(always)]
    pub fn SWI0(&mut self) -> SWI0_W<'_, SWIER1_SPEC> {
        SWI0_W::new(self, 0)
    }
    #[doc = "Bit 1 - Software rising edge event trigger on line"]
    #[inline(always)]
    pub fn SWI1(&mut self) -> SWI1_W<'_, SWIER1_SPEC> {
        SWI1_W::new(self, 1)
    }
    #[doc = "Bit 2 - Software rising edge event trigger on line"]
    #[inline(always)]
    pub fn SWI2(&mut self) -> SWI2_W<'_, SWIER1_SPEC> {
        SWI2_W::new(self, 2)
    }
    #[doc = "Bit 3 - Software rising edge event trigger on line"]
    #[inline(always)]
    pub fn SWI3(&mut self) -> SWI3_W<'_, SWIER1_SPEC> {
        SWI3_W::new(self, 3)
    }
    #[doc = "Bit 4 - Software rising edge event trigger on line"]
    #[inline(always)]
    pub fn SWI4(&mut self) -> SWI4_W<'_, SWIER1_SPEC> {
        SWI4_W::new(self, 4)
    }
    #[doc = "Bit 5 - Software rising edge event trigger on line"]
    #[inline(always)]
    pub fn SWI5(&mut self) -> SWI5_W<'_, SWIER1_SPEC> {
        SWI5_W::new(self, 5)
    }
    #[doc = "Bit 6 - Software rising edge event trigger on line"]
    #[inline(always)]
    pub fn SWI6(&mut self) -> SWI6_W<'_, SWIER1_SPEC> {
        SWI6_W::new(self, 6)
    }
    #[doc = "Bit 7 - Software rising edge event trigger on line"]
    #[inline(always)]
    pub fn SWI7(&mut self) -> SWI7_W<'_, SWIER1_SPEC> {
        SWI7_W::new(self, 7)
    }
    #[doc = "Bit 8 - Software rising edge event trigger on line"]
    #[inline(always)]
    pub fn SWI8(&mut self) -> SWI8_W<'_, SWIER1_SPEC> {
        SWI8_W::new(self, 8)
    }
    #[doc = "Bit 9 - Software rising edge event trigger on line"]
    #[inline(always)]
    pub fn SWI9(&mut self) -> SWI9_W<'_, SWIER1_SPEC> {
        SWI9_W::new(self, 9)
    }
    #[doc = "Bit 10 - Software rising edge event trigger on line"]
    #[inline(always)]
    pub fn SWI10(&mut self) -> SWI10_W<'_, SWIER1_SPEC> {
        SWI10_W::new(self, 10)
    }
    #[doc = "Bit 11 - Software rising edge event trigger on line"]
    #[inline(always)]
    pub fn SWI11(&mut self) -> SWI11_W<'_, SWIER1_SPEC> {
        SWI11_W::new(self, 11)
    }
    #[doc = "Bit 12 - Software rising edge event trigger on line"]
    #[inline(always)]
    pub fn SWI12(&mut self) -> SWI12_W<'_, SWIER1_SPEC> {
        SWI12_W::new(self, 12)
    }
    #[doc = "Bit 13 - Software rising edge event trigger on line"]
    #[inline(always)]
    pub fn SWI13(&mut self) -> SWI13_W<'_, SWIER1_SPEC> {
        SWI13_W::new(self, 13)
    }
    #[doc = "Bit 14 - Software rising edge event trigger on line"]
    #[inline(always)]
    pub fn SWI14(&mut self) -> SWI14_W<'_, SWIER1_SPEC> {
        SWI14_W::new(self, 14)
    }
    #[doc = "Bit 15 - Software rising edge event trigger on line"]
    #[inline(always)]
    pub fn SWI15(&mut self) -> SWI15_W<'_, SWIER1_SPEC> {
        SWI15_W::new(self, 15)
    }
}
#[doc = "EXTI software interrupt event register\n\nYou can [`read`](crate::Reg::read) this register and get [`swier1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swier1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SWIER1_SPEC;
impl crate::RegisterSpec for SWIER1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swier1::R`](R) reader structure"]
impl crate::Readable for SWIER1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`swier1::W`](W) writer structure"]
impl crate::Writable for SWIER1_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets SWIER1 to value 0"]
impl crate::Resettable for SWIER1_SPEC {}

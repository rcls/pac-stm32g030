#[doc = "Register `OAR1` reader"]
pub type R = crate::R<OAR1_SPEC>;
#[doc = "Register `OAR1` writer"]
pub type W = crate::W<OAR1_SPEC>;
#[doc = "Field `OA1_0` reader - Interface address"]
pub type OA1_0_R = crate::BitReader;
#[doc = "Field `OA1_0` writer - Interface address"]
pub type OA1_0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OA1_7_1` reader - Interface address"]
pub type OA1_7_1_R = crate::FieldReader;
#[doc = "Field `OA1_7_1` writer - Interface address"]
pub type OA1_7_1_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `OA1_8_9` reader - Interface address"]
pub type OA1_8_9_R = crate::FieldReader;
#[doc = "Field `OA1_8_9` writer - Interface address"]
pub type OA1_8_9_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `OA1MODE` reader - Own Address 1 10-bit mode"]
pub type OA1MODE_R = crate::BitReader;
#[doc = "Field `OA1MODE` writer - Own Address 1 10-bit mode"]
pub type OA1MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OA1EN` reader - Own Address 1 enable"]
pub type OA1EN_R = crate::BitReader;
#[doc = "Field `OA1EN` writer - Own Address 1 enable"]
pub type OA1EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Interface address"]
    #[inline(always)]
    pub fn OA1_0(&self) -> OA1_0_R {
        OA1_0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:7 - Interface address"]
    #[inline(always)]
    pub fn OA1_7_1(&self) -> OA1_7_1_R {
        OA1_7_1_R::new(((self.bits >> 1) & 0x7f) as u8)
    }
    #[doc = "Bits 8:9 - Interface address"]
    #[inline(always)]
    pub fn OA1_8_9(&self) -> OA1_8_9_R {
        OA1_8_9_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 10 - Own Address 1 10-bit mode"]
    #[inline(always)]
    pub fn OA1MODE(&self) -> OA1MODE_R {
        OA1MODE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 15 - Own Address 1 enable"]
    #[inline(always)]
    pub fn OA1EN(&self) -> OA1EN_R {
        OA1EN_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Interface address"]
    #[inline(always)]
    pub fn OA1_0(&mut self) -> OA1_0_W<'_, OAR1_SPEC> {
        OA1_0_W::new(self, 0)
    }
    #[doc = "Bits 1:7 - Interface address"]
    #[inline(always)]
    pub fn OA1_7_1(&mut self) -> OA1_7_1_W<'_, OAR1_SPEC> {
        OA1_7_1_W::new(self, 1)
    }
    #[doc = "Bits 8:9 - Interface address"]
    #[inline(always)]
    pub fn OA1_8_9(&mut self) -> OA1_8_9_W<'_, OAR1_SPEC> {
        OA1_8_9_W::new(self, 8)
    }
    #[doc = "Bit 10 - Own Address 1 10-bit mode"]
    #[inline(always)]
    pub fn OA1MODE(&mut self) -> OA1MODE_W<'_, OAR1_SPEC> {
        OA1MODE_W::new(self, 10)
    }
    #[doc = "Bit 15 - Own Address 1 enable"]
    #[inline(always)]
    pub fn OA1EN(&mut self) -> OA1EN_W<'_, OAR1_SPEC> {
        OA1EN_W::new(self, 15)
    }
}
#[doc = "Own address register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`oar1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`oar1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OAR1_SPEC;
impl crate::RegisterSpec for OAR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`oar1::R`](R) reader structure"]
impl crate::Readable for OAR1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`oar1::W`](W) writer structure"]
impl crate::Writable for OAR1_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets OAR1 to value 0"]
impl crate::Resettable for OAR1_SPEC {}

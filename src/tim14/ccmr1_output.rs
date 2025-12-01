#[doc = "Register `CCMR1_Output` reader"]
pub type R = crate::R<CCMR1_OUTPUT_SPEC>;
#[doc = "Register `CCMR1_Output` writer"]
pub type W = crate::W<CCMR1_OUTPUT_SPEC>;
#[doc = "Field `CC1S` reader - CC1S"]
pub type CC1S_R = crate::FieldReader;
#[doc = "Field `CC1S` writer - CC1S"]
pub type CC1S_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `OC1FE` reader - OC1FE"]
pub type OC1FE_R = crate::BitReader;
#[doc = "Field `OC1FE` writer - OC1FE"]
pub type OC1FE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OC1PE` reader - OC1PE"]
pub type OC1PE_R = crate::BitReader;
#[doc = "Field `OC1PE` writer - OC1PE"]
pub type OC1PE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OC1M` reader - OC1M"]
pub type OC1M_R = crate::FieldReader;
#[doc = "Field `OC1M` writer - OC1M"]
pub type OC1M_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `OC1CE` reader - OC1CE"]
pub type OC1CE_R = crate::BitReader;
#[doc = "Field `OC1CE` writer - OC1CE"]
pub type OC1CE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OC1M_3` reader - Output Compare 1 mode - bit 3"]
pub type OC1M_3_R = crate::BitReader;
#[doc = "Field `OC1M_3` writer - Output Compare 1 mode - bit 3"]
pub type OC1M_3_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - CC1S"]
    #[inline(always)]
    pub fn CC1S(&self) -> CC1S_R {
        CC1S_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - OC1FE"]
    #[inline(always)]
    pub fn OC1FE(&self) -> OC1FE_R {
        OC1FE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - OC1PE"]
    #[inline(always)]
    pub fn OC1PE(&self) -> OC1PE_R {
        OC1PE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - OC1M"]
    #[inline(always)]
    pub fn OC1M(&self) -> OC1M_R {
        OC1M_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - OC1CE"]
    #[inline(always)]
    pub fn OC1CE(&self) -> OC1CE_R {
        OC1CE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 16 - Output Compare 1 mode - bit 3"]
    #[inline(always)]
    pub fn OC1M_3(&self) -> OC1M_3_R {
        OC1M_3_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - CC1S"]
    #[inline(always)]
    pub fn CC1S(&mut self) -> CC1S_W<'_, CCMR1_OUTPUT_SPEC> {
        CC1S_W::new(self, 0)
    }
    #[doc = "Bit 2 - OC1FE"]
    #[inline(always)]
    pub fn OC1FE(&mut self) -> OC1FE_W<'_, CCMR1_OUTPUT_SPEC> {
        OC1FE_W::new(self, 2)
    }
    #[doc = "Bit 3 - OC1PE"]
    #[inline(always)]
    pub fn OC1PE(&mut self) -> OC1PE_W<'_, CCMR1_OUTPUT_SPEC> {
        OC1PE_W::new(self, 3)
    }
    #[doc = "Bits 4:6 - OC1M"]
    #[inline(always)]
    pub fn OC1M(&mut self) -> OC1M_W<'_, CCMR1_OUTPUT_SPEC> {
        OC1M_W::new(self, 4)
    }
    #[doc = "Bit 7 - OC1CE"]
    #[inline(always)]
    pub fn OC1CE(&mut self) -> OC1CE_W<'_, CCMR1_OUTPUT_SPEC> {
        OC1CE_W::new(self, 7)
    }
    #[doc = "Bit 16 - Output Compare 1 mode - bit 3"]
    #[inline(always)]
    pub fn OC1M_3(&mut self) -> OC1M_3_W<'_, CCMR1_OUTPUT_SPEC> {
        OC1M_3_W::new(self, 16)
    }
}
#[doc = "capture/compare mode register 1 (output mode)\n\nYou can [`read`](crate::Reg::read) this register and get [`ccmr1_output::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccmr1_output::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CCMR1_OUTPUT_SPEC;
impl crate::RegisterSpec for CCMR1_OUTPUT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ccmr1_output::R`](R) reader structure"]
impl crate::Readable for CCMR1_OUTPUT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ccmr1_output::W`](W) writer structure"]
impl crate::Writable for CCMR1_OUTPUT_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets CCMR1_Output to value 0"]
impl crate::Resettable for CCMR1_OUTPUT_SPEC {}

#[doc = "Register `OR1` reader"]
pub type R = crate::R<OR1_SPEC>;
#[doc = "Register `OR1` writer"]
pub type W = crate::W<OR1_SPEC>;
#[doc = "Field `OCREF_CLR` reader - Ocref_clr source selection"]
pub type OCREF_CLR_R = crate::BitReader;
#[doc = "Field `OCREF_CLR` writer - Ocref_clr source selection"]
pub type OCREF_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Ocref_clr source selection"]
    #[inline(always)]
    pub fn OCREF_CLR(&self) -> OCREF_CLR_R {
        OCREF_CLR_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Ocref_clr source selection"]
    #[inline(always)]
    pub fn OCREF_CLR(&mut self) -> OCREF_CLR_W<'_, OR1_SPEC> {
        OCREF_CLR_W::new(self, 0)
    }
}
#[doc = "option register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`or1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`or1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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

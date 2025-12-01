#[doc = "Register `OPTR` reader"]
pub type R = crate::R<OPTR_SPEC>;
#[doc = "Register `OPTR` writer"]
pub type W = crate::W<OPTR_SPEC>;
#[doc = "Field `RDP` reader - Read protection level"]
pub type RDP_R = crate::FieldReader;
#[doc = "Field `RDP` writer - Read protection level"]
pub type RDP_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `nRST_STOP` reader - nRST_STOP"]
pub type N_RST_STOP_R = crate::BitReader;
#[doc = "Field `nRST_STOP` writer - nRST_STOP"]
pub type N_RST_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `nRST_STDBY` reader - nRST_STDBY"]
pub type N_RST_STDBY_R = crate::BitReader;
#[doc = "Field `nRST_STDBY` writer - nRST_STDBY"]
pub type N_RST_STDBY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IDWG_SW` reader - Independent watchdog selection"]
pub type IDWG_SW_R = crate::BitReader;
#[doc = "Field `IDWG_SW` writer - Independent watchdog selection"]
pub type IDWG_SW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IWDG_STOP` reader - Independent watchdog counter freeze in Stop mode"]
pub type IWDG_STOP_R = crate::BitReader;
#[doc = "Field `IWDG_STOP` writer - Independent watchdog counter freeze in Stop mode"]
pub type IWDG_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IWDG_STDBY` reader - Independent watchdog counter freeze in Standby mode"]
pub type IWDG_STDBY_R = crate::BitReader;
#[doc = "Field `IWDG_STDBY` writer - Independent watchdog counter freeze in Standby mode"]
pub type IWDG_STDBY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WWDG_SW` reader - Window watchdog selection"]
pub type WWDG_SW_R = crate::BitReader;
#[doc = "Field `WWDG_SW` writer - Window watchdog selection"]
pub type WWDG_SW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `nSWAP_BANK` reader - nSWAP_BANK"]
pub type N_SWAP_BANK_R = crate::BitReader;
#[doc = "Field `nSWAP_BANK` writer - nSWAP_BANK"]
pub type N_SWAP_BANK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DUAL_BANK` reader - DUAL_BANK"]
pub type DUAL_BANK_R = crate::BitReader;
#[doc = "Field `DUAL_BANK` writer - DUAL_BANK"]
pub type DUAL_BANK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RAM_PARITY_CHECK` reader - SRAM parity check control"]
pub type RAM_PARITY_CHECK_R = crate::BitReader;
#[doc = "Field `RAM_PARITY_CHECK` writer - SRAM parity check control"]
pub type RAM_PARITY_CHECK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `nBOOT_SEL` reader - nBOOT_SEL"]
pub type N_BOOT_SEL_R = crate::BitReader;
#[doc = "Field `nBOOT_SEL` writer - nBOOT_SEL"]
pub type N_BOOT_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `nBOOT1` reader - Boot configuration"]
pub type N_BOOT1_R = crate::BitReader;
#[doc = "Field `nBOOT1` writer - Boot configuration"]
pub type N_BOOT1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `nBOOT0` reader - nBOOT0 option bit"]
pub type N_BOOT0_R = crate::BitReader;
#[doc = "Field `nBOOT0` writer - nBOOT0 option bit"]
pub type N_BOOT0_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - Read protection level"]
    #[inline(always)]
    pub fn RDP(&self) -> RDP_R {
        RDP_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 13 - nRST_STOP"]
    #[inline(always)]
    pub fn nRST_STOP(&self) -> N_RST_STOP_R {
        N_RST_STOP_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - nRST_STDBY"]
    #[inline(always)]
    pub fn nRST_STDBY(&self) -> N_RST_STDBY_R {
        N_RST_STDBY_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - Independent watchdog selection"]
    #[inline(always)]
    pub fn IDWG_SW(&self) -> IDWG_SW_R {
        IDWG_SW_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Independent watchdog counter freeze in Stop mode"]
    #[inline(always)]
    pub fn IWDG_STOP(&self) -> IWDG_STOP_R {
        IWDG_STOP_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Independent watchdog counter freeze in Standby mode"]
    #[inline(always)]
    pub fn IWDG_STDBY(&self) -> IWDG_STDBY_R {
        IWDG_STDBY_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Window watchdog selection"]
    #[inline(always)]
    pub fn WWDG_SW(&self) -> WWDG_SW_R {
        WWDG_SW_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - nSWAP_BANK"]
    #[inline(always)]
    pub fn nSWAP_BANK(&self) -> N_SWAP_BANK_R {
        N_SWAP_BANK_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - DUAL_BANK"]
    #[inline(always)]
    pub fn DUAL_BANK(&self) -> DUAL_BANK_R {
        DUAL_BANK_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - SRAM parity check control"]
    #[inline(always)]
    pub fn RAM_PARITY_CHECK(&self) -> RAM_PARITY_CHECK_R {
        RAM_PARITY_CHECK_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 24 - nBOOT_SEL"]
    #[inline(always)]
    pub fn nBOOT_SEL(&self) -> N_BOOT_SEL_R {
        N_BOOT_SEL_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Boot configuration"]
    #[inline(always)]
    pub fn nBOOT1(&self) -> N_BOOT1_R {
        N_BOOT1_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - nBOOT0 option bit"]
    #[inline(always)]
    pub fn nBOOT0(&self) -> N_BOOT0_R {
        N_BOOT0_R::new(((self.bits >> 26) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Read protection level"]
    #[inline(always)]
    pub fn RDP(&mut self) -> RDP_W<'_, OPTR_SPEC> {
        RDP_W::new(self, 0)
    }
    #[doc = "Bit 13 - nRST_STOP"]
    #[inline(always)]
    pub fn nRST_STOP(&mut self) -> N_RST_STOP_W<'_, OPTR_SPEC> {
        N_RST_STOP_W::new(self, 13)
    }
    #[doc = "Bit 14 - nRST_STDBY"]
    #[inline(always)]
    pub fn nRST_STDBY(&mut self) -> N_RST_STDBY_W<'_, OPTR_SPEC> {
        N_RST_STDBY_W::new(self, 14)
    }
    #[doc = "Bit 16 - Independent watchdog selection"]
    #[inline(always)]
    pub fn IDWG_SW(&mut self) -> IDWG_SW_W<'_, OPTR_SPEC> {
        IDWG_SW_W::new(self, 16)
    }
    #[doc = "Bit 17 - Independent watchdog counter freeze in Stop mode"]
    #[inline(always)]
    pub fn IWDG_STOP(&mut self) -> IWDG_STOP_W<'_, OPTR_SPEC> {
        IWDG_STOP_W::new(self, 17)
    }
    #[doc = "Bit 18 - Independent watchdog counter freeze in Standby mode"]
    #[inline(always)]
    pub fn IWDG_STDBY(&mut self) -> IWDG_STDBY_W<'_, OPTR_SPEC> {
        IWDG_STDBY_W::new(self, 18)
    }
    #[doc = "Bit 19 - Window watchdog selection"]
    #[inline(always)]
    pub fn WWDG_SW(&mut self) -> WWDG_SW_W<'_, OPTR_SPEC> {
        WWDG_SW_W::new(self, 19)
    }
    #[doc = "Bit 20 - nSWAP_BANK"]
    #[inline(always)]
    pub fn nSWAP_BANK(&mut self) -> N_SWAP_BANK_W<'_, OPTR_SPEC> {
        N_SWAP_BANK_W::new(self, 20)
    }
    #[doc = "Bit 21 - DUAL_BANK"]
    #[inline(always)]
    pub fn DUAL_BANK(&mut self) -> DUAL_BANK_W<'_, OPTR_SPEC> {
        DUAL_BANK_W::new(self, 21)
    }
    #[doc = "Bit 22 - SRAM parity check control"]
    #[inline(always)]
    pub fn RAM_PARITY_CHECK(&mut self) -> RAM_PARITY_CHECK_W<'_, OPTR_SPEC> {
        RAM_PARITY_CHECK_W::new(self, 22)
    }
    #[doc = "Bit 24 - nBOOT_SEL"]
    #[inline(always)]
    pub fn nBOOT_SEL(&mut self) -> N_BOOT_SEL_W<'_, OPTR_SPEC> {
        N_BOOT_SEL_W::new(self, 24)
    }
    #[doc = "Bit 25 - Boot configuration"]
    #[inline(always)]
    pub fn nBOOT1(&mut self) -> N_BOOT1_W<'_, OPTR_SPEC> {
        N_BOOT1_W::new(self, 25)
    }
    #[doc = "Bit 26 - nBOOT0 option bit"]
    #[inline(always)]
    pub fn nBOOT0(&mut self) -> N_BOOT0_W<'_, OPTR_SPEC> {
        N_BOOT0_W::new(self, 26)
    }
}
#[doc = "Flash option register\n\nYou can [`read`](crate::Reg::read) this register and get [`optr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`optr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OPTR_SPEC;
impl crate::RegisterSpec for OPTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`optr::R`](R) reader structure"]
impl crate::Readable for OPTR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`optr::W`](W) writer structure"]
impl crate::Writable for OPTR_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets OPTR to value 0xf000_0000"]
impl crate::Resettable for OPTR_SPEC {
    const RESET_VALUE: u32 = 0xf000_0000;
}

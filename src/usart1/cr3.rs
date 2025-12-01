#[doc = "Register `CR3` reader"]
pub type R = crate::R<CR3_SPEC>;
#[doc = "Register `CR3` writer"]
pub type W = crate::W<CR3_SPEC>;
#[doc = "Field `EIE` reader - Error interrupt enable"]
pub type EIE_R = crate::BitReader;
#[doc = "Field `EIE` writer - Error interrupt enable"]
pub type EIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IREN` reader - Ir mode enable"]
pub type IREN_R = crate::BitReader;
#[doc = "Field `IREN` writer - Ir mode enable"]
pub type IREN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IRLP` reader - Ir low-power"]
pub type IRLP_R = crate::BitReader;
#[doc = "Field `IRLP` writer - Ir low-power"]
pub type IRLP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HDSEL` reader - Half-duplex selection"]
pub type HDSEL_R = crate::BitReader;
#[doc = "Field `HDSEL` writer - Half-duplex selection"]
pub type HDSEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NACK` reader - Smartcard NACK enable"]
pub type NACK_R = crate::BitReader;
#[doc = "Field `NACK` writer - Smartcard NACK enable"]
pub type NACK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCEN` reader - Smartcard mode enable"]
pub type SCEN_R = crate::BitReader;
#[doc = "Field `SCEN` writer - Smartcard mode enable"]
pub type SCEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMAR` reader - DMA enable receiver"]
pub type DMAR_R = crate::BitReader;
#[doc = "Field `DMAR` writer - DMA enable receiver"]
pub type DMAR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMAT` reader - DMA enable transmitter"]
pub type DMAT_R = crate::BitReader;
#[doc = "Field `DMAT` writer - DMA enable transmitter"]
pub type DMAT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTSE` reader - RTS enable"]
pub type RTSE_R = crate::BitReader;
#[doc = "Field `RTSE` writer - RTS enable"]
pub type RTSE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTSE` reader - CTS enable"]
pub type CTSE_R = crate::BitReader;
#[doc = "Field `CTSE` writer - CTS enable"]
pub type CTSE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTSIE` reader - CTS interrupt enable"]
pub type CTSIE_R = crate::BitReader;
#[doc = "Field `CTSIE` writer - CTS interrupt enable"]
pub type CTSIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ONEBIT` reader - One sample bit method enable"]
pub type ONEBIT_R = crate::BitReader;
#[doc = "Field `ONEBIT` writer - One sample bit method enable"]
pub type ONEBIT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVRDIS` reader - Overrun Disable"]
pub type OVRDIS_R = crate::BitReader;
#[doc = "Field `OVRDIS` writer - Overrun Disable"]
pub type OVRDIS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DDRE` reader - DMA Disable on Reception Error"]
pub type DDRE_R = crate::BitReader;
#[doc = "Field `DDRE` writer - DMA Disable on Reception Error"]
pub type DDRE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DEM` reader - Driver enable mode"]
pub type DEM_R = crate::BitReader;
#[doc = "Field `DEM` writer - Driver enable mode"]
pub type DEM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DEP` reader - Driver enable polarity selection"]
pub type DEP_R = crate::BitReader;
#[doc = "Field `DEP` writer - Driver enable polarity selection"]
pub type DEP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCARCNT` reader - Smartcard auto-retry count"]
pub type SCARCNT_R = crate::FieldReader;
#[doc = "Field `SCARCNT` writer - Smartcard auto-retry count"]
pub type SCARCNT_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `WUS` reader - Wakeup from Stop mode interrupt flag selection"]
pub type WUS_R = crate::FieldReader;
#[doc = "Field `WUS` writer - Wakeup from Stop mode interrupt flag selection"]
pub type WUS_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `WUFIE` reader - Wakeup from Stop mode interrupt enable"]
pub type WUFIE_R = crate::BitReader;
#[doc = "Field `WUFIE` writer - Wakeup from Stop mode interrupt enable"]
pub type WUFIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXFTIE` reader - threshold interrupt enable"]
pub type TXFTIE_R = crate::BitReader;
#[doc = "Field `TXFTIE` writer - threshold interrupt enable"]
pub type TXFTIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TCBGTIE` reader - Tr Complete before guard time, interrupt enable"]
pub type TCBGTIE_R = crate::BitReader;
#[doc = "Field `TCBGTIE` writer - Tr Complete before guard time, interrupt enable"]
pub type TCBGTIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXFTCFG` reader - Receive FIFO threshold configuration"]
pub type RXFTCFG_R = crate::FieldReader;
#[doc = "Field `RXFTCFG` writer - Receive FIFO threshold configuration"]
pub type RXFTCFG_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `RXFTIE` reader - RXFIFO threshold interrupt enable"]
pub type RXFTIE_R = crate::BitReader;
#[doc = "Field `RXFTIE` writer - RXFIFO threshold interrupt enable"]
pub type RXFTIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXFTCFG` reader - TXFIFO threshold configuration"]
pub type TXFTCFG_R = crate::FieldReader;
#[doc = "Field `TXFTCFG` writer - TXFIFO threshold configuration"]
pub type TXFTCFG_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bit 0 - Error interrupt enable"]
    #[inline(always)]
    pub fn EIE(&self) -> EIE_R {
        EIE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Ir mode enable"]
    #[inline(always)]
    pub fn IREN(&self) -> IREN_R {
        IREN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Ir low-power"]
    #[inline(always)]
    pub fn IRLP(&self) -> IRLP_R {
        IRLP_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Half-duplex selection"]
    #[inline(always)]
    pub fn HDSEL(&self) -> HDSEL_R {
        HDSEL_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Smartcard NACK enable"]
    #[inline(always)]
    pub fn NACK(&self) -> NACK_R {
        NACK_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Smartcard mode enable"]
    #[inline(always)]
    pub fn SCEN(&self) -> SCEN_R {
        SCEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - DMA enable receiver"]
    #[inline(always)]
    pub fn DMAR(&self) -> DMAR_R {
        DMAR_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - DMA enable transmitter"]
    #[inline(always)]
    pub fn DMAT(&self) -> DMAT_R {
        DMAT_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - RTS enable"]
    #[inline(always)]
    pub fn RTSE(&self) -> RTSE_R {
        RTSE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - CTS enable"]
    #[inline(always)]
    pub fn CTSE(&self) -> CTSE_R {
        CTSE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - CTS interrupt enable"]
    #[inline(always)]
    pub fn CTSIE(&self) -> CTSIE_R {
        CTSIE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - One sample bit method enable"]
    #[inline(always)]
    pub fn ONEBIT(&self) -> ONEBIT_R {
        ONEBIT_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Overrun Disable"]
    #[inline(always)]
    pub fn OVRDIS(&self) -> OVRDIS_R {
        OVRDIS_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - DMA Disable on Reception Error"]
    #[inline(always)]
    pub fn DDRE(&self) -> DDRE_R {
        DDRE_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Driver enable mode"]
    #[inline(always)]
    pub fn DEM(&self) -> DEM_R {
        DEM_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Driver enable polarity selection"]
    #[inline(always)]
    pub fn DEP(&self) -> DEP_R {
        DEP_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 17:19 - Smartcard auto-retry count"]
    #[inline(always)]
    pub fn SCARCNT(&self) -> SCARCNT_R {
        SCARCNT_R::new(((self.bits >> 17) & 7) as u8)
    }
    #[doc = "Bits 20:21 - Wakeup from Stop mode interrupt flag selection"]
    #[inline(always)]
    pub fn WUS(&self) -> WUS_R {
        WUS_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bit 22 - Wakeup from Stop mode interrupt enable"]
    #[inline(always)]
    pub fn WUFIE(&self) -> WUFIE_R {
        WUFIE_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - threshold interrupt enable"]
    #[inline(always)]
    pub fn TXFTIE(&self) -> TXFTIE_R {
        TXFTIE_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Tr Complete before guard time, interrupt enable"]
    #[inline(always)]
    pub fn TCBGTIE(&self) -> TCBGTIE_R {
        TCBGTIE_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bits 25:27 - Receive FIFO threshold configuration"]
    #[inline(always)]
    pub fn RXFTCFG(&self) -> RXFTCFG_R {
        RXFTCFG_R::new(((self.bits >> 25) & 7) as u8)
    }
    #[doc = "Bit 28 - RXFIFO threshold interrupt enable"]
    #[inline(always)]
    pub fn RXFTIE(&self) -> RXFTIE_R {
        RXFTIE_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bits 29:31 - TXFIFO threshold configuration"]
    #[inline(always)]
    pub fn TXFTCFG(&self) -> TXFTCFG_R {
        TXFTCFG_R::new(((self.bits >> 29) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Error interrupt enable"]
    #[inline(always)]
    pub fn EIE(&mut self) -> EIE_W<'_, CR3_SPEC> {
        EIE_W::new(self, 0)
    }
    #[doc = "Bit 1 - Ir mode enable"]
    #[inline(always)]
    pub fn IREN(&mut self) -> IREN_W<'_, CR3_SPEC> {
        IREN_W::new(self, 1)
    }
    #[doc = "Bit 2 - Ir low-power"]
    #[inline(always)]
    pub fn IRLP(&mut self) -> IRLP_W<'_, CR3_SPEC> {
        IRLP_W::new(self, 2)
    }
    #[doc = "Bit 3 - Half-duplex selection"]
    #[inline(always)]
    pub fn HDSEL(&mut self) -> HDSEL_W<'_, CR3_SPEC> {
        HDSEL_W::new(self, 3)
    }
    #[doc = "Bit 4 - Smartcard NACK enable"]
    #[inline(always)]
    pub fn NACK(&mut self) -> NACK_W<'_, CR3_SPEC> {
        NACK_W::new(self, 4)
    }
    #[doc = "Bit 5 - Smartcard mode enable"]
    #[inline(always)]
    pub fn SCEN(&mut self) -> SCEN_W<'_, CR3_SPEC> {
        SCEN_W::new(self, 5)
    }
    #[doc = "Bit 6 - DMA enable receiver"]
    #[inline(always)]
    pub fn DMAR(&mut self) -> DMAR_W<'_, CR3_SPEC> {
        DMAR_W::new(self, 6)
    }
    #[doc = "Bit 7 - DMA enable transmitter"]
    #[inline(always)]
    pub fn DMAT(&mut self) -> DMAT_W<'_, CR3_SPEC> {
        DMAT_W::new(self, 7)
    }
    #[doc = "Bit 8 - RTS enable"]
    #[inline(always)]
    pub fn RTSE(&mut self) -> RTSE_W<'_, CR3_SPEC> {
        RTSE_W::new(self, 8)
    }
    #[doc = "Bit 9 - CTS enable"]
    #[inline(always)]
    pub fn CTSE(&mut self) -> CTSE_W<'_, CR3_SPEC> {
        CTSE_W::new(self, 9)
    }
    #[doc = "Bit 10 - CTS interrupt enable"]
    #[inline(always)]
    pub fn CTSIE(&mut self) -> CTSIE_W<'_, CR3_SPEC> {
        CTSIE_W::new(self, 10)
    }
    #[doc = "Bit 11 - One sample bit method enable"]
    #[inline(always)]
    pub fn ONEBIT(&mut self) -> ONEBIT_W<'_, CR3_SPEC> {
        ONEBIT_W::new(self, 11)
    }
    #[doc = "Bit 12 - Overrun Disable"]
    #[inline(always)]
    pub fn OVRDIS(&mut self) -> OVRDIS_W<'_, CR3_SPEC> {
        OVRDIS_W::new(self, 12)
    }
    #[doc = "Bit 13 - DMA Disable on Reception Error"]
    #[inline(always)]
    pub fn DDRE(&mut self) -> DDRE_W<'_, CR3_SPEC> {
        DDRE_W::new(self, 13)
    }
    #[doc = "Bit 14 - Driver enable mode"]
    #[inline(always)]
    pub fn DEM(&mut self) -> DEM_W<'_, CR3_SPEC> {
        DEM_W::new(self, 14)
    }
    #[doc = "Bit 15 - Driver enable polarity selection"]
    #[inline(always)]
    pub fn DEP(&mut self) -> DEP_W<'_, CR3_SPEC> {
        DEP_W::new(self, 15)
    }
    #[doc = "Bits 17:19 - Smartcard auto-retry count"]
    #[inline(always)]
    pub fn SCARCNT(&mut self) -> SCARCNT_W<'_, CR3_SPEC> {
        SCARCNT_W::new(self, 17)
    }
    #[doc = "Bits 20:21 - Wakeup from Stop mode interrupt flag selection"]
    #[inline(always)]
    pub fn WUS(&mut self) -> WUS_W<'_, CR3_SPEC> {
        WUS_W::new(self, 20)
    }
    #[doc = "Bit 22 - Wakeup from Stop mode interrupt enable"]
    #[inline(always)]
    pub fn WUFIE(&mut self) -> WUFIE_W<'_, CR3_SPEC> {
        WUFIE_W::new(self, 22)
    }
    #[doc = "Bit 23 - threshold interrupt enable"]
    #[inline(always)]
    pub fn TXFTIE(&mut self) -> TXFTIE_W<'_, CR3_SPEC> {
        TXFTIE_W::new(self, 23)
    }
    #[doc = "Bit 24 - Tr Complete before guard time, interrupt enable"]
    #[inline(always)]
    pub fn TCBGTIE(&mut self) -> TCBGTIE_W<'_, CR3_SPEC> {
        TCBGTIE_W::new(self, 24)
    }
    #[doc = "Bits 25:27 - Receive FIFO threshold configuration"]
    #[inline(always)]
    pub fn RXFTCFG(&mut self) -> RXFTCFG_W<'_, CR3_SPEC> {
        RXFTCFG_W::new(self, 25)
    }
    #[doc = "Bit 28 - RXFIFO threshold interrupt enable"]
    #[inline(always)]
    pub fn RXFTIE(&mut self) -> RXFTIE_W<'_, CR3_SPEC> {
        RXFTIE_W::new(self, 28)
    }
    #[doc = "Bits 29:31 - TXFIFO threshold configuration"]
    #[inline(always)]
    pub fn TXFTCFG(&mut self) -> TXFTCFG_W<'_, CR3_SPEC> {
        TXFTCFG_W::new(self, 29)
    }
}
#[doc = "Control register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`cr3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CR3_SPEC;
impl crate::RegisterSpec for CR3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr3::R`](R) reader structure"]
impl crate::Readable for CR3_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cr3::W`](W) writer structure"]
impl crate::Writable for CR3_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets CR3 to value 0"]
impl crate::Resettable for CR3_SPEC {}

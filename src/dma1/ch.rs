#[repr(C)]
#[doc = "Cluster for CH\\[%s\\]"]
pub struct CH {
    pub CR: CR,
    pub NDTR: NDTR,
    pub PAR: PAR,
    pub MAR: MAR,
}
impl CH {
    #[doc = "0x00 - DMA channel 1 configuration register"]
    #[inline(always)]
    pub const fn CR(&self) -> &CR {
        &self.CR
    }
    #[doc = "0x04 - DMA channel x number of data register"]
    #[inline(always)]
    pub const fn NDTR(&self) -> &NDTR {
        &self.NDTR
    }
    #[doc = "0x08 - DMA channel x peripheral address register"]
    #[inline(always)]
    pub const fn PAR(&self) -> &PAR {
        &self.PAR
    }
    #[doc = "0x0c - DMA channel x memory address register"]
    #[inline(always)]
    pub const fn MAR(&self) -> &MAR {
        &self.MAR
    }
}
#[doc = "CR (rw) register accessor: DMA channel 1 configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr`] module"]
pub type CR = crate::Reg<cr::CR_SPEC>;
#[doc = "DMA channel 1 configuration register"]
pub mod cr;
#[doc = "NDTR (rw) register accessor: DMA channel x number of data register\n\nYou can [`read`](crate::Reg::read) this register and get [`ndtr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ndtr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ndtr`] module"]
pub type NDTR = crate::Reg<ndtr::NDTR_SPEC>;
#[doc = "DMA channel x number of data register"]
pub mod ndtr;
#[doc = "PAR (rw) register accessor: DMA channel x peripheral address register\n\nYou can [`read`](crate::Reg::read) this register and get [`par::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`par::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@par`] module"]
pub type PAR = crate::Reg<par::PAR_SPEC>;
#[doc = "DMA channel x peripheral address register"]
pub mod par;
#[doc = "MAR (rw) register accessor: DMA channel x memory address register\n\nYou can [`read`](crate::Reg::read) this register and get [`mar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mar`] module"]
pub type MAR = crate::Reg<mar::MAR_SPEC>;
#[doc = "DMA channel x memory address register"]
pub mod mar;

#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    pub CCR: [CCR; 12],
    _reserved1: [u8; 0xd0],
    pub RG0CR: RG0CR,
    pub RG1CR: RG1CR,
    pub RG2CR: RG2CR,
    pub RG3CR: RG3CR,
    _reserved5: [u8; 0x30],
    pub RGSR: RGSR,
    pub RGCFR: RGCFR,
}
impl RegisterBlock {
    #[doc = "0x00..0x30 - DMAMux - DMA request line multiplexer channel x control register"]
    #[inline(always)]
    pub const fn CCR(&self, n: usize) -> &CCR {
        &self.CCR[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x00..0x30 - DMAMux - DMA request line multiplexer channel x control register"]
    #[inline(always)]
    pub fn CCR_iter(&self) -> impl Iterator<Item = &CCR> {
        self.CCR.iter()
    }
    #[doc = "0x100 - DMAMux - DMA request generator channel x control register"]
    #[inline(always)]
    pub const fn RG0CR(&self) -> &RG0CR {
        &self.RG0CR
    }
    #[doc = "0x104 - DMAMux - DMA request generator channel x control register"]
    #[inline(always)]
    pub const fn RG1CR(&self) -> &RG1CR {
        &self.RG1CR
    }
    #[doc = "0x108 - DMAMux - DMA request generator channel x control register"]
    #[inline(always)]
    pub const fn RG2CR(&self) -> &RG2CR {
        &self.RG2CR
    }
    #[doc = "0x10c - DMAMux - DMA request generator channel x control register"]
    #[inline(always)]
    pub const fn RG3CR(&self) -> &RG3CR {
        &self.RG3CR
    }
    #[doc = "0x140 - DMAMux - DMA request generator status register"]
    #[inline(always)]
    pub const fn RGSR(&self) -> &RGSR {
        &self.RGSR
    }
    #[doc = "0x144 - DMAMux - DMA request generator clear flag register"]
    #[inline(always)]
    pub const fn RGCFR(&self) -> &RGCFR {
        &self.RGCFR
    }
}
#[doc = "CCR (rw) register accessor: DMAMux - DMA request line multiplexer channel x control register\n\nYou can [`read`](crate::Reg::read) this register and get [`ccr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccr`] module"]
pub type CCR = crate::Reg<ccr::CCR_SPEC>;
#[doc = "DMAMux - DMA request line multiplexer channel x control register"]
pub mod ccr;
#[doc = "RG0CR (rw) register accessor: DMAMux - DMA request generator channel x control register\n\nYou can [`read`](crate::Reg::read) this register and get [`rg0cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rg0cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rg0cr`] module"]
pub type RG0CR = crate::Reg<rg0cr::RG0CR_SPEC>;
#[doc = "DMAMux - DMA request generator channel x control register"]
pub mod rg0cr;
#[doc = "RG1CR (rw) register accessor: DMAMux - DMA request generator channel x control register\n\nYou can [`read`](crate::Reg::read) this register and get [`rg1cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rg1cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rg1cr`] module"]
pub type RG1CR = crate::Reg<rg1cr::RG1CR_SPEC>;
#[doc = "DMAMux - DMA request generator channel x control register"]
pub mod rg1cr;
#[doc = "RG2CR (rw) register accessor: DMAMux - DMA request generator channel x control register\n\nYou can [`read`](crate::Reg::read) this register and get [`rg2cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rg2cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rg2cr`] module"]
pub type RG2CR = crate::Reg<rg2cr::RG2CR_SPEC>;
#[doc = "DMAMux - DMA request generator channel x control register"]
pub mod rg2cr;
#[doc = "RG3CR (rw) register accessor: DMAMux - DMA request generator channel x control register\n\nYou can [`read`](crate::Reg::read) this register and get [`rg3cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rg3cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rg3cr`] module"]
pub type RG3CR = crate::Reg<rg3cr::RG3CR_SPEC>;
#[doc = "DMAMux - DMA request generator channel x control register"]
pub mod rg3cr;
#[doc = "RGSR (r) register accessor: DMAMux - DMA request generator status register\n\nYou can [`read`](crate::Reg::read) this register and get [`rgsr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rgsr`] module"]
pub type RGSR = crate::Reg<rgsr::RGSR_SPEC>;
#[doc = "DMAMux - DMA request generator status register"]
pub mod rgsr;
#[doc = "RGCFR (w) register accessor: DMAMux - DMA request generator clear flag register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rgcfr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rgcfr`] module"]
pub type RGCFR = crate::Reg<rgcfr::RGCFR_SPEC>;
#[doc = "DMAMux - DMA request generator clear flag register"]
pub mod rgcfr;

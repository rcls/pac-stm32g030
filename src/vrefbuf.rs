#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    pub CSR: CSR,
    pub CCR: CCR,
}
impl RegisterBlock {
    #[doc = "0x00 - VREFBUF control and status register"]
    #[inline(always)]
    pub const fn CSR(&self) -> &CSR {
        &self.CSR
    }
    #[doc = "0x04 - VREFBUF calibration control register"]
    #[inline(always)]
    pub const fn CCR(&self) -> &CCR {
        &self.CCR
    }
}
#[doc = "CSR (rw) register accessor: VREFBUF control and status register\n\nYou can [`read`](crate::Reg::read) this register and get [`csr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csr`] module"]
pub type CSR = crate::Reg<csr::CSR_SPEC>;
#[doc = "VREFBUF control and status register"]
pub mod csr;
#[doc = "CCR (rw) register accessor: VREFBUF calibration control register\n\nYou can [`read`](crate::Reg::read) this register and get [`ccr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccr`] module"]
pub type CCR = crate::Reg<ccr::CCR_SPEC>;
#[doc = "VREFBUF calibration control register"]
pub mod ccr;

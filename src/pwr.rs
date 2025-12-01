#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    pub CR1: CR1,
    pub CR2: CR2,
    pub CR3: CR3,
    pub CR4: CR4,
    pub SR1: SR1,
    pub SR2: SR2,
    pub SCR: SCR,
    _reserved7: [u8; 0x04],
    pub PUCRA: PUCRA,
    pub PDCRA: PDCRA,
    pub PUCRB: PUCRB,
    pub PDCRB: PDCRB,
    pub PUCRC: PUCRC,
    pub PDCRC: PDCRC,
    pub PUCRD: PUCRD,
    pub PDCRD: PDCRD,
    pub PUCRE: PUCRE,
    pub PDCRE: PDCRE,
    pub PUCRF: PUCRF,
    pub PDCRF: PDCRF,
}
impl RegisterBlock {
    #[doc = "0x00 - Power control register 1"]
    #[inline(always)]
    pub const fn CR1(&self) -> &CR1 {
        &self.CR1
    }
    #[doc = "0x04 - Power control register 2"]
    #[inline(always)]
    pub const fn CR2(&self) -> &CR2 {
        &self.CR2
    }
    #[doc = "0x08 - Power control register 3"]
    #[inline(always)]
    pub const fn CR3(&self) -> &CR3 {
        &self.CR3
    }
    #[doc = "0x0c - Power control register 4"]
    #[inline(always)]
    pub const fn CR4(&self) -> &CR4 {
        &self.CR4
    }
    #[doc = "0x10 - Power status register 1"]
    #[inline(always)]
    pub const fn SR1(&self) -> &SR1 {
        &self.SR1
    }
    #[doc = "0x14 - Power status register 2"]
    #[inline(always)]
    pub const fn SR2(&self) -> &SR2 {
        &self.SR2
    }
    #[doc = "0x18 - Power status clear register"]
    #[inline(always)]
    pub const fn SCR(&self) -> &SCR {
        &self.SCR
    }
    #[doc = "0x20 - Power Port A pull-up control register"]
    #[inline(always)]
    pub const fn PUCRA(&self) -> &PUCRA {
        &self.PUCRA
    }
    #[doc = "0x24 - Power Port A pull-down control register"]
    #[inline(always)]
    pub const fn PDCRA(&self) -> &PDCRA {
        &self.PDCRA
    }
    #[doc = "0x28 - Power Port B pull-up control register"]
    #[inline(always)]
    pub const fn PUCRB(&self) -> &PUCRB {
        &self.PUCRB
    }
    #[doc = "0x2c - Power Port B pull-down control register"]
    #[inline(always)]
    pub const fn PDCRB(&self) -> &PDCRB {
        &self.PDCRB
    }
    #[doc = "0x30 - Power Port C pull-up control register"]
    #[inline(always)]
    pub const fn PUCRC(&self) -> &PUCRC {
        &self.PUCRC
    }
    #[doc = "0x34 - Power Port C pull-down control register"]
    #[inline(always)]
    pub const fn PDCRC(&self) -> &PDCRC {
        &self.PDCRC
    }
    #[doc = "0x38 - Power Port D pull-up control register"]
    #[inline(always)]
    pub const fn PUCRD(&self) -> &PUCRD {
        &self.PUCRD
    }
    #[doc = "0x3c - Power Port D pull-down control register"]
    #[inline(always)]
    pub const fn PDCRD(&self) -> &PDCRD {
        &self.PDCRD
    }
    #[doc = "0x40 - Power Port E pull-UP control register"]
    #[inline(always)]
    pub const fn PUCRE(&self) -> &PUCRE {
        &self.PUCRE
    }
    #[doc = "0x44 - Power Port E pull-down control register"]
    #[inline(always)]
    pub const fn PDCRE(&self) -> &PDCRE {
        &self.PDCRE
    }
    #[doc = "0x48 - Power Port F pull-up control register"]
    #[inline(always)]
    pub const fn PUCRF(&self) -> &PUCRF {
        &self.PUCRF
    }
    #[doc = "0x4c - Power Port F pull-down control register"]
    #[inline(always)]
    pub const fn PDCRF(&self) -> &PDCRF {
        &self.PDCRF
    }
}
#[doc = "CR1 (rw) register accessor: Power control register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`cr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr1`] module"]
pub type CR1 = crate::Reg<cr1::CR1_SPEC>;
#[doc = "Power control register 1"]
pub mod cr1;
#[doc = "CR2 (rw) register accessor: Power control register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`cr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr2`] module"]
pub type CR2 = crate::Reg<cr2::CR2_SPEC>;
#[doc = "Power control register 2"]
pub mod cr2;
#[doc = "CR3 (rw) register accessor: Power control register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`cr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr3`] module"]
pub type CR3 = crate::Reg<cr3::CR3_SPEC>;
#[doc = "Power control register 3"]
pub mod cr3;
#[doc = "CR4 (rw) register accessor: Power control register 4\n\nYou can [`read`](crate::Reg::read) this register and get [`cr4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr4`] module"]
pub type CR4 = crate::Reg<cr4::CR4_SPEC>;
#[doc = "Power control register 4"]
pub mod cr4;
#[doc = "SR1 (r) register accessor: Power status register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`sr1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sr1`] module"]
pub type SR1 = crate::Reg<sr1::SR1_SPEC>;
#[doc = "Power status register 1"]
pub mod sr1;
#[doc = "SR2 (r) register accessor: Power status register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`sr2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sr2`] module"]
pub type SR2 = crate::Reg<sr2::SR2_SPEC>;
#[doc = "Power status register 2"]
pub mod sr2;
#[doc = "SCR (w) register accessor: Power status clear register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scr`] module"]
pub type SCR = crate::Reg<scr::SCR_SPEC>;
#[doc = "Power status clear register"]
pub mod scr;
#[doc = "PUCRA (rw) register accessor: Power Port A pull-up control register\n\nYou can [`read`](crate::Reg::read) this register and get [`pucra::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pucra::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pucra`] module"]
pub type PUCRA = crate::Reg<pucra::PUCRA_SPEC>;
#[doc = "Power Port A pull-up control register"]
pub mod pucra;
#[doc = "PDCRA (rw) register accessor: Power Port A pull-down control register\n\nYou can [`read`](crate::Reg::read) this register and get [`pdcra::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pdcra::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pdcra`] module"]
pub type PDCRA = crate::Reg<pdcra::PDCRA_SPEC>;
#[doc = "Power Port A pull-down control register"]
pub mod pdcra;
#[doc = "PUCRB (rw) register accessor: Power Port B pull-up control register\n\nYou can [`read`](crate::Reg::read) this register and get [`pucrb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pucrb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pucrb`] module"]
pub type PUCRB = crate::Reg<pucrb::PUCRB_SPEC>;
#[doc = "Power Port B pull-up control register"]
pub mod pucrb;
#[doc = "PDCRB (rw) register accessor: Power Port B pull-down control register\n\nYou can [`read`](crate::Reg::read) this register and get [`pdcrb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pdcrb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pdcrb`] module"]
pub type PDCRB = crate::Reg<pdcrb::PDCRB_SPEC>;
#[doc = "Power Port B pull-down control register"]
pub mod pdcrb;
#[doc = "PUCRC (rw) register accessor: Power Port C pull-up control register\n\nYou can [`read`](crate::Reg::read) this register and get [`pucrc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pucrc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pucrc`] module"]
pub type PUCRC = crate::Reg<pucrc::PUCRC_SPEC>;
#[doc = "Power Port C pull-up control register"]
pub mod pucrc;
#[doc = "PDCRC (rw) register accessor: Power Port C pull-down control register\n\nYou can [`read`](crate::Reg::read) this register and get [`pdcrc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pdcrc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pdcrc`] module"]
pub type PDCRC = crate::Reg<pdcrc::PDCRC_SPEC>;
#[doc = "Power Port C pull-down control register"]
pub mod pdcrc;
#[doc = "PUCRD (rw) register accessor: Power Port D pull-up control register\n\nYou can [`read`](crate::Reg::read) this register and get [`pucrd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pucrd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pucrd`] module"]
pub type PUCRD = crate::Reg<pucrd::PUCRD_SPEC>;
#[doc = "Power Port D pull-up control register"]
pub mod pucrd;
#[doc = "PDCRD (rw) register accessor: Power Port D pull-down control register\n\nYou can [`read`](crate::Reg::read) this register and get [`pdcrd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pdcrd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pdcrd`] module"]
pub type PDCRD = crate::Reg<pdcrd::PDCRD_SPEC>;
#[doc = "Power Port D pull-down control register"]
pub mod pdcrd;
#[doc = "PUCRE (rw) register accessor: Power Port E pull-UP control register\n\nYou can [`read`](crate::Reg::read) this register and get [`pucre::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pucre::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pucre`] module"]
pub type PUCRE = crate::Reg<pucre::PUCRE_SPEC>;
#[doc = "Power Port E pull-UP control register"]
pub mod pucre;
#[doc = "PDCRE (rw) register accessor: Power Port E pull-down control register\n\nYou can [`read`](crate::Reg::read) this register and get [`pdcre::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pdcre::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pdcre`] module"]
pub type PDCRE = crate::Reg<pdcre::PDCRE_SPEC>;
#[doc = "Power Port E pull-down control register"]
pub mod pdcre;
#[doc = "PUCRF (rw) register accessor: Power Port F pull-up control register\n\nYou can [`read`](crate::Reg::read) this register and get [`pucrf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pucrf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pucrf`] module"]
pub type PUCRF = crate::Reg<pucrf::PUCRF_SPEC>;
#[doc = "Power Port F pull-up control register"]
pub mod pucrf;
#[doc = "PDCRF (rw) register accessor: Power Port F pull-down control register\n\nYou can [`read`](crate::Reg::read) this register and get [`pdcrf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pdcrf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pdcrf`] module"]
pub type PDCRF = crate::Reg<pdcrf::PDCRF_SPEC>;
#[doc = "Power Port F pull-down control register"]
pub mod pdcrf;

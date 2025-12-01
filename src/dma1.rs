#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    pub ISR: ISR,
    pub IFCR: IFCR,
    pub CH: (),
}
impl RegisterBlock {
    #[doc = "0x00 - DMA interrupt status register"]
    #[inline(always)]
    pub const fn ISR(&self) -> &ISR {
        &self.ISR
    }
    #[doc = "0x04 - DMA interrupt flag clear register"]
    #[inline(always)]
    pub const fn IFCR(&self) -> &IFCR {
        &self.IFCR
    }
    #[doc = "0x08..0x78 - Cluster for CH\\[%s\\]"]
    #[inline(always)]
    pub const fn CH(&self, n: usize) -> &CH {
        #[allow(clippy::no_effect)]
        [(); 7][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(8)
                .add(20 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x08..0x78 - Cluster for CH\\[%s\\]"]
    #[inline(always)]
    pub fn CH_iter(&self) -> impl Iterator<Item = &CH> {
        (0..7).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(8)
                .add(20 * n)
                .cast()
        })
    }
}
#[doc = "ISR (r) register accessor: DMA interrupt status register\n\nYou can [`read`](crate::Reg::read) this register and get [`isr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isr`] module"]
pub type ISR = crate::Reg<isr::ISR_SPEC>;
#[doc = "DMA interrupt status register"]
pub mod isr;
#[doc = "IFCR (w) register accessor: DMA interrupt flag clear register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ifcr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ifcr`] module"]
pub type IFCR = crate::Reg<ifcr::IFCR_SPEC>;
#[doc = "DMA interrupt flag clear register"]
pub mod ifcr;
#[doc = "Cluster for CH\\[%s\\]"]
pub use self::ch::CH;
#[doc = r"Cluster"]
#[doc = "Cluster for CH\\[%s\\]"]
pub mod ch;

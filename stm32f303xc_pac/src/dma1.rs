#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - DMA interrupt status register (DMA_ISR)"]
    pub isr: ISR,
    #[doc = "0x04 - DMA interrupt flag clear register (DMA_IFCR)"]
    pub ifcr: IFCR,
    #[doc = "0x08..0x18 - Channel cluster: CCR?, CNDTR?, CPAR?, and CMAR? registers"]
    pub ch1: CH,
    _reserved3: [u8; 0x04],
    #[doc = "0x1c..0x2c - Channel cluster: CCR?, CNDTR?, CPAR?, and CMAR? registers"]
    pub ch2: CH,
    _reserved4: [u8; 0x04],
    #[doc = "0x30..0x40 - Channel cluster: CCR?, CNDTR?, CPAR?, and CMAR? registers"]
    pub ch3: CH,
    _reserved5: [u8; 0x04],
    #[doc = "0x44..0x54 - Channel cluster: CCR?, CNDTR?, CPAR?, and CMAR? registers"]
    pub ch4: CH,
    _reserved6: [u8; 0x04],
    #[doc = "0x58..0x68 - Channel cluster: CCR?, CNDTR?, CPAR?, and CMAR? registers"]
    pub ch5: CH,
    _reserved7: [u8; 0x04],
    #[doc = "0x6c..0x7c - Channel cluster: CCR?, CNDTR?, CPAR?, and CMAR? registers"]
    pub ch6: CH,
    _reserved8: [u8; 0x04],
    #[doc = "0x80..0x90 - Channel cluster: CCR?, CNDTR?, CPAR?, and CMAR? registers"]
    pub ch7: CH,
}
#[doc = "Channel cluster: CCR?, CNDTR?, CPAR?, and CMAR? registers"]
pub use self::ch::CH;
#[doc = r"Cluster"]
#[doc = "Channel cluster: CCR?, CNDTR?, CPAR?, and CMAR? registers"]
pub mod ch;
#[doc = "ISR (r) register accessor: an alias for `Reg<ISR_SPEC>`"]
pub type ISR = crate::Reg<isr::ISR_SPEC>;
#[doc = "DMA interrupt status register (DMA_ISR)"]
pub mod isr;
#[doc = "IFCR (w) register accessor: an alias for `Reg<IFCR_SPEC>`"]
pub type IFCR = crate::Reg<ifcr::IFCR_SPEC>;
#[doc = "DMA interrupt flag clear register (DMA_IFCR)"]
pub mod ifcr;

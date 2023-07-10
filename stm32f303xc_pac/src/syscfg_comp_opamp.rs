#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - configuration register 1"]
    pub syscfg_cfgr1: SYSCFG_CFGR1,
    #[doc = "0x04 - CCM SRAM protection register"]
    pub syscfg_rcr: SYSCFG_RCR,
    #[doc = "0x08 - external interrupt configuration register 1"]
    pub syscfg_exticr1: SYSCFG_EXTICR1,
    #[doc = "0x0c - external interrupt configuration register 2"]
    pub syscfg_exticr2: SYSCFG_EXTICR2,
    #[doc = "0x10 - external interrupt configuration register 3"]
    pub syscfg_exticr3: SYSCFG_EXTICR3,
    #[doc = "0x14 - external interrupt configuration register 4"]
    pub syscfg_exticr4: SYSCFG_EXTICR4,
    #[doc = "0x18 - configuration register 2"]
    pub syscfg_cfgr2: SYSCFG_CFGR2,
    #[doc = "0x1c - control and status register"]
    pub comp1_csr: COMP1_CSR,
    #[doc = "0x20 - control and status register"]
    pub comp2_csr: COMP2_CSR,
    #[doc = "0x24 - control and status register"]
    pub comp3_csr: COMP3_CSR,
    #[doc = "0x28 - control and status register"]
    pub comp4_csr: COMP4_CSR,
    #[doc = "0x2c - control and status register"]
    pub comp5_csr: COMP5_CSR,
    #[doc = "0x30 - control and status register"]
    pub comp6_csr: COMP6_CSR,
    #[doc = "0x34 - control and status register"]
    pub comp7_csr: COMP7_CSR,
    #[doc = "0x38 - control register"]
    pub opamp1_csr: OPAMP1_CSR,
    #[doc = "0x3c - control register"]
    pub opamp2_csr: OPAMP2_CSR,
    #[doc = "0x40 - control register"]
    pub opamp3_csr: OPAMP3_CSR,
    #[doc = "0x44 - control register"]
    pub opamp4_csr: OPAMP4_CSR,
}
#[doc = "SYSCFG_CFGR1 (rw) register accessor: an alias for `Reg<SYSCFG_CFGR1_SPEC>`"]
pub type SYSCFG_CFGR1 = crate::Reg<syscfg_cfgr1::SYSCFG_CFGR1_SPEC>;
#[doc = "configuration register 1"]
pub mod syscfg_cfgr1;
#[doc = "SYSCFG_EXTICR1 (rw) register accessor: an alias for `Reg<SYSCFG_EXTICR1_SPEC>`"]
pub type SYSCFG_EXTICR1 = crate::Reg<syscfg_exticr1::SYSCFG_EXTICR1_SPEC>;
#[doc = "external interrupt configuration register 1"]
pub mod syscfg_exticr1;
#[doc = "SYSCFG_EXTICR2 (rw) register accessor: an alias for `Reg<SYSCFG_EXTICR2_SPEC>`"]
pub type SYSCFG_EXTICR2 = crate::Reg<syscfg_exticr2::SYSCFG_EXTICR2_SPEC>;
#[doc = "external interrupt configuration register 2"]
pub mod syscfg_exticr2;
#[doc = "SYSCFG_EXTICR3 (rw) register accessor: an alias for `Reg<SYSCFG_EXTICR3_SPEC>`"]
pub type SYSCFG_EXTICR3 = crate::Reg<syscfg_exticr3::SYSCFG_EXTICR3_SPEC>;
#[doc = "external interrupt configuration register 3"]
pub mod syscfg_exticr3;
#[doc = "SYSCFG_EXTICR4 (rw) register accessor: an alias for `Reg<SYSCFG_EXTICR4_SPEC>`"]
pub type SYSCFG_EXTICR4 = crate::Reg<syscfg_exticr4::SYSCFG_EXTICR4_SPEC>;
#[doc = "external interrupt configuration register 4"]
pub mod syscfg_exticr4;
#[doc = "SYSCFG_CFGR2 (rw) register accessor: an alias for `Reg<SYSCFG_CFGR2_SPEC>`"]
pub type SYSCFG_CFGR2 = crate::Reg<syscfg_cfgr2::SYSCFG_CFGR2_SPEC>;
#[doc = "configuration register 2"]
pub mod syscfg_cfgr2;
#[doc = "SYSCFG_RCR (rw) register accessor: an alias for `Reg<SYSCFG_RCR_SPEC>`"]
pub type SYSCFG_RCR = crate::Reg<syscfg_rcr::SYSCFG_RCR_SPEC>;
#[doc = "CCM SRAM protection register"]
pub mod syscfg_rcr;
#[doc = "COMP1_CSR (rw) register accessor: an alias for `Reg<COMP1_CSR_SPEC>`"]
pub type COMP1_CSR = crate::Reg<comp1_csr::COMP1_CSR_SPEC>;
#[doc = "control and status register"]
pub mod comp1_csr;
#[doc = "COMP2_CSR (rw) register accessor: an alias for `Reg<COMP2_CSR_SPEC>`"]
pub type COMP2_CSR = crate::Reg<comp2_csr::COMP2_CSR_SPEC>;
#[doc = "control and status register"]
pub mod comp2_csr;
#[doc = "COMP3_CSR (rw) register accessor: an alias for `Reg<COMP3_CSR_SPEC>`"]
pub type COMP3_CSR = crate::Reg<comp3_csr::COMP3_CSR_SPEC>;
#[doc = "control and status register"]
pub mod comp3_csr;
#[doc = "COMP4_CSR (rw) register accessor: an alias for `Reg<COMP4_CSR_SPEC>`"]
pub type COMP4_CSR = crate::Reg<comp4_csr::COMP4_CSR_SPEC>;
#[doc = "control and status register"]
pub mod comp4_csr;
#[doc = "COMP5_CSR (rw) register accessor: an alias for `Reg<COMP5_CSR_SPEC>`"]
pub type COMP5_CSR = crate::Reg<comp5_csr::COMP5_CSR_SPEC>;
#[doc = "control and status register"]
pub mod comp5_csr;
#[doc = "COMP6_CSR (rw) register accessor: an alias for `Reg<COMP6_CSR_SPEC>`"]
pub type COMP6_CSR = crate::Reg<comp6_csr::COMP6_CSR_SPEC>;
#[doc = "control and status register"]
pub mod comp6_csr;
#[doc = "COMP7_CSR (rw) register accessor: an alias for `Reg<COMP7_CSR_SPEC>`"]
pub type COMP7_CSR = crate::Reg<comp7_csr::COMP7_CSR_SPEC>;
#[doc = "control and status register"]
pub mod comp7_csr;
#[doc = "OPAMP1_CSR (rw) register accessor: an alias for `Reg<OPAMP1_CSR_SPEC>`"]
pub type OPAMP1_CSR = crate::Reg<opamp1_csr::OPAMP1_CSR_SPEC>;
#[doc = "control register"]
pub mod opamp1_csr;
#[doc = "OPAMP2_CSR (rw) register accessor: an alias for `Reg<OPAMP2_CSR_SPEC>`"]
pub type OPAMP2_CSR = crate::Reg<opamp2_csr::OPAMP2_CSR_SPEC>;
#[doc = "control register"]
pub mod opamp2_csr;
#[doc = "OPAMP3_CSR (rw) register accessor: an alias for `Reg<OPAMP3_CSR_SPEC>`"]
pub type OPAMP3_CSR = crate::Reg<opamp3_csr::OPAMP3_CSR_SPEC>;
#[doc = "control register"]
pub mod opamp3_csr;
#[doc = "OPAMP4_CSR (rw) register accessor: an alias for `Reg<OPAMP4_CSR_SPEC>`"]
pub type OPAMP4_CSR = crate::Reg<opamp4_csr::OPAMP4_CSR_SPEC>;
#[doc = "control register"]
pub mod opamp4_csr;

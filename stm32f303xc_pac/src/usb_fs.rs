#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - endpoint 0 register"]
    pub usb_ep0r: USB_EP0R,
    #[doc = "0x04 - endpoint 1 register"]
    pub usb_ep1r: USB_EP1R,
    #[doc = "0x08 - endpoint 2 register"]
    pub usb_ep2r: USB_EP2R,
    #[doc = "0x0c - endpoint 3 register"]
    pub usb_ep3r: USB_EP3R,
    #[doc = "0x10 - endpoint 4 register"]
    pub usb_ep4r: USB_EP4R,
    #[doc = "0x14 - endpoint 5 register"]
    pub usb_ep5r: USB_EP5R,
    #[doc = "0x18 - endpoint 6 register"]
    pub usb_ep6r: USB_EP6R,
    #[doc = "0x1c - endpoint 7 register"]
    pub usb_ep7r: USB_EP7R,
    _reserved8: [u8; 0x20],
    #[doc = "0x40 - control register"]
    pub usb_cntr: USB_CNTR,
    #[doc = "0x44 - interrupt status register"]
    pub istr: ISTR,
    #[doc = "0x48 - frame number register"]
    pub fnr: FNR,
    #[doc = "0x4c - device address"]
    pub daddr: DADDR,
    #[doc = "0x50 - Buffer table address"]
    pub btable: BTABLE,
}
#[doc = "USB_EP0R (rw) register accessor: an alias for `Reg<USB_EP0R_SPEC>`"]
pub type USB_EP0R = crate::Reg<usb_ep0r::USB_EP0R_SPEC>;
#[doc = "endpoint 0 register"]
pub mod usb_ep0r;
#[doc = "USB_EP1R (rw) register accessor: an alias for `Reg<USB_EP1R_SPEC>`"]
pub type USB_EP1R = crate::Reg<usb_ep1r::USB_EP1R_SPEC>;
#[doc = "endpoint 1 register"]
pub mod usb_ep1r;
#[doc = "USB_EP2R (rw) register accessor: an alias for `Reg<USB_EP2R_SPEC>`"]
pub type USB_EP2R = crate::Reg<usb_ep2r::USB_EP2R_SPEC>;
#[doc = "endpoint 2 register"]
pub mod usb_ep2r;
#[doc = "USB_EP3R (rw) register accessor: an alias for `Reg<USB_EP3R_SPEC>`"]
pub type USB_EP3R = crate::Reg<usb_ep3r::USB_EP3R_SPEC>;
#[doc = "endpoint 3 register"]
pub mod usb_ep3r;
#[doc = "USB_EP4R (rw) register accessor: an alias for `Reg<USB_EP4R_SPEC>`"]
pub type USB_EP4R = crate::Reg<usb_ep4r::USB_EP4R_SPEC>;
#[doc = "endpoint 4 register"]
pub mod usb_ep4r;
#[doc = "USB_EP5R (rw) register accessor: an alias for `Reg<USB_EP5R_SPEC>`"]
pub type USB_EP5R = crate::Reg<usb_ep5r::USB_EP5R_SPEC>;
#[doc = "endpoint 5 register"]
pub mod usb_ep5r;
#[doc = "USB_EP6R (rw) register accessor: an alias for `Reg<USB_EP6R_SPEC>`"]
pub type USB_EP6R = crate::Reg<usb_ep6r::USB_EP6R_SPEC>;
#[doc = "endpoint 6 register"]
pub mod usb_ep6r;
#[doc = "USB_EP7R (rw) register accessor: an alias for `Reg<USB_EP7R_SPEC>`"]
pub type USB_EP7R = crate::Reg<usb_ep7r::USB_EP7R_SPEC>;
#[doc = "endpoint 7 register"]
pub mod usb_ep7r;
#[doc = "USB_CNTR (rw) register accessor: an alias for `Reg<USB_CNTR_SPEC>`"]
pub type USB_CNTR = crate::Reg<usb_cntr::USB_CNTR_SPEC>;
#[doc = "control register"]
pub mod usb_cntr;
#[doc = "ISTR (rw) register accessor: an alias for `Reg<ISTR_SPEC>`"]
pub type ISTR = crate::Reg<istr::ISTR_SPEC>;
#[doc = "interrupt status register"]
pub mod istr;
#[doc = "FNR (r) register accessor: an alias for `Reg<FNR_SPEC>`"]
pub type FNR = crate::Reg<fnr::FNR_SPEC>;
#[doc = "frame number register"]
pub mod fnr;
#[doc = "DADDR (rw) register accessor: an alias for `Reg<DADDR_SPEC>`"]
pub type DADDR = crate::Reg<daddr::DADDR_SPEC>;
#[doc = "device address"]
pub mod daddr;
#[doc = "BTABLE (rw) register accessor: an alias for `Reg<BTABLE_SPEC>`"]
pub type BTABLE = crate::Reg<btable::BTABLE_SPEC>;
#[doc = "Buffer table address"]
pub mod btable;

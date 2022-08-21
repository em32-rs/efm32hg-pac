#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - System Control Register"]
    pub ctrl: CTRL,
    #[doc = "0x04 - System Status Register"]
    pub status: STATUS,
    #[doc = "0x08 - Interrupt Flag Register"]
    pub if_: IF,
    #[doc = "0x0c - Interrupt Flag Set Register"]
    pub ifs: IFS,
    #[doc = "0x10 - Interrupt Flag Clear Register"]
    pub ifc: IFC,
    #[doc = "0x14 - Interrupt Enable Register"]
    pub ien: IEN,
    #[doc = "0x18 - I/O Routing Register"]
    pub route: ROUTE,
    _reserved7: [u8; 0x0003_bfec],
    #[doc = "0x3c008 - AHB Configuration Register"]
    pub gahbcfg: GAHBCFG,
    #[doc = "0x3c00c - USB Configuration Register"]
    pub gusbcfg: GUSBCFG,
    #[doc = "0x3c010 - Reset Register"]
    pub grstctl: GRSTCTL,
    #[doc = "0x3c014 - Interrupt Register"]
    pub gintsts: GINTSTS,
    #[doc = "0x3c018 - Interrupt Mask Register"]
    pub gintmsk: GINTMSK,
    #[doc = "0x3c01c - Receive Status Debug Read Register"]
    pub grxstsr: GRXSTSR,
    #[doc = "0x3c020 - Receive Status Read and Pop Register"]
    pub grxstsp: GRXSTSP,
    #[doc = "0x3c024 - Receive FIFO Size Register"]
    pub grxfsiz: GRXFSIZ,
    #[doc = "0x3c028 - Non-periodic Transmit FIFO Size Register"]
    pub gnptxfsiz: GNPTXFSIZ,
    _reserved16: [u8; 0x30],
    #[doc = "0x3c05c - Global DFIFO Configuration Register"]
    pub gdfifocfg: GDFIFOCFG,
    _reserved17: [u8; 0xa4],
    #[doc = "0x3c104 - Device IN Endpoint Transmit FIFO 1 Size Register"]
    pub dieptxf1: DIEPTXF1,
    #[doc = "0x3c108 - Device IN Endpoint Transmit FIFO 2 Size Register"]
    pub dieptxf2: DIEPTXF2,
    #[doc = "0x3c10c - Device IN Endpoint Transmit FIFO 3 Size Register"]
    pub dieptxf3: DIEPTXF3,
    _reserved20: [u8; 0x06f0],
    #[doc = "0x3c800 - Device Configuration Register"]
    pub dcfg: DCFG,
    #[doc = "0x3c804 - Device Control Register"]
    pub dctl: DCTL,
    #[doc = "0x3c808 - Device Status Register"]
    pub dsts: DSTS,
    _reserved23: [u8; 0x04],
    #[doc = "0x3c810 - Device IN Endpoint Common Interrupt Mask Register"]
    pub diepmsk: DIEPMSK,
    #[doc = "0x3c814 - Device OUT Endpoint Common Interrupt Mask Register"]
    pub doepmsk: DOEPMSK,
    #[doc = "0x3c818 - Device All Endpoints Interrupt Register"]
    pub daint: DAINT,
    #[doc = "0x3c81c - Device All Endpoints Interrupt Mask Register"]
    pub daintmsk: DAINTMSK,
    _reserved27: [u8; 0x14],
    #[doc = "0x3c834 - Device IN Endpoint FIFO Empty Interrupt Mask Register"]
    pub diepempmsk: DIEPEMPMSK,
    _reserved28: [u8; 0xc8],
    #[doc = "0x3c900 - Device IN Endpoint 0 Control Register"]
    pub diep0ctl: DIEP0CTL,
    _reserved29: [u8; 0x04],
    #[doc = "0x3c908 - Device IN Endpoint 0 Interrupt Register"]
    pub diep0int: DIEP0INT,
    _reserved30: [u8; 0x04],
    #[doc = "0x3c910 - Device IN Endpoint 0 Transfer Size Register"]
    pub diep0tsiz: DIEP0TSIZ,
    #[doc = "0x3c914 - Device IN Endpoint 0 DMA Address Register"]
    pub diep0dmaaddr: DIEP0DMAADDR,
    #[doc = "0x3c918 - Device IN Endpoint 0 Transmit FIFO Status Register"]
    pub diep0txfsts: DIEP0TXFSTS,
    _reserved33: [u8; 0x04],
    #[doc = "0x3c920 - Device IN Endpoint x+1 Control Register"]
    pub diep0_ctl: DIEP0_CTL,
    _reserved34: [u8; 0x04],
    #[doc = "0x3c928 - Device IN Endpoint x+1 Interrupt Register"]
    pub diep0_int: DIEP0_INT,
    _reserved35: [u8; 0x04],
    #[doc = "0x3c930 - Device IN Endpoint x+1 Transfer Size Register"]
    pub diep0_tsiz: DIEP0_TSIZ,
    #[doc = "0x3c934 - Device IN Endpoint x+1 DMA Address Register"]
    pub diep0_dmaaddr: DIEP0_DMAADDR,
    #[doc = "0x3c938 - Device IN Endpoint x+1 Transmit FIFO Status Register"]
    pub diep0_txfsts: DIEP0_TXFSTS,
    _reserved38: [u8; 0x04],
    #[doc = "0x3c940 - Device IN Endpoint x+1 Control Register"]
    pub diep1_ctl: DIEP1_CTL,
    _reserved39: [u8; 0x04],
    #[doc = "0x3c948 - Device IN Endpoint x+1 Interrupt Register"]
    pub diep1_int: DIEP1_INT,
    _reserved40: [u8; 0x04],
    #[doc = "0x3c950 - Device IN Endpoint x+1 Transfer Size Register"]
    pub diep1_tsiz: DIEP1_TSIZ,
    #[doc = "0x3c954 - Device IN Endpoint x+1 DMA Address Register"]
    pub diep1_dmaaddr: DIEP1_DMAADDR,
    #[doc = "0x3c958 - Device IN Endpoint x+1 Transmit FIFO Status Register"]
    pub diep1_txfsts: DIEP1_TXFSTS,
    _reserved43: [u8; 0x04],
    #[doc = "0x3c960 - Device IN Endpoint x+1 Control Register"]
    pub diep2_ctl: DIEP2_CTL,
    _reserved44: [u8; 0x04],
    #[doc = "0x3c968 - Device IN Endpoint x+1 Interrupt Register"]
    pub diep2_int: DIEP2_INT,
    _reserved45: [u8; 0x04],
    #[doc = "0x3c970 - Device IN Endpoint x+1 Transfer Size Register"]
    pub diep2_tsiz: DIEP2_TSIZ,
    #[doc = "0x3c974 - Device IN Endpoint x+1 DMA Address Register"]
    pub diep2_dmaaddr: DIEP2_DMAADDR,
    #[doc = "0x3c978 - Device IN Endpoint x+1 Transmit FIFO Status Register"]
    pub diep2_txfsts: DIEP2_TXFSTS,
    _reserved48: [u8; 0x0184],
    #[doc = "0x3cb00 - Device OUT Endpoint 0 Control Register"]
    pub doep0ctl: DOEP0CTL,
    _reserved49: [u8; 0x04],
    #[doc = "0x3cb08 - Device OUT Endpoint 0 Interrupt Register"]
    pub doep0int: DOEP0INT,
    _reserved50: [u8; 0x04],
    #[doc = "0x3cb10 - Device OUT Endpoint 0 Transfer Size Register"]
    pub doep0tsiz: DOEP0TSIZ,
    #[doc = "0x3cb14 - Device OUT Endpoint 0 DMA Address Register"]
    pub doep0dmaaddr: DOEP0DMAADDR,
    _reserved52: [u8; 0x08],
    #[doc = "0x3cb20 - Device OUT Endpoint x+1 Control Register"]
    pub doep0_ctl: DOEP0_CTL,
    _reserved53: [u8; 0x04],
    #[doc = "0x3cb28 - Device OUT Endpoint x+1 Interrupt Register"]
    pub doep0_int: DOEP0_INT,
    _reserved54: [u8; 0x04],
    #[doc = "0x3cb30 - Device OUT Endpoint x+1 Transfer Size Register"]
    pub doep0_tsiz: DOEP0_TSIZ,
    #[doc = "0x3cb34 - Device OUT Endpoint x+1 DMA Address Register"]
    pub doep0_dmaaddr: DOEP0_DMAADDR,
    _reserved56: [u8; 0x08],
    #[doc = "0x3cb40 - Device OUT Endpoint x+1 Control Register"]
    pub doep1_ctl: DOEP1_CTL,
    _reserved57: [u8; 0x04],
    #[doc = "0x3cb48 - Device OUT Endpoint x+1 Interrupt Register"]
    pub doep1_int: DOEP1_INT,
    _reserved58: [u8; 0x04],
    #[doc = "0x3cb50 - Device OUT Endpoint x+1 Transfer Size Register"]
    pub doep1_tsiz: DOEP1_TSIZ,
    #[doc = "0x3cb54 - Device OUT Endpoint x+1 DMA Address Register"]
    pub doep1_dmaaddr: DOEP1_DMAADDR,
    _reserved60: [u8; 0x08],
    #[doc = "0x3cb60 - Device OUT Endpoint x+1 Control Register"]
    pub doep2_ctl: DOEP2_CTL,
    _reserved61: [u8; 0x04],
    #[doc = "0x3cb68 - Device OUT Endpoint x+1 Interrupt Register"]
    pub doep2_int: DOEP2_INT,
    _reserved62: [u8; 0x04],
    #[doc = "0x3cb70 - Device OUT Endpoint x+1 Transfer Size Register"]
    pub doep2_tsiz: DOEP2_TSIZ,
    #[doc = "0x3cb74 - Device OUT Endpoint x+1 DMA Address Register"]
    pub doep2_dmaaddr: DOEP2_DMAADDR,
    _reserved64: [u8; 0x0288],
    #[doc = "0x3ce00 - Power and Clock Gating Control Register"]
    pub pcgcctl: PCGCCTL,
}
#[doc = "CTRL (rw) register accessor: an alias for `Reg<CTRL_SPEC>`"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "System Control Register"]
pub mod ctrl;
#[doc = "STATUS (r) register accessor: an alias for `Reg<STATUS_SPEC>`"]
pub type STATUS = crate::Reg<status::STATUS_SPEC>;
#[doc = "System Status Register"]
pub mod status;
#[doc = "IF (r) register accessor: an alias for `Reg<IF_SPEC>`"]
pub type IF = crate::Reg<if_::IF_SPEC>;
#[doc = "Interrupt Flag Register"]
pub mod if_;
#[doc = "IFS (w) register accessor: an alias for `Reg<IFS_SPEC>`"]
pub type IFS = crate::Reg<ifs::IFS_SPEC>;
#[doc = "Interrupt Flag Set Register"]
pub mod ifs;
#[doc = "IFC (w) register accessor: an alias for `Reg<IFC_SPEC>`"]
pub type IFC = crate::Reg<ifc::IFC_SPEC>;
#[doc = "Interrupt Flag Clear Register"]
pub mod ifc;
#[doc = "IEN (rw) register accessor: an alias for `Reg<IEN_SPEC>`"]
pub type IEN = crate::Reg<ien::IEN_SPEC>;
#[doc = "Interrupt Enable Register"]
pub mod ien;
#[doc = "ROUTE (rw) register accessor: an alias for `Reg<ROUTE_SPEC>`"]
pub type ROUTE = crate::Reg<route::ROUTE_SPEC>;
#[doc = "I/O Routing Register"]
pub mod route;
#[doc = "GAHBCFG (rw) register accessor: an alias for `Reg<GAHBCFG_SPEC>`"]
pub type GAHBCFG = crate::Reg<gahbcfg::GAHBCFG_SPEC>;
#[doc = "AHB Configuration Register"]
pub mod gahbcfg;
#[doc = "GUSBCFG (rw) register accessor: an alias for `Reg<GUSBCFG_SPEC>`"]
pub type GUSBCFG = crate::Reg<gusbcfg::GUSBCFG_SPEC>;
#[doc = "USB Configuration Register"]
pub mod gusbcfg;
#[doc = "GRSTCTL (rw) register accessor: an alias for `Reg<GRSTCTL_SPEC>`"]
pub type GRSTCTL = crate::Reg<grstctl::GRSTCTL_SPEC>;
#[doc = "Reset Register"]
pub mod grstctl;
#[doc = "GINTSTS (rw) register accessor: an alias for `Reg<GINTSTS_SPEC>`"]
pub type GINTSTS = crate::Reg<gintsts::GINTSTS_SPEC>;
#[doc = "Interrupt Register"]
pub mod gintsts;
#[doc = "GINTMSK (rw) register accessor: an alias for `Reg<GINTMSK_SPEC>`"]
pub type GINTMSK = crate::Reg<gintmsk::GINTMSK_SPEC>;
#[doc = "Interrupt Mask Register"]
pub mod gintmsk;
#[doc = "GRXSTSR (r) register accessor: an alias for `Reg<GRXSTSR_SPEC>`"]
pub type GRXSTSR = crate::Reg<grxstsr::GRXSTSR_SPEC>;
#[doc = "Receive Status Debug Read Register"]
pub mod grxstsr;
#[doc = "GRXSTSP (r) register accessor: an alias for `Reg<GRXSTSP_SPEC>`"]
pub type GRXSTSP = crate::Reg<grxstsp::GRXSTSP_SPEC>;
#[doc = "Receive Status Read and Pop Register"]
pub mod grxstsp;
#[doc = "GRXFSIZ (rw) register accessor: an alias for `Reg<GRXFSIZ_SPEC>`"]
pub type GRXFSIZ = crate::Reg<grxfsiz::GRXFSIZ_SPEC>;
#[doc = "Receive FIFO Size Register"]
pub mod grxfsiz;
#[doc = "GNPTXFSIZ (rw) register accessor: an alias for `Reg<GNPTXFSIZ_SPEC>`"]
pub type GNPTXFSIZ = crate::Reg<gnptxfsiz::GNPTXFSIZ_SPEC>;
#[doc = "Non-periodic Transmit FIFO Size Register"]
pub mod gnptxfsiz;
#[doc = "GDFIFOCFG (rw) register accessor: an alias for `Reg<GDFIFOCFG_SPEC>`"]
pub type GDFIFOCFG = crate::Reg<gdfifocfg::GDFIFOCFG_SPEC>;
#[doc = "Global DFIFO Configuration Register"]
pub mod gdfifocfg;
#[doc = "DIEPTXF1 (rw) register accessor: an alias for `Reg<DIEPTXF1_SPEC>`"]
pub type DIEPTXF1 = crate::Reg<dieptxf1::DIEPTXF1_SPEC>;
#[doc = "Device IN Endpoint Transmit FIFO 1 Size Register"]
pub mod dieptxf1;
#[doc = "DIEPTXF2 (rw) register accessor: an alias for `Reg<DIEPTXF2_SPEC>`"]
pub type DIEPTXF2 = crate::Reg<dieptxf2::DIEPTXF2_SPEC>;
#[doc = "Device IN Endpoint Transmit FIFO 2 Size Register"]
pub mod dieptxf2;
#[doc = "DIEPTXF3 (rw) register accessor: an alias for `Reg<DIEPTXF3_SPEC>`"]
pub type DIEPTXF3 = crate::Reg<dieptxf3::DIEPTXF3_SPEC>;
#[doc = "Device IN Endpoint Transmit FIFO 3 Size Register"]
pub mod dieptxf3;
#[doc = "DCFG (rw) register accessor: an alias for `Reg<DCFG_SPEC>`"]
pub type DCFG = crate::Reg<dcfg::DCFG_SPEC>;
#[doc = "Device Configuration Register"]
pub mod dcfg;
#[doc = "DCTL (rw) register accessor: an alias for `Reg<DCTL_SPEC>`"]
pub type DCTL = crate::Reg<dctl::DCTL_SPEC>;
#[doc = "Device Control Register"]
pub mod dctl;
#[doc = "DSTS (r) register accessor: an alias for `Reg<DSTS_SPEC>`"]
pub type DSTS = crate::Reg<dsts::DSTS_SPEC>;
#[doc = "Device Status Register"]
pub mod dsts;
#[doc = "DIEPMSK (rw) register accessor: an alias for `Reg<DIEPMSK_SPEC>`"]
pub type DIEPMSK = crate::Reg<diepmsk::DIEPMSK_SPEC>;
#[doc = "Device IN Endpoint Common Interrupt Mask Register"]
pub mod diepmsk;
#[doc = "DOEPMSK (rw) register accessor: an alias for `Reg<DOEPMSK_SPEC>`"]
pub type DOEPMSK = crate::Reg<doepmsk::DOEPMSK_SPEC>;
#[doc = "Device OUT Endpoint Common Interrupt Mask Register"]
pub mod doepmsk;
#[doc = "DAINT (r) register accessor: an alias for `Reg<DAINT_SPEC>`"]
pub type DAINT = crate::Reg<daint::DAINT_SPEC>;
#[doc = "Device All Endpoints Interrupt Register"]
pub mod daint;
#[doc = "DAINTMSK (rw) register accessor: an alias for `Reg<DAINTMSK_SPEC>`"]
pub type DAINTMSK = crate::Reg<daintmsk::DAINTMSK_SPEC>;
#[doc = "Device All Endpoints Interrupt Mask Register"]
pub mod daintmsk;
#[doc = "DIEPEMPMSK (rw) register accessor: an alias for `Reg<DIEPEMPMSK_SPEC>`"]
pub type DIEPEMPMSK = crate::Reg<diepempmsk::DIEPEMPMSK_SPEC>;
#[doc = "Device IN Endpoint FIFO Empty Interrupt Mask Register"]
pub mod diepempmsk;
#[doc = "DIEP0CTL (rw) register accessor: an alias for `Reg<DIEP0CTL_SPEC>`"]
pub type DIEP0CTL = crate::Reg<diep0ctl::DIEP0CTL_SPEC>;
#[doc = "Device IN Endpoint 0 Control Register"]
pub mod diep0ctl;
#[doc = "DIEP0INT (rw) register accessor: an alias for `Reg<DIEP0INT_SPEC>`"]
pub type DIEP0INT = crate::Reg<diep0int::DIEP0INT_SPEC>;
#[doc = "Device IN Endpoint 0 Interrupt Register"]
pub mod diep0int;
#[doc = "DIEP0TSIZ (rw) register accessor: an alias for `Reg<DIEP0TSIZ_SPEC>`"]
pub type DIEP0TSIZ = crate::Reg<diep0tsiz::DIEP0TSIZ_SPEC>;
#[doc = "Device IN Endpoint 0 Transfer Size Register"]
pub mod diep0tsiz;
#[doc = "DIEP0DMAADDR (rw) register accessor: an alias for `Reg<DIEP0DMAADDR_SPEC>`"]
pub type DIEP0DMAADDR = crate::Reg<diep0dmaaddr::DIEP0DMAADDR_SPEC>;
#[doc = "Device IN Endpoint 0 DMA Address Register"]
pub mod diep0dmaaddr;
#[doc = "DIEP0TXFSTS (r) register accessor: an alias for `Reg<DIEP0TXFSTS_SPEC>`"]
pub type DIEP0TXFSTS = crate::Reg<diep0txfsts::DIEP0TXFSTS_SPEC>;
#[doc = "Device IN Endpoint 0 Transmit FIFO Status Register"]
pub mod diep0txfsts;
#[doc = "DIEP0_CTL (rw) register accessor: an alias for `Reg<DIEP0_CTL_SPEC>`"]
pub type DIEP0_CTL = crate::Reg<diep0_ctl::DIEP0_CTL_SPEC>;
#[doc = "Device IN Endpoint x+1 Control Register"]
pub mod diep0_ctl;
#[doc = "DIEP0_INT (rw) register accessor: an alias for `Reg<DIEP0_INT_SPEC>`"]
pub type DIEP0_INT = crate::Reg<diep0_int::DIEP0_INT_SPEC>;
#[doc = "Device IN Endpoint x+1 Interrupt Register"]
pub mod diep0_int;
#[doc = "DIEP0_TSIZ (rw) register accessor: an alias for `Reg<DIEP0_TSIZ_SPEC>`"]
pub type DIEP0_TSIZ = crate::Reg<diep0_tsiz::DIEP0_TSIZ_SPEC>;
#[doc = "Device IN Endpoint x+1 Transfer Size Register"]
pub mod diep0_tsiz;
#[doc = "DIEP0_DMAADDR (rw) register accessor: an alias for `Reg<DIEP0_DMAADDR_SPEC>`"]
pub type DIEP0_DMAADDR = crate::Reg<diep0_dmaaddr::DIEP0_DMAADDR_SPEC>;
#[doc = "Device IN Endpoint x+1 DMA Address Register"]
pub mod diep0_dmaaddr;
#[doc = "DIEP0_TXFSTS (r) register accessor: an alias for `Reg<DIEP0_TXFSTS_SPEC>`"]
pub type DIEP0_TXFSTS = crate::Reg<diep0_txfsts::DIEP0_TXFSTS_SPEC>;
#[doc = "Device IN Endpoint x+1 Transmit FIFO Status Register"]
pub mod diep0_txfsts;
#[doc = "DIEP1_CTL (rw) register accessor: an alias for `Reg<DIEP1_CTL_SPEC>`"]
pub type DIEP1_CTL = crate::Reg<diep1_ctl::DIEP1_CTL_SPEC>;
#[doc = "Device IN Endpoint x+1 Control Register"]
pub mod diep1_ctl;
#[doc = "DIEP1_INT (rw) register accessor: an alias for `Reg<DIEP1_INT_SPEC>`"]
pub type DIEP1_INT = crate::Reg<diep1_int::DIEP1_INT_SPEC>;
#[doc = "Device IN Endpoint x+1 Interrupt Register"]
pub mod diep1_int;
#[doc = "DIEP1_TSIZ (rw) register accessor: an alias for `Reg<DIEP1_TSIZ_SPEC>`"]
pub type DIEP1_TSIZ = crate::Reg<diep1_tsiz::DIEP1_TSIZ_SPEC>;
#[doc = "Device IN Endpoint x+1 Transfer Size Register"]
pub mod diep1_tsiz;
#[doc = "DIEP1_DMAADDR (rw) register accessor: an alias for `Reg<DIEP1_DMAADDR_SPEC>`"]
pub type DIEP1_DMAADDR = crate::Reg<diep1_dmaaddr::DIEP1_DMAADDR_SPEC>;
#[doc = "Device IN Endpoint x+1 DMA Address Register"]
pub mod diep1_dmaaddr;
#[doc = "DIEP1_TXFSTS (r) register accessor: an alias for `Reg<DIEP1_TXFSTS_SPEC>`"]
pub type DIEP1_TXFSTS = crate::Reg<diep1_txfsts::DIEP1_TXFSTS_SPEC>;
#[doc = "Device IN Endpoint x+1 Transmit FIFO Status Register"]
pub mod diep1_txfsts;
#[doc = "DIEP2_CTL (rw) register accessor: an alias for `Reg<DIEP2_CTL_SPEC>`"]
pub type DIEP2_CTL = crate::Reg<diep2_ctl::DIEP2_CTL_SPEC>;
#[doc = "Device IN Endpoint x+1 Control Register"]
pub mod diep2_ctl;
#[doc = "DIEP2_INT (rw) register accessor: an alias for `Reg<DIEP2_INT_SPEC>`"]
pub type DIEP2_INT = crate::Reg<diep2_int::DIEP2_INT_SPEC>;
#[doc = "Device IN Endpoint x+1 Interrupt Register"]
pub mod diep2_int;
#[doc = "DIEP2_TSIZ (rw) register accessor: an alias for `Reg<DIEP2_TSIZ_SPEC>`"]
pub type DIEP2_TSIZ = crate::Reg<diep2_tsiz::DIEP2_TSIZ_SPEC>;
#[doc = "Device IN Endpoint x+1 Transfer Size Register"]
pub mod diep2_tsiz;
#[doc = "DIEP2_DMAADDR (rw) register accessor: an alias for `Reg<DIEP2_DMAADDR_SPEC>`"]
pub type DIEP2_DMAADDR = crate::Reg<diep2_dmaaddr::DIEP2_DMAADDR_SPEC>;
#[doc = "Device IN Endpoint x+1 DMA Address Register"]
pub mod diep2_dmaaddr;
#[doc = "DIEP2_TXFSTS (r) register accessor: an alias for `Reg<DIEP2_TXFSTS_SPEC>`"]
pub type DIEP2_TXFSTS = crate::Reg<diep2_txfsts::DIEP2_TXFSTS_SPEC>;
#[doc = "Device IN Endpoint x+1 Transmit FIFO Status Register"]
pub mod diep2_txfsts;
#[doc = "DOEP0CTL (rw) register accessor: an alias for `Reg<DOEP0CTL_SPEC>`"]
pub type DOEP0CTL = crate::Reg<doep0ctl::DOEP0CTL_SPEC>;
#[doc = "Device OUT Endpoint 0 Control Register"]
pub mod doep0ctl;
#[doc = "DOEP0INT (rw) register accessor: an alias for `Reg<DOEP0INT_SPEC>`"]
pub type DOEP0INT = crate::Reg<doep0int::DOEP0INT_SPEC>;
#[doc = "Device OUT Endpoint 0 Interrupt Register"]
pub mod doep0int;
#[doc = "DOEP0TSIZ (rw) register accessor: an alias for `Reg<DOEP0TSIZ_SPEC>`"]
pub type DOEP0TSIZ = crate::Reg<doep0tsiz::DOEP0TSIZ_SPEC>;
#[doc = "Device OUT Endpoint 0 Transfer Size Register"]
pub mod doep0tsiz;
#[doc = "DOEP0DMAADDR (rw) register accessor: an alias for `Reg<DOEP0DMAADDR_SPEC>`"]
pub type DOEP0DMAADDR = crate::Reg<doep0dmaaddr::DOEP0DMAADDR_SPEC>;
#[doc = "Device OUT Endpoint 0 DMA Address Register"]
pub mod doep0dmaaddr;
#[doc = "DOEP0_CTL (rw) register accessor: an alias for `Reg<DOEP0_CTL_SPEC>`"]
pub type DOEP0_CTL = crate::Reg<doep0_ctl::DOEP0_CTL_SPEC>;
#[doc = "Device OUT Endpoint x+1 Control Register"]
pub mod doep0_ctl;
#[doc = "DOEP0_INT (rw) register accessor: an alias for `Reg<DOEP0_INT_SPEC>`"]
pub type DOEP0_INT = crate::Reg<doep0_int::DOEP0_INT_SPEC>;
#[doc = "Device OUT Endpoint x+1 Interrupt Register"]
pub mod doep0_int;
#[doc = "DOEP0_TSIZ (rw) register accessor: an alias for `Reg<DOEP0_TSIZ_SPEC>`"]
pub type DOEP0_TSIZ = crate::Reg<doep0_tsiz::DOEP0_TSIZ_SPEC>;
#[doc = "Device OUT Endpoint x+1 Transfer Size Register"]
pub mod doep0_tsiz;
#[doc = "DOEP0_DMAADDR (rw) register accessor: an alias for `Reg<DOEP0_DMAADDR_SPEC>`"]
pub type DOEP0_DMAADDR = crate::Reg<doep0_dmaaddr::DOEP0_DMAADDR_SPEC>;
#[doc = "Device OUT Endpoint x+1 DMA Address Register"]
pub mod doep0_dmaaddr;
#[doc = "DOEP1_CTL (rw) register accessor: an alias for `Reg<DOEP1_CTL_SPEC>`"]
pub type DOEP1_CTL = crate::Reg<doep1_ctl::DOEP1_CTL_SPEC>;
#[doc = "Device OUT Endpoint x+1 Control Register"]
pub mod doep1_ctl;
#[doc = "DOEP1_INT (rw) register accessor: an alias for `Reg<DOEP1_INT_SPEC>`"]
pub type DOEP1_INT = crate::Reg<doep1_int::DOEP1_INT_SPEC>;
#[doc = "Device OUT Endpoint x+1 Interrupt Register"]
pub mod doep1_int;
#[doc = "DOEP1_TSIZ (rw) register accessor: an alias for `Reg<DOEP1_TSIZ_SPEC>`"]
pub type DOEP1_TSIZ = crate::Reg<doep1_tsiz::DOEP1_TSIZ_SPEC>;
#[doc = "Device OUT Endpoint x+1 Transfer Size Register"]
pub mod doep1_tsiz;
#[doc = "DOEP1_DMAADDR (rw) register accessor: an alias for `Reg<DOEP1_DMAADDR_SPEC>`"]
pub type DOEP1_DMAADDR = crate::Reg<doep1_dmaaddr::DOEP1_DMAADDR_SPEC>;
#[doc = "Device OUT Endpoint x+1 DMA Address Register"]
pub mod doep1_dmaaddr;
#[doc = "DOEP2_CTL (rw) register accessor: an alias for `Reg<DOEP2_CTL_SPEC>`"]
pub type DOEP2_CTL = crate::Reg<doep2_ctl::DOEP2_CTL_SPEC>;
#[doc = "Device OUT Endpoint x+1 Control Register"]
pub mod doep2_ctl;
#[doc = "DOEP2_INT (rw) register accessor: an alias for `Reg<DOEP2_INT_SPEC>`"]
pub type DOEP2_INT = crate::Reg<doep2_int::DOEP2_INT_SPEC>;
#[doc = "Device OUT Endpoint x+1 Interrupt Register"]
pub mod doep2_int;
#[doc = "DOEP2_TSIZ (rw) register accessor: an alias for `Reg<DOEP2_TSIZ_SPEC>`"]
pub type DOEP2_TSIZ = crate::Reg<doep2_tsiz::DOEP2_TSIZ_SPEC>;
#[doc = "Device OUT Endpoint x+1 Transfer Size Register"]
pub mod doep2_tsiz;
#[doc = "DOEP2_DMAADDR (rw) register accessor: an alias for `Reg<DOEP2_DMAADDR_SPEC>`"]
pub type DOEP2_DMAADDR = crate::Reg<doep2_dmaaddr::DOEP2_DMAADDR_SPEC>;
#[doc = "Device OUT Endpoint x+1 DMA Address Register"]
pub mod doep2_dmaaddr;
#[doc = "PCGCCTL (rw) register accessor: an alias for `Reg<PCGCCTL_SPEC>`"]
pub type PCGCCTL = crate::Reg<pcgcctl::PCGCCTL_SPEC>;
#[doc = "Power and Clock Gating Control Register"]
pub mod pcgcctl;

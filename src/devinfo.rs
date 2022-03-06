#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Calibration temperature and checksum"]
    pub cal: crate::Reg<cal::CAL_SPEC>,
    #[doc = "0x04 - ADC0 Calibration register 0"]
    pub adc0cal0: crate::Reg<adc0cal0::ADC0CAL0_SPEC>,
    #[doc = "0x08 - ADC0 Calibration register 1"]
    pub adc0cal1: crate::Reg<adc0cal1::ADC0CAL1_SPEC>,
    #[doc = "0x0c - ADC0 Calibration register 2"]
    pub adc0cal2: crate::Reg<adc0cal2::ADC0CAL2_SPEC>,
    _reserved4: [u8; 0x08],
    #[doc = "0x18 - IDAC0 calibration register"]
    pub idac0cal0: crate::Reg<idac0cal0::IDAC0CAL0_SPEC>,
    #[doc = "0x1c - USHFRCO calibration register"]
    pub ushfrcocal0: crate::Reg<ushfrcocal0::USHFRCOCAL0_SPEC>,
    _reserved6: [u8; 0x04],
    #[doc = "0x24 - AUXHFRCO calibration register 0"]
    pub auxhfrcocal0: crate::Reg<auxhfrcocal0::AUXHFRCOCAL0_SPEC>,
    #[doc = "0x28 - AUXHFRCO calibration register 1"]
    pub auxhfrcocal1: crate::Reg<auxhfrcocal1::AUXHFRCOCAL1_SPEC>,
    _reserved8: [u8; 0x03],
    #[doc = "0x2c - HFRCO calibration register 0"]
    pub hfrcocal0: crate::Reg<hfrcocal0::HFRCOCAL0_SPEC>,
    #[doc = "0x30 - HFRCO calibration register 1"]
    pub hfrcocal1: crate::Reg<hfrcocal1::HFRCOCAL1_SPEC>,
    _reserved10: [u8; 0x0f],
    #[doc = "0x40 - Low 32 bits of device unique number"]
    pub uniquel: crate::Reg<uniquel::UNIQUEL_SPEC>,
    #[doc = "0x44 - High 32 bits of device unique number"]
    pub uniqueh: crate::Reg<uniqueh::UNIQUEH_SPEC>,
    #[doc = "0x48 - Flash and SRAM Memory size in KiloBytes"]
    pub msize: crate::Reg<msize::MSIZE_SPEC>,
    #[doc = "0x4c - Part description"]
    pub part: crate::Reg<part::PART_SPEC>,
}
#[doc = "CAL register accessor: an alias for `Reg<CAL_SPEC>`"]
pub type CAL = crate::Reg<cal::CAL_SPEC>;
#[doc = "Calibration temperature and checksum"]
pub mod cal;
#[doc = "ADC0CAL0 register accessor: an alias for `Reg<ADC0CAL0_SPEC>`"]
pub type ADC0CAL0 = crate::Reg<adc0cal0::ADC0CAL0_SPEC>;
#[doc = "ADC0 Calibration register 0"]
pub mod adc0cal0;
#[doc = "ADC0CAL1 register accessor: an alias for `Reg<ADC0CAL1_SPEC>`"]
pub type ADC0CAL1 = crate::Reg<adc0cal1::ADC0CAL1_SPEC>;
#[doc = "ADC0 Calibration register 1"]
pub mod adc0cal1;
#[doc = "ADC0CAL2 register accessor: an alias for `Reg<ADC0CAL2_SPEC>`"]
pub type ADC0CAL2 = crate::Reg<adc0cal2::ADC0CAL2_SPEC>;
#[doc = "ADC0 Calibration register 2"]
pub mod adc0cal2;
#[doc = "IDAC0CAL0 register accessor: an alias for `Reg<IDAC0CAL0_SPEC>`"]
pub type IDAC0CAL0 = crate::Reg<idac0cal0::IDAC0CAL0_SPEC>;
#[doc = "IDAC0 calibration register"]
pub mod idac0cal0;
#[doc = "USHFRCOCAL0 register accessor: an alias for `Reg<USHFRCOCAL0_SPEC>`"]
pub type USHFRCOCAL0 = crate::Reg<ushfrcocal0::USHFRCOCAL0_SPEC>;
#[doc = "USHFRCO calibration register"]
pub mod ushfrcocal0;
#[doc = "AUXHFRCOCAL0 register accessor: an alias for `Reg<AUXHFRCOCAL0_SPEC>`"]
pub type AUXHFRCOCAL0 = crate::Reg<auxhfrcocal0::AUXHFRCOCAL0_SPEC>;
#[doc = "AUXHFRCO calibration register 0"]
pub mod auxhfrcocal0;
#[doc = "AUXHFRCOCAL1 register accessor: an alias for `Reg<AUXHFRCOCAL1_SPEC>`"]
pub type AUXHFRCOCAL1 = crate::Reg<auxhfrcocal1::AUXHFRCOCAL1_SPEC>;
#[doc = "AUXHFRCO calibration register 1"]
pub mod auxhfrcocal1;
#[doc = "HFRCOCAL0 register accessor: an alias for `Reg<HFRCOCAL0_SPEC>`"]
pub type HFRCOCAL0 = crate::Reg<hfrcocal0::HFRCOCAL0_SPEC>;
#[doc = "HFRCO calibration register 0"]
pub mod hfrcocal0;
#[doc = "HFRCOCAL1 register accessor: an alias for `Reg<HFRCOCAL1_SPEC>`"]
pub type HFRCOCAL1 = crate::Reg<hfrcocal1::HFRCOCAL1_SPEC>;
#[doc = "HFRCO calibration register 1"]
pub mod hfrcocal1;
#[doc = "UNIQUEL register accessor: an alias for `Reg<UNIQUEL_SPEC>`"]
pub type UNIQUEL = crate::Reg<uniquel::UNIQUEL_SPEC>;
#[doc = "Low 32 bits of device unique number"]
pub mod uniquel;
#[doc = "UNIQUEH register accessor: an alias for `Reg<UNIQUEH_SPEC>`"]
pub type UNIQUEH = crate::Reg<uniqueh::UNIQUEH_SPEC>;
#[doc = "High 32 bits of device unique number"]
pub mod uniqueh;
#[doc = "MSIZE register accessor: an alias for `Reg<MSIZE_SPEC>`"]
pub type MSIZE = crate::Reg<msize::MSIZE_SPEC>;
#[doc = "Flash and SRAM Memory size in KiloBytes"]
pub mod msize;
#[doc = "PART register accessor: an alias for `Reg<PART_SPEC>`"]
pub type PART = crate::Reg<part::PART_SPEC>;
#[doc = "Part description"]
pub mod part;

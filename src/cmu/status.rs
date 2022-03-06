#[doc = "Register `STATUS` reader"]
pub struct R(crate::R<STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `HFRCOENS` reader - HFRCO Enable Status"]
pub struct HFRCOENS_R(crate::FieldReader<bool, bool>);
impl HFRCOENS_R {
    pub(crate) fn new(bits: bool) -> Self {
        HFRCOENS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HFRCOENS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HFRCORDY` reader - HFRCO Ready"]
pub struct HFRCORDY_R(crate::FieldReader<bool, bool>);
impl HFRCORDY_R {
    pub(crate) fn new(bits: bool) -> Self {
        HFRCORDY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HFRCORDY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HFXOENS` reader - HFXO Enable Status"]
pub struct HFXOENS_R(crate::FieldReader<bool, bool>);
impl HFXOENS_R {
    pub(crate) fn new(bits: bool) -> Self {
        HFXOENS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HFXOENS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HFXORDY` reader - HFXO Ready"]
pub struct HFXORDY_R(crate::FieldReader<bool, bool>);
impl HFXORDY_R {
    pub(crate) fn new(bits: bool) -> Self {
        HFXORDY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HFXORDY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AUXHFRCOENS` reader - AUXHFRCO Enable Status"]
pub struct AUXHFRCOENS_R(crate::FieldReader<bool, bool>);
impl AUXHFRCOENS_R {
    pub(crate) fn new(bits: bool) -> Self {
        AUXHFRCOENS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AUXHFRCOENS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AUXHFRCORDY` reader - AUXHFRCO Ready"]
pub struct AUXHFRCORDY_R(crate::FieldReader<bool, bool>);
impl AUXHFRCORDY_R {
    pub(crate) fn new(bits: bool) -> Self {
        AUXHFRCORDY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AUXHFRCORDY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LFRCOENS` reader - LFRCO Enable Status"]
pub struct LFRCOENS_R(crate::FieldReader<bool, bool>);
impl LFRCOENS_R {
    pub(crate) fn new(bits: bool) -> Self {
        LFRCOENS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LFRCOENS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LFRCORDY` reader - LFRCO Ready"]
pub struct LFRCORDY_R(crate::FieldReader<bool, bool>);
impl LFRCORDY_R {
    pub(crate) fn new(bits: bool) -> Self {
        LFRCORDY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LFRCORDY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LFXOENS` reader - LFXO Enable Status"]
pub struct LFXOENS_R(crate::FieldReader<bool, bool>);
impl LFXOENS_R {
    pub(crate) fn new(bits: bool) -> Self {
        LFXOENS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LFXOENS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LFXORDY` reader - LFXO Ready"]
pub struct LFXORDY_R(crate::FieldReader<bool, bool>);
impl LFXORDY_R {
    pub(crate) fn new(bits: bool) -> Self {
        LFXORDY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LFXORDY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HFRCOSEL` reader - HFRCO Selected"]
pub struct HFRCOSEL_R(crate::FieldReader<bool, bool>);
impl HFRCOSEL_R {
    pub(crate) fn new(bits: bool) -> Self {
        HFRCOSEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HFRCOSEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HFXOSEL` reader - HFXO Selected"]
pub struct HFXOSEL_R(crate::FieldReader<bool, bool>);
impl HFXOSEL_R {
    pub(crate) fn new(bits: bool) -> Self {
        HFXOSEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HFXOSEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LFRCOSEL` reader - LFRCO Selected"]
pub struct LFRCOSEL_R(crate::FieldReader<bool, bool>);
impl LFRCOSEL_R {
    pub(crate) fn new(bits: bool) -> Self {
        LFRCOSEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LFRCOSEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LFXOSEL` reader - LFXO Selected"]
pub struct LFXOSEL_R(crate::FieldReader<bool, bool>);
impl LFXOSEL_R {
    pub(crate) fn new(bits: bool) -> Self {
        LFXOSEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LFXOSEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CALBSY` reader - Calibration Busy"]
pub struct CALBSY_R(crate::FieldReader<bool, bool>);
impl CALBSY_R {
    pub(crate) fn new(bits: bool) -> Self {
        CALBSY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CALBSY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USBCLFXOSEL` reader - USBC LFXO Selected"]
pub struct USBCLFXOSEL_R(crate::FieldReader<bool, bool>);
impl USBCLFXOSEL_R {
    pub(crate) fn new(bits: bool) -> Self {
        USBCLFXOSEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USBCLFXOSEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USBCLFRCOSEL` reader - USBC LFRCO Selected"]
pub struct USBCLFRCOSEL_R(crate::FieldReader<bool, bool>);
impl USBCLFRCOSEL_R {
    pub(crate) fn new(bits: bool) -> Self {
        USBCLFRCOSEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USBCLFRCOSEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USBCUSHFRCOSEL` reader - USBC USHFRCO Selected"]
pub struct USBCUSHFRCOSEL_R(crate::FieldReader<bool, bool>);
impl USBCUSHFRCOSEL_R {
    pub(crate) fn new(bits: bool) -> Self {
        USBCUSHFRCOSEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USBCUSHFRCOSEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USBCHFCLKSYNC` reader - USBC is synchronous to HFCLK"]
pub struct USBCHFCLKSYNC_R(crate::FieldReader<bool, bool>);
impl USBCHFCLKSYNC_R {
    pub(crate) fn new(bits: bool) -> Self {
        USBCHFCLKSYNC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USBCHFCLKSYNC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USHFRCOENS` reader - USHFRCO Enable Status"]
pub struct USHFRCOENS_R(crate::FieldReader<bool, bool>);
impl USHFRCOENS_R {
    pub(crate) fn new(bits: bool) -> Self {
        USHFRCOENS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USHFRCOENS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USHFRCORDY` reader - USHFRCO Ready"]
pub struct USHFRCORDY_R(crate::FieldReader<bool, bool>);
impl USHFRCORDY_R {
    pub(crate) fn new(bits: bool) -> Self {
        USHFRCORDY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USHFRCORDY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USHFRCOSUSPEND` reader - USHFRCO is suspended"]
pub struct USHFRCOSUSPEND_R(crate::FieldReader<bool, bool>);
impl USHFRCOSUSPEND_R {
    pub(crate) fn new(bits: bool) -> Self {
        USHFRCOSUSPEND_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USHFRCOSUSPEND_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USHFRCODIV2SEL` reader - USHFRCODIV2 Selected"]
pub struct USHFRCODIV2SEL_R(crate::FieldReader<bool, bool>);
impl USHFRCODIV2SEL_R {
    pub(crate) fn new(bits: bool) -> Self {
        USHFRCODIV2SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USHFRCODIV2SEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - HFRCO Enable Status"]
    #[inline(always)]
    pub fn hfrcoens(&self) -> HFRCOENS_R {
        HFRCOENS_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - HFRCO Ready"]
    #[inline(always)]
    pub fn hfrcordy(&self) -> HFRCORDY_R {
        HFRCORDY_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - HFXO Enable Status"]
    #[inline(always)]
    pub fn hfxoens(&self) -> HFXOENS_R {
        HFXOENS_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - HFXO Ready"]
    #[inline(always)]
    pub fn hfxordy(&self) -> HFXORDY_R {
        HFXORDY_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - AUXHFRCO Enable Status"]
    #[inline(always)]
    pub fn auxhfrcoens(&self) -> AUXHFRCOENS_R {
        AUXHFRCOENS_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - AUXHFRCO Ready"]
    #[inline(always)]
    pub fn auxhfrcordy(&self) -> AUXHFRCORDY_R {
        AUXHFRCORDY_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - LFRCO Enable Status"]
    #[inline(always)]
    pub fn lfrcoens(&self) -> LFRCOENS_R {
        LFRCOENS_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - LFRCO Ready"]
    #[inline(always)]
    pub fn lfrcordy(&self) -> LFRCORDY_R {
        LFRCORDY_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - LFXO Enable Status"]
    #[inline(always)]
    pub fn lfxoens(&self) -> LFXOENS_R {
        LFXOENS_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - LFXO Ready"]
    #[inline(always)]
    pub fn lfxordy(&self) -> LFXORDY_R {
        LFXORDY_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - HFRCO Selected"]
    #[inline(always)]
    pub fn hfrcosel(&self) -> HFRCOSEL_R {
        HFRCOSEL_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - HFXO Selected"]
    #[inline(always)]
    pub fn hfxosel(&self) -> HFXOSEL_R {
        HFXOSEL_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - LFRCO Selected"]
    #[inline(always)]
    pub fn lfrcosel(&self) -> LFRCOSEL_R {
        LFRCOSEL_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - LFXO Selected"]
    #[inline(always)]
    pub fn lfxosel(&self) -> LFXOSEL_R {
        LFXOSEL_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Calibration Busy"]
    #[inline(always)]
    pub fn calbsy(&self) -> CALBSY_R {
        CALBSY_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 16 - USBC LFXO Selected"]
    #[inline(always)]
    pub fn usbclfxosel(&self) -> USBCLFXOSEL_R {
        USBCLFXOSEL_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - USBC LFRCO Selected"]
    #[inline(always)]
    pub fn usbclfrcosel(&self) -> USBCLFRCOSEL_R {
        USBCLFRCOSEL_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - USBC USHFRCO Selected"]
    #[inline(always)]
    pub fn usbcushfrcosel(&self) -> USBCUSHFRCOSEL_R {
        USBCUSHFRCOSEL_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 20 - USBC is synchronous to HFCLK"]
    #[inline(always)]
    pub fn usbchfclksync(&self) -> USBCHFCLKSYNC_R {
        USBCHFCLKSYNC_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - USHFRCO Enable Status"]
    #[inline(always)]
    pub fn ushfrcoens(&self) -> USHFRCOENS_R {
        USHFRCOENS_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - USHFRCO Ready"]
    #[inline(always)]
    pub fn ushfrcordy(&self) -> USHFRCORDY_R {
        USHFRCORDY_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - USHFRCO is suspended"]
    #[inline(always)]
    pub fn ushfrcosuspend(&self) -> USHFRCOSUSPEND_R {
        USHFRCOSUSPEND_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 26 - USHFRCODIV2 Selected"]
    #[inline(always)]
    pub fn ushfrcodiv2sel(&self) -> USHFRCODIV2SEL_R {
        USHFRCODIV2SEL_R::new(((self.bits >> 26) & 0x01) != 0)
    }
}
#[doc = "Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [status](index.html) module"]
pub struct STATUS_SPEC;
impl crate::RegisterSpec for STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [status::R](R) reader structure"]
impl crate::Readable for STATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets STATUS to value 0x0403"]
impl crate::Resettable for STATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0403
    }
}

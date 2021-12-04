#[doc = "Register `IF` reader"]
pub struct R(crate::R<IF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `HFRCORDY` reader - HFRCO Ready Interrupt Flag"]
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
#[doc = "Field `HFXORDY` reader - HFXO Ready Interrupt Flag"]
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
#[doc = "Field `LFRCORDY` reader - LFRCO Ready Interrupt Flag"]
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
#[doc = "Field `LFXORDY` reader - LFXO Ready Interrupt Flag"]
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
#[doc = "Field `AUXHFRCORDY` reader - AUXHFRCO Ready Interrupt Flag"]
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
#[doc = "Field `CALRDY` reader - Calibration Ready Interrupt Flag"]
pub struct CALRDY_R(crate::FieldReader<bool, bool>);
impl CALRDY_R {
    pub(crate) fn new(bits: bool) -> Self {
        CALRDY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CALRDY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CALOF` reader - Calibration Overflow Interrupt Flag"]
pub struct CALOF_R(crate::FieldReader<bool, bool>);
impl CALOF_R {
    pub(crate) fn new(bits: bool) -> Self {
        CALOF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CALOF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USHFRCORDY` reader - USHFRCO Ready Interrupt Flag"]
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
#[doc = "Field `USBCHFOSCSEL` reader - USBC HF-oscillator Selected Interrupt Flag"]
pub struct USBCHFOSCSEL_R(crate::FieldReader<bool, bool>);
impl USBCHFOSCSEL_R {
    pub(crate) fn new(bits: bool) -> Self {
        USBCHFOSCSEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USBCHFOSCSEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - HFRCO Ready Interrupt Flag"]
    #[inline(always)]
    pub fn hfrcordy(&self) -> HFRCORDY_R {
        HFRCORDY_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - HFXO Ready Interrupt Flag"]
    #[inline(always)]
    pub fn hfxordy(&self) -> HFXORDY_R {
        HFXORDY_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - LFRCO Ready Interrupt Flag"]
    #[inline(always)]
    pub fn lfrcordy(&self) -> LFRCORDY_R {
        LFRCORDY_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - LFXO Ready Interrupt Flag"]
    #[inline(always)]
    pub fn lfxordy(&self) -> LFXORDY_R {
        LFXORDY_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - AUXHFRCO Ready Interrupt Flag"]
    #[inline(always)]
    pub fn auxhfrcordy(&self) -> AUXHFRCORDY_R {
        AUXHFRCORDY_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Calibration Ready Interrupt Flag"]
    #[inline(always)]
    pub fn calrdy(&self) -> CALRDY_R {
        CALRDY_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Calibration Overflow Interrupt Flag"]
    #[inline(always)]
    pub fn calof(&self) -> CALOF_R {
        CALOF_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 8 - USHFRCO Ready Interrupt Flag"]
    #[inline(always)]
    pub fn ushfrcordy(&self) -> USHFRCORDY_R {
        USHFRCORDY_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - USBC HF-oscillator Selected Interrupt Flag"]
    #[inline(always)]
    pub fn usbchfoscsel(&self) -> USBCHFOSCSEL_R {
        USBCHFOSCSEL_R::new(((self.bits >> 9) & 0x01) != 0)
    }
}
#[doc = "Interrupt Flag Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [if_](index.html) module"]
pub struct IF_SPEC;
impl crate::RegisterSpec for IF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [if_::R](R) reader structure"]
impl crate::Readable for IF_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets IF to value 0x01"]
impl crate::Resettable for IF_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x01
    }
}

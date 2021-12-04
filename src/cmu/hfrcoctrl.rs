#[doc = "Register `HFRCOCTRL` reader"]
pub struct R(crate::R<HFRCOCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HFRCOCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HFRCOCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HFRCOCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HFRCOCTRL` writer"]
pub struct W(crate::W<HFRCOCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HFRCOCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<HFRCOCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HFRCOCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TUNING` reader - HFRCO Tuning Value"]
pub struct TUNING_R(crate::FieldReader<u8, u8>);
impl TUNING_R {
    pub(crate) fn new(bits: u8) -> Self {
        TUNING_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TUNING_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TUNING` writer - HFRCO Tuning Value"]
pub struct TUNING_W<'a> {
    w: &'a mut W,
}
impl<'a> TUNING_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
#[doc = "HFRCO Band Select\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum BAND_A {
    #[doc = "0: 1 MHz band. NOTE: Also set the TUNING value (bits 7:0) when changing band."]
    _1MHZ = 0,
    #[doc = "1: 7 MHz band. NOTE: Also set the TUNING value (bits 7:0) when changing band."]
    _7MHZ = 1,
    #[doc = "2: 11 MHz band. NOTE: Also set the TUNING value (bits 7:0) when changing band."]
    _11MHZ = 2,
    #[doc = "3: 14 MHz band. NOTE: Also set the TUNING value (bits 7:0) when changing band."]
    _14MHZ = 3,
    #[doc = "4: 21 MHz band. NOTE: Also set the TUNING value (bits 7:0) when changing band."]
    _21MHZ = 4,
}
impl From<BAND_A> for u8 {
    #[inline(always)]
    fn from(variant: BAND_A) -> Self {
        variant as _
    }
}
#[doc = "Field `BAND` reader - HFRCO Band Select"]
pub struct BAND_R(crate::FieldReader<u8, BAND_A>);
impl BAND_R {
    pub(crate) fn new(bits: u8) -> Self {
        BAND_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<BAND_A> {
        match self.bits {
            0 => Some(BAND_A::_1MHZ),
            1 => Some(BAND_A::_7MHZ),
            2 => Some(BAND_A::_11MHZ),
            3 => Some(BAND_A::_14MHZ),
            4 => Some(BAND_A::_21MHZ),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_1MHZ`"]
    #[inline(always)]
    pub fn is_1mhz(&self) -> bool {
        **self == BAND_A::_1MHZ
    }
    #[doc = "Checks if the value of the field is `_7MHZ`"]
    #[inline(always)]
    pub fn is_7mhz(&self) -> bool {
        **self == BAND_A::_7MHZ
    }
    #[doc = "Checks if the value of the field is `_11MHZ`"]
    #[inline(always)]
    pub fn is_11mhz(&self) -> bool {
        **self == BAND_A::_11MHZ
    }
    #[doc = "Checks if the value of the field is `_14MHZ`"]
    #[inline(always)]
    pub fn is_14mhz(&self) -> bool {
        **self == BAND_A::_14MHZ
    }
    #[doc = "Checks if the value of the field is `_21MHZ`"]
    #[inline(always)]
    pub fn is_21mhz(&self) -> bool {
        **self == BAND_A::_21MHZ
    }
}
impl core::ops::Deref for BAND_R {
    type Target = crate::FieldReader<u8, BAND_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BAND` writer - HFRCO Band Select"]
pub struct BAND_W<'a> {
    w: &'a mut W,
}
impl<'a> BAND_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BAND_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "1 MHz band. NOTE: Also set the TUNING value (bits 7:0) when changing band."]
    #[inline(always)]
    pub fn _1mhz(self) -> &'a mut W {
        self.variant(BAND_A::_1MHZ)
    }
    #[doc = "7 MHz band. NOTE: Also set the TUNING value (bits 7:0) when changing band."]
    #[inline(always)]
    pub fn _7mhz(self) -> &'a mut W {
        self.variant(BAND_A::_7MHZ)
    }
    #[doc = "11 MHz band. NOTE: Also set the TUNING value (bits 7:0) when changing band."]
    #[inline(always)]
    pub fn _11mhz(self) -> &'a mut W {
        self.variant(BAND_A::_11MHZ)
    }
    #[doc = "14 MHz band. NOTE: Also set the TUNING value (bits 7:0) when changing band."]
    #[inline(always)]
    pub fn _14mhz(self) -> &'a mut W {
        self.variant(BAND_A::_14MHZ)
    }
    #[doc = "21 MHz band. NOTE: Also set the TUNING value (bits 7:0) when changing band."]
    #[inline(always)]
    pub fn _21mhz(self) -> &'a mut W {
        self.variant(BAND_A::_21MHZ)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | ((value as u32 & 0x07) << 8);
        self.w
    }
}
#[doc = "Field `SUDELAY` reader - HFRCO Start-up Delay"]
pub struct SUDELAY_R(crate::FieldReader<u8, u8>);
impl SUDELAY_R {
    pub(crate) fn new(bits: u8) -> Self {
        SUDELAY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SUDELAY_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SUDELAY` writer - HFRCO Start-up Delay"]
pub struct SUDELAY_W<'a> {
    w: &'a mut W,
}
impl<'a> SUDELAY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 12)) | ((value as u32 & 0x1f) << 12);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - HFRCO Tuning Value"]
    #[inline(always)]
    pub fn tuning(&self) -> TUNING_R {
        TUNING_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:10 - HFRCO Band Select"]
    #[inline(always)]
    pub fn band(&self) -> BAND_R {
        BAND_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    #[doc = "Bits 12:16 - HFRCO Start-up Delay"]
    #[inline(always)]
    pub fn sudelay(&self) -> SUDELAY_R {
        SUDELAY_R::new(((self.bits >> 12) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - HFRCO Tuning Value"]
    #[inline(always)]
    pub fn tuning(&mut self) -> TUNING_W {
        TUNING_W { w: self }
    }
    #[doc = "Bits 8:10 - HFRCO Band Select"]
    #[inline(always)]
    pub fn band(&mut self) -> BAND_W {
        BAND_W { w: self }
    }
    #[doc = "Bits 12:16 - HFRCO Start-up Delay"]
    #[inline(always)]
    pub fn sudelay(&mut self) -> SUDELAY_W {
        SUDELAY_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "HFRCO Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hfrcoctrl](index.html) module"]
pub struct HFRCOCTRL_SPEC;
impl crate::RegisterSpec for HFRCOCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hfrcoctrl::R](R) reader structure"]
impl crate::Readable for HFRCOCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hfrcoctrl::W](W) writer structure"]
impl crate::Writable for HFRCOCTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets HFRCOCTRL to value 0x0380"]
impl crate::Resettable for HFRCOCTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0380
    }
}

#[doc = "Register `USHFRCOCONF` reader"]
pub struct R(crate::R<USHFRCOCONF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USHFRCOCONF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USHFRCOCONF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USHFRCOCONF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `USHFRCOCONF` writer"]
pub struct W(crate::W<USHFRCOCONF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<USHFRCOCONF_SPEC>;
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
impl From<crate::W<USHFRCOCONF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<USHFRCOCONF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "USHFRCO Band Select\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum BAND_A {
    #[doc = "1: 48 MHz band. NOTE: Also set the TUNING and FINETUNING value when changing band."]
    _48MHZ = 1,
    #[doc = "3: 24 MHz band. NOTE: Also set the TUNING and FINETUNING value when changing band."]
    _24MHZ = 3,
}
impl From<BAND_A> for u8 {
    #[inline(always)]
    fn from(variant: BAND_A) -> Self {
        variant as _
    }
}
#[doc = "Field `BAND` reader - USHFRCO Band Select"]
pub struct BAND_R(crate::FieldReader<u8, BAND_A>);
impl BAND_R {
    pub(crate) fn new(bits: u8) -> Self {
        BAND_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<BAND_A> {
        match self.bits {
            1 => Some(BAND_A::_48MHZ),
            3 => Some(BAND_A::_24MHZ),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_48MHZ`"]
    #[inline(always)]
    pub fn is_48mhz(&self) -> bool {
        **self == BAND_A::_48MHZ
    }
    #[doc = "Checks if the value of the field is `_24MHZ`"]
    #[inline(always)]
    pub fn is_24mhz(&self) -> bool {
        **self == BAND_A::_24MHZ
    }
}
impl core::ops::Deref for BAND_R {
    type Target = crate::FieldReader<u8, BAND_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BAND` writer - USHFRCO Band Select"]
pub struct BAND_W<'a> {
    w: &'a mut W,
}
impl<'a> BAND_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BAND_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "48 MHz band. NOTE: Also set the TUNING and FINETUNING value when changing band."]
    #[inline(always)]
    pub fn _48mhz(self) -> &'a mut W {
        self.variant(BAND_A::_48MHZ)
    }
    #[doc = "24 MHz band. NOTE: Also set the TUNING and FINETUNING value when changing band."]
    #[inline(always)]
    pub fn _24mhz(self) -> &'a mut W {
        self.variant(BAND_A::_24MHZ)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | (value as u32 & 0x07);
        self.w
    }
}
#[doc = "Field `USHFRCODIV2DIS` reader - USHFRCO divider for HFCLK disable"]
pub struct USHFRCODIV2DIS_R(crate::FieldReader<bool, bool>);
impl USHFRCODIV2DIS_R {
    pub(crate) fn new(bits: bool) -> Self {
        USHFRCODIV2DIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USHFRCODIV2DIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USHFRCODIV2DIS` writer - USHFRCO divider for HFCLK disable"]
pub struct USHFRCODIV2DIS_W<'a> {
    w: &'a mut W,
}
impl<'a> USHFRCODIV2DIS_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - USHFRCO Band Select"]
    #[inline(always)]
    pub fn band(&self) -> BAND_R {
        BAND_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bit 4 - USHFRCO divider for HFCLK disable"]
    #[inline(always)]
    pub fn ushfrcodiv2dis(&self) -> USHFRCODIV2DIS_R {
        USHFRCODIV2DIS_R::new(((self.bits >> 4) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - USHFRCO Band Select"]
    #[inline(always)]
    pub fn band(&mut self) -> BAND_W {
        BAND_W { w: self }
    }
    #[doc = "Bit 4 - USHFRCO divider for HFCLK disable"]
    #[inline(always)]
    pub fn ushfrcodiv2dis(&mut self) -> USHFRCODIV2DIS_W {
        USHFRCODIV2DIS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USHFRCO Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ushfrcoconf](index.html) module"]
pub struct USHFRCOCONF_SPEC;
impl crate::RegisterSpec for USHFRCOCONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ushfrcoconf::R](R) reader structure"]
impl crate::Readable for USHFRCOCONF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ushfrcoconf::W](W) writer structure"]
impl crate::Writable for USHFRCOCONF_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets USHFRCOCONF to value 0x01"]
impl crate::Resettable for USHFRCOCONF_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x01
    }
}

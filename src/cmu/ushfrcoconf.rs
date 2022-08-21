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
#[doc = "Field `BAND` reader - USHFRCO Band Select"]
pub type BAND_R = crate::FieldReader<u8, BAND_A>;
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
impl BAND_R {
    #[doc = "Get enumerated values variant"]
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
        *self == BAND_A::_48MHZ
    }
    #[doc = "Checks if the value of the field is `_24MHZ`"]
    #[inline(always)]
    pub fn is_24mhz(&self) -> bool {
        *self == BAND_A::_24MHZ
    }
}
#[doc = "Field `BAND` writer - USHFRCO Band Select"]
pub type BAND_W<'a, const O: u8> = crate::FieldWriter<'a, u32, USHFRCOCONF_SPEC, u8, BAND_A, 3, O>;
impl<'a, const O: u8> BAND_W<'a, O> {
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
}
#[doc = "Field `USHFRCODIV2DIS` reader - USHFRCO divider for HFCLK disable"]
pub type USHFRCODIV2DIS_R = crate::BitReader<bool>;
#[doc = "Field `USHFRCODIV2DIS` writer - USHFRCO divider for HFCLK disable"]
pub type USHFRCODIV2DIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, USHFRCOCONF_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:2 - USHFRCO Band Select"]
    #[inline(always)]
    pub fn band(&self) -> BAND_R {
        BAND_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 4 - USHFRCO divider for HFCLK disable"]
    #[inline(always)]
    pub fn ushfrcodiv2dis(&self) -> USHFRCODIV2DIS_R {
        USHFRCODIV2DIS_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - USHFRCO Band Select"]
    #[inline(always)]
    pub fn band(&mut self) -> BAND_W<0> {
        BAND_W::new(self)
    }
    #[doc = "Bit 4 - USHFRCO divider for HFCLK disable"]
    #[inline(always)]
    pub fn ushfrcodiv2dis(&mut self) -> USHFRCODIV2DIS_W<4> {
        USHFRCODIV2DIS_W::new(self)
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

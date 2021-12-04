#[doc = "Register `DCFG` reader"]
pub struct R(crate::R<DCFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DCFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DCFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DCFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DCFG` writer"]
pub struct W(crate::W<DCFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DCFG_SPEC>;
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
impl From<crate::W<DCFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DCFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Device Speed\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DEVSPD_A {
    #[doc = "2: Low speed (PHY clock is 6 MHz). If you select 6 MHz LS mode, you must do a soft reset."]
    LS = 2,
    #[doc = "3: Full speed (PHY clock is 48 MHz)."]
    FS = 3,
}
impl From<DEVSPD_A> for u8 {
    #[inline(always)]
    fn from(variant: DEVSPD_A) -> Self {
        variant as _
    }
}
#[doc = "Field `DEVSPD` reader - Device Speed"]
pub struct DEVSPD_R(crate::FieldReader<u8, DEVSPD_A>);
impl DEVSPD_R {
    pub(crate) fn new(bits: u8) -> Self {
        DEVSPD_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<DEVSPD_A> {
        match self.bits {
            2 => Some(DEVSPD_A::LS),
            3 => Some(DEVSPD_A::FS),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `LS`"]
    #[inline(always)]
    pub fn is_ls(&self) -> bool {
        **self == DEVSPD_A::LS
    }
    #[doc = "Checks if the value of the field is `FS`"]
    #[inline(always)]
    pub fn is_fs(&self) -> bool {
        **self == DEVSPD_A::FS
    }
}
impl core::ops::Deref for DEVSPD_R {
    type Target = crate::FieldReader<u8, DEVSPD_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DEVSPD` writer - Device Speed"]
pub struct DEVSPD_W<'a> {
    w: &'a mut W,
}
impl<'a> DEVSPD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DEVSPD_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Low speed (PHY clock is 6 MHz). If you select 6 MHz LS mode, you must do a soft reset."]
    #[inline(always)]
    pub fn ls(self) -> &'a mut W {
        self.variant(DEVSPD_A::LS)
    }
    #[doc = "Full speed (PHY clock is 48 MHz)."]
    #[inline(always)]
    pub fn fs(self) -> &'a mut W {
        self.variant(DEVSPD_A::FS)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
#[doc = "Field `NZSTSOUTHSHK` reader - Non-Zero-Length Status OUT Handshake"]
pub struct NZSTSOUTHSHK_R(crate::FieldReader<bool, bool>);
impl NZSTSOUTHSHK_R {
    pub(crate) fn new(bits: bool) -> Self {
        NZSTSOUTHSHK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NZSTSOUTHSHK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NZSTSOUTHSHK` writer - Non-Zero-Length Status OUT Handshake"]
pub struct NZSTSOUTHSHK_W<'a> {
    w: &'a mut W,
}
impl<'a> NZSTSOUTHSHK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Field `ENA32KHZSUSP` reader - Enable 32 KHz Suspend mode"]
pub struct ENA32KHZSUSP_R(crate::FieldReader<bool, bool>);
impl ENA32KHZSUSP_R {
    pub(crate) fn new(bits: bool) -> Self {
        ENA32KHZSUSP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ENA32KHZSUSP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ENA32KHZSUSP` writer - Enable 32 KHz Suspend mode"]
pub struct ENA32KHZSUSP_W<'a> {
    w: &'a mut W,
}
impl<'a> ENA32KHZSUSP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "Field `DEVADDR` reader - Device Address"]
pub struct DEVADDR_R(crate::FieldReader<u8, u8>);
impl DEVADDR_R {
    pub(crate) fn new(bits: u8) -> Self {
        DEVADDR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DEVADDR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DEVADDR` writer - Device Address"]
pub struct DEVADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> DEVADDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 4)) | ((value as u32 & 0x7f) << 4);
        self.w
    }
}
#[doc = "Periodic Frame Interval\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PERFRINT_A {
    #[doc = "0: 80% of the frame interval."]
    _80PCNT = 0,
    #[doc = "1: 85% of the frame interval."]
    _85PCNT = 1,
    #[doc = "2: 90% of the frame interval."]
    _90PCNT = 2,
    #[doc = "3: 95% of the frame interval."]
    _95PCNT = 3,
}
impl From<PERFRINT_A> for u8 {
    #[inline(always)]
    fn from(variant: PERFRINT_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PERFRINT` reader - Periodic Frame Interval"]
pub struct PERFRINT_R(crate::FieldReader<u8, PERFRINT_A>);
impl PERFRINT_R {
    pub(crate) fn new(bits: u8) -> Self {
        PERFRINT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PERFRINT_A {
        match self.bits {
            0 => PERFRINT_A::_80PCNT,
            1 => PERFRINT_A::_85PCNT,
            2 => PERFRINT_A::_90PCNT,
            3 => PERFRINT_A::_95PCNT,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_80PCNT`"]
    #[inline(always)]
    pub fn is_80pcnt(&self) -> bool {
        **self == PERFRINT_A::_80PCNT
    }
    #[doc = "Checks if the value of the field is `_85PCNT`"]
    #[inline(always)]
    pub fn is_85pcnt(&self) -> bool {
        **self == PERFRINT_A::_85PCNT
    }
    #[doc = "Checks if the value of the field is `_90PCNT`"]
    #[inline(always)]
    pub fn is_90pcnt(&self) -> bool {
        **self == PERFRINT_A::_90PCNT
    }
    #[doc = "Checks if the value of the field is `_95PCNT`"]
    #[inline(always)]
    pub fn is_95pcnt(&self) -> bool {
        **self == PERFRINT_A::_95PCNT
    }
}
impl core::ops::Deref for PERFRINT_R {
    type Target = crate::FieldReader<u8, PERFRINT_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PERFRINT` writer - Periodic Frame Interval"]
pub struct PERFRINT_W<'a> {
    w: &'a mut W,
}
impl<'a> PERFRINT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PERFRINT_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "80% of the frame interval."]
    #[inline(always)]
    pub fn _80pcnt(self) -> &'a mut W {
        self.variant(PERFRINT_A::_80PCNT)
    }
    #[doc = "85% of the frame interval."]
    #[inline(always)]
    pub fn _85pcnt(self) -> &'a mut W {
        self.variant(PERFRINT_A::_85PCNT)
    }
    #[doc = "90% of the frame interval."]
    #[inline(always)]
    pub fn _90pcnt(self) -> &'a mut W {
        self.variant(PERFRINT_A::_90PCNT)
    }
    #[doc = "95% of the frame interval."]
    #[inline(always)]
    pub fn _95pcnt(self) -> &'a mut W {
        self.variant(PERFRINT_A::_95PCNT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 11)) | ((value as u32 & 0x03) << 11);
        self.w
    }
}
#[doc = "Field `ERRATICINTMSK` reader - \"\""]
pub struct ERRATICINTMSK_R(crate::FieldReader<bool, bool>);
impl ERRATICINTMSK_R {
    pub(crate) fn new(bits: bool) -> Self {
        ERRATICINTMSK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ERRATICINTMSK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ERRATICINTMSK` writer - \"\""]
pub struct ERRATICINTMSK_W<'a> {
    w: &'a mut W,
}
impl<'a> ERRATICINTMSK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | ((value as u32 & 0x01) << 15);
        self.w
    }
}
#[doc = "Field `RESVALID` reader - Resume Validation Period"]
pub struct RESVALID_R(crate::FieldReader<u8, u8>);
impl RESVALID_R {
    pub(crate) fn new(bits: u8) -> Self {
        RESVALID_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RESVALID_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RESVALID` writer - Resume Validation Period"]
pub struct RESVALID_W<'a> {
    w: &'a mut W,
}
impl<'a> RESVALID_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 26)) | ((value as u32 & 0x3f) << 26);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Device Speed"]
    #[inline(always)]
    pub fn devspd(&self) -> DEVSPD_R {
        DEVSPD_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bit 2 - Non-Zero-Length Status OUT Handshake"]
    #[inline(always)]
    pub fn nzstsouthshk(&self) -> NZSTSOUTHSHK_R {
        NZSTSOUTHSHK_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Enable 32 KHz Suspend mode"]
    #[inline(always)]
    pub fn ena32khzsusp(&self) -> ENA32KHZSUSP_R {
        ENA32KHZSUSP_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 4:10 - Device Address"]
    #[inline(always)]
    pub fn devaddr(&self) -> DEVADDR_R {
        DEVADDR_R::new(((self.bits >> 4) & 0x7f) as u8)
    }
    #[doc = "Bits 11:12 - Periodic Frame Interval"]
    #[inline(always)]
    pub fn perfrint(&self) -> PERFRINT_R {
        PERFRINT_R::new(((self.bits >> 11) & 0x03) as u8)
    }
    #[doc = "Bit 15 - \"\""]
    #[inline(always)]
    pub fn erraticintmsk(&self) -> ERRATICINTMSK_R {
        ERRATICINTMSK_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 26:31 - Resume Validation Period"]
    #[inline(always)]
    pub fn resvalid(&self) -> RESVALID_R {
        RESVALID_R::new(((self.bits >> 26) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Device Speed"]
    #[inline(always)]
    pub fn devspd(&mut self) -> DEVSPD_W {
        DEVSPD_W { w: self }
    }
    #[doc = "Bit 2 - Non-Zero-Length Status OUT Handshake"]
    #[inline(always)]
    pub fn nzstsouthshk(&mut self) -> NZSTSOUTHSHK_W {
        NZSTSOUTHSHK_W { w: self }
    }
    #[doc = "Bit 3 - Enable 32 KHz Suspend mode"]
    #[inline(always)]
    pub fn ena32khzsusp(&mut self) -> ENA32KHZSUSP_W {
        ENA32KHZSUSP_W { w: self }
    }
    #[doc = "Bits 4:10 - Device Address"]
    #[inline(always)]
    pub fn devaddr(&mut self) -> DEVADDR_W {
        DEVADDR_W { w: self }
    }
    #[doc = "Bits 11:12 - Periodic Frame Interval"]
    #[inline(always)]
    pub fn perfrint(&mut self) -> PERFRINT_W {
        PERFRINT_W { w: self }
    }
    #[doc = "Bit 15 - \"\""]
    #[inline(always)]
    pub fn erraticintmsk(&mut self) -> ERRATICINTMSK_W {
        ERRATICINTMSK_W { w: self }
    }
    #[doc = "Bits 26:31 - Resume Validation Period"]
    #[inline(always)]
    pub fn resvalid(&mut self) -> RESVALID_W {
        RESVALID_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Device Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dcfg](index.html) module"]
pub struct DCFG_SPEC;
impl crate::RegisterSpec for DCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dcfg::R](R) reader structure"]
impl crate::Readable for DCFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dcfg::W](W) writer structure"]
impl crate::Writable for DCFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DCFG to value 0x0800_0000"]
impl crate::Resettable for DCFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0800_0000
    }
}

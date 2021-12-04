#[doc = "Register `DTOGEN` reader"]
pub struct R(crate::R<DTOGEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DTOGEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DTOGEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DTOGEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DTOGEN` writer"]
pub struct W(crate::W<DTOGEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DTOGEN_SPEC>;
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
impl From<crate::W<DTOGEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DTOGEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DTOGCC0EN` reader - DTI CC0 Output Generation Enable"]
pub struct DTOGCC0EN_R(crate::FieldReader<bool, bool>);
impl DTOGCC0EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        DTOGCC0EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DTOGCC0EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DTOGCC0EN` writer - DTI CC0 Output Generation Enable"]
pub struct DTOGCC0EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DTOGCC0EN_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
#[doc = "Field `DTOGCC1EN` reader - DTI CC1 Output Generation Enable"]
pub struct DTOGCC1EN_R(crate::FieldReader<bool, bool>);
impl DTOGCC1EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        DTOGCC1EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DTOGCC1EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DTOGCC1EN` writer - DTI CC1 Output Generation Enable"]
pub struct DTOGCC1EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DTOGCC1EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Field `DTOGCC2EN` reader - DTI CC2 Output Generation Enable"]
pub struct DTOGCC2EN_R(crate::FieldReader<bool, bool>);
impl DTOGCC2EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        DTOGCC2EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DTOGCC2EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DTOGCC2EN` writer - DTI CC2 Output Generation Enable"]
pub struct DTOGCC2EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DTOGCC2EN_W<'a> {
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
#[doc = "Field `DTOGCDTI0EN` reader - DTI CDTI0 Output Generation Enable"]
pub struct DTOGCDTI0EN_R(crate::FieldReader<bool, bool>);
impl DTOGCDTI0EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        DTOGCDTI0EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DTOGCDTI0EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DTOGCDTI0EN` writer - DTI CDTI0 Output Generation Enable"]
pub struct DTOGCDTI0EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DTOGCDTI0EN_W<'a> {
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
#[doc = "Field `DTOGCDTI1EN` reader - DTI CDTI1 Output Generation Enable"]
pub struct DTOGCDTI1EN_R(crate::FieldReader<bool, bool>);
impl DTOGCDTI1EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        DTOGCDTI1EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DTOGCDTI1EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DTOGCDTI1EN` writer - DTI CDTI1 Output Generation Enable"]
pub struct DTOGCDTI1EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DTOGCDTI1EN_W<'a> {
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
#[doc = "Field `DTOGCDTI2EN` reader - DTI CDTI2 Output Generation Enable"]
pub struct DTOGCDTI2EN_R(crate::FieldReader<bool, bool>);
impl DTOGCDTI2EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        DTOGCDTI2EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DTOGCDTI2EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DTOGCDTI2EN` writer - DTI CDTI2 Output Generation Enable"]
pub struct DTOGCDTI2EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DTOGCDTI2EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - DTI CC0 Output Generation Enable"]
    #[inline(always)]
    pub fn dtogcc0en(&self) -> DTOGCC0EN_R {
        DTOGCC0EN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - DTI CC1 Output Generation Enable"]
    #[inline(always)]
    pub fn dtogcc1en(&self) -> DTOGCC1EN_R {
        DTOGCC1EN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - DTI CC2 Output Generation Enable"]
    #[inline(always)]
    pub fn dtogcc2en(&self) -> DTOGCC2EN_R {
        DTOGCC2EN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - DTI CDTI0 Output Generation Enable"]
    #[inline(always)]
    pub fn dtogcdti0en(&self) -> DTOGCDTI0EN_R {
        DTOGCDTI0EN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - DTI CDTI1 Output Generation Enable"]
    #[inline(always)]
    pub fn dtogcdti1en(&self) -> DTOGCDTI1EN_R {
        DTOGCDTI1EN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - DTI CDTI2 Output Generation Enable"]
    #[inline(always)]
    pub fn dtogcdti2en(&self) -> DTOGCDTI2EN_R {
        DTOGCDTI2EN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DTI CC0 Output Generation Enable"]
    #[inline(always)]
    pub fn dtogcc0en(&mut self) -> DTOGCC0EN_W {
        DTOGCC0EN_W { w: self }
    }
    #[doc = "Bit 1 - DTI CC1 Output Generation Enable"]
    #[inline(always)]
    pub fn dtogcc1en(&mut self) -> DTOGCC1EN_W {
        DTOGCC1EN_W { w: self }
    }
    #[doc = "Bit 2 - DTI CC2 Output Generation Enable"]
    #[inline(always)]
    pub fn dtogcc2en(&mut self) -> DTOGCC2EN_W {
        DTOGCC2EN_W { w: self }
    }
    #[doc = "Bit 3 - DTI CDTI0 Output Generation Enable"]
    #[inline(always)]
    pub fn dtogcdti0en(&mut self) -> DTOGCDTI0EN_W {
        DTOGCDTI0EN_W { w: self }
    }
    #[doc = "Bit 4 - DTI CDTI1 Output Generation Enable"]
    #[inline(always)]
    pub fn dtogcdti1en(&mut self) -> DTOGCDTI1EN_W {
        DTOGCDTI1EN_W { w: self }
    }
    #[doc = "Bit 5 - DTI CDTI2 Output Generation Enable"]
    #[inline(always)]
    pub fn dtogcdti2en(&mut self) -> DTOGCDTI2EN_W {
        DTOGCDTI2EN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DTI Output Generation Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dtogen](index.html) module"]
pub struct DTOGEN_SPEC;
impl crate::RegisterSpec for DTOGEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dtogen::R](R) reader structure"]
impl crate::Readable for DTOGEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dtogen::W](W) writer structure"]
impl crate::Writable for DTOGEN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DTOGEN to value 0"]
impl crate::Resettable for DTOGEN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

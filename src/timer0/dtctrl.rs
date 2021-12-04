#[doc = "Register `DTCTRL` reader"]
pub struct R(crate::R<DTCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DTCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DTCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DTCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DTCTRL` writer"]
pub struct W(crate::W<DTCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DTCTRL_SPEC>;
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
impl From<crate::W<DTCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DTCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DTEN` reader - DTI Enable"]
pub struct DTEN_R(crate::FieldReader<bool, bool>);
impl DTEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        DTEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DTEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DTEN` writer - DTI Enable"]
pub struct DTEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DTEN_W<'a> {
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
#[doc = "Field `DTDAS` reader - DTI Automatic Start-up Functionality"]
pub struct DTDAS_R(crate::FieldReader<bool, bool>);
impl DTDAS_R {
    pub(crate) fn new(bits: bool) -> Self {
        DTDAS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DTDAS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DTDAS` writer - DTI Automatic Start-up Functionality"]
pub struct DTDAS_W<'a> {
    w: &'a mut W,
}
impl<'a> DTDAS_W<'a> {
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
#[doc = "Field `DTIPOL` reader - DTI Inactive Polarity"]
pub struct DTIPOL_R(crate::FieldReader<bool, bool>);
impl DTIPOL_R {
    pub(crate) fn new(bits: bool) -> Self {
        DTIPOL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DTIPOL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DTIPOL` writer - DTI Inactive Polarity"]
pub struct DTIPOL_W<'a> {
    w: &'a mut W,
}
impl<'a> DTIPOL_W<'a> {
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
#[doc = "Field `DTCINV` reader - DTI Complementary Output Invert."]
pub struct DTCINV_R(crate::FieldReader<bool, bool>);
impl DTCINV_R {
    pub(crate) fn new(bits: bool) -> Self {
        DTCINV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DTCINV_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DTCINV` writer - DTI Complementary Output Invert."]
pub struct DTCINV_W<'a> {
    w: &'a mut W,
}
impl<'a> DTCINV_W<'a> {
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
#[doc = "DTI PRS Source Channel Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DTPRSSEL_A {
    #[doc = "0: PRS Channel 0 selected as input"]
    PRSCH0 = 0,
    #[doc = "1: PRS Channel 1 selected as input"]
    PRSCH1 = 1,
    #[doc = "2: PRS Channel 2 selected as input"]
    PRSCH2 = 2,
    #[doc = "3: PRS Channel 3 selected as input"]
    PRSCH3 = 3,
    #[doc = "4: PRS Channel 4 selected as input"]
    PRSCH4 = 4,
    #[doc = "5: PRS Channel 5 selected as input"]
    PRSCH5 = 5,
}
impl From<DTPRSSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: DTPRSSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `DTPRSSEL` reader - DTI PRS Source Channel Select"]
pub struct DTPRSSEL_R(crate::FieldReader<u8, DTPRSSEL_A>);
impl DTPRSSEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        DTPRSSEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<DTPRSSEL_A> {
        match self.bits {
            0 => Some(DTPRSSEL_A::PRSCH0),
            1 => Some(DTPRSSEL_A::PRSCH1),
            2 => Some(DTPRSSEL_A::PRSCH2),
            3 => Some(DTPRSSEL_A::PRSCH3),
            4 => Some(DTPRSSEL_A::PRSCH4),
            5 => Some(DTPRSSEL_A::PRSCH5),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `PRSCH0`"]
    #[inline(always)]
    pub fn is_prsch0(&self) -> bool {
        **self == DTPRSSEL_A::PRSCH0
    }
    #[doc = "Checks if the value of the field is `PRSCH1`"]
    #[inline(always)]
    pub fn is_prsch1(&self) -> bool {
        **self == DTPRSSEL_A::PRSCH1
    }
    #[doc = "Checks if the value of the field is `PRSCH2`"]
    #[inline(always)]
    pub fn is_prsch2(&self) -> bool {
        **self == DTPRSSEL_A::PRSCH2
    }
    #[doc = "Checks if the value of the field is `PRSCH3`"]
    #[inline(always)]
    pub fn is_prsch3(&self) -> bool {
        **self == DTPRSSEL_A::PRSCH3
    }
    #[doc = "Checks if the value of the field is `PRSCH4`"]
    #[inline(always)]
    pub fn is_prsch4(&self) -> bool {
        **self == DTPRSSEL_A::PRSCH4
    }
    #[doc = "Checks if the value of the field is `PRSCH5`"]
    #[inline(always)]
    pub fn is_prsch5(&self) -> bool {
        **self == DTPRSSEL_A::PRSCH5
    }
}
impl core::ops::Deref for DTPRSSEL_R {
    type Target = crate::FieldReader<u8, DTPRSSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DTPRSSEL` writer - DTI PRS Source Channel Select"]
pub struct DTPRSSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> DTPRSSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DTPRSSEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "PRS Channel 0 selected as input"]
    #[inline(always)]
    pub fn prsch0(self) -> &'a mut W {
        self.variant(DTPRSSEL_A::PRSCH0)
    }
    #[doc = "PRS Channel 1 selected as input"]
    #[inline(always)]
    pub fn prsch1(self) -> &'a mut W {
        self.variant(DTPRSSEL_A::PRSCH1)
    }
    #[doc = "PRS Channel 2 selected as input"]
    #[inline(always)]
    pub fn prsch2(self) -> &'a mut W {
        self.variant(DTPRSSEL_A::PRSCH2)
    }
    #[doc = "PRS Channel 3 selected as input"]
    #[inline(always)]
    pub fn prsch3(self) -> &'a mut W {
        self.variant(DTPRSSEL_A::PRSCH3)
    }
    #[doc = "PRS Channel 4 selected as input"]
    #[inline(always)]
    pub fn prsch4(self) -> &'a mut W {
        self.variant(DTPRSSEL_A::PRSCH4)
    }
    #[doc = "PRS Channel 5 selected as input"]
    #[inline(always)]
    pub fn prsch5(self) -> &'a mut W {
        self.variant(DTPRSSEL_A::PRSCH5)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | ((value as u32 & 0x07) << 4);
        self.w
    }
}
#[doc = "Field `DTPRSEN` reader - DTI PRS Source Enable"]
pub struct DTPRSEN_R(crate::FieldReader<bool, bool>);
impl DTPRSEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        DTPRSEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DTPRSEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DTPRSEN` writer - DTI PRS Source Enable"]
pub struct DTPRSEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DTPRSEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | ((value as u32 & 0x01) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - DTI Enable"]
    #[inline(always)]
    pub fn dten(&self) -> DTEN_R {
        DTEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - DTI Automatic Start-up Functionality"]
    #[inline(always)]
    pub fn dtdas(&self) -> DTDAS_R {
        DTDAS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - DTI Inactive Polarity"]
    #[inline(always)]
    pub fn dtipol(&self) -> DTIPOL_R {
        DTIPOL_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - DTI Complementary Output Invert."]
    #[inline(always)]
    pub fn dtcinv(&self) -> DTCINV_R {
        DTCINV_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 4:6 - DTI PRS Source Channel Select"]
    #[inline(always)]
    pub fn dtprssel(&self) -> DTPRSSEL_R {
        DTPRSSEL_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    #[doc = "Bit 24 - DTI PRS Source Enable"]
    #[inline(always)]
    pub fn dtprsen(&self) -> DTPRSEN_R {
        DTPRSEN_R::new(((self.bits >> 24) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DTI Enable"]
    #[inline(always)]
    pub fn dten(&mut self) -> DTEN_W {
        DTEN_W { w: self }
    }
    #[doc = "Bit 1 - DTI Automatic Start-up Functionality"]
    #[inline(always)]
    pub fn dtdas(&mut self) -> DTDAS_W {
        DTDAS_W { w: self }
    }
    #[doc = "Bit 2 - DTI Inactive Polarity"]
    #[inline(always)]
    pub fn dtipol(&mut self) -> DTIPOL_W {
        DTIPOL_W { w: self }
    }
    #[doc = "Bit 3 - DTI Complementary Output Invert."]
    #[inline(always)]
    pub fn dtcinv(&mut self) -> DTCINV_W {
        DTCINV_W { w: self }
    }
    #[doc = "Bits 4:6 - DTI PRS Source Channel Select"]
    #[inline(always)]
    pub fn dtprssel(&mut self) -> DTPRSSEL_W {
        DTPRSSEL_W { w: self }
    }
    #[doc = "Bit 24 - DTI PRS Source Enable"]
    #[inline(always)]
    pub fn dtprsen(&mut self) -> DTPRSEN_W {
        DTPRSEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DTI Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dtctrl](index.html) module"]
pub struct DTCTRL_SPEC;
impl crate::RegisterSpec for DTCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dtctrl::R](R) reader structure"]
impl crate::Readable for DTCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dtctrl::W](W) writer structure"]
impl crate::Writable for DTCTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DTCTRL to value 0"]
impl crate::Resettable for DTCTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

#[doc = "Register `IEN` reader"]
pub struct R(crate::R<IEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IEN` writer"]
pub struct W(crate::W<IEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IEN_SPEC>;
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
impl From<crate::W<IEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CH0DONE` reader - DMA Channel 0 Complete Interrupt Enable"]
pub struct CH0DONE_R(crate::FieldReader<bool, bool>);
impl CH0DONE_R {
    pub(crate) fn new(bits: bool) -> Self {
        CH0DONE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH0DONE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH0DONE` writer - DMA Channel 0 Complete Interrupt Enable"]
pub struct CH0DONE_W<'a> {
    w: &'a mut W,
}
impl<'a> CH0DONE_W<'a> {
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
#[doc = "Field `CH1DONE` reader - DMA Channel 1 Complete Interrupt Enable"]
pub struct CH1DONE_R(crate::FieldReader<bool, bool>);
impl CH1DONE_R {
    pub(crate) fn new(bits: bool) -> Self {
        CH1DONE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH1DONE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH1DONE` writer - DMA Channel 1 Complete Interrupt Enable"]
pub struct CH1DONE_W<'a> {
    w: &'a mut W,
}
impl<'a> CH1DONE_W<'a> {
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
#[doc = "Field `CH2DONE` reader - DMA Channel 2 Complete Interrupt Enable"]
pub struct CH2DONE_R(crate::FieldReader<bool, bool>);
impl CH2DONE_R {
    pub(crate) fn new(bits: bool) -> Self {
        CH2DONE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH2DONE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH2DONE` writer - DMA Channel 2 Complete Interrupt Enable"]
pub struct CH2DONE_W<'a> {
    w: &'a mut W,
}
impl<'a> CH2DONE_W<'a> {
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
#[doc = "Field `CH3DONE` reader - DMA Channel 3 Complete Interrupt Enable"]
pub struct CH3DONE_R(crate::FieldReader<bool, bool>);
impl CH3DONE_R {
    pub(crate) fn new(bits: bool) -> Self {
        CH3DONE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH3DONE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH3DONE` writer - DMA Channel 3 Complete Interrupt Enable"]
pub struct CH3DONE_W<'a> {
    w: &'a mut W,
}
impl<'a> CH3DONE_W<'a> {
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
#[doc = "Field `CH4DONE` reader - DMA Channel 4 Complete Interrupt Enable"]
pub struct CH4DONE_R(crate::FieldReader<bool, bool>);
impl CH4DONE_R {
    pub(crate) fn new(bits: bool) -> Self {
        CH4DONE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH4DONE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH4DONE` writer - DMA Channel 4 Complete Interrupt Enable"]
pub struct CH4DONE_W<'a> {
    w: &'a mut W,
}
impl<'a> CH4DONE_W<'a> {
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
#[doc = "Field `CH5DONE` reader - DMA Channel 5 Complete Interrupt Enable"]
pub struct CH5DONE_R(crate::FieldReader<bool, bool>);
impl CH5DONE_R {
    pub(crate) fn new(bits: bool) -> Self {
        CH5DONE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH5DONE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH5DONE` writer - DMA Channel 5 Complete Interrupt Enable"]
pub struct CH5DONE_W<'a> {
    w: &'a mut W,
}
impl<'a> CH5DONE_W<'a> {
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
#[doc = "Field `ERR` reader - DMA Error Interrupt Flag Enable"]
pub struct ERR_R(crate::FieldReader<bool, bool>);
impl ERR_R {
    pub(crate) fn new(bits: bool) -> Self {
        ERR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ERR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ERR` writer - DMA Error Interrupt Flag Enable"]
pub struct ERR_W<'a> {
    w: &'a mut W,
}
impl<'a> ERR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | ((value as u32 & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - DMA Channel 0 Complete Interrupt Enable"]
    #[inline(always)]
    pub fn ch0done(&self) -> CH0DONE_R {
        CH0DONE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - DMA Channel 1 Complete Interrupt Enable"]
    #[inline(always)]
    pub fn ch1done(&self) -> CH1DONE_R {
        CH1DONE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - DMA Channel 2 Complete Interrupt Enable"]
    #[inline(always)]
    pub fn ch2done(&self) -> CH2DONE_R {
        CH2DONE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - DMA Channel 3 Complete Interrupt Enable"]
    #[inline(always)]
    pub fn ch3done(&self) -> CH3DONE_R {
        CH3DONE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - DMA Channel 4 Complete Interrupt Enable"]
    #[inline(always)]
    pub fn ch4done(&self) -> CH4DONE_R {
        CH4DONE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - DMA Channel 5 Complete Interrupt Enable"]
    #[inline(always)]
    pub fn ch5done(&self) -> CH5DONE_R {
        CH5DONE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 31 - DMA Error Interrupt Flag Enable"]
    #[inline(always)]
    pub fn err(&self) -> ERR_R {
        ERR_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DMA Channel 0 Complete Interrupt Enable"]
    #[inline(always)]
    pub fn ch0done(&mut self) -> CH0DONE_W {
        CH0DONE_W { w: self }
    }
    #[doc = "Bit 1 - DMA Channel 1 Complete Interrupt Enable"]
    #[inline(always)]
    pub fn ch1done(&mut self) -> CH1DONE_W {
        CH1DONE_W { w: self }
    }
    #[doc = "Bit 2 - DMA Channel 2 Complete Interrupt Enable"]
    #[inline(always)]
    pub fn ch2done(&mut self) -> CH2DONE_W {
        CH2DONE_W { w: self }
    }
    #[doc = "Bit 3 - DMA Channel 3 Complete Interrupt Enable"]
    #[inline(always)]
    pub fn ch3done(&mut self) -> CH3DONE_W {
        CH3DONE_W { w: self }
    }
    #[doc = "Bit 4 - DMA Channel 4 Complete Interrupt Enable"]
    #[inline(always)]
    pub fn ch4done(&mut self) -> CH4DONE_W {
        CH4DONE_W { w: self }
    }
    #[doc = "Bit 5 - DMA Channel 5 Complete Interrupt Enable"]
    #[inline(always)]
    pub fn ch5done(&mut self) -> CH5DONE_W {
        CH5DONE_W { w: self }
    }
    #[doc = "Bit 31 - DMA Error Interrupt Flag Enable"]
    #[inline(always)]
    pub fn err(&mut self) -> ERR_W {
        ERR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ien](index.html) module"]
pub struct IEN_SPEC;
impl crate::RegisterSpec for IEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ien::R](R) reader structure"]
impl crate::Readable for IEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ien::W](W) writer structure"]
impl crate::Writable for IEN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IEN to value 0"]
impl crate::Resettable for IEN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

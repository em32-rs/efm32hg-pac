#[doc = "Register `INSENSE` reader"]
pub struct R(crate::R<INSENSE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INSENSE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INSENSE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INSENSE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INSENSE` writer"]
pub struct W(crate::W<INSENSE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INSENSE_SPEC>;
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
impl From<crate::W<INSENSE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INSENSE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INT` reader - Interrupt Sense Enable"]
pub struct INT_R(crate::FieldReader<bool, bool>);
impl INT_R {
    pub(crate) fn new(bits: bool) -> Self {
        INT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INT` writer - Interrupt Sense Enable"]
pub struct INT_W<'a> {
    w: &'a mut W,
}
impl<'a> INT_W<'a> {
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
#[doc = "Field `PRS` reader - PRS Sense Enable"]
pub struct PRS_R(crate::FieldReader<bool, bool>);
impl PRS_R {
    pub(crate) fn new(bits: bool) -> Self {
        PRS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRS` writer - PRS Sense Enable"]
pub struct PRS_W<'a> {
    w: &'a mut W,
}
impl<'a> PRS_W<'a> {
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
impl R {
    #[doc = "Bit 0 - Interrupt Sense Enable"]
    #[inline(always)]
    pub fn int(&self) -> INT_R {
        INT_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - PRS Sense Enable"]
    #[inline(always)]
    pub fn prs(&self) -> PRS_R {
        PRS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Interrupt Sense Enable"]
    #[inline(always)]
    pub fn int(&mut self) -> INT_W {
        INT_W { w: self }
    }
    #[doc = "Bit 1 - PRS Sense Enable"]
    #[inline(always)]
    pub fn prs(&mut self) -> PRS_W {
        PRS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Input Sense Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [insense](index.html) module"]
pub struct INSENSE_SPEC;
impl crate::RegisterSpec for INSENSE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [insense::R](R) reader structure"]
impl crate::Readable for INSENSE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [insense::W](W) writer structure"]
impl crate::Writable for INSENSE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INSENSE to value 0x03"]
impl crate::Resettable for INSENSE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x03
    }
}

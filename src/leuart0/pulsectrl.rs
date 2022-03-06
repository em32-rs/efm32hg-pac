#[doc = "Register `PULSECTRL` reader"]
pub struct R(crate::R<PULSECTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PULSECTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PULSECTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PULSECTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PULSECTRL` writer"]
pub struct W(crate::W<PULSECTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PULSECTRL_SPEC>;
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
impl From<crate::W<PULSECTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PULSECTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PULSEW` reader - Pulse Width"]
pub struct PULSEW_R(crate::FieldReader<u8, u8>);
impl PULSEW_R {
    pub(crate) fn new(bits: u8) -> Self {
        PULSEW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PULSEW_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PULSEW` writer - Pulse Width"]
pub struct PULSEW_W<'a> {
    w: &'a mut W,
}
impl<'a> PULSEW_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
#[doc = "Field `PULSEEN` reader - Pulse Generator/Extender Enable"]
pub struct PULSEEN_R(crate::FieldReader<bool, bool>);
impl PULSEEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        PULSEEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PULSEEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PULSEEN` writer - Pulse Generator/Extender Enable"]
pub struct PULSEEN_W<'a> {
    w: &'a mut W,
}
impl<'a> PULSEEN_W<'a> {
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
#[doc = "Field `PULSEFILT` reader - Pulse Filter"]
pub struct PULSEFILT_R(crate::FieldReader<bool, bool>);
impl PULSEFILT_R {
    pub(crate) fn new(bits: bool) -> Self {
        PULSEFILT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PULSEFILT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PULSEFILT` writer - Pulse Filter"]
pub struct PULSEFILT_W<'a> {
    w: &'a mut W,
}
impl<'a> PULSEFILT_W<'a> {
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
    #[doc = "Bits 0:3 - Pulse Width"]
    #[inline(always)]
    pub fn pulsew(&self) -> PULSEW_R {
        PULSEW_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - Pulse Generator/Extender Enable"]
    #[inline(always)]
    pub fn pulseen(&self) -> PULSEEN_R {
        PULSEEN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Pulse Filter"]
    #[inline(always)]
    pub fn pulsefilt(&self) -> PULSEFILT_R {
        PULSEFILT_R::new(((self.bits >> 5) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Pulse Width"]
    #[inline(always)]
    pub fn pulsew(&mut self) -> PULSEW_W {
        PULSEW_W { w: self }
    }
    #[doc = "Bit 4 - Pulse Generator/Extender Enable"]
    #[inline(always)]
    pub fn pulseen(&mut self) -> PULSEEN_W {
        PULSEEN_W { w: self }
    }
    #[doc = "Bit 5 - Pulse Filter"]
    #[inline(always)]
    pub fn pulsefilt(&mut self) -> PULSEFILT_W {
        PULSEFILT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Pulse Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pulsectrl](index.html) module"]
pub struct PULSECTRL_SPEC;
impl crate::RegisterSpec for PULSECTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pulsectrl::R](R) reader structure"]
impl crate::Readable for PULSECTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pulsectrl::W](W) writer structure"]
impl crate::Writable for PULSECTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PULSECTRL to value 0"]
impl crate::Resettable for PULSECTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

#[doc = "Register `BIASPROG` reader"]
pub struct R(crate::R<BIASPROG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BIASPROG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BIASPROG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BIASPROG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BIASPROG` writer"]
pub struct W(crate::W<BIASPROG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BIASPROG_SPEC>;
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
impl From<crate::W<BIASPROG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BIASPROG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BIASPROG` reader - Bias Programming Value"]
pub struct BIASPROG_R(crate::FieldReader<u8, u8>);
impl BIASPROG_R {
    pub(crate) fn new(bits: u8) -> Self {
        BIASPROG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BIASPROG_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BIASPROG` writer - Bias Programming Value"]
pub struct BIASPROG_W<'a> {
    w: &'a mut W,
}
impl<'a> BIASPROG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
#[doc = "Field `HALFBIAS` reader - Half Bias Current"]
pub struct HALFBIAS_R(crate::FieldReader<bool, bool>);
impl HALFBIAS_R {
    pub(crate) fn new(bits: bool) -> Self {
        HALFBIAS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HALFBIAS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HALFBIAS` writer - Half Bias Current"]
pub struct HALFBIAS_W<'a> {
    w: &'a mut W,
}
impl<'a> HALFBIAS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
#[doc = "Field `COMPBIAS` reader - Comparator Bias Value"]
pub struct COMPBIAS_R(crate::FieldReader<u8, u8>);
impl COMPBIAS_R {
    pub(crate) fn new(bits: u8) -> Self {
        COMPBIAS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COMPBIAS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `COMPBIAS` writer - Comparator Bias Value"]
pub struct COMPBIAS_W<'a> {
    w: &'a mut W,
}
impl<'a> COMPBIAS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | ((value as u32 & 0x0f) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Bias Programming Value"]
    #[inline(always)]
    pub fn biasprog(&self) -> BIASPROG_R {
        BIASPROG_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 6 - Half Bias Current"]
    #[inline(always)]
    pub fn halfbias(&self) -> HALFBIAS_R {
        HALFBIAS_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bits 8:11 - Comparator Bias Value"]
    #[inline(always)]
    pub fn compbias(&self) -> COMPBIAS_R {
        COMPBIAS_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Bias Programming Value"]
    #[inline(always)]
    pub fn biasprog(&mut self) -> BIASPROG_W {
        BIASPROG_W { w: self }
    }
    #[doc = "Bit 6 - Half Bias Current"]
    #[inline(always)]
    pub fn halfbias(&mut self) -> HALFBIAS_W {
        HALFBIAS_W { w: self }
    }
    #[doc = "Bits 8:11 - Comparator Bias Value"]
    #[inline(always)]
    pub fn compbias(&mut self) -> COMPBIAS_W {
        COMPBIAS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Bias Programming Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [biasprog](index.html) module"]
pub struct BIASPROG_SPEC;
impl crate::RegisterSpec for BIASPROG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [biasprog::R](R) reader structure"]
impl crate::Readable for BIASPROG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [biasprog::W](W) writer structure"]
impl crate::Writable for BIASPROG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BIASPROG to value 0x0747"]
impl crate::Resettable for BIASPROG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0747
    }
}

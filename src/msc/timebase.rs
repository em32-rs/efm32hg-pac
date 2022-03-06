#[doc = "Register `TIMEBASE` reader"]
pub struct R(crate::R<TIMEBASE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIMEBASE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIMEBASE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIMEBASE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TIMEBASE` writer"]
pub struct W(crate::W<TIMEBASE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIMEBASE_SPEC>;
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
impl From<crate::W<TIMEBASE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIMEBASE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BASE` reader - Timebase used by MSC to time flash writes and erases"]
pub struct BASE_R(crate::FieldReader<u8, u8>);
impl BASE_R {
    pub(crate) fn new(bits: u8) -> Self {
        BASE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BASE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BASE` writer - Timebase used by MSC to time flash writes and erases"]
pub struct BASE_W<'a> {
    w: &'a mut W,
}
impl<'a> BASE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | (value as u32 & 0x3f);
        self.w
    }
}
#[doc = "Field `PERIOD` reader - Sets the timebase period"]
pub struct PERIOD_R(crate::FieldReader<bool, bool>);
impl PERIOD_R {
    pub(crate) fn new(bits: bool) -> Self {
        PERIOD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PERIOD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PERIOD` writer - Sets the timebase period"]
pub struct PERIOD_W<'a> {
    w: &'a mut W,
}
impl<'a> PERIOD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:5 - Timebase used by MSC to time flash writes and erases"]
    #[inline(always)]
    pub fn base(&self) -> BASE_R {
        BASE_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 16 - Sets the timebase period"]
    #[inline(always)]
    pub fn period(&self) -> PERIOD_R {
        PERIOD_R::new(((self.bits >> 16) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - Timebase used by MSC to time flash writes and erases"]
    #[inline(always)]
    pub fn base(&mut self) -> BASE_W {
        BASE_W { w: self }
    }
    #[doc = "Bit 16 - Sets the timebase period"]
    #[inline(always)]
    pub fn period(&mut self) -> PERIOD_W {
        PERIOD_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Flash Write and Erase Timebase\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timebase](index.html) module"]
pub struct TIMEBASE_SPEC;
impl crate::RegisterSpec for TIMEBASE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [timebase::R](R) reader structure"]
impl crate::Readable for TIMEBASE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [timebase::W](W) writer structure"]
impl crate::Writable for TIMEBASE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TIMEBASE to value 0x10"]
impl crate::Resettable for TIMEBASE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x10
    }
}

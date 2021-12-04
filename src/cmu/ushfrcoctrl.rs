#[doc = "Register `USHFRCOCTRL` reader"]
pub struct R(crate::R<USHFRCOCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USHFRCOCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USHFRCOCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USHFRCOCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `USHFRCOCTRL` writer"]
pub struct W(crate::W<USHFRCOCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<USHFRCOCTRL_SPEC>;
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
impl From<crate::W<USHFRCOCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<USHFRCOCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TUNING` reader - USHFRCO frequency adjust"]
pub struct TUNING_R(crate::FieldReader<u8, u8>);
impl TUNING_R {
    pub(crate) fn new(bits: u8) -> Self {
        TUNING_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TUNING_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TUNING` writer - USHFRCO frequency adjust"]
pub struct TUNING_W<'a> {
    w: &'a mut W,
}
impl<'a> TUNING_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | (value as u32 & 0x7f);
        self.w
    }
}
#[doc = "Field `DITHEN` reader - USHFRCO dither enable"]
pub struct DITHEN_R(crate::FieldReader<bool, bool>);
impl DITHEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        DITHEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DITHEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DITHEN` writer - USHFRCO dither enable"]
pub struct DITHEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DITHEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
#[doc = "Field `SUSPEND` reader - USHFRCO suspend"]
pub struct SUSPEND_R(crate::FieldReader<bool, bool>);
impl SUSPEND_R {
    pub(crate) fn new(bits: bool) -> Self {
        SUSPEND_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SUSPEND_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SUSPEND` writer - USHFRCO suspend"]
pub struct SUSPEND_W<'a> {
    w: &'a mut W,
}
impl<'a> SUSPEND_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
#[doc = "Field `TIMEOUT` reader - USHFRCO Timeout"]
pub struct TIMEOUT_R(crate::FieldReader<u8, u8>);
impl TIMEOUT_R {
    pub(crate) fn new(bits: u8) -> Self {
        TIMEOUT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIMEOUT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIMEOUT` writer - USHFRCO Timeout"]
pub struct TIMEOUT_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMEOUT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 12)) | ((value as u32 & 0xff) << 12);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:6 - USHFRCO frequency adjust"]
    #[inline(always)]
    pub fn tuning(&self) -> TUNING_R {
        TUNING_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 8 - USHFRCO dither enable"]
    #[inline(always)]
    pub fn dithen(&self) -> DITHEN_R {
        DITHEN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - USHFRCO suspend"]
    #[inline(always)]
    pub fn suspend(&self) -> SUSPEND_R {
        SUSPEND_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bits 12:19 - USHFRCO Timeout"]
    #[inline(always)]
    pub fn timeout(&self) -> TIMEOUT_R {
        TIMEOUT_R::new(((self.bits >> 12) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - USHFRCO frequency adjust"]
    #[inline(always)]
    pub fn tuning(&mut self) -> TUNING_W {
        TUNING_W { w: self }
    }
    #[doc = "Bit 8 - USHFRCO dither enable"]
    #[inline(always)]
    pub fn dithen(&mut self) -> DITHEN_W {
        DITHEN_W { w: self }
    }
    #[doc = "Bit 9 - USHFRCO suspend"]
    #[inline(always)]
    pub fn suspend(&mut self) -> SUSPEND_W {
        SUSPEND_W { w: self }
    }
    #[doc = "Bits 12:19 - USHFRCO Timeout"]
    #[inline(always)]
    pub fn timeout(&mut self) -> TIMEOUT_W {
        TIMEOUT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USHFRCO Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ushfrcoctrl](index.html) module"]
pub struct USHFRCOCTRL_SPEC;
impl crate::RegisterSpec for USHFRCOCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ushfrcoctrl::R](R) reader structure"]
impl crate::Readable for USHFRCOCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ushfrcoctrl::W](W) writer structure"]
impl crate::Writable for USHFRCOCTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets USHFRCOCTRL to value 0x000f_f040"]
impl crate::Resettable for USHFRCOCTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x000f_f040
    }
}

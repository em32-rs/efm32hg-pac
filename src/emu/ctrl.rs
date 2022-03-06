#[doc = "Register `CTRL` reader"]
pub struct R(crate::R<CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRL` writer"]
pub struct W(crate::W<CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRL_SPEC>;
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
impl From<crate::W<CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EMVREG` reader - Energy Mode Voltage Regulator Control"]
pub struct EMVREG_R(crate::FieldReader<bool, bool>);
impl EMVREG_R {
    pub(crate) fn new(bits: bool) -> Self {
        EMVREG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EMVREG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EMVREG` writer - Energy Mode Voltage Regulator Control"]
pub struct EMVREG_W<'a> {
    w: &'a mut W,
}
impl<'a> EMVREG_W<'a> {
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
#[doc = "Field `EM2BLOCK` reader - Energy Mode 2 Block"]
pub struct EM2BLOCK_R(crate::FieldReader<bool, bool>);
impl EM2BLOCK_R {
    pub(crate) fn new(bits: bool) -> Self {
        EM2BLOCK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EM2BLOCK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EM2BLOCK` writer - Energy Mode 2 Block"]
pub struct EM2BLOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> EM2BLOCK_W<'a> {
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
#[doc = "Field `EM4CTRL` reader - Energy Mode 4 Control"]
pub struct EM4CTRL_R(crate::FieldReader<u8, u8>);
impl EM4CTRL_R {
    pub(crate) fn new(bits: u8) -> Self {
        EM4CTRL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EM4CTRL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EM4CTRL` writer - Energy Mode 4 Control"]
pub struct EM4CTRL_W<'a> {
    w: &'a mut W,
}
impl<'a> EM4CTRL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | ((value as u32 & 0x03) << 2);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Energy Mode Voltage Regulator Control"]
    #[inline(always)]
    pub fn emvreg(&self) -> EMVREG_R {
        EMVREG_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Energy Mode 2 Block"]
    #[inline(always)]
    pub fn em2block(&self) -> EM2BLOCK_R {
        EM2BLOCK_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bits 2:3 - Energy Mode 4 Control"]
    #[inline(always)]
    pub fn em4ctrl(&self) -> EM4CTRL_R {
        EM4CTRL_R::new(((self.bits >> 2) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Energy Mode Voltage Regulator Control"]
    #[inline(always)]
    pub fn emvreg(&mut self) -> EMVREG_W {
        EMVREG_W { w: self }
    }
    #[doc = "Bit 1 - Energy Mode 2 Block"]
    #[inline(always)]
    pub fn em2block(&mut self) -> EM2BLOCK_W {
        EM2BLOCK_W { w: self }
    }
    #[doc = "Bits 2:3 - Energy Mode 4 Control"]
    #[inline(always)]
    pub fn em4ctrl(&mut self) -> EM4CTRL_W {
        EM4CTRL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl](index.html) module"]
pub struct CTRL_SPEC;
impl crate::RegisterSpec for CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctrl::R](R) reader structure"]
impl crate::Readable for CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrl::W](W) writer structure"]
impl crate::Writable for CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CTRL to value 0"]
impl crate::Resettable for CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

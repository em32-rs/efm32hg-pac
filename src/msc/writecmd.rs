#[doc = "Register `WRITECMD` writer"]
pub struct W(crate::W<WRITECMD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WRITECMD_SPEC>;
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
impl From<crate::W<WRITECMD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WRITECMD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LADDRIM` writer - Load MSC_ADDRB into ADDR"]
pub struct LADDRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> LADDRIM_W<'a> {
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
#[doc = "Field `ERASEPAGE` writer - Erase Page"]
pub struct ERASEPAGE_W<'a> {
    w: &'a mut W,
}
impl<'a> ERASEPAGE_W<'a> {
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
#[doc = "Field `WRITEEND` writer - End Write Mode"]
pub struct WRITEEND_W<'a> {
    w: &'a mut W,
}
impl<'a> WRITEEND_W<'a> {
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
#[doc = "Field `WRITEONCE` writer - Word Write-Once Trigger"]
pub struct WRITEONCE_W<'a> {
    w: &'a mut W,
}
impl<'a> WRITEONCE_W<'a> {
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
#[doc = "Field `WRITETRIG` writer - Word Write Sequence Trigger"]
pub struct WRITETRIG_W<'a> {
    w: &'a mut W,
}
impl<'a> WRITETRIG_W<'a> {
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
#[doc = "Field `ERASEABORT` writer - Abort erase sequence"]
pub struct ERASEABORT_W<'a> {
    w: &'a mut W,
}
impl<'a> ERASEABORT_W<'a> {
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
#[doc = "Field `ERASEMAIN0` writer - Mass erase region 0"]
pub struct ERASEMAIN0_W<'a> {
    w: &'a mut W,
}
impl<'a> ERASEMAIN0_W<'a> {
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
#[doc = "Field `CLEARWDATA` writer - Clear WDATA state"]
pub struct CLEARWDATA_W<'a> {
    w: &'a mut W,
}
impl<'a> CLEARWDATA_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | ((value as u32 & 0x01) << 12);
        self.w
    }
}
impl W {
    #[doc = "Bit 0 - Load MSC_ADDRB into ADDR"]
    #[inline(always)]
    pub fn laddrim(&mut self) -> LADDRIM_W {
        LADDRIM_W { w: self }
    }
    #[doc = "Bit 1 - Erase Page"]
    #[inline(always)]
    pub fn erasepage(&mut self) -> ERASEPAGE_W {
        ERASEPAGE_W { w: self }
    }
    #[doc = "Bit 2 - End Write Mode"]
    #[inline(always)]
    pub fn writeend(&mut self) -> WRITEEND_W {
        WRITEEND_W { w: self }
    }
    #[doc = "Bit 3 - Word Write-Once Trigger"]
    #[inline(always)]
    pub fn writeonce(&mut self) -> WRITEONCE_W {
        WRITEONCE_W { w: self }
    }
    #[doc = "Bit 4 - Word Write Sequence Trigger"]
    #[inline(always)]
    pub fn writetrig(&mut self) -> WRITETRIG_W {
        WRITETRIG_W { w: self }
    }
    #[doc = "Bit 5 - Abort erase sequence"]
    #[inline(always)]
    pub fn eraseabort(&mut self) -> ERASEABORT_W {
        ERASEABORT_W { w: self }
    }
    #[doc = "Bit 8 - Mass erase region 0"]
    #[inline(always)]
    pub fn erasemain0(&mut self) -> ERASEMAIN0_W {
        ERASEMAIN0_W { w: self }
    }
    #[doc = "Bit 12 - Clear WDATA state"]
    #[inline(always)]
    pub fn clearwdata(&mut self) -> CLEARWDATA_W {
        CLEARWDATA_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Write Command Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [writecmd](index.html) module"]
pub struct WRITECMD_SPEC;
impl crate::RegisterSpec for WRITECMD_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [writecmd::W](W) writer structure"]
impl crate::Writable for WRITECMD_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets WRITECMD to value 0"]
impl crate::Resettable for WRITECMD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

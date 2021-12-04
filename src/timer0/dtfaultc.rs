#[doc = "Register `DTFAULTC` writer"]
pub struct W(crate::W<DTFAULTC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DTFAULTC_SPEC>;
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
impl From<crate::W<DTFAULTC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DTFAULTC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DTPRS0FC` writer - DTI PRS0 Fault Clear"]
pub struct DTPRS0FC_W<'a> {
    w: &'a mut W,
}
impl<'a> DTPRS0FC_W<'a> {
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
#[doc = "Field `DTPRS1FC` writer - DTI PRS1 Fault Clear"]
pub struct DTPRS1FC_W<'a> {
    w: &'a mut W,
}
impl<'a> DTPRS1FC_W<'a> {
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
#[doc = "Field `DTDBGFC` writer - DTI Debugger Fault Clear"]
pub struct DTDBGFC_W<'a> {
    w: &'a mut W,
}
impl<'a> DTDBGFC_W<'a> {
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
#[doc = "Field `TLOCKUPFC` writer - DTI Lockup Fault Clear"]
pub struct TLOCKUPFC_W<'a> {
    w: &'a mut W,
}
impl<'a> TLOCKUPFC_W<'a> {
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
impl W {
    #[doc = "Bit 0 - DTI PRS0 Fault Clear"]
    #[inline(always)]
    pub fn dtprs0fc(&mut self) -> DTPRS0FC_W {
        DTPRS0FC_W { w: self }
    }
    #[doc = "Bit 1 - DTI PRS1 Fault Clear"]
    #[inline(always)]
    pub fn dtprs1fc(&mut self) -> DTPRS1FC_W {
        DTPRS1FC_W { w: self }
    }
    #[doc = "Bit 2 - DTI Debugger Fault Clear"]
    #[inline(always)]
    pub fn dtdbgfc(&mut self) -> DTDBGFC_W {
        DTDBGFC_W { w: self }
    }
    #[doc = "Bit 3 - DTI Lockup Fault Clear"]
    #[inline(always)]
    pub fn tlockupfc(&mut self) -> TLOCKUPFC_W {
        TLOCKUPFC_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DTI Fault Clear Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dtfaultc](index.html) module"]
pub struct DTFAULTC_SPEC;
impl crate::RegisterSpec for DTFAULTC_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [dtfaultc::W](W) writer structure"]
impl crate::Writable for DTFAULTC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DTFAULTC to value 0"]
impl crate::Resettable for DTFAULTC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

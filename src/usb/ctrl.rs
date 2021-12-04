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
#[doc = "Field `DMPUAP` reader - DMPU Active Polarity"]
pub struct DMPUAP_R(crate::FieldReader<bool, bool>);
impl DMPUAP_R {
    pub(crate) fn new(bits: bool) -> Self {
        DMPUAP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMPUAP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DMPUAP` writer - DMPU Active Polarity"]
pub struct DMPUAP_W<'a> {
    w: &'a mut W,
}
impl<'a> DMPUAP_W<'a> {
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
#[doc = "Low Energy Mode Oscillator Control\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum LEMOSCCTRL_A {
    #[doc = "0: Low Energy Mode has no effect on neither USBC or USHFRCO."]
    NONE = 0,
    #[doc = "1: The USBC clock is gated when Low Energy Mode is active."]
    GATE = 1,
    #[doc = "2: The USBC clock is gated, and USHFRCO is suspended (if not selected as HFCLK) when Low Energy Mode is active."]
    SUSPEND = 2,
}
impl From<LEMOSCCTRL_A> for u8 {
    #[inline(always)]
    fn from(variant: LEMOSCCTRL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `LEMOSCCTRL` reader - Low Energy Mode Oscillator Control"]
pub struct LEMOSCCTRL_R(crate::FieldReader<u8, LEMOSCCTRL_A>);
impl LEMOSCCTRL_R {
    pub(crate) fn new(bits: u8) -> Self {
        LEMOSCCTRL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<LEMOSCCTRL_A> {
        match self.bits {
            0 => Some(LEMOSCCTRL_A::NONE),
            1 => Some(LEMOSCCTRL_A::GATE),
            2 => Some(LEMOSCCTRL_A::SUSPEND),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        **self == LEMOSCCTRL_A::NONE
    }
    #[doc = "Checks if the value of the field is `GATE`"]
    #[inline(always)]
    pub fn is_gate(&self) -> bool {
        **self == LEMOSCCTRL_A::GATE
    }
    #[doc = "Checks if the value of the field is `SUSPEND`"]
    #[inline(always)]
    pub fn is_suspend(&self) -> bool {
        **self == LEMOSCCTRL_A::SUSPEND
    }
}
impl core::ops::Deref for LEMOSCCTRL_R {
    type Target = crate::FieldReader<u8, LEMOSCCTRL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LEMOSCCTRL` writer - Low Energy Mode Oscillator Control"]
pub struct LEMOSCCTRL_W<'a> {
    w: &'a mut W,
}
impl<'a> LEMOSCCTRL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LEMOSCCTRL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Low Energy Mode has no effect on neither USBC or USHFRCO."]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(LEMOSCCTRL_A::NONE)
    }
    #[doc = "The USBC clock is gated when Low Energy Mode is active."]
    #[inline(always)]
    pub fn gate(self) -> &'a mut W {
        self.variant(LEMOSCCTRL_A::GATE)
    }
    #[doc = "The USBC clock is gated, and USHFRCO is suspended (if not selected as HFCLK) when Low Energy Mode is active."]
    #[inline(always)]
    pub fn suspend(self) -> &'a mut W {
        self.variant(LEMOSCCTRL_A::SUSPEND)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | ((value as u32 & 0x03) << 4);
        self.w
    }
}
#[doc = "Field `LEMPHYCTRL` reader - Low Energy Mode USB PHY Control"]
pub struct LEMPHYCTRL_R(crate::FieldReader<bool, bool>);
impl LEMPHYCTRL_R {
    pub(crate) fn new(bits: bool) -> Self {
        LEMPHYCTRL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LEMPHYCTRL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LEMPHYCTRL` writer - Low Energy Mode USB PHY Control"]
pub struct LEMPHYCTRL_W<'a> {
    w: &'a mut W,
}
impl<'a> LEMPHYCTRL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
#[doc = "Field `LEMIDLEEN` reader - Low Energy Mode on Bus Idle Enable"]
pub struct LEMIDLEEN_R(crate::FieldReader<bool, bool>);
impl LEMIDLEEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        LEMIDLEEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LEMIDLEEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LEMIDLEEN` writer - Low Energy Mode on Bus Idle Enable"]
pub struct LEMIDLEEN_W<'a> {
    w: &'a mut W,
}
impl<'a> LEMIDLEEN_W<'a> {
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
#[doc = "Field `LEMNAKEN` reader - Low Energy Mode on OUT NAK Enable"]
pub struct LEMNAKEN_R(crate::FieldReader<bool, bool>);
impl LEMNAKEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        LEMNAKEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LEMNAKEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LEMNAKEN` writer - Low Energy Mode on OUT NAK Enable"]
pub struct LEMNAKEN_W<'a> {
    w: &'a mut W,
}
impl<'a> LEMNAKEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u32 & 0x01) << 10);
        self.w
    }
}
#[doc = "Field `LEMADDRMEN` reader - Low Energy Mode on Device Address Mismatch Enable"]
pub struct LEMADDRMEN_R(crate::FieldReader<bool, bool>);
impl LEMADDRMEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        LEMADDRMEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LEMADDRMEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LEMADDRMEN` writer - Low Energy Mode on Device Address Mismatch Enable"]
pub struct LEMADDRMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> LEMADDRMEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | ((value as u32 & 0x01) << 11);
        self.w
    }
}
#[doc = "Field `VREGDIS` reader - Voltage Regulator Disable"]
pub struct VREGDIS_R(crate::FieldReader<bool, bool>);
impl VREGDIS_R {
    pub(crate) fn new(bits: bool) -> Self {
        VREGDIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VREGDIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VREGDIS` writer - Voltage Regulator Disable"]
pub struct VREGDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> VREGDIS_W<'a> {
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
#[doc = "Field `VREGOSEN` reader - VREGO Sense Enable"]
pub struct VREGOSEN_R(crate::FieldReader<bool, bool>);
impl VREGOSEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        VREGOSEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VREGOSEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VREGOSEN` writer - VREGO Sense Enable"]
pub struct VREGOSEN_W<'a> {
    w: &'a mut W,
}
impl<'a> VREGOSEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | ((value as u32 & 0x01) << 17);
        self.w
    }
}
#[doc = "Field `BIASPROGEM01` reader - Regulator Bias Programming Value in EM0/1"]
pub struct BIASPROGEM01_R(crate::FieldReader<u8, u8>);
impl BIASPROGEM01_R {
    pub(crate) fn new(bits: u8) -> Self {
        BIASPROGEM01_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BIASPROGEM01_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BIASPROGEM01` writer - Regulator Bias Programming Value in EM0/1"]
pub struct BIASPROGEM01_W<'a> {
    w: &'a mut W,
}
impl<'a> BIASPROGEM01_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 20)) | ((value as u32 & 0x03) << 20);
        self.w
    }
}
#[doc = "Field `BIASPROGEM23` reader - Regulator Bias Programming Value in EM2/3"]
pub struct BIASPROGEM23_R(crate::FieldReader<u8, u8>);
impl BIASPROGEM23_R {
    pub(crate) fn new(bits: u8) -> Self {
        BIASPROGEM23_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BIASPROGEM23_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BIASPROGEM23` writer - Regulator Bias Programming Value in EM2/3"]
pub struct BIASPROGEM23_W<'a> {
    w: &'a mut W,
}
impl<'a> BIASPROGEM23_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | ((value as u32 & 0x03) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bit 1 - DMPU Active Polarity"]
    #[inline(always)]
    pub fn dmpuap(&self) -> DMPUAP_R {
        DMPUAP_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bits 4:5 - Low Energy Mode Oscillator Control"]
    #[inline(always)]
    pub fn lemoscctrl(&self) -> LEMOSCCTRL_R {
        LEMOSCCTRL_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bit 7 - Low Energy Mode USB PHY Control"]
    #[inline(always)]
    pub fn lemphyctrl(&self) -> LEMPHYCTRL_R {
        LEMPHYCTRL_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Low Energy Mode on Bus Idle Enable"]
    #[inline(always)]
    pub fn lemidleen(&self) -> LEMIDLEEN_R {
        LEMIDLEEN_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Low Energy Mode on OUT NAK Enable"]
    #[inline(always)]
    pub fn lemnaken(&self) -> LEMNAKEN_R {
        LEMNAKEN_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Low Energy Mode on Device Address Mismatch Enable"]
    #[inline(always)]
    pub fn lemaddrmen(&self) -> LEMADDRMEN_R {
        LEMADDRMEN_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Voltage Regulator Disable"]
    #[inline(always)]
    pub fn vregdis(&self) -> VREGDIS_R {
        VREGDIS_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - VREGO Sense Enable"]
    #[inline(always)]
    pub fn vregosen(&self) -> VREGOSEN_R {
        VREGOSEN_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bits 20:21 - Regulator Bias Programming Value in EM0/1"]
    #[inline(always)]
    pub fn biasprogem01(&self) -> BIASPROGEM01_R {
        BIASPROGEM01_R::new(((self.bits >> 20) & 0x03) as u8)
    }
    #[doc = "Bits 24:25 - Regulator Bias Programming Value in EM2/3"]
    #[inline(always)]
    pub fn biasprogem23(&self) -> BIASPROGEM23_R {
        BIASPROGEM23_R::new(((self.bits >> 24) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 1 - DMPU Active Polarity"]
    #[inline(always)]
    pub fn dmpuap(&mut self) -> DMPUAP_W {
        DMPUAP_W { w: self }
    }
    #[doc = "Bits 4:5 - Low Energy Mode Oscillator Control"]
    #[inline(always)]
    pub fn lemoscctrl(&mut self) -> LEMOSCCTRL_W {
        LEMOSCCTRL_W { w: self }
    }
    #[doc = "Bit 7 - Low Energy Mode USB PHY Control"]
    #[inline(always)]
    pub fn lemphyctrl(&mut self) -> LEMPHYCTRL_W {
        LEMPHYCTRL_W { w: self }
    }
    #[doc = "Bit 9 - Low Energy Mode on Bus Idle Enable"]
    #[inline(always)]
    pub fn lemidleen(&mut self) -> LEMIDLEEN_W {
        LEMIDLEEN_W { w: self }
    }
    #[doc = "Bit 10 - Low Energy Mode on OUT NAK Enable"]
    #[inline(always)]
    pub fn lemnaken(&mut self) -> LEMNAKEN_W {
        LEMNAKEN_W { w: self }
    }
    #[doc = "Bit 11 - Low Energy Mode on Device Address Mismatch Enable"]
    #[inline(always)]
    pub fn lemaddrmen(&mut self) -> LEMADDRMEN_W {
        LEMADDRMEN_W { w: self }
    }
    #[doc = "Bit 16 - Voltage Regulator Disable"]
    #[inline(always)]
    pub fn vregdis(&mut self) -> VREGDIS_W {
        VREGDIS_W { w: self }
    }
    #[doc = "Bit 17 - VREGO Sense Enable"]
    #[inline(always)]
    pub fn vregosen(&mut self) -> VREGOSEN_W {
        VREGOSEN_W { w: self }
    }
    #[doc = "Bits 20:21 - Regulator Bias Programming Value in EM0/1"]
    #[inline(always)]
    pub fn biasprogem01(&mut self) -> BIASPROGEM01_W {
        BIASPROGEM01_W { w: self }
    }
    #[doc = "Bits 24:25 - Regulator Bias Programming Value in EM2/3"]
    #[inline(always)]
    pub fn biasprogem23(&mut self) -> BIASPROGEM23_W {
        BIASPROGEM23_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "System Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl](index.html) module"]
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
#[doc = "`reset()` method sets CTRL to value 0x20"]
impl crate::Resettable for CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x20
    }
}

#[doc = "Register `DCTL` reader"]
pub struct R(crate::R<DCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DCTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DCTL` writer"]
pub struct W(crate::W<DCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DCTL_SPEC>;
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
impl From<crate::W<DCTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RMTWKUPSIG` reader - Remote Wakeup Signaling"]
pub struct RMTWKUPSIG_R(crate::FieldReader<bool, bool>);
impl RMTWKUPSIG_R {
    pub(crate) fn new(bits: bool) -> Self {
        RMTWKUPSIG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RMTWKUPSIG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RMTWKUPSIG` writer - Remote Wakeup Signaling"]
pub struct RMTWKUPSIG_W<'a> {
    w: &'a mut W,
}
impl<'a> RMTWKUPSIG_W<'a> {
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
#[doc = "Field `SFTDISCON` reader - Soft Disconnect"]
pub struct SFTDISCON_R(crate::FieldReader<bool, bool>);
impl SFTDISCON_R {
    pub(crate) fn new(bits: bool) -> Self {
        SFTDISCON_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SFTDISCON_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SFTDISCON` writer - Soft Disconnect"]
pub struct SFTDISCON_W<'a> {
    w: &'a mut W,
}
impl<'a> SFTDISCON_W<'a> {
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
#[doc = "Field `GNPINNAKSTS` reader - Global Non-periodic IN NAK Status"]
pub struct GNPINNAKSTS_R(crate::FieldReader<bool, bool>);
impl GNPINNAKSTS_R {
    pub(crate) fn new(bits: bool) -> Self {
        GNPINNAKSTS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GNPINNAKSTS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GOUTNAKSTS` reader - Global OUT NAK Status"]
pub struct GOUTNAKSTS_R(crate::FieldReader<bool, bool>);
impl GOUTNAKSTS_R {
    pub(crate) fn new(bits: bool) -> Self {
        GOUTNAKSTS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GOUTNAKSTS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Test Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TSTCTL_A {
    #[doc = "0: Test mode disabled."]
    DISABLE = 0,
    #[doc = "1: Test_J mode."]
    J = 1,
    #[doc = "2: Test_K mode."]
    K = 2,
    #[doc = "3: Test_SE0_NAK mode."]
    SE0NAK = 3,
    #[doc = "4: Test_Packet mode."]
    PACKET = 4,
    #[doc = "5: Test_Force_Enable."]
    FORCE = 5,
}
impl From<TSTCTL_A> for u8 {
    #[inline(always)]
    fn from(variant: TSTCTL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `TSTCTL` reader - Test Control"]
pub struct TSTCTL_R(crate::FieldReader<u8, TSTCTL_A>);
impl TSTCTL_R {
    pub(crate) fn new(bits: u8) -> Self {
        TSTCTL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TSTCTL_A> {
        match self.bits {
            0 => Some(TSTCTL_A::DISABLE),
            1 => Some(TSTCTL_A::J),
            2 => Some(TSTCTL_A::K),
            3 => Some(TSTCTL_A::SE0NAK),
            4 => Some(TSTCTL_A::PACKET),
            5 => Some(TSTCTL_A::FORCE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == TSTCTL_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `J`"]
    #[inline(always)]
    pub fn is_j(&self) -> bool {
        **self == TSTCTL_A::J
    }
    #[doc = "Checks if the value of the field is `K`"]
    #[inline(always)]
    pub fn is_k(&self) -> bool {
        **self == TSTCTL_A::K
    }
    #[doc = "Checks if the value of the field is `SE0NAK`"]
    #[inline(always)]
    pub fn is_se0nak(&self) -> bool {
        **self == TSTCTL_A::SE0NAK
    }
    #[doc = "Checks if the value of the field is `PACKET`"]
    #[inline(always)]
    pub fn is_packet(&self) -> bool {
        **self == TSTCTL_A::PACKET
    }
    #[doc = "Checks if the value of the field is `FORCE`"]
    #[inline(always)]
    pub fn is_force(&self) -> bool {
        **self == TSTCTL_A::FORCE
    }
}
impl core::ops::Deref for TSTCTL_R {
    type Target = crate::FieldReader<u8, TSTCTL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TSTCTL` writer - Test Control"]
pub struct TSTCTL_W<'a> {
    w: &'a mut W,
}
impl<'a> TSTCTL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TSTCTL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Test mode disabled."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(TSTCTL_A::DISABLE)
    }
    #[doc = "Test_J mode."]
    #[inline(always)]
    pub fn j(self) -> &'a mut W {
        self.variant(TSTCTL_A::J)
    }
    #[doc = "Test_K mode."]
    #[inline(always)]
    pub fn k(self) -> &'a mut W {
        self.variant(TSTCTL_A::K)
    }
    #[doc = "Test_SE0_NAK mode."]
    #[inline(always)]
    pub fn se0nak(self) -> &'a mut W {
        self.variant(TSTCTL_A::SE0NAK)
    }
    #[doc = "Test_Packet mode."]
    #[inline(always)]
    pub fn packet(self) -> &'a mut W {
        self.variant(TSTCTL_A::PACKET)
    }
    #[doc = "Test_Force_Enable."]
    #[inline(always)]
    pub fn force(self) -> &'a mut W {
        self.variant(TSTCTL_A::FORCE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | ((value as u32 & 0x07) << 4);
        self.w
    }
}
#[doc = "Field `SGNPINNAK` writer - Set Global Non-periodic IN NAK"]
pub struct SGNPINNAK_W<'a> {
    w: &'a mut W,
}
impl<'a> SGNPINNAK_W<'a> {
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
#[doc = "Field `CGNPINNAK` writer - Clear Global Non-periodic IN NAK"]
pub struct CGNPINNAK_W<'a> {
    w: &'a mut W,
}
impl<'a> CGNPINNAK_W<'a> {
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
#[doc = "Field `SGOUTNAK` writer - Set Global OUT NAK"]
pub struct SGOUTNAK_W<'a> {
    w: &'a mut W,
}
impl<'a> SGOUTNAK_W<'a> {
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
#[doc = "Field `CGOUTNAK` writer - Clear Global OUT NAK"]
pub struct CGOUTNAK_W<'a> {
    w: &'a mut W,
}
impl<'a> CGOUTNAK_W<'a> {
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
#[doc = "Field `PWRONPRGDONE` reader - Power-On Programming Done"]
pub struct PWRONPRGDONE_R(crate::FieldReader<bool, bool>);
impl PWRONPRGDONE_R {
    pub(crate) fn new(bits: bool) -> Self {
        PWRONPRGDONE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PWRONPRGDONE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PWRONPRGDONE` writer - Power-On Programming Done"]
pub struct PWRONPRGDONE_W<'a> {
    w: &'a mut W,
}
impl<'a> PWRONPRGDONE_W<'a> {
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
#[doc = "Field `IGNRFRMNUM` reader - Ignore Frame number For Isochronous End points"]
pub struct IGNRFRMNUM_R(crate::FieldReader<bool, bool>);
impl IGNRFRMNUM_R {
    pub(crate) fn new(bits: bool) -> Self {
        IGNRFRMNUM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IGNRFRMNUM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IGNRFRMNUM` writer - Ignore Frame number For Isochronous End points"]
pub struct IGNRFRMNUM_W<'a> {
    w: &'a mut W,
}
impl<'a> IGNRFRMNUM_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | ((value as u32 & 0x01) << 15);
        self.w
    }
}
#[doc = "Field `NAKONBBLE` reader - NAK on Babble Error"]
pub struct NAKONBBLE_R(crate::FieldReader<bool, bool>);
impl NAKONBBLE_R {
    pub(crate) fn new(bits: bool) -> Self {
        NAKONBBLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NAKONBBLE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NAKONBBLE` writer - NAK on Babble Error"]
pub struct NAKONBBLE_W<'a> {
    w: &'a mut W,
}
impl<'a> NAKONBBLE_W<'a> {
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
    #[doc = "Bit 0 - Remote Wakeup Signaling"]
    #[inline(always)]
    pub fn rmtwkupsig(&self) -> RMTWKUPSIG_R {
        RMTWKUPSIG_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Soft Disconnect"]
    #[inline(always)]
    pub fn sftdiscon(&self) -> SFTDISCON_R {
        SFTDISCON_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Global Non-periodic IN NAK Status"]
    #[inline(always)]
    pub fn gnpinnaksts(&self) -> GNPINNAKSTS_R {
        GNPINNAKSTS_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Global OUT NAK Status"]
    #[inline(always)]
    pub fn goutnaksts(&self) -> GOUTNAKSTS_R {
        GOUTNAKSTS_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 4:6 - Test Control"]
    #[inline(always)]
    pub fn tstctl(&self) -> TSTCTL_R {
        TSTCTL_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    #[doc = "Bit 11 - Power-On Programming Done"]
    #[inline(always)]
    pub fn pwronprgdone(&self) -> PWRONPRGDONE_R {
        PWRONPRGDONE_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Ignore Frame number For Isochronous End points"]
    #[inline(always)]
    pub fn ignrfrmnum(&self) -> IGNRFRMNUM_R {
        IGNRFRMNUM_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - NAK on Babble Error"]
    #[inline(always)]
    pub fn nakonbble(&self) -> NAKONBBLE_R {
        NAKONBBLE_R::new(((self.bits >> 16) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Remote Wakeup Signaling"]
    #[inline(always)]
    pub fn rmtwkupsig(&mut self) -> RMTWKUPSIG_W {
        RMTWKUPSIG_W { w: self }
    }
    #[doc = "Bit 1 - Soft Disconnect"]
    #[inline(always)]
    pub fn sftdiscon(&mut self) -> SFTDISCON_W {
        SFTDISCON_W { w: self }
    }
    #[doc = "Bits 4:6 - Test Control"]
    #[inline(always)]
    pub fn tstctl(&mut self) -> TSTCTL_W {
        TSTCTL_W { w: self }
    }
    #[doc = "Bit 7 - Set Global Non-periodic IN NAK"]
    #[inline(always)]
    pub fn sgnpinnak(&mut self) -> SGNPINNAK_W {
        SGNPINNAK_W { w: self }
    }
    #[doc = "Bit 8 - Clear Global Non-periodic IN NAK"]
    #[inline(always)]
    pub fn cgnpinnak(&mut self) -> CGNPINNAK_W {
        CGNPINNAK_W { w: self }
    }
    #[doc = "Bit 9 - Set Global OUT NAK"]
    #[inline(always)]
    pub fn sgoutnak(&mut self) -> SGOUTNAK_W {
        SGOUTNAK_W { w: self }
    }
    #[doc = "Bit 10 - Clear Global OUT NAK"]
    #[inline(always)]
    pub fn cgoutnak(&mut self) -> CGOUTNAK_W {
        CGOUTNAK_W { w: self }
    }
    #[doc = "Bit 11 - Power-On Programming Done"]
    #[inline(always)]
    pub fn pwronprgdone(&mut self) -> PWRONPRGDONE_W {
        PWRONPRGDONE_W { w: self }
    }
    #[doc = "Bit 15 - Ignore Frame number For Isochronous End points"]
    #[inline(always)]
    pub fn ignrfrmnum(&mut self) -> IGNRFRMNUM_W {
        IGNRFRMNUM_W { w: self }
    }
    #[doc = "Bit 16 - NAK on Babble Error"]
    #[inline(always)]
    pub fn nakonbble(&mut self) -> NAKONBBLE_W {
        NAKONBBLE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Device Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dctl](index.html) module"]
pub struct DCTL_SPEC;
impl crate::RegisterSpec for DCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dctl::R](R) reader structure"]
impl crate::Readable for DCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dctl::W](W) writer structure"]
impl crate::Writable for DCTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DCTL to value 0x02"]
impl crate::Resettable for DCTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x02
    }
}

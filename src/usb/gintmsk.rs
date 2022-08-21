#[doc = "Register `GINTMSK` reader"]
pub struct R(crate::R<GINTMSK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GINTMSK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GINTMSK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GINTMSK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GINTMSK` writer"]
pub struct W(crate::W<GINTMSK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GINTMSK_SPEC>;
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
impl From<crate::W<GINTMSK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GINTMSK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MODEMISMSK` reader - Mode Mismatch Interrupt Mask"]
pub type MODEMISMSK_R = crate::BitReader<bool>;
#[doc = "Field `MODEMISMSK` writer - Mode Mismatch Interrupt Mask"]
pub type MODEMISMSK_W<'a, const O: u8> = crate::BitWriter<'a, u32, GINTMSK_SPEC, bool, O>;
#[doc = "Field `SOFMSK` reader - Start of Frame Mask"]
pub type SOFMSK_R = crate::BitReader<bool>;
#[doc = "Field `SOFMSK` writer - Start of Frame Mask"]
pub type SOFMSK_W<'a, const O: u8> = crate::BitWriter<'a, u32, GINTMSK_SPEC, bool, O>;
#[doc = "Field `RXFLVLMSK` reader - Receive FIFO Non-Empty Mask"]
pub type RXFLVLMSK_R = crate::BitReader<bool>;
#[doc = "Field `RXFLVLMSK` writer - Receive FIFO Non-Empty Mask"]
pub type RXFLVLMSK_W<'a, const O: u8> = crate::BitWriter<'a, u32, GINTMSK_SPEC, bool, O>;
#[doc = "Field `GINNAKEFFMSK` reader - Global Non-periodic IN NAK Effective Mask"]
pub type GINNAKEFFMSK_R = crate::BitReader<bool>;
#[doc = "Field `GINNAKEFFMSK` writer - Global Non-periodic IN NAK Effective Mask"]
pub type GINNAKEFFMSK_W<'a, const O: u8> = crate::BitWriter<'a, u32, GINTMSK_SPEC, bool, O>;
#[doc = "Field `GOUTNAKEFFMSK` reader - Global OUT NAK Effective Mask"]
pub type GOUTNAKEFFMSK_R = crate::BitReader<bool>;
#[doc = "Field `GOUTNAKEFFMSK` writer - Global OUT NAK Effective Mask"]
pub type GOUTNAKEFFMSK_W<'a, const O: u8> = crate::BitWriter<'a, u32, GINTMSK_SPEC, bool, O>;
#[doc = "Field `ERLYSUSPMSK` reader - Early Suspend Mask"]
pub type ERLYSUSPMSK_R = crate::BitReader<bool>;
#[doc = "Field `ERLYSUSPMSK` writer - Early Suspend Mask"]
pub type ERLYSUSPMSK_W<'a, const O: u8> = crate::BitWriter<'a, u32, GINTMSK_SPEC, bool, O>;
#[doc = "Field `USBSUSPMSK` reader - USB Suspend Mask"]
pub type USBSUSPMSK_R = crate::BitReader<bool>;
#[doc = "Field `USBSUSPMSK` writer - USB Suspend Mask"]
pub type USBSUSPMSK_W<'a, const O: u8> = crate::BitWriter<'a, u32, GINTMSK_SPEC, bool, O>;
#[doc = "Field `USBRSTMSK` reader - USB Reset Mask"]
pub type USBRSTMSK_R = crate::BitReader<bool>;
#[doc = "Field `USBRSTMSK` writer - USB Reset Mask"]
pub type USBRSTMSK_W<'a, const O: u8> = crate::BitWriter<'a, u32, GINTMSK_SPEC, bool, O>;
#[doc = "Field `ENUMDONEMSK` reader - Enumeration Done Mask"]
pub type ENUMDONEMSK_R = crate::BitReader<bool>;
#[doc = "Field `ENUMDONEMSK` writer - Enumeration Done Mask"]
pub type ENUMDONEMSK_W<'a, const O: u8> = crate::BitWriter<'a, u32, GINTMSK_SPEC, bool, O>;
#[doc = "Field `ISOOUTDROPMSK` reader - Isochronous OUT Packet Dropped Interrupt Mask"]
pub type ISOOUTDROPMSK_R = crate::BitReader<bool>;
#[doc = "Field `ISOOUTDROPMSK` writer - Isochronous OUT Packet Dropped Interrupt Mask"]
pub type ISOOUTDROPMSK_W<'a, const O: u8> = crate::BitWriter<'a, u32, GINTMSK_SPEC, bool, O>;
#[doc = "Field `EOPFMSK` reader - End of Periodic Frame Interrupt Mask"]
pub type EOPFMSK_R = crate::BitReader<bool>;
#[doc = "Field `EOPFMSK` writer - End of Periodic Frame Interrupt Mask"]
pub type EOPFMSK_W<'a, const O: u8> = crate::BitWriter<'a, u32, GINTMSK_SPEC, bool, O>;
#[doc = "Field `IEPINTMSK` reader - IN Endpoints Interrupt Mask"]
pub type IEPINTMSK_R = crate::BitReader<bool>;
#[doc = "Field `IEPINTMSK` writer - IN Endpoints Interrupt Mask"]
pub type IEPINTMSK_W<'a, const O: u8> = crate::BitWriter<'a, u32, GINTMSK_SPEC, bool, O>;
#[doc = "Field `OEPINTMSK` reader - OUT Endpoints Interrupt Mask"]
pub type OEPINTMSK_R = crate::BitReader<bool>;
#[doc = "Field `OEPINTMSK` writer - OUT Endpoints Interrupt Mask"]
pub type OEPINTMSK_W<'a, const O: u8> = crate::BitWriter<'a, u32, GINTMSK_SPEC, bool, O>;
#[doc = "Field `INCOMPISOINMSK` reader - Incomplete Isochronous IN Transfer Mask"]
pub type INCOMPISOINMSK_R = crate::BitReader<bool>;
#[doc = "Field `INCOMPISOINMSK` writer - Incomplete Isochronous IN Transfer Mask"]
pub type INCOMPISOINMSK_W<'a, const O: u8> = crate::BitWriter<'a, u32, GINTMSK_SPEC, bool, O>;
#[doc = "Field `INCOMPLPMSK` reader - Incomplete Periodic Transfer Mask"]
pub type INCOMPLPMSK_R = crate::BitReader<bool>;
#[doc = "Field `INCOMPLPMSK` writer - Incomplete Periodic Transfer Mask"]
pub type INCOMPLPMSK_W<'a, const O: u8> = crate::BitWriter<'a, u32, GINTMSK_SPEC, bool, O>;
#[doc = "Field `FETSUSPMSK` reader - Data Fetch Suspended Mask"]
pub type FETSUSPMSK_R = crate::BitReader<bool>;
#[doc = "Field `FETSUSPMSK` writer - Data Fetch Suspended Mask"]
pub type FETSUSPMSK_W<'a, const O: u8> = crate::BitWriter<'a, u32, GINTMSK_SPEC, bool, O>;
#[doc = "Field `RESETDETMSK` reader - Reset detected Interrupt Mask"]
pub type RESETDETMSK_R = crate::BitReader<bool>;
#[doc = "Field `RESETDETMSK` writer - Reset detected Interrupt Mask"]
pub type RESETDETMSK_W<'a, const O: u8> = crate::BitWriter<'a, u32, GINTMSK_SPEC, bool, O>;
#[doc = "Field `WKUPINTMSK` reader - Resume/Remote Wakeup Detected Interrupt Mask"]
pub type WKUPINTMSK_R = crate::BitReader<bool>;
#[doc = "Field `WKUPINTMSK` writer - Resume/Remote Wakeup Detected Interrupt Mask"]
pub type WKUPINTMSK_W<'a, const O: u8> = crate::BitWriter<'a, u32, GINTMSK_SPEC, bool, O>;
impl R {
    #[doc = "Bit 1 - Mode Mismatch Interrupt Mask"]
    #[inline(always)]
    pub fn modemismsk(&self) -> MODEMISMSK_R {
        MODEMISMSK_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - Start of Frame Mask"]
    #[inline(always)]
    pub fn sofmsk(&self) -> SOFMSK_R {
        SOFMSK_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Receive FIFO Non-Empty Mask"]
    #[inline(always)]
    pub fn rxflvlmsk(&self) -> RXFLVLMSK_R {
        RXFLVLMSK_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - Global Non-periodic IN NAK Effective Mask"]
    #[inline(always)]
    pub fn ginnakeffmsk(&self) -> GINNAKEFFMSK_R {
        GINNAKEFFMSK_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Global OUT NAK Effective Mask"]
    #[inline(always)]
    pub fn goutnakeffmsk(&self) -> GOUTNAKEFFMSK_R {
        GOUTNAKEFFMSK_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 10 - Early Suspend Mask"]
    #[inline(always)]
    pub fn erlysuspmsk(&self) -> ERLYSUSPMSK_R {
        ERLYSUSPMSK_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - USB Suspend Mask"]
    #[inline(always)]
    pub fn usbsuspmsk(&self) -> USBSUSPMSK_R {
        USBSUSPMSK_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - USB Reset Mask"]
    #[inline(always)]
    pub fn usbrstmsk(&self) -> USBRSTMSK_R {
        USBRSTMSK_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Enumeration Done Mask"]
    #[inline(always)]
    pub fn enumdonemsk(&self) -> ENUMDONEMSK_R {
        ENUMDONEMSK_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Isochronous OUT Packet Dropped Interrupt Mask"]
    #[inline(always)]
    pub fn isooutdropmsk(&self) -> ISOOUTDROPMSK_R {
        ISOOUTDROPMSK_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - End of Periodic Frame Interrupt Mask"]
    #[inline(always)]
    pub fn eopfmsk(&self) -> EOPFMSK_R {
        EOPFMSK_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 18 - IN Endpoints Interrupt Mask"]
    #[inline(always)]
    pub fn iepintmsk(&self) -> IEPINTMSK_R {
        IEPINTMSK_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - OUT Endpoints Interrupt Mask"]
    #[inline(always)]
    pub fn oepintmsk(&self) -> OEPINTMSK_R {
        OEPINTMSK_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Incomplete Isochronous IN Transfer Mask"]
    #[inline(always)]
    pub fn incompisoinmsk(&self) -> INCOMPISOINMSK_R {
        INCOMPISOINMSK_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Incomplete Periodic Transfer Mask"]
    #[inline(always)]
    pub fn incomplpmsk(&self) -> INCOMPLPMSK_R {
        INCOMPLPMSK_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Data Fetch Suspended Mask"]
    #[inline(always)]
    pub fn fetsuspmsk(&self) -> FETSUSPMSK_R {
        FETSUSPMSK_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Reset detected Interrupt Mask"]
    #[inline(always)]
    pub fn resetdetmsk(&self) -> RESETDETMSK_R {
        RESETDETMSK_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 31 - Resume/Remote Wakeup Detected Interrupt Mask"]
    #[inline(always)]
    pub fn wkupintmsk(&self) -> WKUPINTMSK_R {
        WKUPINTMSK_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Mode Mismatch Interrupt Mask"]
    #[inline(always)]
    pub fn modemismsk(&mut self) -> MODEMISMSK_W<1> {
        MODEMISMSK_W::new(self)
    }
    #[doc = "Bit 3 - Start of Frame Mask"]
    #[inline(always)]
    pub fn sofmsk(&mut self) -> SOFMSK_W<3> {
        SOFMSK_W::new(self)
    }
    #[doc = "Bit 4 - Receive FIFO Non-Empty Mask"]
    #[inline(always)]
    pub fn rxflvlmsk(&mut self) -> RXFLVLMSK_W<4> {
        RXFLVLMSK_W::new(self)
    }
    #[doc = "Bit 6 - Global Non-periodic IN NAK Effective Mask"]
    #[inline(always)]
    pub fn ginnakeffmsk(&mut self) -> GINNAKEFFMSK_W<6> {
        GINNAKEFFMSK_W::new(self)
    }
    #[doc = "Bit 7 - Global OUT NAK Effective Mask"]
    #[inline(always)]
    pub fn goutnakeffmsk(&mut self) -> GOUTNAKEFFMSK_W<7> {
        GOUTNAKEFFMSK_W::new(self)
    }
    #[doc = "Bit 10 - Early Suspend Mask"]
    #[inline(always)]
    pub fn erlysuspmsk(&mut self) -> ERLYSUSPMSK_W<10> {
        ERLYSUSPMSK_W::new(self)
    }
    #[doc = "Bit 11 - USB Suspend Mask"]
    #[inline(always)]
    pub fn usbsuspmsk(&mut self) -> USBSUSPMSK_W<11> {
        USBSUSPMSK_W::new(self)
    }
    #[doc = "Bit 12 - USB Reset Mask"]
    #[inline(always)]
    pub fn usbrstmsk(&mut self) -> USBRSTMSK_W<12> {
        USBRSTMSK_W::new(self)
    }
    #[doc = "Bit 13 - Enumeration Done Mask"]
    #[inline(always)]
    pub fn enumdonemsk(&mut self) -> ENUMDONEMSK_W<13> {
        ENUMDONEMSK_W::new(self)
    }
    #[doc = "Bit 14 - Isochronous OUT Packet Dropped Interrupt Mask"]
    #[inline(always)]
    pub fn isooutdropmsk(&mut self) -> ISOOUTDROPMSK_W<14> {
        ISOOUTDROPMSK_W::new(self)
    }
    #[doc = "Bit 15 - End of Periodic Frame Interrupt Mask"]
    #[inline(always)]
    pub fn eopfmsk(&mut self) -> EOPFMSK_W<15> {
        EOPFMSK_W::new(self)
    }
    #[doc = "Bit 18 - IN Endpoints Interrupt Mask"]
    #[inline(always)]
    pub fn iepintmsk(&mut self) -> IEPINTMSK_W<18> {
        IEPINTMSK_W::new(self)
    }
    #[doc = "Bit 19 - OUT Endpoints Interrupt Mask"]
    #[inline(always)]
    pub fn oepintmsk(&mut self) -> OEPINTMSK_W<19> {
        OEPINTMSK_W::new(self)
    }
    #[doc = "Bit 20 - Incomplete Isochronous IN Transfer Mask"]
    #[inline(always)]
    pub fn incompisoinmsk(&mut self) -> INCOMPISOINMSK_W<20> {
        INCOMPISOINMSK_W::new(self)
    }
    #[doc = "Bit 21 - Incomplete Periodic Transfer Mask"]
    #[inline(always)]
    pub fn incomplpmsk(&mut self) -> INCOMPLPMSK_W<21> {
        INCOMPLPMSK_W::new(self)
    }
    #[doc = "Bit 22 - Data Fetch Suspended Mask"]
    #[inline(always)]
    pub fn fetsuspmsk(&mut self) -> FETSUSPMSK_W<22> {
        FETSUSPMSK_W::new(self)
    }
    #[doc = "Bit 23 - Reset detected Interrupt Mask"]
    #[inline(always)]
    pub fn resetdetmsk(&mut self) -> RESETDETMSK_W<23> {
        RESETDETMSK_W::new(self)
    }
    #[doc = "Bit 31 - Resume/Remote Wakeup Detected Interrupt Mask"]
    #[inline(always)]
    pub fn wkupintmsk(&mut self) -> WKUPINTMSK_W<31> {
        WKUPINTMSK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gintmsk](index.html) module"]
pub struct GINTMSK_SPEC;
impl crate::RegisterSpec for GINTMSK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gintmsk::R](R) reader structure"]
impl crate::Readable for GINTMSK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gintmsk::W](W) writer structure"]
impl crate::Writable for GINTMSK_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets GINTMSK to value 0"]
impl crate::Resettable for GINTMSK_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

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
pub type DMPUAP_R = crate::BitReader<bool>;
#[doc = "Field `DMPUAP` writer - DMPU Active Polarity"]
pub type DMPUAP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `LEMOSCCTRL` reader - Low Energy Mode Oscillator Control"]
pub type LEMOSCCTRL_R = crate::FieldReader<u8, LEMOSCCTRL_A>;
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
impl LEMOSCCTRL_R {
    #[doc = "Get enumerated values variant"]
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
        *self == LEMOSCCTRL_A::NONE
    }
    #[doc = "Checks if the value of the field is `GATE`"]
    #[inline(always)]
    pub fn is_gate(&self) -> bool {
        *self == LEMOSCCTRL_A::GATE
    }
    #[doc = "Checks if the value of the field is `SUSPEND`"]
    #[inline(always)]
    pub fn is_suspend(&self) -> bool {
        *self == LEMOSCCTRL_A::SUSPEND
    }
}
#[doc = "Field `LEMOSCCTRL` writer - Low Energy Mode Oscillator Control"]
pub type LEMOSCCTRL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CTRL_SPEC, u8, LEMOSCCTRL_A, 2, O>;
impl<'a, const O: u8> LEMOSCCTRL_W<'a, O> {
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
}
#[doc = "Field `LEMPHYCTRL` reader - Low Energy Mode USB PHY Control"]
pub type LEMPHYCTRL_R = crate::BitReader<bool>;
#[doc = "Field `LEMPHYCTRL` writer - Low Energy Mode USB PHY Control"]
pub type LEMPHYCTRL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `LEMIDLEEN` reader - Low Energy Mode on Bus Idle Enable"]
pub type LEMIDLEEN_R = crate::BitReader<bool>;
#[doc = "Field `LEMIDLEEN` writer - Low Energy Mode on Bus Idle Enable"]
pub type LEMIDLEEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `LEMNAKEN` reader - Low Energy Mode on OUT NAK Enable"]
pub type LEMNAKEN_R = crate::BitReader<bool>;
#[doc = "Field `LEMNAKEN` writer - Low Energy Mode on OUT NAK Enable"]
pub type LEMNAKEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `LEMADDRMEN` reader - Low Energy Mode on Device Address Mismatch Enable"]
pub type LEMADDRMEN_R = crate::BitReader<bool>;
#[doc = "Field `LEMADDRMEN` writer - Low Energy Mode on Device Address Mismatch Enable"]
pub type LEMADDRMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `VREGDIS` reader - Voltage Regulator Disable"]
pub type VREGDIS_R = crate::BitReader<bool>;
#[doc = "Field `VREGDIS` writer - Voltage Regulator Disable"]
pub type VREGDIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `VREGOSEN` reader - VREGO Sense Enable"]
pub type VREGOSEN_R = crate::BitReader<bool>;
#[doc = "Field `VREGOSEN` writer - VREGO Sense Enable"]
pub type VREGOSEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `BIASPROGEM01` reader - Regulator Bias Programming Value in EM0/1"]
pub type BIASPROGEM01_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BIASPROGEM01` writer - Regulator Bias Programming Value in EM0/1"]
pub type BIASPROGEM01_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTRL_SPEC, u8, u8, 2, O>;
#[doc = "Field `BIASPROGEM23` reader - Regulator Bias Programming Value in EM2/3"]
pub type BIASPROGEM23_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BIASPROGEM23` writer - Regulator Bias Programming Value in EM2/3"]
pub type BIASPROGEM23_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTRL_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bit 1 - DMPU Active Polarity"]
    #[inline(always)]
    pub fn dmpuap(&self) -> DMPUAP_R {
        DMPUAP_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 4:5 - Low Energy Mode Oscillator Control"]
    #[inline(always)]
    pub fn lemoscctrl(&self) -> LEMOSCCTRL_R {
        LEMOSCCTRL_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 7 - Low Energy Mode USB PHY Control"]
    #[inline(always)]
    pub fn lemphyctrl(&self) -> LEMPHYCTRL_R {
        LEMPHYCTRL_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 9 - Low Energy Mode on Bus Idle Enable"]
    #[inline(always)]
    pub fn lemidleen(&self) -> LEMIDLEEN_R {
        LEMIDLEEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Low Energy Mode on OUT NAK Enable"]
    #[inline(always)]
    pub fn lemnaken(&self) -> LEMNAKEN_R {
        LEMNAKEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Low Energy Mode on Device Address Mismatch Enable"]
    #[inline(always)]
    pub fn lemaddrmen(&self) -> LEMADDRMEN_R {
        LEMADDRMEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 16 - Voltage Regulator Disable"]
    #[inline(always)]
    pub fn vregdis(&self) -> VREGDIS_R {
        VREGDIS_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - VREGO Sense Enable"]
    #[inline(always)]
    pub fn vregosen(&self) -> VREGOSEN_R {
        VREGOSEN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 20:21 - Regulator Bias Programming Value in EM0/1"]
    #[inline(always)]
    pub fn biasprogem01(&self) -> BIASPROGEM01_R {
        BIASPROGEM01_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 24:25 - Regulator Bias Programming Value in EM2/3"]
    #[inline(always)]
    pub fn biasprogem23(&self) -> BIASPROGEM23_R {
        BIASPROGEM23_R::new(((self.bits >> 24) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 1 - DMPU Active Polarity"]
    #[inline(always)]
    pub fn dmpuap(&mut self) -> DMPUAP_W<1> {
        DMPUAP_W::new(self)
    }
    #[doc = "Bits 4:5 - Low Energy Mode Oscillator Control"]
    #[inline(always)]
    pub fn lemoscctrl(&mut self) -> LEMOSCCTRL_W<4> {
        LEMOSCCTRL_W::new(self)
    }
    #[doc = "Bit 7 - Low Energy Mode USB PHY Control"]
    #[inline(always)]
    pub fn lemphyctrl(&mut self) -> LEMPHYCTRL_W<7> {
        LEMPHYCTRL_W::new(self)
    }
    #[doc = "Bit 9 - Low Energy Mode on Bus Idle Enable"]
    #[inline(always)]
    pub fn lemidleen(&mut self) -> LEMIDLEEN_W<9> {
        LEMIDLEEN_W::new(self)
    }
    #[doc = "Bit 10 - Low Energy Mode on OUT NAK Enable"]
    #[inline(always)]
    pub fn lemnaken(&mut self) -> LEMNAKEN_W<10> {
        LEMNAKEN_W::new(self)
    }
    #[doc = "Bit 11 - Low Energy Mode on Device Address Mismatch Enable"]
    #[inline(always)]
    pub fn lemaddrmen(&mut self) -> LEMADDRMEN_W<11> {
        LEMADDRMEN_W::new(self)
    }
    #[doc = "Bit 16 - Voltage Regulator Disable"]
    #[inline(always)]
    pub fn vregdis(&mut self) -> VREGDIS_W<16> {
        VREGDIS_W::new(self)
    }
    #[doc = "Bit 17 - VREGO Sense Enable"]
    #[inline(always)]
    pub fn vregosen(&mut self) -> VREGOSEN_W<17> {
        VREGOSEN_W::new(self)
    }
    #[doc = "Bits 20:21 - Regulator Bias Programming Value in EM0/1"]
    #[inline(always)]
    pub fn biasprogem01(&mut self) -> BIASPROGEM01_W<20> {
        BIASPROGEM01_W::new(self)
    }
    #[doc = "Bits 24:25 - Regulator Bias Programming Value in EM2/3"]
    #[inline(always)]
    pub fn biasprogem23(&mut self) -> BIASPROGEM23_W<24> {
        BIASPROGEM23_W::new(self)
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

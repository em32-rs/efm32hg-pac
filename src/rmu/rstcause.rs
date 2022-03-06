#[doc = "Register `RSTCAUSE` reader"]
pub struct R(crate::R<RSTCAUSE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RSTCAUSE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RSTCAUSE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RSTCAUSE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `PORST` reader - Power On Reset"]
pub struct PORST_R(crate::FieldReader<bool, bool>);
impl PORST_R {
    pub(crate) fn new(bits: bool) -> Self {
        PORST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PORST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BODUNREGRST` reader - Brown Out Detector Unregulated Domain Reset"]
pub struct BODUNREGRST_R(crate::FieldReader<bool, bool>);
impl BODUNREGRST_R {
    pub(crate) fn new(bits: bool) -> Self {
        BODUNREGRST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BODUNREGRST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BODREGRST` reader - Brown Out Detector Regulated Domain Reset"]
pub struct BODREGRST_R(crate::FieldReader<bool, bool>);
impl BODREGRST_R {
    pub(crate) fn new(bits: bool) -> Self {
        BODREGRST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BODREGRST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EXTRST` reader - External Pin Reset"]
pub struct EXTRST_R(crate::FieldReader<bool, bool>);
impl EXTRST_R {
    pub(crate) fn new(bits: bool) -> Self {
        EXTRST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EXTRST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WDOGRST` reader - Watchdog Reset"]
pub struct WDOGRST_R(crate::FieldReader<bool, bool>);
impl WDOGRST_R {
    pub(crate) fn new(bits: bool) -> Self {
        WDOGRST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WDOGRST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LOCKUPRST` reader - LOCKUP Reset"]
pub struct LOCKUPRST_R(crate::FieldReader<bool, bool>);
impl LOCKUPRST_R {
    pub(crate) fn new(bits: bool) -> Self {
        LOCKUPRST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LOCKUPRST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SYSREQRST` reader - System Request Reset"]
pub struct SYSREQRST_R(crate::FieldReader<bool, bool>);
impl SYSREQRST_R {
    pub(crate) fn new(bits: bool) -> Self {
        SYSREQRST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SYSREQRST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EM4RST` reader - EM4 Reset"]
pub struct EM4RST_R(crate::FieldReader<bool, bool>);
impl EM4RST_R {
    pub(crate) fn new(bits: bool) -> Self {
        EM4RST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EM4RST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EM4WURST` reader - EM4 Wake-up Reset"]
pub struct EM4WURST_R(crate::FieldReader<bool, bool>);
impl EM4WURST_R {
    pub(crate) fn new(bits: bool) -> Self {
        EM4WURST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EM4WURST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BODAVDD0` reader - AVDD0 Bod Reset"]
pub struct BODAVDD0_R(crate::FieldReader<bool, bool>);
impl BODAVDD0_R {
    pub(crate) fn new(bits: bool) -> Self {
        BODAVDD0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BODAVDD0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BODAVDD1` reader - AVDD1 Bod Reset"]
pub struct BODAVDD1_R(crate::FieldReader<bool, bool>);
impl BODAVDD1_R {
    pub(crate) fn new(bits: bool) -> Self {
        BODAVDD1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BODAVDD1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Power On Reset"]
    #[inline(always)]
    pub fn porst(&self) -> PORST_R {
        PORST_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Brown Out Detector Unregulated Domain Reset"]
    #[inline(always)]
    pub fn bodunregrst(&self) -> BODUNREGRST_R {
        BODUNREGRST_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Brown Out Detector Regulated Domain Reset"]
    #[inline(always)]
    pub fn bodregrst(&self) -> BODREGRST_R {
        BODREGRST_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - External Pin Reset"]
    #[inline(always)]
    pub fn extrst(&self) -> EXTRST_R {
        EXTRST_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Watchdog Reset"]
    #[inline(always)]
    pub fn wdogrst(&self) -> WDOGRST_R {
        WDOGRST_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - LOCKUP Reset"]
    #[inline(always)]
    pub fn lockuprst(&self) -> LOCKUPRST_R {
        LOCKUPRST_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - System Request Reset"]
    #[inline(always)]
    pub fn sysreqrst(&self) -> SYSREQRST_R {
        SYSREQRST_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - EM4 Reset"]
    #[inline(always)]
    pub fn em4rst(&self) -> EM4RST_R {
        EM4RST_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - EM4 Wake-up Reset"]
    #[inline(always)]
    pub fn em4wurst(&self) -> EM4WURST_R {
        EM4WURST_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - AVDD0 Bod Reset"]
    #[inline(always)]
    pub fn bodavdd0(&self) -> BODAVDD0_R {
        BODAVDD0_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - AVDD1 Bod Reset"]
    #[inline(always)]
    pub fn bodavdd1(&self) -> BODAVDD1_R {
        BODAVDD1_R::new(((self.bits >> 10) & 0x01) != 0)
    }
}
#[doc = "Reset Cause Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rstcause](index.html) module"]
pub struct RSTCAUSE_SPEC;
impl crate::RegisterSpec for RSTCAUSE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rstcause::R](R) reader structure"]
impl crate::Readable for RSTCAUSE_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RSTCAUSE to value 0"]
impl crate::Resettable for RSTCAUSE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

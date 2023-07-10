#[doc = "Register `BDCR` reader"]
pub struct R(crate::R<BDCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BDCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BDCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BDCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BDCR` writer"]
pub struct W(crate::W<BDCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BDCR_SPEC>;
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
impl From<crate::W<BDCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BDCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LSEON` reader - External Low Speed oscillator enable"]
pub type LSEON_R = crate::BitReader<LSEON_A>;
#[doc = "External Low Speed oscillator enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LSEON_A {
    #[doc = "0: LSE oscillator Off"]
    OFF = 0,
    #[doc = "1: LSE oscillator On"]
    ON = 1,
}
impl From<LSEON_A> for bool {
    #[inline(always)]
    fn from(variant: LSEON_A) -> Self {
        variant as u8 != 0
    }
}
impl LSEON_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LSEON_A {
        match self.bits {
            false => LSEON_A::OFF,
            true => LSEON_A::ON,
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == LSEON_A::OFF
    }
    #[doc = "Checks if the value of the field is `ON`"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == LSEON_A::ON
    }
}
#[doc = "Field `LSEON` writer - External Low Speed oscillator enable"]
pub type LSEON_W<'a, const O: u8> = crate::BitWriter<'a, BDCR_SPEC, O, LSEON_A>;
impl<'a, const O: u8> LSEON_W<'a, O> {
    #[doc = "LSE oscillator Off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(LSEON_A::OFF)
    }
    #[doc = "LSE oscillator On"]
    #[inline(always)]
    pub fn on(self) -> &'a mut W {
        self.variant(LSEON_A::ON)
    }
}
#[doc = "Field `LSERDY` reader - External Low Speed oscillator ready"]
pub type LSERDY_R = crate::BitReader<LSERDYR_A>;
#[doc = "External Low Speed oscillator ready\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LSERDYR_A {
    #[doc = "0: LSE oscillator not ready"]
    NOT_READY = 0,
    #[doc = "1: LSE oscillator ready"]
    READY = 1,
}
impl From<LSERDYR_A> for bool {
    #[inline(always)]
    fn from(variant: LSERDYR_A) -> Self {
        variant as u8 != 0
    }
}
impl LSERDY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LSERDYR_A {
        match self.bits {
            false => LSERDYR_A::NOT_READY,
            true => LSERDYR_A::READY,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_READY`"]
    #[inline(always)]
    pub fn is_not_ready(&self) -> bool {
        *self == LSERDYR_A::NOT_READY
    }
    #[doc = "Checks if the value of the field is `READY`"]
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == LSERDYR_A::READY
    }
}
#[doc = "Field `LSEBYP` reader - External Low Speed oscillator bypass"]
pub type LSEBYP_R = crate::BitReader<LSEBYP_A>;
#[doc = "External Low Speed oscillator bypass\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LSEBYP_A {
    #[doc = "0: LSE crystal oscillator not bypassed"]
    NOT_BYPASSED = 0,
    #[doc = "1: LSE crystal oscillator bypassed with external clock"]
    BYPASSED = 1,
}
impl From<LSEBYP_A> for bool {
    #[inline(always)]
    fn from(variant: LSEBYP_A) -> Self {
        variant as u8 != 0
    }
}
impl LSEBYP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LSEBYP_A {
        match self.bits {
            false => LSEBYP_A::NOT_BYPASSED,
            true => LSEBYP_A::BYPASSED,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_BYPASSED`"]
    #[inline(always)]
    pub fn is_not_bypassed(&self) -> bool {
        *self == LSEBYP_A::NOT_BYPASSED
    }
    #[doc = "Checks if the value of the field is `BYPASSED`"]
    #[inline(always)]
    pub fn is_bypassed(&self) -> bool {
        *self == LSEBYP_A::BYPASSED
    }
}
#[doc = "Field `LSEBYP` writer - External Low Speed oscillator bypass"]
pub type LSEBYP_W<'a, const O: u8> = crate::BitWriter<'a, BDCR_SPEC, O, LSEBYP_A>;
impl<'a, const O: u8> LSEBYP_W<'a, O> {
    #[doc = "LSE crystal oscillator not bypassed"]
    #[inline(always)]
    pub fn not_bypassed(self) -> &'a mut W {
        self.variant(LSEBYP_A::NOT_BYPASSED)
    }
    #[doc = "LSE crystal oscillator bypassed with external clock"]
    #[inline(always)]
    pub fn bypassed(self) -> &'a mut W {
        self.variant(LSEBYP_A::BYPASSED)
    }
}
#[doc = "Field `LSEDRV` reader - LSE oscillator drive capability"]
pub type LSEDRV_R = crate::FieldReader<LSEDRV_A>;
#[doc = "LSE oscillator drive capability\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LSEDRV_A {
    #[doc = "0: Low drive capacity"]
    LOW = 0,
    #[doc = "1: Medium-high drive capacity"]
    MEDIUM_HIGH = 1,
    #[doc = "2: Medium-low drive capacity"]
    MEDIUM_LOW = 2,
    #[doc = "3: High drive capacity"]
    HIGH = 3,
}
impl From<LSEDRV_A> for u8 {
    #[inline(always)]
    fn from(variant: LSEDRV_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for LSEDRV_A {
    type Ux = u8;
}
impl LSEDRV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LSEDRV_A {
        match self.bits {
            0 => LSEDRV_A::LOW,
            1 => LSEDRV_A::MEDIUM_HIGH,
            2 => LSEDRV_A::MEDIUM_LOW,
            3 => LSEDRV_A::HIGH,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == LSEDRV_A::LOW
    }
    #[doc = "Checks if the value of the field is `MEDIUM_HIGH`"]
    #[inline(always)]
    pub fn is_medium_high(&self) -> bool {
        *self == LSEDRV_A::MEDIUM_HIGH
    }
    #[doc = "Checks if the value of the field is `MEDIUM_LOW`"]
    #[inline(always)]
    pub fn is_medium_low(&self) -> bool {
        *self == LSEDRV_A::MEDIUM_LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == LSEDRV_A::HIGH
    }
}
#[doc = "Field `LSEDRV` writer - LSE oscillator drive capability"]
pub type LSEDRV_W<'a, const O: u8> = crate::FieldWriterSafe<'a, BDCR_SPEC, 2, O, LSEDRV_A>;
impl<'a, const O: u8> LSEDRV_W<'a, O> {
    #[doc = "Low drive capacity"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(LSEDRV_A::LOW)
    }
    #[doc = "Medium-high drive capacity"]
    #[inline(always)]
    pub fn medium_high(self) -> &'a mut W {
        self.variant(LSEDRV_A::MEDIUM_HIGH)
    }
    #[doc = "Medium-low drive capacity"]
    #[inline(always)]
    pub fn medium_low(self) -> &'a mut W {
        self.variant(LSEDRV_A::MEDIUM_LOW)
    }
    #[doc = "High drive capacity"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(LSEDRV_A::HIGH)
    }
}
#[doc = "Field `RTCSEL` reader - RTC clock source selection"]
pub type RTCSEL_R = crate::FieldReader<RTCSEL_A>;
#[doc = "RTC clock source selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RTCSEL_A {
    #[doc = "0: No clock"]
    NO_CLOCK = 0,
    #[doc = "1: LSE oscillator clock used as RTC clock"]
    LSE = 1,
    #[doc = "2: LSI oscillator clock used as RTC clock"]
    LSI = 2,
    #[doc = "3: HSE oscillator clock divided by a prescaler used as RTC clock"]
    HSE = 3,
}
impl From<RTCSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: RTCSEL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for RTCSEL_A {
    type Ux = u8;
}
impl RTCSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RTCSEL_A {
        match self.bits {
            0 => RTCSEL_A::NO_CLOCK,
            1 => RTCSEL_A::LSE,
            2 => RTCSEL_A::LSI,
            3 => RTCSEL_A::HSE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NO_CLOCK`"]
    #[inline(always)]
    pub fn is_no_clock(&self) -> bool {
        *self == RTCSEL_A::NO_CLOCK
    }
    #[doc = "Checks if the value of the field is `LSE`"]
    #[inline(always)]
    pub fn is_lse(&self) -> bool {
        *self == RTCSEL_A::LSE
    }
    #[doc = "Checks if the value of the field is `LSI`"]
    #[inline(always)]
    pub fn is_lsi(&self) -> bool {
        *self == RTCSEL_A::LSI
    }
    #[doc = "Checks if the value of the field is `HSE`"]
    #[inline(always)]
    pub fn is_hse(&self) -> bool {
        *self == RTCSEL_A::HSE
    }
}
#[doc = "Field `RTCSEL` writer - RTC clock source selection"]
pub type RTCSEL_W<'a, const O: u8> = crate::FieldWriterSafe<'a, BDCR_SPEC, 2, O, RTCSEL_A>;
impl<'a, const O: u8> RTCSEL_W<'a, O> {
    #[doc = "No clock"]
    #[inline(always)]
    pub fn no_clock(self) -> &'a mut W {
        self.variant(RTCSEL_A::NO_CLOCK)
    }
    #[doc = "LSE oscillator clock used as RTC clock"]
    #[inline(always)]
    pub fn lse(self) -> &'a mut W {
        self.variant(RTCSEL_A::LSE)
    }
    #[doc = "LSI oscillator clock used as RTC clock"]
    #[inline(always)]
    pub fn lsi(self) -> &'a mut W {
        self.variant(RTCSEL_A::LSI)
    }
    #[doc = "HSE oscillator clock divided by a prescaler used as RTC clock"]
    #[inline(always)]
    pub fn hse(self) -> &'a mut W {
        self.variant(RTCSEL_A::HSE)
    }
}
#[doc = "Field `RTCEN` reader - RTC clock enable"]
pub type RTCEN_R = crate::BitReader<RTCEN_A>;
#[doc = "RTC clock enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RTCEN_A {
    #[doc = "0: RTC clock disabled"]
    DISABLED = 0,
    #[doc = "1: RTC clock enabled"]
    ENABLED = 1,
}
impl From<RTCEN_A> for bool {
    #[inline(always)]
    fn from(variant: RTCEN_A) -> Self {
        variant as u8 != 0
    }
}
impl RTCEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RTCEN_A {
        match self.bits {
            false => RTCEN_A::DISABLED,
            true => RTCEN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RTCEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RTCEN_A::ENABLED
    }
}
#[doc = "Field `RTCEN` writer - RTC clock enable"]
pub type RTCEN_W<'a, const O: u8> = crate::BitWriter<'a, BDCR_SPEC, O, RTCEN_A>;
impl<'a, const O: u8> RTCEN_W<'a, O> {
    #[doc = "RTC clock disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RTCEN_A::DISABLED)
    }
    #[doc = "RTC clock enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RTCEN_A::ENABLED)
    }
}
#[doc = "Field `BDRST` reader - Backup domain software reset"]
pub type BDRST_R = crate::BitReader<BDRST_A>;
#[doc = "Backup domain software reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BDRST_A {
    #[doc = "0: Reset not activated"]
    DISABLED = 0,
    #[doc = "1: Reset the entire RTC domain"]
    ENABLED = 1,
}
impl From<BDRST_A> for bool {
    #[inline(always)]
    fn from(variant: BDRST_A) -> Self {
        variant as u8 != 0
    }
}
impl BDRST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BDRST_A {
        match self.bits {
            false => BDRST_A::DISABLED,
            true => BDRST_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == BDRST_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == BDRST_A::ENABLED
    }
}
#[doc = "Field `BDRST` writer - Backup domain software reset"]
pub type BDRST_W<'a, const O: u8> = crate::BitWriter<'a, BDCR_SPEC, O, BDRST_A>;
impl<'a, const O: u8> BDRST_W<'a, O> {
    #[doc = "Reset not activated"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(BDRST_A::DISABLED)
    }
    #[doc = "Reset the entire RTC domain"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(BDRST_A::ENABLED)
    }
}
impl R {
    #[doc = "Bit 0 - External Low Speed oscillator enable"]
    #[inline(always)]
    pub fn lseon(&self) -> LSEON_R {
        LSEON_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - External Low Speed oscillator ready"]
    #[inline(always)]
    pub fn lserdy(&self) -> LSERDY_R {
        LSERDY_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - External Low Speed oscillator bypass"]
    #[inline(always)]
    pub fn lsebyp(&self) -> LSEBYP_R {
        LSEBYP_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:4 - LSE oscillator drive capability"]
    #[inline(always)]
    pub fn lsedrv(&self) -> LSEDRV_R {
        LSEDRV_R::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bits 8:9 - RTC clock source selection"]
    #[inline(always)]
    pub fn rtcsel(&self) -> RTCSEL_R {
        RTCSEL_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 15 - RTC clock enable"]
    #[inline(always)]
    pub fn rtcen(&self) -> RTCEN_R {
        RTCEN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Backup domain software reset"]
    #[inline(always)]
    pub fn bdrst(&self) -> BDRST_R {
        BDRST_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - External Low Speed oscillator enable"]
    #[inline(always)]
    #[must_use]
    pub fn lseon(&mut self) -> LSEON_W<0> {
        LSEON_W::new(self)
    }
    #[doc = "Bit 2 - External Low Speed oscillator bypass"]
    #[inline(always)]
    #[must_use]
    pub fn lsebyp(&mut self) -> LSEBYP_W<2> {
        LSEBYP_W::new(self)
    }
    #[doc = "Bits 3:4 - LSE oscillator drive capability"]
    #[inline(always)]
    #[must_use]
    pub fn lsedrv(&mut self) -> LSEDRV_W<3> {
        LSEDRV_W::new(self)
    }
    #[doc = "Bits 8:9 - RTC clock source selection"]
    #[inline(always)]
    #[must_use]
    pub fn rtcsel(&mut self) -> RTCSEL_W<8> {
        RTCSEL_W::new(self)
    }
    #[doc = "Bit 15 - RTC clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn rtcen(&mut self) -> RTCEN_W<15> {
        RTCEN_W::new(self)
    }
    #[doc = "Bit 16 - Backup domain software reset"]
    #[inline(always)]
    #[must_use]
    pub fn bdrst(&mut self) -> BDRST_W<16> {
        BDRST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Backup domain control register (RCC_BDCR)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bdcr](index.html) module"]
pub struct BDCR_SPEC;
impl crate::RegisterSpec for BDCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bdcr::R](R) reader structure"]
impl crate::Readable for BDCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bdcr::W](W) writer structure"]
impl crate::Writable for BDCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BDCR to value 0"]
impl crate::Resettable for BDCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

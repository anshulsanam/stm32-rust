#[doc = "Register `CR` reader"]
pub struct R(crate::R<CR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CR` writer"]
pub struct W(crate::W<CR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CR_SPEC>;
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
impl From<crate::W<CR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WUCKSEL` reader - Wakeup clock selection"]
pub type WUCKSEL_R = crate::FieldReader<WUCKSEL_A>;
#[doc = "Wakeup clock selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum WUCKSEL_A {
    #[doc = "0: RTC/16 clock is selected"]
    DIV16 = 0,
    #[doc = "1: RTC/8 clock is selected"]
    DIV8 = 1,
    #[doc = "2: RTC/4 clock is selected"]
    DIV4 = 2,
    #[doc = "3: RTC/2 clock is selected"]
    DIV2 = 3,
    #[doc = "4: ck_spre (usually 1 Hz) clock is selected"]
    CLOCK_SPARE = 4,
    #[doc = "6: ck_spre (usually 1 Hz) clock is selected and 2^16 is added to the WUT counter value"]
    CLOCK_SPARE_WITH_OFFSET = 6,
}
impl From<WUCKSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: WUCKSEL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for WUCKSEL_A {
    type Ux = u8;
}
impl WUCKSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<WUCKSEL_A> {
        match self.bits {
            0 => Some(WUCKSEL_A::DIV16),
            1 => Some(WUCKSEL_A::DIV8),
            2 => Some(WUCKSEL_A::DIV4),
            3 => Some(WUCKSEL_A::DIV2),
            4 => Some(WUCKSEL_A::CLOCK_SPARE),
            6 => Some(WUCKSEL_A::CLOCK_SPARE_WITH_OFFSET),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DIV16`"]
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == WUCKSEL_A::DIV16
    }
    #[doc = "Checks if the value of the field is `DIV8`"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == WUCKSEL_A::DIV8
    }
    #[doc = "Checks if the value of the field is `DIV4`"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == WUCKSEL_A::DIV4
    }
    #[doc = "Checks if the value of the field is `DIV2`"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == WUCKSEL_A::DIV2
    }
    #[doc = "Checks if the value of the field is `CLOCK_SPARE`"]
    #[inline(always)]
    pub fn is_clock_spare(&self) -> bool {
        *self == WUCKSEL_A::CLOCK_SPARE
    }
    #[doc = "Checks if the value of the field is `CLOCK_SPARE_WITH_OFFSET`"]
    #[inline(always)]
    pub fn is_clock_spare_with_offset(&self) -> bool {
        *self == WUCKSEL_A::CLOCK_SPARE_WITH_OFFSET
    }
}
#[doc = "Field `WUCKSEL` writer - Wakeup clock selection"]
pub type WUCKSEL_W<'a, const O: u8> = crate::FieldWriter<'a, CR_SPEC, 3, O, WUCKSEL_A>;
impl<'a, const O: u8> WUCKSEL_W<'a, O> {
    #[doc = "RTC/16 clock is selected"]
    #[inline(always)]
    pub fn div16(self) -> &'a mut W {
        self.variant(WUCKSEL_A::DIV16)
    }
    #[doc = "RTC/8 clock is selected"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut W {
        self.variant(WUCKSEL_A::DIV8)
    }
    #[doc = "RTC/4 clock is selected"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut W {
        self.variant(WUCKSEL_A::DIV4)
    }
    #[doc = "RTC/2 clock is selected"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut W {
        self.variant(WUCKSEL_A::DIV2)
    }
    #[doc = "ck_spre (usually 1 Hz) clock is selected"]
    #[inline(always)]
    pub fn clock_spare(self) -> &'a mut W {
        self.variant(WUCKSEL_A::CLOCK_SPARE)
    }
    #[doc = "ck_spre (usually 1 Hz) clock is selected and 2^16 is added to the WUT counter value"]
    #[inline(always)]
    pub fn clock_spare_with_offset(self) -> &'a mut W {
        self.variant(WUCKSEL_A::CLOCK_SPARE_WITH_OFFSET)
    }
}
#[doc = "Field `TSEDGE` reader - Time-stamp event active edge"]
pub type TSEDGE_R = crate::BitReader<TSEDGE_A>;
#[doc = "Time-stamp event active edge\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TSEDGE_A {
    #[doc = "0: RTC_TS input rising edge generates a time-stamp event"]
    RISING_EDGE = 0,
    #[doc = "1: RTC_TS input falling edge generates a time-stamp event"]
    FALLING_EDGE = 1,
}
impl From<TSEDGE_A> for bool {
    #[inline(always)]
    fn from(variant: TSEDGE_A) -> Self {
        variant as u8 != 0
    }
}
impl TSEDGE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TSEDGE_A {
        match self.bits {
            false => TSEDGE_A::RISING_EDGE,
            true => TSEDGE_A::FALLING_EDGE,
        }
    }
    #[doc = "Checks if the value of the field is `RISING_EDGE`"]
    #[inline(always)]
    pub fn is_rising_edge(&self) -> bool {
        *self == TSEDGE_A::RISING_EDGE
    }
    #[doc = "Checks if the value of the field is `FALLING_EDGE`"]
    #[inline(always)]
    pub fn is_falling_edge(&self) -> bool {
        *self == TSEDGE_A::FALLING_EDGE
    }
}
#[doc = "Field `TSEDGE` writer - Time-stamp event active edge"]
pub type TSEDGE_W<'a, const O: u8> = crate::BitWriter<'a, CR_SPEC, O, TSEDGE_A>;
impl<'a, const O: u8> TSEDGE_W<'a, O> {
    #[doc = "RTC_TS input rising edge generates a time-stamp event"]
    #[inline(always)]
    pub fn rising_edge(self) -> &'a mut W {
        self.variant(TSEDGE_A::RISING_EDGE)
    }
    #[doc = "RTC_TS input falling edge generates a time-stamp event"]
    #[inline(always)]
    pub fn falling_edge(self) -> &'a mut W {
        self.variant(TSEDGE_A::FALLING_EDGE)
    }
}
#[doc = "Field `REFCKON` reader - Reference clock detection enable (50 or 60 Hz)"]
pub type REFCKON_R = crate::BitReader<REFCKON_A>;
#[doc = "Reference clock detection enable (50 or 60 Hz)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REFCKON_A {
    #[doc = "0: RTC_REFIN detection disabled"]
    DISABLED = 0,
    #[doc = "1: RTC_REFIN detection enabled"]
    ENABLED = 1,
}
impl From<REFCKON_A> for bool {
    #[inline(always)]
    fn from(variant: REFCKON_A) -> Self {
        variant as u8 != 0
    }
}
impl REFCKON_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REFCKON_A {
        match self.bits {
            false => REFCKON_A::DISABLED,
            true => REFCKON_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == REFCKON_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == REFCKON_A::ENABLED
    }
}
#[doc = "Field `REFCKON` writer - Reference clock detection enable (50 or 60 Hz)"]
pub type REFCKON_W<'a, const O: u8> = crate::BitWriter<'a, CR_SPEC, O, REFCKON_A>;
impl<'a, const O: u8> REFCKON_W<'a, O> {
    #[doc = "RTC_REFIN detection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REFCKON_A::DISABLED)
    }
    #[doc = "RTC_REFIN detection enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REFCKON_A::ENABLED)
    }
}
#[doc = "Field `BYPSHAD` reader - Bypass the shadow registers"]
pub type BYPSHAD_R = crate::BitReader<BYPSHAD_A>;
#[doc = "Bypass the shadow registers\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BYPSHAD_A {
    #[doc = "0: Calendar values (when reading from RTC_SSR, RTC_TR, and RTC_DR) are taken from the shadow registers, which are updated once every two RTCCLK cycles"]
    SHADOW_REG = 0,
    #[doc = "1: Calendar values (when reading from RTC_SSR, RTC_TR, and RTC_DR) are taken directly from the calendar counters"]
    BYPASS_SHADOW_REG = 1,
}
impl From<BYPSHAD_A> for bool {
    #[inline(always)]
    fn from(variant: BYPSHAD_A) -> Self {
        variant as u8 != 0
    }
}
impl BYPSHAD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BYPSHAD_A {
        match self.bits {
            false => BYPSHAD_A::SHADOW_REG,
            true => BYPSHAD_A::BYPASS_SHADOW_REG,
        }
    }
    #[doc = "Checks if the value of the field is `SHADOW_REG`"]
    #[inline(always)]
    pub fn is_shadow_reg(&self) -> bool {
        *self == BYPSHAD_A::SHADOW_REG
    }
    #[doc = "Checks if the value of the field is `BYPASS_SHADOW_REG`"]
    #[inline(always)]
    pub fn is_bypass_shadow_reg(&self) -> bool {
        *self == BYPSHAD_A::BYPASS_SHADOW_REG
    }
}
#[doc = "Field `BYPSHAD` writer - Bypass the shadow registers"]
pub type BYPSHAD_W<'a, const O: u8> = crate::BitWriter<'a, CR_SPEC, O, BYPSHAD_A>;
impl<'a, const O: u8> BYPSHAD_W<'a, O> {
    #[doc = "Calendar values (when reading from RTC_SSR, RTC_TR, and RTC_DR) are taken from the shadow registers, which are updated once every two RTCCLK cycles"]
    #[inline(always)]
    pub fn shadow_reg(self) -> &'a mut W {
        self.variant(BYPSHAD_A::SHADOW_REG)
    }
    #[doc = "Calendar values (when reading from RTC_SSR, RTC_TR, and RTC_DR) are taken directly from the calendar counters"]
    #[inline(always)]
    pub fn bypass_shadow_reg(self) -> &'a mut W {
        self.variant(BYPSHAD_A::BYPASS_SHADOW_REG)
    }
}
#[doc = "Field `FMT` reader - Hour format"]
pub type FMT_R = crate::BitReader<FMT_A>;
#[doc = "Hour format\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FMT_A {
    #[doc = "0: 24 hour/day format"]
    TWENTY_FOUR_HOUR = 0,
    #[doc = "1: AM/PM hour format"]
    AM_PM = 1,
}
impl From<FMT_A> for bool {
    #[inline(always)]
    fn from(variant: FMT_A) -> Self {
        variant as u8 != 0
    }
}
impl FMT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FMT_A {
        match self.bits {
            false => FMT_A::TWENTY_FOUR_HOUR,
            true => FMT_A::AM_PM,
        }
    }
    #[doc = "Checks if the value of the field is `TWENTY_FOUR_HOUR`"]
    #[inline(always)]
    pub fn is_twenty_four_hour(&self) -> bool {
        *self == FMT_A::TWENTY_FOUR_HOUR
    }
    #[doc = "Checks if the value of the field is `AM_PM`"]
    #[inline(always)]
    pub fn is_am_pm(&self) -> bool {
        *self == FMT_A::AM_PM
    }
}
#[doc = "Field `FMT` writer - Hour format"]
pub type FMT_W<'a, const O: u8> = crate::BitWriter<'a, CR_SPEC, O, FMT_A>;
impl<'a, const O: u8> FMT_W<'a, O> {
    #[doc = "24 hour/day format"]
    #[inline(always)]
    pub fn twenty_four_hour(self) -> &'a mut W {
        self.variant(FMT_A::TWENTY_FOUR_HOUR)
    }
    #[doc = "AM/PM hour format"]
    #[inline(always)]
    pub fn am_pm(self) -> &'a mut W {
        self.variant(FMT_A::AM_PM)
    }
}
#[doc = "Field `ALRAE` reader - Alarm A enable"]
pub type ALRAE_R = crate::BitReader<ALRAE_A>;
#[doc = "Alarm A enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ALRAE_A {
    #[doc = "0: Alarm A disabled"]
    DISABLED = 0,
    #[doc = "1: Alarm A enabled"]
    ENABLED = 1,
}
impl From<ALRAE_A> for bool {
    #[inline(always)]
    fn from(variant: ALRAE_A) -> Self {
        variant as u8 != 0
    }
}
impl ALRAE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ALRAE_A {
        match self.bits {
            false => ALRAE_A::DISABLED,
            true => ALRAE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ALRAE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ALRAE_A::ENABLED
    }
}
#[doc = "Field `ALRAE` writer - Alarm A enable"]
pub type ALRAE_W<'a, const O: u8> = crate::BitWriter<'a, CR_SPEC, O, ALRAE_A>;
impl<'a, const O: u8> ALRAE_W<'a, O> {
    #[doc = "Alarm A disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ALRAE_A::DISABLED)
    }
    #[doc = "Alarm A enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ALRAE_A::ENABLED)
    }
}
#[doc = "Field `ALRBE` reader - Alarm B enable"]
pub type ALRBE_R = crate::BitReader<ALRBE_A>;
#[doc = "Alarm B enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ALRBE_A {
    #[doc = "0: Alarm B disabled"]
    DISABLED = 0,
    #[doc = "1: Alarm B enabled"]
    ENABLED = 1,
}
impl From<ALRBE_A> for bool {
    #[inline(always)]
    fn from(variant: ALRBE_A) -> Self {
        variant as u8 != 0
    }
}
impl ALRBE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ALRBE_A {
        match self.bits {
            false => ALRBE_A::DISABLED,
            true => ALRBE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ALRBE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ALRBE_A::ENABLED
    }
}
#[doc = "Field `ALRBE` writer - Alarm B enable"]
pub type ALRBE_W<'a, const O: u8> = crate::BitWriter<'a, CR_SPEC, O, ALRBE_A>;
impl<'a, const O: u8> ALRBE_W<'a, O> {
    #[doc = "Alarm B disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ALRBE_A::DISABLED)
    }
    #[doc = "Alarm B enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ALRBE_A::ENABLED)
    }
}
#[doc = "Field `WUTE` reader - Wakeup timer enable"]
pub type WUTE_R = crate::BitReader<WUTE_A>;
#[doc = "Wakeup timer enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WUTE_A {
    #[doc = "0: Wakeup timer disabled"]
    DISABLED = 0,
    #[doc = "1: Wakeup timer enabled"]
    ENABLED = 1,
}
impl From<WUTE_A> for bool {
    #[inline(always)]
    fn from(variant: WUTE_A) -> Self {
        variant as u8 != 0
    }
}
impl WUTE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WUTE_A {
        match self.bits {
            false => WUTE_A::DISABLED,
            true => WUTE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == WUTE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == WUTE_A::ENABLED
    }
}
#[doc = "Field `WUTE` writer - Wakeup timer enable"]
pub type WUTE_W<'a, const O: u8> = crate::BitWriter<'a, CR_SPEC, O, WUTE_A>;
impl<'a, const O: u8> WUTE_W<'a, O> {
    #[doc = "Wakeup timer disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(WUTE_A::DISABLED)
    }
    #[doc = "Wakeup timer enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(WUTE_A::ENABLED)
    }
}
#[doc = "Field `TSE` reader - Time stamp enable"]
pub type TSE_R = crate::BitReader<TSE_A>;
#[doc = "Time stamp enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TSE_A {
    #[doc = "0: Timestamp disabled"]
    DISABLED = 0,
    #[doc = "1: Timestamp enabled"]
    ENABLED = 1,
}
impl From<TSE_A> for bool {
    #[inline(always)]
    fn from(variant: TSE_A) -> Self {
        variant as u8 != 0
    }
}
impl TSE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TSE_A {
        match self.bits {
            false => TSE_A::DISABLED,
            true => TSE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TSE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TSE_A::ENABLED
    }
}
#[doc = "Field `TSE` writer - Time stamp enable"]
pub type TSE_W<'a, const O: u8> = crate::BitWriter<'a, CR_SPEC, O, TSE_A>;
impl<'a, const O: u8> TSE_W<'a, O> {
    #[doc = "Timestamp disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TSE_A::DISABLED)
    }
    #[doc = "Timestamp enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TSE_A::ENABLED)
    }
}
#[doc = "Field `ALRAIE` reader - Alarm A interrupt enable"]
pub type ALRAIE_R = crate::BitReader<ALRAIE_A>;
#[doc = "Alarm A interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ALRAIE_A {
    #[doc = "0: Alarm A interrupt disabled"]
    DISABLED = 0,
    #[doc = "1: Alarm A interrupt enabled"]
    ENABLED = 1,
}
impl From<ALRAIE_A> for bool {
    #[inline(always)]
    fn from(variant: ALRAIE_A) -> Self {
        variant as u8 != 0
    }
}
impl ALRAIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ALRAIE_A {
        match self.bits {
            false => ALRAIE_A::DISABLED,
            true => ALRAIE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ALRAIE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ALRAIE_A::ENABLED
    }
}
#[doc = "Field `ALRAIE` writer - Alarm A interrupt enable"]
pub type ALRAIE_W<'a, const O: u8> = crate::BitWriter<'a, CR_SPEC, O, ALRAIE_A>;
impl<'a, const O: u8> ALRAIE_W<'a, O> {
    #[doc = "Alarm A interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ALRAIE_A::DISABLED)
    }
    #[doc = "Alarm A interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ALRAIE_A::ENABLED)
    }
}
#[doc = "Field `ALRBIE` reader - Alarm B interrupt enable"]
pub type ALRBIE_R = crate::BitReader<ALRBIE_A>;
#[doc = "Alarm B interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ALRBIE_A {
    #[doc = "0: Alarm B Interrupt disabled"]
    DISABLED = 0,
    #[doc = "1: Alarm B Interrupt enabled"]
    ENABLED = 1,
}
impl From<ALRBIE_A> for bool {
    #[inline(always)]
    fn from(variant: ALRBIE_A) -> Self {
        variant as u8 != 0
    }
}
impl ALRBIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ALRBIE_A {
        match self.bits {
            false => ALRBIE_A::DISABLED,
            true => ALRBIE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ALRBIE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ALRBIE_A::ENABLED
    }
}
#[doc = "Field `ALRBIE` writer - Alarm B interrupt enable"]
pub type ALRBIE_W<'a, const O: u8> = crate::BitWriter<'a, CR_SPEC, O, ALRBIE_A>;
impl<'a, const O: u8> ALRBIE_W<'a, O> {
    #[doc = "Alarm B Interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ALRBIE_A::DISABLED)
    }
    #[doc = "Alarm B Interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ALRBIE_A::ENABLED)
    }
}
#[doc = "Field `WUTIE` reader - Wakeup timer interrupt enable"]
pub type WUTIE_R = crate::BitReader<WUTIE_A>;
#[doc = "Wakeup timer interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WUTIE_A {
    #[doc = "0: Wakeup timer interrupt disabled"]
    DISABLED = 0,
    #[doc = "1: Wakeup timer interrupt enabled"]
    ENABLED = 1,
}
impl From<WUTIE_A> for bool {
    #[inline(always)]
    fn from(variant: WUTIE_A) -> Self {
        variant as u8 != 0
    }
}
impl WUTIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WUTIE_A {
        match self.bits {
            false => WUTIE_A::DISABLED,
            true => WUTIE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == WUTIE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == WUTIE_A::ENABLED
    }
}
#[doc = "Field `WUTIE` writer - Wakeup timer interrupt enable"]
pub type WUTIE_W<'a, const O: u8> = crate::BitWriter<'a, CR_SPEC, O, WUTIE_A>;
impl<'a, const O: u8> WUTIE_W<'a, O> {
    #[doc = "Wakeup timer interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(WUTIE_A::DISABLED)
    }
    #[doc = "Wakeup timer interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(WUTIE_A::ENABLED)
    }
}
#[doc = "Field `TSIE` reader - Time-stamp interrupt enable"]
pub type TSIE_R = crate::BitReader<TSIE_A>;
#[doc = "Time-stamp interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TSIE_A {
    #[doc = "0: Time-stamp Interrupt disabled"]
    DISABLED = 0,
    #[doc = "1: Time-stamp Interrupt enabled"]
    ENABLED = 1,
}
impl From<TSIE_A> for bool {
    #[inline(always)]
    fn from(variant: TSIE_A) -> Self {
        variant as u8 != 0
    }
}
impl TSIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TSIE_A {
        match self.bits {
            false => TSIE_A::DISABLED,
            true => TSIE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TSIE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TSIE_A::ENABLED
    }
}
#[doc = "Field `TSIE` writer - Time-stamp interrupt enable"]
pub type TSIE_W<'a, const O: u8> = crate::BitWriter<'a, CR_SPEC, O, TSIE_A>;
impl<'a, const O: u8> TSIE_W<'a, O> {
    #[doc = "Time-stamp Interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TSIE_A::DISABLED)
    }
    #[doc = "Time-stamp Interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TSIE_A::ENABLED)
    }
}
#[doc = "Field `ADD1H` reader - Add 1 hour (summer time change)"]
pub type ADD1H_R = crate::BitReader<ADD1HW_A>;
#[doc = "Add 1 hour (summer time change)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADD1HW_A {
    #[doc = "1: Adds 1 hour to the current time. This can be used for summer time change outside initialization mode"]
    ADD1 = 1,
}
impl From<ADD1HW_A> for bool {
    #[inline(always)]
    fn from(variant: ADD1HW_A) -> Self {
        variant as u8 != 0
    }
}
impl ADD1H_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<ADD1HW_A> {
        match self.bits {
            true => Some(ADD1HW_A::ADD1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `ADD1`"]
    #[inline(always)]
    pub fn is_add1(&self) -> bool {
        *self == ADD1HW_A::ADD1
    }
}
#[doc = "Field `ADD1H` writer - Add 1 hour (summer time change)"]
pub type ADD1H_W<'a, const O: u8> = crate::BitWriter<'a, CR_SPEC, O, ADD1HW_A>;
impl<'a, const O: u8> ADD1H_W<'a, O> {
    #[doc = "Adds 1 hour to the current time. This can be used for summer time change outside initialization mode"]
    #[inline(always)]
    pub fn add1(self) -> &'a mut W {
        self.variant(ADD1HW_A::ADD1)
    }
}
#[doc = "Field `SUB1H` reader - Subtract 1 hour (winter time change)"]
pub type SUB1H_R = crate::BitReader<SUB1HW_A>;
#[doc = "Subtract 1 hour (winter time change)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SUB1HW_A {
    #[doc = "1: Subtracts 1 hour to the current time. This can be used for winter time change outside initialization mode"]
    SUB1 = 1,
}
impl From<SUB1HW_A> for bool {
    #[inline(always)]
    fn from(variant: SUB1HW_A) -> Self {
        variant as u8 != 0
    }
}
impl SUB1H_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SUB1HW_A> {
        match self.bits {
            true => Some(SUB1HW_A::SUB1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `SUB1`"]
    #[inline(always)]
    pub fn is_sub1(&self) -> bool {
        *self == SUB1HW_A::SUB1
    }
}
#[doc = "Field `SUB1H` writer - Subtract 1 hour (winter time change)"]
pub type SUB1H_W<'a, const O: u8> = crate::BitWriter<'a, CR_SPEC, O, SUB1HW_A>;
impl<'a, const O: u8> SUB1H_W<'a, O> {
    #[doc = "Subtracts 1 hour to the current time. This can be used for winter time change outside initialization mode"]
    #[inline(always)]
    pub fn sub1(self) -> &'a mut W {
        self.variant(SUB1HW_A::SUB1)
    }
}
#[doc = "Field `BKP` reader - Backup"]
pub type BKP_R = crate::BitReader<BKP_A>;
#[doc = "Backup\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BKP_A {
    #[doc = "0: Daylight Saving Time change has not been performed"]
    DST_NOT_CHANGED = 0,
    #[doc = "1: Daylight Saving Time change has been performed"]
    DST_CHANGED = 1,
}
impl From<BKP_A> for bool {
    #[inline(always)]
    fn from(variant: BKP_A) -> Self {
        variant as u8 != 0
    }
}
impl BKP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BKP_A {
        match self.bits {
            false => BKP_A::DST_NOT_CHANGED,
            true => BKP_A::DST_CHANGED,
        }
    }
    #[doc = "Checks if the value of the field is `DST_NOT_CHANGED`"]
    #[inline(always)]
    pub fn is_dst_not_changed(&self) -> bool {
        *self == BKP_A::DST_NOT_CHANGED
    }
    #[doc = "Checks if the value of the field is `DST_CHANGED`"]
    #[inline(always)]
    pub fn is_dst_changed(&self) -> bool {
        *self == BKP_A::DST_CHANGED
    }
}
#[doc = "Field `BKP` writer - Backup"]
pub type BKP_W<'a, const O: u8> = crate::BitWriter<'a, CR_SPEC, O, BKP_A>;
impl<'a, const O: u8> BKP_W<'a, O> {
    #[doc = "Daylight Saving Time change has not been performed"]
    #[inline(always)]
    pub fn dst_not_changed(self) -> &'a mut W {
        self.variant(BKP_A::DST_NOT_CHANGED)
    }
    #[doc = "Daylight Saving Time change has been performed"]
    #[inline(always)]
    pub fn dst_changed(self) -> &'a mut W {
        self.variant(BKP_A::DST_CHANGED)
    }
}
#[doc = "Field `COSEL` reader - Calibration output selection"]
pub type COSEL_R = crate::BitReader<COSEL_A>;
#[doc = "Calibration output selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum COSEL_A {
    #[doc = "0: Calibration output is 512 Hz (with default prescaler setting)"]
    CAL_FREQ_512HZ = 0,
    #[doc = "1: Calibration output is 1 Hz (with default prescaler setting)"]
    CAL_FREQ_1HZ = 1,
}
impl From<COSEL_A> for bool {
    #[inline(always)]
    fn from(variant: COSEL_A) -> Self {
        variant as u8 != 0
    }
}
impl COSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COSEL_A {
        match self.bits {
            false => COSEL_A::CAL_FREQ_512HZ,
            true => COSEL_A::CAL_FREQ_1HZ,
        }
    }
    #[doc = "Checks if the value of the field is `CAL_FREQ_512HZ`"]
    #[inline(always)]
    pub fn is_cal_freq_512hz(&self) -> bool {
        *self == COSEL_A::CAL_FREQ_512HZ
    }
    #[doc = "Checks if the value of the field is `CAL_FREQ_1HZ`"]
    #[inline(always)]
    pub fn is_cal_freq_1hz(&self) -> bool {
        *self == COSEL_A::CAL_FREQ_1HZ
    }
}
#[doc = "Field `COSEL` writer - Calibration output selection"]
pub type COSEL_W<'a, const O: u8> = crate::BitWriter<'a, CR_SPEC, O, COSEL_A>;
impl<'a, const O: u8> COSEL_W<'a, O> {
    #[doc = "Calibration output is 512 Hz (with default prescaler setting)"]
    #[inline(always)]
    pub fn cal_freq_512hz(self) -> &'a mut W {
        self.variant(COSEL_A::CAL_FREQ_512HZ)
    }
    #[doc = "Calibration output is 1 Hz (with default prescaler setting)"]
    #[inline(always)]
    pub fn cal_freq_1hz(self) -> &'a mut W {
        self.variant(COSEL_A::CAL_FREQ_1HZ)
    }
}
#[doc = "Field `POL` reader - Output polarity"]
pub type POL_R = crate::BitReader<POL_A>;
#[doc = "Output polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum POL_A {
    #[doc = "0: The pin is high when ALRAF/ALRBF/WUTF is asserted (depending on OSEL\\[1:0\\])"]
    HIGH = 0,
    #[doc = "1: The pin is low when ALRAF/ALRBF/WUTF is asserted (depending on OSEL\\[1:0\\])"]
    LOW = 1,
}
impl From<POL_A> for bool {
    #[inline(always)]
    fn from(variant: POL_A) -> Self {
        variant as u8 != 0
    }
}
impl POL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> POL_A {
        match self.bits {
            false => POL_A::HIGH,
            true => POL_A::LOW,
        }
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == POL_A::HIGH
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == POL_A::LOW
    }
}
#[doc = "Field `POL` writer - Output polarity"]
pub type POL_W<'a, const O: u8> = crate::BitWriter<'a, CR_SPEC, O, POL_A>;
impl<'a, const O: u8> POL_W<'a, O> {
    #[doc = "The pin is high when ALRAF/ALRBF/WUTF is asserted (depending on OSEL\\[1:0\\])"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(POL_A::HIGH)
    }
    #[doc = "The pin is low when ALRAF/ALRBF/WUTF is asserted (depending on OSEL\\[1:0\\])"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(POL_A::LOW)
    }
}
#[doc = "Field `OSEL` reader - Output selection"]
pub type OSEL_R = crate::FieldReader<OSEL_A>;
#[doc = "Output selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum OSEL_A {
    #[doc = "0: Output disabled"]
    DISABLED = 0,
    #[doc = "1: Alarm A output enabled"]
    ALARM_A = 1,
    #[doc = "2: Alarm B output enabled"]
    ALARM_B = 2,
    #[doc = "3: Wakeup output enabled"]
    WAKEUP = 3,
}
impl From<OSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: OSEL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for OSEL_A {
    type Ux = u8;
}
impl OSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OSEL_A {
        match self.bits {
            0 => OSEL_A::DISABLED,
            1 => OSEL_A::ALARM_A,
            2 => OSEL_A::ALARM_B,
            3 => OSEL_A::WAKEUP,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == OSEL_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ALARM_A`"]
    #[inline(always)]
    pub fn is_alarm_a(&self) -> bool {
        *self == OSEL_A::ALARM_A
    }
    #[doc = "Checks if the value of the field is `ALARM_B`"]
    #[inline(always)]
    pub fn is_alarm_b(&self) -> bool {
        *self == OSEL_A::ALARM_B
    }
    #[doc = "Checks if the value of the field is `WAKEUP`"]
    #[inline(always)]
    pub fn is_wakeup(&self) -> bool {
        *self == OSEL_A::WAKEUP
    }
}
#[doc = "Field `OSEL` writer - Output selection"]
pub type OSEL_W<'a, const O: u8> = crate::FieldWriterSafe<'a, CR_SPEC, 2, O, OSEL_A>;
impl<'a, const O: u8> OSEL_W<'a, O> {
    #[doc = "Output disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(OSEL_A::DISABLED)
    }
    #[doc = "Alarm A output enabled"]
    #[inline(always)]
    pub fn alarm_a(self) -> &'a mut W {
        self.variant(OSEL_A::ALARM_A)
    }
    #[doc = "Alarm B output enabled"]
    #[inline(always)]
    pub fn alarm_b(self) -> &'a mut W {
        self.variant(OSEL_A::ALARM_B)
    }
    #[doc = "Wakeup output enabled"]
    #[inline(always)]
    pub fn wakeup(self) -> &'a mut W {
        self.variant(OSEL_A::WAKEUP)
    }
}
#[doc = "Field `COE` reader - Calibration output enable"]
pub type COE_R = crate::BitReader<COE_A>;
#[doc = "Calibration output enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum COE_A {
    #[doc = "0: Calibration output disabled"]
    DISABLED = 0,
    #[doc = "1: Calibration output enabled"]
    ENABLED = 1,
}
impl From<COE_A> for bool {
    #[inline(always)]
    fn from(variant: COE_A) -> Self {
        variant as u8 != 0
    }
}
impl COE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COE_A {
        match self.bits {
            false => COE_A::DISABLED,
            true => COE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == COE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == COE_A::ENABLED
    }
}
#[doc = "Field `COE` writer - Calibration output enable"]
pub type COE_W<'a, const O: u8> = crate::BitWriter<'a, CR_SPEC, O, COE_A>;
impl<'a, const O: u8> COE_W<'a, O> {
    #[doc = "Calibration output disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(COE_A::DISABLED)
    }
    #[doc = "Calibration output enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(COE_A::ENABLED)
    }
}
impl R {
    #[doc = "Bits 0:2 - Wakeup clock selection"]
    #[inline(always)]
    pub fn wucksel(&self) -> WUCKSEL_R {
        WUCKSEL_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - Time-stamp event active edge"]
    #[inline(always)]
    pub fn tsedge(&self) -> TSEDGE_R {
        TSEDGE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Reference clock detection enable (50 or 60 Hz)"]
    #[inline(always)]
    pub fn refckon(&self) -> REFCKON_R {
        REFCKON_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Bypass the shadow registers"]
    #[inline(always)]
    pub fn bypshad(&self) -> BYPSHAD_R {
        BYPSHAD_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Hour format"]
    #[inline(always)]
    pub fn fmt(&self) -> FMT_R {
        FMT_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - Alarm A enable"]
    #[inline(always)]
    pub fn alrae(&self) -> ALRAE_R {
        ALRAE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Alarm B enable"]
    #[inline(always)]
    pub fn alrbe(&self) -> ALRBE_R {
        ALRBE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Wakeup timer enable"]
    #[inline(always)]
    pub fn wute(&self) -> WUTE_R {
        WUTE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Time stamp enable"]
    #[inline(always)]
    pub fn tse(&self) -> TSE_R {
        TSE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Alarm A interrupt enable"]
    #[inline(always)]
    pub fn alraie(&self) -> ALRAIE_R {
        ALRAIE_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Alarm B interrupt enable"]
    #[inline(always)]
    pub fn alrbie(&self) -> ALRBIE_R {
        ALRBIE_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Wakeup timer interrupt enable"]
    #[inline(always)]
    pub fn wutie(&self) -> WUTIE_R {
        WUTIE_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Time-stamp interrupt enable"]
    #[inline(always)]
    pub fn tsie(&self) -> TSIE_R {
        TSIE_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Add 1 hour (summer time change)"]
    #[inline(always)]
    pub fn add1h(&self) -> ADD1H_R {
        ADD1H_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Subtract 1 hour (winter time change)"]
    #[inline(always)]
    pub fn sub1h(&self) -> SUB1H_R {
        SUB1H_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Backup"]
    #[inline(always)]
    pub fn bkp(&self) -> BKP_R {
        BKP_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Calibration output selection"]
    #[inline(always)]
    pub fn cosel(&self) -> COSEL_R {
        COSEL_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Output polarity"]
    #[inline(always)]
    pub fn pol(&self) -> POL_R {
        POL_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bits 21:22 - Output selection"]
    #[inline(always)]
    pub fn osel(&self) -> OSEL_R {
        OSEL_R::new(((self.bits >> 21) & 3) as u8)
    }
    #[doc = "Bit 23 - Calibration output enable"]
    #[inline(always)]
    pub fn coe(&self) -> COE_R {
        COE_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Wakeup clock selection"]
    #[inline(always)]
    #[must_use]
    pub fn wucksel(&mut self) -> WUCKSEL_W<0> {
        WUCKSEL_W::new(self)
    }
    #[doc = "Bit 3 - Time-stamp event active edge"]
    #[inline(always)]
    #[must_use]
    pub fn tsedge(&mut self) -> TSEDGE_W<3> {
        TSEDGE_W::new(self)
    }
    #[doc = "Bit 4 - Reference clock detection enable (50 or 60 Hz)"]
    #[inline(always)]
    #[must_use]
    pub fn refckon(&mut self) -> REFCKON_W<4> {
        REFCKON_W::new(self)
    }
    #[doc = "Bit 5 - Bypass the shadow registers"]
    #[inline(always)]
    #[must_use]
    pub fn bypshad(&mut self) -> BYPSHAD_W<5> {
        BYPSHAD_W::new(self)
    }
    #[doc = "Bit 6 - Hour format"]
    #[inline(always)]
    #[must_use]
    pub fn fmt(&mut self) -> FMT_W<6> {
        FMT_W::new(self)
    }
    #[doc = "Bit 8 - Alarm A enable"]
    #[inline(always)]
    #[must_use]
    pub fn alrae(&mut self) -> ALRAE_W<8> {
        ALRAE_W::new(self)
    }
    #[doc = "Bit 9 - Alarm B enable"]
    #[inline(always)]
    #[must_use]
    pub fn alrbe(&mut self) -> ALRBE_W<9> {
        ALRBE_W::new(self)
    }
    #[doc = "Bit 10 - Wakeup timer enable"]
    #[inline(always)]
    #[must_use]
    pub fn wute(&mut self) -> WUTE_W<10> {
        WUTE_W::new(self)
    }
    #[doc = "Bit 11 - Time stamp enable"]
    #[inline(always)]
    #[must_use]
    pub fn tse(&mut self) -> TSE_W<11> {
        TSE_W::new(self)
    }
    #[doc = "Bit 12 - Alarm A interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn alraie(&mut self) -> ALRAIE_W<12> {
        ALRAIE_W::new(self)
    }
    #[doc = "Bit 13 - Alarm B interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn alrbie(&mut self) -> ALRBIE_W<13> {
        ALRBIE_W::new(self)
    }
    #[doc = "Bit 14 - Wakeup timer interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn wutie(&mut self) -> WUTIE_W<14> {
        WUTIE_W::new(self)
    }
    #[doc = "Bit 15 - Time-stamp interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn tsie(&mut self) -> TSIE_W<15> {
        TSIE_W::new(self)
    }
    #[doc = "Bit 16 - Add 1 hour (summer time change)"]
    #[inline(always)]
    #[must_use]
    pub fn add1h(&mut self) -> ADD1H_W<16> {
        ADD1H_W::new(self)
    }
    #[doc = "Bit 17 - Subtract 1 hour (winter time change)"]
    #[inline(always)]
    #[must_use]
    pub fn sub1h(&mut self) -> SUB1H_W<17> {
        SUB1H_W::new(self)
    }
    #[doc = "Bit 18 - Backup"]
    #[inline(always)]
    #[must_use]
    pub fn bkp(&mut self) -> BKP_W<18> {
        BKP_W::new(self)
    }
    #[doc = "Bit 19 - Calibration output selection"]
    #[inline(always)]
    #[must_use]
    pub fn cosel(&mut self) -> COSEL_W<19> {
        COSEL_W::new(self)
    }
    #[doc = "Bit 20 - Output polarity"]
    #[inline(always)]
    #[must_use]
    pub fn pol(&mut self) -> POL_W<20> {
        POL_W::new(self)
    }
    #[doc = "Bits 21:22 - Output selection"]
    #[inline(always)]
    #[must_use]
    pub fn osel(&mut self) -> OSEL_W<21> {
        OSEL_W::new(self)
    }
    #[doc = "Bit 23 - Calibration output enable"]
    #[inline(always)]
    #[must_use]
    pub fn coe(&mut self) -> COE_W<23> {
        COE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr](index.html) module"]
pub struct CR_SPEC;
impl crate::RegisterSpec for CR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cr::R](R) reader structure"]
impl crate::Readable for CR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cr::W](W) writer structure"]
impl crate::Writable for CR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CR to value 0"]
impl crate::Resettable for CR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

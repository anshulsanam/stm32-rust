#[doc = "Register `ISR` reader"]
pub struct R(crate::R<ISR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ISR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ISR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ISR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ISR` writer"]
pub struct W(crate::W<ISR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ISR_SPEC>;
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
impl From<crate::W<ISR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ISR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADRDY` reader - ADRDY"]
pub type ADRDY_R = crate::BitReader<ADRDYR_A>;
#[doc = "ADRDY\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADRDYR_A {
    #[doc = "0: ADC is not ready to start conversion"]
    NOT_READY = 0,
    #[doc = "1: ADC is ready to start conversion"]
    READY = 1,
}
impl From<ADRDYR_A> for bool {
    #[inline(always)]
    fn from(variant: ADRDYR_A) -> Self {
        variant as u8 != 0
    }
}
impl ADRDY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADRDYR_A {
        match self.bits {
            false => ADRDYR_A::NOT_READY,
            true => ADRDYR_A::READY,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_READY`"]
    #[inline(always)]
    pub fn is_not_ready(&self) -> bool {
        *self == ADRDYR_A::NOT_READY
    }
    #[doc = "Checks if the value of the field is `READY`"]
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == ADRDYR_A::READY
    }
}
#[doc = "ADRDY\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADRDYW_AW {
    #[doc = "1: Clear ADC is ready to start conversion flag"]
    CLEAR = 1,
}
impl From<ADRDYW_AW> for bool {
    #[inline(always)]
    fn from(variant: ADRDYW_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADRDY` writer - ADRDY"]
pub type ADRDY_W<'a, const O: u8> = crate::BitWriter<'a, ISR_SPEC, O, ADRDYW_AW>;
impl<'a, const O: u8> ADRDY_W<'a, O> {
    #[doc = "Clear ADC is ready to start conversion flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(ADRDYW_AW::CLEAR)
    }
}
#[doc = "Field `EOSMP` reader - EOSMP"]
pub type EOSMP_R = crate::BitReader<EOSMPR_A>;
#[doc = "EOSMP\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EOSMPR_A {
    #[doc = "0: End of sampling phase no yet reached"]
    NOT_ENDED = 0,
    #[doc = "1: End of sampling phase reached"]
    ENDED = 1,
}
impl From<EOSMPR_A> for bool {
    #[inline(always)]
    fn from(variant: EOSMPR_A) -> Self {
        variant as u8 != 0
    }
}
impl EOSMP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EOSMPR_A {
        match self.bits {
            false => EOSMPR_A::NOT_ENDED,
            true => EOSMPR_A::ENDED,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_ENDED`"]
    #[inline(always)]
    pub fn is_not_ended(&self) -> bool {
        *self == EOSMPR_A::NOT_ENDED
    }
    #[doc = "Checks if the value of the field is `ENDED`"]
    #[inline(always)]
    pub fn is_ended(&self) -> bool {
        *self == EOSMPR_A::ENDED
    }
}
#[doc = "EOSMP\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EOSMPW_AW {
    #[doc = "1: Clear end of sampling phase reached flag"]
    CLEAR = 1,
}
impl From<EOSMPW_AW> for bool {
    #[inline(always)]
    fn from(variant: EOSMPW_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EOSMP` writer - EOSMP"]
pub type EOSMP_W<'a, const O: u8> = crate::BitWriter<'a, ISR_SPEC, O, EOSMPW_AW>;
impl<'a, const O: u8> EOSMP_W<'a, O> {
    #[doc = "Clear end of sampling phase reached flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(EOSMPW_AW::CLEAR)
    }
}
#[doc = "Field `EOC` reader - EOC"]
pub type EOC_R = crate::BitReader<EOCR_A>;
#[doc = "EOC\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EOCR_A {
    #[doc = "0: Regular conversion is not complete"]
    NOT_COMPLETE = 0,
    #[doc = "1: Regular conversion complete"]
    COMPLETE = 1,
}
impl From<EOCR_A> for bool {
    #[inline(always)]
    fn from(variant: EOCR_A) -> Self {
        variant as u8 != 0
    }
}
impl EOC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EOCR_A {
        match self.bits {
            false => EOCR_A::NOT_COMPLETE,
            true => EOCR_A::COMPLETE,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_COMPLETE`"]
    #[inline(always)]
    pub fn is_not_complete(&self) -> bool {
        *self == EOCR_A::NOT_COMPLETE
    }
    #[doc = "Checks if the value of the field is `COMPLETE`"]
    #[inline(always)]
    pub fn is_complete(&self) -> bool {
        *self == EOCR_A::COMPLETE
    }
}
#[doc = "EOC\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EOCW_AW {
    #[doc = "1: Clear regular conversion complete flag"]
    CLEAR = 1,
}
impl From<EOCW_AW> for bool {
    #[inline(always)]
    fn from(variant: EOCW_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EOC` writer - EOC"]
pub type EOC_W<'a, const O: u8> = crate::BitWriter<'a, ISR_SPEC, O, EOCW_AW>;
impl<'a, const O: u8> EOC_W<'a, O> {
    #[doc = "Clear regular conversion complete flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(EOCW_AW::CLEAR)
    }
}
#[doc = "Field `EOS` reader - EOS"]
pub type EOS_R = crate::BitReader<EOSR_A>;
#[doc = "EOS\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EOSR_A {
    #[doc = "0: Regular sequence is not complete"]
    NOT_COMPLETE = 0,
    #[doc = "1: Regular sequence complete"]
    COMPLETE = 1,
}
impl From<EOSR_A> for bool {
    #[inline(always)]
    fn from(variant: EOSR_A) -> Self {
        variant as u8 != 0
    }
}
impl EOS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EOSR_A {
        match self.bits {
            false => EOSR_A::NOT_COMPLETE,
            true => EOSR_A::COMPLETE,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_COMPLETE`"]
    #[inline(always)]
    pub fn is_not_complete(&self) -> bool {
        *self == EOSR_A::NOT_COMPLETE
    }
    #[doc = "Checks if the value of the field is `COMPLETE`"]
    #[inline(always)]
    pub fn is_complete(&self) -> bool {
        *self == EOSR_A::COMPLETE
    }
}
#[doc = "EOS\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EOSW_AW {
    #[doc = "1: Clear regular sequence complete flag"]
    CLEAR = 1,
}
impl From<EOSW_AW> for bool {
    #[inline(always)]
    fn from(variant: EOSW_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EOS` writer - EOS"]
pub type EOS_W<'a, const O: u8> = crate::BitWriter<'a, ISR_SPEC, O, EOSW_AW>;
impl<'a, const O: u8> EOS_W<'a, O> {
    #[doc = "Clear regular sequence complete flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(EOSW_AW::CLEAR)
    }
}
#[doc = "Field `OVR` reader - OVR"]
pub type OVR_R = crate::BitReader<OVRR_A>;
#[doc = "OVR\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OVRR_A {
    #[doc = "0: No overrun occurred"]
    NO_OVERRUN = 0,
    #[doc = "1: Overrun occurred"]
    OVERRUN = 1,
}
impl From<OVRR_A> for bool {
    #[inline(always)]
    fn from(variant: OVRR_A) -> Self {
        variant as u8 != 0
    }
}
impl OVR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OVRR_A {
        match self.bits {
            false => OVRR_A::NO_OVERRUN,
            true => OVRR_A::OVERRUN,
        }
    }
    #[doc = "Checks if the value of the field is `NO_OVERRUN`"]
    #[inline(always)]
    pub fn is_no_overrun(&self) -> bool {
        *self == OVRR_A::NO_OVERRUN
    }
    #[doc = "Checks if the value of the field is `OVERRUN`"]
    #[inline(always)]
    pub fn is_overrun(&self) -> bool {
        *self == OVRR_A::OVERRUN
    }
}
#[doc = "OVR\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OVRW_AW {
    #[doc = "1: Clear overrun occurred flag"]
    CLEAR = 1,
}
impl From<OVRW_AW> for bool {
    #[inline(always)]
    fn from(variant: OVRW_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OVR` writer - OVR"]
pub type OVR_W<'a, const O: u8> = crate::BitWriter<'a, ISR_SPEC, O, OVRW_AW>;
impl<'a, const O: u8> OVR_W<'a, O> {
    #[doc = "Clear overrun occurred flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(OVRW_AW::CLEAR)
    }
}
#[doc = "Field `JEOC` reader - JEOC"]
pub type JEOC_R = crate::BitReader<JEOCR_A>;
#[doc = "JEOC\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JEOCR_A {
    #[doc = "0: Injected conversion is not complete"]
    NOT_COMPLETE = 0,
    #[doc = "1: Injected conversion complete"]
    COMPLETE = 1,
}
impl From<JEOCR_A> for bool {
    #[inline(always)]
    fn from(variant: JEOCR_A) -> Self {
        variant as u8 != 0
    }
}
impl JEOC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> JEOCR_A {
        match self.bits {
            false => JEOCR_A::NOT_COMPLETE,
            true => JEOCR_A::COMPLETE,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_COMPLETE`"]
    #[inline(always)]
    pub fn is_not_complete(&self) -> bool {
        *self == JEOCR_A::NOT_COMPLETE
    }
    #[doc = "Checks if the value of the field is `COMPLETE`"]
    #[inline(always)]
    pub fn is_complete(&self) -> bool {
        *self == JEOCR_A::COMPLETE
    }
}
#[doc = "JEOC\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JEOCW_AW {
    #[doc = "1: Clear injected conversion complete flag"]
    CLEAR = 1,
}
impl From<JEOCW_AW> for bool {
    #[inline(always)]
    fn from(variant: JEOCW_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `JEOC` writer - JEOC"]
pub type JEOC_W<'a, const O: u8> = crate::BitWriter<'a, ISR_SPEC, O, JEOCW_AW>;
impl<'a, const O: u8> JEOC_W<'a, O> {
    #[doc = "Clear injected conversion complete flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(JEOCW_AW::CLEAR)
    }
}
#[doc = "Field `JEOS` reader - JEOS"]
pub type JEOS_R = crate::BitReader<JEOSR_A>;
#[doc = "JEOS\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JEOSR_A {
    #[doc = "0: Injected sequence is not complete"]
    NOT_COMPLETE = 0,
    #[doc = "1: Injected sequence complete"]
    COMPLETE = 1,
}
impl From<JEOSR_A> for bool {
    #[inline(always)]
    fn from(variant: JEOSR_A) -> Self {
        variant as u8 != 0
    }
}
impl JEOS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> JEOSR_A {
        match self.bits {
            false => JEOSR_A::NOT_COMPLETE,
            true => JEOSR_A::COMPLETE,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_COMPLETE`"]
    #[inline(always)]
    pub fn is_not_complete(&self) -> bool {
        *self == JEOSR_A::NOT_COMPLETE
    }
    #[doc = "Checks if the value of the field is `COMPLETE`"]
    #[inline(always)]
    pub fn is_complete(&self) -> bool {
        *self == JEOSR_A::COMPLETE
    }
}
#[doc = "JEOS\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JEOSW_AW {
    #[doc = "1: Clear Injected sequence complete flag"]
    CLEAR = 1,
}
impl From<JEOSW_AW> for bool {
    #[inline(always)]
    fn from(variant: JEOSW_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `JEOS` writer - JEOS"]
pub type JEOS_W<'a, const O: u8> = crate::BitWriter<'a, ISR_SPEC, O, JEOSW_AW>;
impl<'a, const O: u8> JEOS_W<'a, O> {
    #[doc = "Clear Injected sequence complete flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(JEOSW_AW::CLEAR)
    }
}
#[doc = "Field `AWD1` reader - AWD1"]
pub type AWD1_R = crate::BitReader<AWD1R_A>;
#[doc = "AWD1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AWD1R_A {
    #[doc = "0: No analog watchdog event occurred"]
    NO_EVENT = 0,
    #[doc = "1: Analog watchdog event occurred"]
    EVENT = 1,
}
impl From<AWD1R_A> for bool {
    #[inline(always)]
    fn from(variant: AWD1R_A) -> Self {
        variant as u8 != 0
    }
}
impl AWD1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AWD1R_A {
        match self.bits {
            false => AWD1R_A::NO_EVENT,
            true => AWD1R_A::EVENT,
        }
    }
    #[doc = "Checks if the value of the field is `NO_EVENT`"]
    #[inline(always)]
    pub fn is_no_event(&self) -> bool {
        *self == AWD1R_A::NO_EVENT
    }
    #[doc = "Checks if the value of the field is `EVENT`"]
    #[inline(always)]
    pub fn is_event(&self) -> bool {
        *self == AWD1R_A::EVENT
    }
}
#[doc = "AWD1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AWD1W_AW {
    #[doc = "1: Clear analog watchdog event occurred flag"]
    CLEAR = 1,
}
impl From<AWD1W_AW> for bool {
    #[inline(always)]
    fn from(variant: AWD1W_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AWD1` writer - AWD1"]
pub type AWD1_W<'a, const O: u8> = crate::BitWriter<'a, ISR_SPEC, O, AWD1W_AW>;
impl<'a, const O: u8> AWD1_W<'a, O> {
    #[doc = "Clear analog watchdog event occurred flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(AWD1W_AW::CLEAR)
    }
}
#[doc = "Field `AWD2` reader - AWD2"]
pub use AWD1_R as AWD2_R;
#[doc = "Field `AWD3` reader - AWD3"]
pub use AWD1_R as AWD3_R;
#[doc = "Field `AWD2` writer - AWD2"]
pub use AWD1_W as AWD2_W;
#[doc = "Field `AWD3` writer - AWD3"]
pub use AWD1_W as AWD3_W;
#[doc = "Field `JQOVF` reader - JQOVF"]
pub type JQOVF_R = crate::BitReader<JQOVFR_A>;
#[doc = "JQOVF\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JQOVFR_A {
    #[doc = "0: No injected context queue overflow has occurred"]
    NO_OVERFLOW = 0,
    #[doc = "1: Injected context queue overflow has occurred"]
    OVERFLOW = 1,
}
impl From<JQOVFR_A> for bool {
    #[inline(always)]
    fn from(variant: JQOVFR_A) -> Self {
        variant as u8 != 0
    }
}
impl JQOVF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> JQOVFR_A {
        match self.bits {
            false => JQOVFR_A::NO_OVERFLOW,
            true => JQOVFR_A::OVERFLOW,
        }
    }
    #[doc = "Checks if the value of the field is `NO_OVERFLOW`"]
    #[inline(always)]
    pub fn is_no_overflow(&self) -> bool {
        *self == JQOVFR_A::NO_OVERFLOW
    }
    #[doc = "Checks if the value of the field is `OVERFLOW`"]
    #[inline(always)]
    pub fn is_overflow(&self) -> bool {
        *self == JQOVFR_A::OVERFLOW
    }
}
#[doc = "JQOVF\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JQOVFW_AW {
    #[doc = "1: Clear injected context queue overflow flag"]
    CLEAR = 1,
}
impl From<JQOVFW_AW> for bool {
    #[inline(always)]
    fn from(variant: JQOVFW_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `JQOVF` writer - JQOVF"]
pub type JQOVF_W<'a, const O: u8> = crate::BitWriter<'a, ISR_SPEC, O, JQOVFW_AW>;
impl<'a, const O: u8> JQOVF_W<'a, O> {
    #[doc = "Clear injected context queue overflow flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(JQOVFW_AW::CLEAR)
    }
}
impl R {
    #[doc = "Bit 0 - ADRDY"]
    #[inline(always)]
    pub fn adrdy(&self) -> ADRDY_R {
        ADRDY_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - EOSMP"]
    #[inline(always)]
    pub fn eosmp(&self) -> EOSMP_R {
        EOSMP_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - EOC"]
    #[inline(always)]
    pub fn eoc(&self) -> EOC_R {
        EOC_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - EOS"]
    #[inline(always)]
    pub fn eos(&self) -> EOS_R {
        EOS_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - OVR"]
    #[inline(always)]
    pub fn ovr(&self) -> OVR_R {
        OVR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - JEOC"]
    #[inline(always)]
    pub fn jeoc(&self) -> JEOC_R {
        JEOC_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - JEOS"]
    #[inline(always)]
    pub fn jeos(&self) -> JEOS_R {
        JEOS_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - AWD1"]
    #[inline(always)]
    pub fn awd1(&self) -> AWD1_R {
        AWD1_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - AWD2"]
    #[inline(always)]
    pub fn awd2(&self) -> AWD2_R {
        AWD2_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - AWD3"]
    #[inline(always)]
    pub fn awd3(&self) -> AWD3_R {
        AWD3_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - JQOVF"]
    #[inline(always)]
    pub fn jqovf(&self) -> JQOVF_R {
        JQOVF_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ADRDY"]
    #[inline(always)]
    #[must_use]
    pub fn adrdy(&mut self) -> ADRDY_W<0> {
        ADRDY_W::new(self)
    }
    #[doc = "Bit 1 - EOSMP"]
    #[inline(always)]
    #[must_use]
    pub fn eosmp(&mut self) -> EOSMP_W<1> {
        EOSMP_W::new(self)
    }
    #[doc = "Bit 2 - EOC"]
    #[inline(always)]
    #[must_use]
    pub fn eoc(&mut self) -> EOC_W<2> {
        EOC_W::new(self)
    }
    #[doc = "Bit 3 - EOS"]
    #[inline(always)]
    #[must_use]
    pub fn eos(&mut self) -> EOS_W<3> {
        EOS_W::new(self)
    }
    #[doc = "Bit 4 - OVR"]
    #[inline(always)]
    #[must_use]
    pub fn ovr(&mut self) -> OVR_W<4> {
        OVR_W::new(self)
    }
    #[doc = "Bit 5 - JEOC"]
    #[inline(always)]
    #[must_use]
    pub fn jeoc(&mut self) -> JEOC_W<5> {
        JEOC_W::new(self)
    }
    #[doc = "Bit 6 - JEOS"]
    #[inline(always)]
    #[must_use]
    pub fn jeos(&mut self) -> JEOS_W<6> {
        JEOS_W::new(self)
    }
    #[doc = "Bit 7 - AWD1"]
    #[inline(always)]
    #[must_use]
    pub fn awd1(&mut self) -> AWD1_W<7> {
        AWD1_W::new(self)
    }
    #[doc = "Bit 8 - AWD2"]
    #[inline(always)]
    #[must_use]
    pub fn awd2(&mut self) -> AWD2_W<8> {
        AWD2_W::new(self)
    }
    #[doc = "Bit 9 - AWD3"]
    #[inline(always)]
    #[must_use]
    pub fn awd3(&mut self) -> AWD3_W<9> {
        AWD3_W::new(self)
    }
    #[doc = "Bit 10 - JQOVF"]
    #[inline(always)]
    #[must_use]
    pub fn jqovf(&mut self) -> JQOVF_W<10> {
        JQOVF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "interrupt and status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [isr](index.html) module"]
pub struct ISR_SPEC;
impl crate::RegisterSpec for ISR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [isr::R](R) reader structure"]
impl crate::Readable for ISR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [isr::W](W) writer structure"]
impl crate::Writable for ISR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ISR to value 0"]
impl crate::Resettable for ISR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

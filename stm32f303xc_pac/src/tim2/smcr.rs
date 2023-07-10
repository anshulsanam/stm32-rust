#[doc = "Register `SMCR` reader"]
pub struct R(crate::R<SMCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SMCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SMCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SMCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SMCR` writer"]
pub struct W(crate::W<SMCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SMCR_SPEC>;
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
impl From<crate::W<SMCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SMCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SMS` reader - Slave mode selection"]
pub type SMS_R = crate::FieldReader<SMS_A>;
#[doc = "Slave mode selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SMS_A {
    #[doc = "0: Slave mode disabled - if CEN = ‘1 then the prescaler is clocked directly by the internal clock."]
    DISABLED = 0,
    #[doc = "1: Encoder mode 1 - Counter counts up/down on TI2FP1 edge depending on TI1FP2 level."]
    ENCODER_MODE_1 = 1,
    #[doc = "2: Encoder mode 2 - Counter counts up/down on TI1FP2 edge depending on TI2FP1 level."]
    ENCODER_MODE_2 = 2,
    #[doc = "3: Encoder mode 3 - Counter counts up/down on both TI1FP1 and TI2FP2 edges depending on the level of the other input."]
    ENCODER_MODE_3 = 3,
    #[doc = "4: Reset Mode - Rising edge of the selected trigger input (TRGI) reinitializes the counter and generates an update of the registers."]
    RESET_MODE = 4,
    #[doc = "5: Gated Mode - The counter clock is enabled when the trigger input (TRGI) is high. The counter stops (but is not reset) as soon as the trigger becomes low. Both start and stop of the counter are controlled."]
    GATED_MODE = 5,
    #[doc = "6: Trigger Mode - The counter starts at a rising edge of the trigger TRGI (but it is not reset). Only the start of the counter is controlled."]
    TRIGGER_MODE = 6,
    #[doc = "7: External Clock Mode 1 - Rising edges of the selected trigger (TRGI) clock the counter."]
    EXT_CLOCK_MODE = 7,
}
impl From<SMS_A> for u8 {
    #[inline(always)]
    fn from(variant: SMS_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SMS_A {
    type Ux = u8;
}
impl SMS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SMS_A {
        match self.bits {
            0 => SMS_A::DISABLED,
            1 => SMS_A::ENCODER_MODE_1,
            2 => SMS_A::ENCODER_MODE_2,
            3 => SMS_A::ENCODER_MODE_3,
            4 => SMS_A::RESET_MODE,
            5 => SMS_A::GATED_MODE,
            6 => SMS_A::TRIGGER_MODE,
            7 => SMS_A::EXT_CLOCK_MODE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SMS_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENCODER_MODE_1`"]
    #[inline(always)]
    pub fn is_encoder_mode_1(&self) -> bool {
        *self == SMS_A::ENCODER_MODE_1
    }
    #[doc = "Checks if the value of the field is `ENCODER_MODE_2`"]
    #[inline(always)]
    pub fn is_encoder_mode_2(&self) -> bool {
        *self == SMS_A::ENCODER_MODE_2
    }
    #[doc = "Checks if the value of the field is `ENCODER_MODE_3`"]
    #[inline(always)]
    pub fn is_encoder_mode_3(&self) -> bool {
        *self == SMS_A::ENCODER_MODE_3
    }
    #[doc = "Checks if the value of the field is `RESET_MODE`"]
    #[inline(always)]
    pub fn is_reset_mode(&self) -> bool {
        *self == SMS_A::RESET_MODE
    }
    #[doc = "Checks if the value of the field is `GATED_MODE`"]
    #[inline(always)]
    pub fn is_gated_mode(&self) -> bool {
        *self == SMS_A::GATED_MODE
    }
    #[doc = "Checks if the value of the field is `TRIGGER_MODE`"]
    #[inline(always)]
    pub fn is_trigger_mode(&self) -> bool {
        *self == SMS_A::TRIGGER_MODE
    }
    #[doc = "Checks if the value of the field is `EXT_CLOCK_MODE`"]
    #[inline(always)]
    pub fn is_ext_clock_mode(&self) -> bool {
        *self == SMS_A::EXT_CLOCK_MODE
    }
}
#[doc = "Field `SMS` writer - Slave mode selection"]
pub type SMS_W<'a, const O: u8> = crate::FieldWriterSafe<'a, SMCR_SPEC, 3, O, SMS_A>;
impl<'a, const O: u8> SMS_W<'a, O> {
    #[doc = "Slave mode disabled - if CEN = ‘1 then the prescaler is clocked directly by the internal clock."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SMS_A::DISABLED)
    }
    #[doc = "Encoder mode 1 - Counter counts up/down on TI2FP1 edge depending on TI1FP2 level."]
    #[inline(always)]
    pub fn encoder_mode_1(self) -> &'a mut W {
        self.variant(SMS_A::ENCODER_MODE_1)
    }
    #[doc = "Encoder mode 2 - Counter counts up/down on TI1FP2 edge depending on TI2FP1 level."]
    #[inline(always)]
    pub fn encoder_mode_2(self) -> &'a mut W {
        self.variant(SMS_A::ENCODER_MODE_2)
    }
    #[doc = "Encoder mode 3 - Counter counts up/down on both TI1FP1 and TI2FP2 edges depending on the level of the other input."]
    #[inline(always)]
    pub fn encoder_mode_3(self) -> &'a mut W {
        self.variant(SMS_A::ENCODER_MODE_3)
    }
    #[doc = "Reset Mode - Rising edge of the selected trigger input (TRGI) reinitializes the counter and generates an update of the registers."]
    #[inline(always)]
    pub fn reset_mode(self) -> &'a mut W {
        self.variant(SMS_A::RESET_MODE)
    }
    #[doc = "Gated Mode - The counter clock is enabled when the trigger input (TRGI) is high. The counter stops (but is not reset) as soon as the trigger becomes low. Both start and stop of the counter are controlled."]
    #[inline(always)]
    pub fn gated_mode(self) -> &'a mut W {
        self.variant(SMS_A::GATED_MODE)
    }
    #[doc = "Trigger Mode - The counter starts at a rising edge of the trigger TRGI (but it is not reset). Only the start of the counter is controlled."]
    #[inline(always)]
    pub fn trigger_mode(self) -> &'a mut W {
        self.variant(SMS_A::TRIGGER_MODE)
    }
    #[doc = "External Clock Mode 1 - Rising edges of the selected trigger (TRGI) clock the counter."]
    #[inline(always)]
    pub fn ext_clock_mode(self) -> &'a mut W {
        self.variant(SMS_A::EXT_CLOCK_MODE)
    }
}
#[doc = "Field `OCCS` reader - OCREF clear selection"]
pub type OCCS_R = crate::BitReader;
#[doc = "Field `OCCS` writer - OCREF clear selection"]
pub type OCCS_W<'a, const O: u8> = crate::BitWriter<'a, SMCR_SPEC, O>;
#[doc = "Field `TS` reader - Trigger selection"]
pub type TS_R = crate::FieldReader<TS_A>;
#[doc = "Trigger selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TS_A {
    #[doc = "0: Internal Trigger 0 (ITR0)"]
    ITR0 = 0,
    #[doc = "1: Internal Trigger 1 (ITR1)"]
    ITR1 = 1,
    #[doc = "2: Internal Trigger 2 (ITR2)"]
    ITR2 = 2,
    #[doc = "4: TI1 Edge Detector (TI1F_ED)"]
    TI1F_ED = 4,
    #[doc = "5: Filtered Timer Input 1 (TI1FP1)"]
    TI1FP1 = 5,
    #[doc = "6: Filtered Timer Input 2 (TI2FP2)"]
    TI2FP2 = 6,
    #[doc = "7: External Trigger input (ETRF)"]
    ETRF = 7,
}
impl From<TS_A> for u8 {
    #[inline(always)]
    fn from(variant: TS_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TS_A {
    type Ux = u8;
}
impl TS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TS_A> {
        match self.bits {
            0 => Some(TS_A::ITR0),
            1 => Some(TS_A::ITR1),
            2 => Some(TS_A::ITR2),
            4 => Some(TS_A::TI1F_ED),
            5 => Some(TS_A::TI1FP1),
            6 => Some(TS_A::TI2FP2),
            7 => Some(TS_A::ETRF),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `ITR0`"]
    #[inline(always)]
    pub fn is_itr0(&self) -> bool {
        *self == TS_A::ITR0
    }
    #[doc = "Checks if the value of the field is `ITR1`"]
    #[inline(always)]
    pub fn is_itr1(&self) -> bool {
        *self == TS_A::ITR1
    }
    #[doc = "Checks if the value of the field is `ITR2`"]
    #[inline(always)]
    pub fn is_itr2(&self) -> bool {
        *self == TS_A::ITR2
    }
    #[doc = "Checks if the value of the field is `TI1F_ED`"]
    #[inline(always)]
    pub fn is_ti1f_ed(&self) -> bool {
        *self == TS_A::TI1F_ED
    }
    #[doc = "Checks if the value of the field is `TI1FP1`"]
    #[inline(always)]
    pub fn is_ti1fp1(&self) -> bool {
        *self == TS_A::TI1FP1
    }
    #[doc = "Checks if the value of the field is `TI2FP2`"]
    #[inline(always)]
    pub fn is_ti2fp2(&self) -> bool {
        *self == TS_A::TI2FP2
    }
    #[doc = "Checks if the value of the field is `ETRF`"]
    #[inline(always)]
    pub fn is_etrf(&self) -> bool {
        *self == TS_A::ETRF
    }
}
#[doc = "Field `TS` writer - Trigger selection"]
pub type TS_W<'a, const O: u8> = crate::FieldWriter<'a, SMCR_SPEC, 3, O, TS_A>;
impl<'a, const O: u8> TS_W<'a, O> {
    #[doc = "Internal Trigger 0 (ITR0)"]
    #[inline(always)]
    pub fn itr0(self) -> &'a mut W {
        self.variant(TS_A::ITR0)
    }
    #[doc = "Internal Trigger 1 (ITR1)"]
    #[inline(always)]
    pub fn itr1(self) -> &'a mut W {
        self.variant(TS_A::ITR1)
    }
    #[doc = "Internal Trigger 2 (ITR2)"]
    #[inline(always)]
    pub fn itr2(self) -> &'a mut W {
        self.variant(TS_A::ITR2)
    }
    #[doc = "TI1 Edge Detector (TI1F_ED)"]
    #[inline(always)]
    pub fn ti1f_ed(self) -> &'a mut W {
        self.variant(TS_A::TI1F_ED)
    }
    #[doc = "Filtered Timer Input 1 (TI1FP1)"]
    #[inline(always)]
    pub fn ti1fp1(self) -> &'a mut W {
        self.variant(TS_A::TI1FP1)
    }
    #[doc = "Filtered Timer Input 2 (TI2FP2)"]
    #[inline(always)]
    pub fn ti2fp2(self) -> &'a mut W {
        self.variant(TS_A::TI2FP2)
    }
    #[doc = "External Trigger input (ETRF)"]
    #[inline(always)]
    pub fn etrf(self) -> &'a mut W {
        self.variant(TS_A::ETRF)
    }
}
#[doc = "Field `MSM` reader - Master/Slave mode"]
pub type MSM_R = crate::BitReader<MSM_A>;
#[doc = "Master/Slave mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSM_A {
    #[doc = "0: No action"]
    NO_SYNC = 0,
    #[doc = "1: The effect of an event on the trigger input (TRGI) is delayed to allow a perfect synchronization between the current timer and its slaves (through TRGO). It is useful if we want to synchronize several timers on a single external event."]
    SYNC = 1,
}
impl From<MSM_A> for bool {
    #[inline(always)]
    fn from(variant: MSM_A) -> Self {
        variant as u8 != 0
    }
}
impl MSM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MSM_A {
        match self.bits {
            false => MSM_A::NO_SYNC,
            true => MSM_A::SYNC,
        }
    }
    #[doc = "Checks if the value of the field is `NO_SYNC`"]
    #[inline(always)]
    pub fn is_no_sync(&self) -> bool {
        *self == MSM_A::NO_SYNC
    }
    #[doc = "Checks if the value of the field is `SYNC`"]
    #[inline(always)]
    pub fn is_sync(&self) -> bool {
        *self == MSM_A::SYNC
    }
}
#[doc = "Field `MSM` writer - Master/Slave mode"]
pub type MSM_W<'a, const O: u8> = crate::BitWriter<'a, SMCR_SPEC, O, MSM_A>;
impl<'a, const O: u8> MSM_W<'a, O> {
    #[doc = "No action"]
    #[inline(always)]
    pub fn no_sync(self) -> &'a mut W {
        self.variant(MSM_A::NO_SYNC)
    }
    #[doc = "The effect of an event on the trigger input (TRGI) is delayed to allow a perfect synchronization between the current timer and its slaves (through TRGO). It is useful if we want to synchronize several timers on a single external event."]
    #[inline(always)]
    pub fn sync(self) -> &'a mut W {
        self.variant(MSM_A::SYNC)
    }
}
#[doc = "Field `ETF` reader - External trigger filter"]
pub type ETF_R = crate::FieldReader<ETF_A>;
#[doc = "External trigger filter\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ETF_A {
    #[doc = "0: No filter, sampling is done at fDTS"]
    NO_FILTER = 0,
    #[doc = "1: fSAMPLING=fCK_INT, N=2"]
    FCK_INT_N2 = 1,
    #[doc = "2: fSAMPLING=fCK_INT, N=4"]
    FCK_INT_N4 = 2,
    #[doc = "3: fSAMPLING=fCK_INT, N=8"]
    FCK_INT_N8 = 3,
    #[doc = "4: fSAMPLING=fDTS/2, N=6"]
    FDTS_DIV2_N6 = 4,
    #[doc = "5: fSAMPLING=fDTS/2, N=8"]
    FDTS_DIV2_N8 = 5,
    #[doc = "6: fSAMPLING=fDTS/4, N=6"]
    FDTS_DIV4_N6 = 6,
    #[doc = "7: fSAMPLING=fDTS/4, N=8"]
    FDTS_DIV4_N8 = 7,
    #[doc = "8: fSAMPLING=fDTS/8, N=6"]
    FDTS_DIV8_N6 = 8,
    #[doc = "9: fSAMPLING=fDTS/8, N=8"]
    FDTS_DIV8_N8 = 9,
    #[doc = "10: fSAMPLING=fDTS/16, N=5"]
    FDTS_DIV16_N5 = 10,
    #[doc = "11: fSAMPLING=fDTS/16, N=6"]
    FDTS_DIV16_N6 = 11,
    #[doc = "12: fSAMPLING=fDTS/16, N=8"]
    FDTS_DIV16_N8 = 12,
    #[doc = "13: fSAMPLING=fDTS/32, N=5"]
    FDTS_DIV32_N5 = 13,
    #[doc = "14: fSAMPLING=fDTS/32, N=6"]
    FDTS_DIV32_N6 = 14,
    #[doc = "15: fSAMPLING=fDTS/32, N=8"]
    FDTS_DIV32_N8 = 15,
}
impl From<ETF_A> for u8 {
    #[inline(always)]
    fn from(variant: ETF_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ETF_A {
    type Ux = u8;
}
impl ETF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ETF_A {
        match self.bits {
            0 => ETF_A::NO_FILTER,
            1 => ETF_A::FCK_INT_N2,
            2 => ETF_A::FCK_INT_N4,
            3 => ETF_A::FCK_INT_N8,
            4 => ETF_A::FDTS_DIV2_N6,
            5 => ETF_A::FDTS_DIV2_N8,
            6 => ETF_A::FDTS_DIV4_N6,
            7 => ETF_A::FDTS_DIV4_N8,
            8 => ETF_A::FDTS_DIV8_N6,
            9 => ETF_A::FDTS_DIV8_N8,
            10 => ETF_A::FDTS_DIV16_N5,
            11 => ETF_A::FDTS_DIV16_N6,
            12 => ETF_A::FDTS_DIV16_N8,
            13 => ETF_A::FDTS_DIV32_N5,
            14 => ETF_A::FDTS_DIV32_N6,
            15 => ETF_A::FDTS_DIV32_N8,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NO_FILTER`"]
    #[inline(always)]
    pub fn is_no_filter(&self) -> bool {
        *self == ETF_A::NO_FILTER
    }
    #[doc = "Checks if the value of the field is `FCK_INT_N2`"]
    #[inline(always)]
    pub fn is_fck_int_n2(&self) -> bool {
        *self == ETF_A::FCK_INT_N2
    }
    #[doc = "Checks if the value of the field is `FCK_INT_N4`"]
    #[inline(always)]
    pub fn is_fck_int_n4(&self) -> bool {
        *self == ETF_A::FCK_INT_N4
    }
    #[doc = "Checks if the value of the field is `FCK_INT_N8`"]
    #[inline(always)]
    pub fn is_fck_int_n8(&self) -> bool {
        *self == ETF_A::FCK_INT_N8
    }
    #[doc = "Checks if the value of the field is `FDTS_DIV2_N6`"]
    #[inline(always)]
    pub fn is_fdts_div2_n6(&self) -> bool {
        *self == ETF_A::FDTS_DIV2_N6
    }
    #[doc = "Checks if the value of the field is `FDTS_DIV2_N8`"]
    #[inline(always)]
    pub fn is_fdts_div2_n8(&self) -> bool {
        *self == ETF_A::FDTS_DIV2_N8
    }
    #[doc = "Checks if the value of the field is `FDTS_DIV4_N6`"]
    #[inline(always)]
    pub fn is_fdts_div4_n6(&self) -> bool {
        *self == ETF_A::FDTS_DIV4_N6
    }
    #[doc = "Checks if the value of the field is `FDTS_DIV4_N8`"]
    #[inline(always)]
    pub fn is_fdts_div4_n8(&self) -> bool {
        *self == ETF_A::FDTS_DIV4_N8
    }
    #[doc = "Checks if the value of the field is `FDTS_DIV8_N6`"]
    #[inline(always)]
    pub fn is_fdts_div8_n6(&self) -> bool {
        *self == ETF_A::FDTS_DIV8_N6
    }
    #[doc = "Checks if the value of the field is `FDTS_DIV8_N8`"]
    #[inline(always)]
    pub fn is_fdts_div8_n8(&self) -> bool {
        *self == ETF_A::FDTS_DIV8_N8
    }
    #[doc = "Checks if the value of the field is `FDTS_DIV16_N5`"]
    #[inline(always)]
    pub fn is_fdts_div16_n5(&self) -> bool {
        *self == ETF_A::FDTS_DIV16_N5
    }
    #[doc = "Checks if the value of the field is `FDTS_DIV16_N6`"]
    #[inline(always)]
    pub fn is_fdts_div16_n6(&self) -> bool {
        *self == ETF_A::FDTS_DIV16_N6
    }
    #[doc = "Checks if the value of the field is `FDTS_DIV16_N8`"]
    #[inline(always)]
    pub fn is_fdts_div16_n8(&self) -> bool {
        *self == ETF_A::FDTS_DIV16_N8
    }
    #[doc = "Checks if the value of the field is `FDTS_DIV32_N5`"]
    #[inline(always)]
    pub fn is_fdts_div32_n5(&self) -> bool {
        *self == ETF_A::FDTS_DIV32_N5
    }
    #[doc = "Checks if the value of the field is `FDTS_DIV32_N6`"]
    #[inline(always)]
    pub fn is_fdts_div32_n6(&self) -> bool {
        *self == ETF_A::FDTS_DIV32_N6
    }
    #[doc = "Checks if the value of the field is `FDTS_DIV32_N8`"]
    #[inline(always)]
    pub fn is_fdts_div32_n8(&self) -> bool {
        *self == ETF_A::FDTS_DIV32_N8
    }
}
#[doc = "Field `ETF` writer - External trigger filter"]
pub type ETF_W<'a, const O: u8> = crate::FieldWriterSafe<'a, SMCR_SPEC, 4, O, ETF_A>;
impl<'a, const O: u8> ETF_W<'a, O> {
    #[doc = "No filter, sampling is done at fDTS"]
    #[inline(always)]
    pub fn no_filter(self) -> &'a mut W {
        self.variant(ETF_A::NO_FILTER)
    }
    #[doc = "fSAMPLING=fCK_INT, N=2"]
    #[inline(always)]
    pub fn fck_int_n2(self) -> &'a mut W {
        self.variant(ETF_A::FCK_INT_N2)
    }
    #[doc = "fSAMPLING=fCK_INT, N=4"]
    #[inline(always)]
    pub fn fck_int_n4(self) -> &'a mut W {
        self.variant(ETF_A::FCK_INT_N4)
    }
    #[doc = "fSAMPLING=fCK_INT, N=8"]
    #[inline(always)]
    pub fn fck_int_n8(self) -> &'a mut W {
        self.variant(ETF_A::FCK_INT_N8)
    }
    #[doc = "fSAMPLING=fDTS/2, N=6"]
    #[inline(always)]
    pub fn fdts_div2_n6(self) -> &'a mut W {
        self.variant(ETF_A::FDTS_DIV2_N6)
    }
    #[doc = "fSAMPLING=fDTS/2, N=8"]
    #[inline(always)]
    pub fn fdts_div2_n8(self) -> &'a mut W {
        self.variant(ETF_A::FDTS_DIV2_N8)
    }
    #[doc = "fSAMPLING=fDTS/4, N=6"]
    #[inline(always)]
    pub fn fdts_div4_n6(self) -> &'a mut W {
        self.variant(ETF_A::FDTS_DIV4_N6)
    }
    #[doc = "fSAMPLING=fDTS/4, N=8"]
    #[inline(always)]
    pub fn fdts_div4_n8(self) -> &'a mut W {
        self.variant(ETF_A::FDTS_DIV4_N8)
    }
    #[doc = "fSAMPLING=fDTS/8, N=6"]
    #[inline(always)]
    pub fn fdts_div8_n6(self) -> &'a mut W {
        self.variant(ETF_A::FDTS_DIV8_N6)
    }
    #[doc = "fSAMPLING=fDTS/8, N=8"]
    #[inline(always)]
    pub fn fdts_div8_n8(self) -> &'a mut W {
        self.variant(ETF_A::FDTS_DIV8_N8)
    }
    #[doc = "fSAMPLING=fDTS/16, N=5"]
    #[inline(always)]
    pub fn fdts_div16_n5(self) -> &'a mut W {
        self.variant(ETF_A::FDTS_DIV16_N5)
    }
    #[doc = "fSAMPLING=fDTS/16, N=6"]
    #[inline(always)]
    pub fn fdts_div16_n6(self) -> &'a mut W {
        self.variant(ETF_A::FDTS_DIV16_N6)
    }
    #[doc = "fSAMPLING=fDTS/16, N=8"]
    #[inline(always)]
    pub fn fdts_div16_n8(self) -> &'a mut W {
        self.variant(ETF_A::FDTS_DIV16_N8)
    }
    #[doc = "fSAMPLING=fDTS/32, N=5"]
    #[inline(always)]
    pub fn fdts_div32_n5(self) -> &'a mut W {
        self.variant(ETF_A::FDTS_DIV32_N5)
    }
    #[doc = "fSAMPLING=fDTS/32, N=6"]
    #[inline(always)]
    pub fn fdts_div32_n6(self) -> &'a mut W {
        self.variant(ETF_A::FDTS_DIV32_N6)
    }
    #[doc = "fSAMPLING=fDTS/32, N=8"]
    #[inline(always)]
    pub fn fdts_div32_n8(self) -> &'a mut W {
        self.variant(ETF_A::FDTS_DIV32_N8)
    }
}
#[doc = "Field `ETPS` reader - External trigger prescaler"]
pub type ETPS_R = crate::FieldReader<ETPS_A>;
#[doc = "External trigger prescaler\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ETPS_A {
    #[doc = "0: Prescaler OFF"]
    DIV1 = 0,
    #[doc = "1: ETRP frequency divided by 2"]
    DIV2 = 1,
    #[doc = "2: ETRP frequency divided by 4"]
    DIV4 = 2,
    #[doc = "3: ETRP frequency divided by 8"]
    DIV8 = 3,
}
impl From<ETPS_A> for u8 {
    #[inline(always)]
    fn from(variant: ETPS_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ETPS_A {
    type Ux = u8;
}
impl ETPS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ETPS_A {
        match self.bits {
            0 => ETPS_A::DIV1,
            1 => ETPS_A::DIV2,
            2 => ETPS_A::DIV4,
            3 => ETPS_A::DIV8,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIV1`"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == ETPS_A::DIV1
    }
    #[doc = "Checks if the value of the field is `DIV2`"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == ETPS_A::DIV2
    }
    #[doc = "Checks if the value of the field is `DIV4`"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == ETPS_A::DIV4
    }
    #[doc = "Checks if the value of the field is `DIV8`"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == ETPS_A::DIV8
    }
}
#[doc = "Field `ETPS` writer - External trigger prescaler"]
pub type ETPS_W<'a, const O: u8> = crate::FieldWriterSafe<'a, SMCR_SPEC, 2, O, ETPS_A>;
impl<'a, const O: u8> ETPS_W<'a, O> {
    #[doc = "Prescaler OFF"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut W {
        self.variant(ETPS_A::DIV1)
    }
    #[doc = "ETRP frequency divided by 2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut W {
        self.variant(ETPS_A::DIV2)
    }
    #[doc = "ETRP frequency divided by 4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut W {
        self.variant(ETPS_A::DIV4)
    }
    #[doc = "ETRP frequency divided by 8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut W {
        self.variant(ETPS_A::DIV8)
    }
}
#[doc = "Field `ECE` reader - External clock enable"]
pub type ECE_R = crate::BitReader<ECE_A>;
#[doc = "External clock enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ECE_A {
    #[doc = "0: External clock mode 2 disabled"]
    DISABLED = 0,
    #[doc = "1: External clock mode 2 enabled. The counter is clocked by any active edge on the ETRF signal."]
    ENABLED = 1,
}
impl From<ECE_A> for bool {
    #[inline(always)]
    fn from(variant: ECE_A) -> Self {
        variant as u8 != 0
    }
}
impl ECE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ECE_A {
        match self.bits {
            false => ECE_A::DISABLED,
            true => ECE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ECE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ECE_A::ENABLED
    }
}
#[doc = "Field `ECE` writer - External clock enable"]
pub type ECE_W<'a, const O: u8> = crate::BitWriter<'a, SMCR_SPEC, O, ECE_A>;
impl<'a, const O: u8> ECE_W<'a, O> {
    #[doc = "External clock mode 2 disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ECE_A::DISABLED)
    }
    #[doc = "External clock mode 2 enabled. The counter is clocked by any active edge on the ETRF signal."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ECE_A::ENABLED)
    }
}
#[doc = "Field `ETP` reader - External trigger polarity"]
pub type ETP_R = crate::BitReader<ETP_A>;
#[doc = "External trigger polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ETP_A {
    #[doc = "0: ETR is noninverted, active at high level or rising edge"]
    NOT_INVERTED = 0,
    #[doc = "1: ETR is inverted, active at low level or falling edge"]
    INVERTED = 1,
}
impl From<ETP_A> for bool {
    #[inline(always)]
    fn from(variant: ETP_A) -> Self {
        variant as u8 != 0
    }
}
impl ETP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ETP_A {
        match self.bits {
            false => ETP_A::NOT_INVERTED,
            true => ETP_A::INVERTED,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_INVERTED`"]
    #[inline(always)]
    pub fn is_not_inverted(&self) -> bool {
        *self == ETP_A::NOT_INVERTED
    }
    #[doc = "Checks if the value of the field is `INVERTED`"]
    #[inline(always)]
    pub fn is_inverted(&self) -> bool {
        *self == ETP_A::INVERTED
    }
}
#[doc = "Field `ETP` writer - External trigger polarity"]
pub type ETP_W<'a, const O: u8> = crate::BitWriter<'a, SMCR_SPEC, O, ETP_A>;
impl<'a, const O: u8> ETP_W<'a, O> {
    #[doc = "ETR is noninverted, active at high level or rising edge"]
    #[inline(always)]
    pub fn not_inverted(self) -> &'a mut W {
        self.variant(ETP_A::NOT_INVERTED)
    }
    #[doc = "ETR is inverted, active at low level or falling edge"]
    #[inline(always)]
    pub fn inverted(self) -> &'a mut W {
        self.variant(ETP_A::INVERTED)
    }
}
#[doc = "Field `SMS_3` reader - Slave mode selection bit3"]
pub type SMS_3_R = crate::BitReader;
#[doc = "Field `SMS_3` writer - Slave mode selection bit3"]
pub type SMS_3_W<'a, const O: u8> = crate::BitWriter<'a, SMCR_SPEC, O>;
impl R {
    #[doc = "Bits 0:2 - Slave mode selection"]
    #[inline(always)]
    pub fn sms(&self) -> SMS_R {
        SMS_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - OCREF clear selection"]
    #[inline(always)]
    pub fn occs(&self) -> OCCS_R {
        OCCS_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - Trigger selection"]
    #[inline(always)]
    pub fn ts(&self) -> TS_R {
        TS_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - Master/Slave mode"]
    #[inline(always)]
    pub fn msm(&self) -> MSM_R {
        MSM_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:11 - External trigger filter"]
    #[inline(always)]
    pub fn etf(&self) -> ETF_R {
        ETF_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:13 - External trigger prescaler"]
    #[inline(always)]
    pub fn etps(&self) -> ETPS_R {
        ETPS_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 14 - External clock enable"]
    #[inline(always)]
    pub fn ece(&self) -> ECE_R {
        ECE_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - External trigger polarity"]
    #[inline(always)]
    pub fn etp(&self) -> ETP_R {
        ETP_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Slave mode selection bit3"]
    #[inline(always)]
    pub fn sms_3(&self) -> SMS_3_R {
        SMS_3_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Slave mode selection"]
    #[inline(always)]
    #[must_use]
    pub fn sms(&mut self) -> SMS_W<0> {
        SMS_W::new(self)
    }
    #[doc = "Bit 3 - OCREF clear selection"]
    #[inline(always)]
    #[must_use]
    pub fn occs(&mut self) -> OCCS_W<3> {
        OCCS_W::new(self)
    }
    #[doc = "Bits 4:6 - Trigger selection"]
    #[inline(always)]
    #[must_use]
    pub fn ts(&mut self) -> TS_W<4> {
        TS_W::new(self)
    }
    #[doc = "Bit 7 - Master/Slave mode"]
    #[inline(always)]
    #[must_use]
    pub fn msm(&mut self) -> MSM_W<7> {
        MSM_W::new(self)
    }
    #[doc = "Bits 8:11 - External trigger filter"]
    #[inline(always)]
    #[must_use]
    pub fn etf(&mut self) -> ETF_W<8> {
        ETF_W::new(self)
    }
    #[doc = "Bits 12:13 - External trigger prescaler"]
    #[inline(always)]
    #[must_use]
    pub fn etps(&mut self) -> ETPS_W<12> {
        ETPS_W::new(self)
    }
    #[doc = "Bit 14 - External clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn ece(&mut self) -> ECE_W<14> {
        ECE_W::new(self)
    }
    #[doc = "Bit 15 - External trigger polarity"]
    #[inline(always)]
    #[must_use]
    pub fn etp(&mut self) -> ETP_W<15> {
        ETP_W::new(self)
    }
    #[doc = "Bit 16 - Slave mode selection bit3"]
    #[inline(always)]
    #[must_use]
    pub fn sms_3(&mut self) -> SMS_3_W<16> {
        SMS_3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "slave mode control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [smcr](index.html) module"]
pub struct SMCR_SPEC;
impl crate::RegisterSpec for SMCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [smcr::R](R) reader structure"]
impl crate::Readable for SMCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [smcr::W](W) writer structure"]
impl crate::Writable for SMCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SMCR to value 0"]
impl crate::Resettable for SMCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

#[doc = "Register `COMP2_CSR` reader"]
pub struct R(crate::R<COMP2_CSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<COMP2_CSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<COMP2_CSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<COMP2_CSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `COMP2_CSR` writer"]
pub struct W(crate::W<COMP2_CSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<COMP2_CSR_SPEC>;
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
impl From<crate::W<COMP2_CSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<COMP2_CSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `COMP2EN` reader - Comparator 2 enable"]
pub type COMP2EN_R = crate::BitReader<COMP2EN_A>;
#[doc = "Comparator 2 enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum COMP2EN_A {
    #[doc = "0: Comparator disabled"]
    DISABLED = 0,
    #[doc = "1: Comparator enabled"]
    ENABLED = 1,
}
impl From<COMP2EN_A> for bool {
    #[inline(always)]
    fn from(variant: COMP2EN_A) -> Self {
        variant as u8 != 0
    }
}
impl COMP2EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COMP2EN_A {
        match self.bits {
            false => COMP2EN_A::DISABLED,
            true => COMP2EN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == COMP2EN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == COMP2EN_A::ENABLED
    }
}
#[doc = "Field `COMP2EN` writer - Comparator 2 enable"]
pub type COMP2EN_W<'a, const O: u8> = crate::BitWriter<'a, COMP2_CSR_SPEC, O, COMP2EN_A>;
impl<'a, const O: u8> COMP2EN_W<'a, O> {
    #[doc = "Comparator disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(COMP2EN_A::DISABLED)
    }
    #[doc = "Comparator enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(COMP2EN_A::ENABLED)
    }
}
#[doc = "Field `COMP2MODE` reader - Comparator 2 mode"]
pub type COMP2MODE_R = crate::FieldReader;
#[doc = "Field `COMP2MODE` writer - Comparator 2 mode"]
pub type COMP2MODE_W<'a, const O: u8> = crate::FieldWriter<'a, COMP2_CSR_SPEC, 2, O>;
#[doc = "Field `COMP2INMSEL` reader - Comparator 2 inverting input selection"]
pub type COMP2INMSEL_R = crate::FieldReader<COMP2INMSEL_A>;
#[doc = "Comparator 2 inverting input selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum COMP2INMSEL_A {
    #[doc = "0: 1/4 of VRefint"]
    ONE_QUARTER_VREF = 0,
    #[doc = "1: 1/2 of VRefint"]
    ONE_HALF_VREF = 1,
    #[doc = "2: 3/4 of VRefint"]
    THREE_QUARTER_VREF = 2,
    #[doc = "3: VRefint"]
    VREF = 3,
    #[doc = "4: PA4 or DAC1_CH1 output if enabled"]
    PA4_DAC1_CH1 = 4,
    #[doc = "5: DAC1_CH2"]
    DAC1_CH2 = 5,
    #[doc = "6: PA2"]
    PA2 = 6,
}
impl From<COMP2INMSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: COMP2INMSEL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for COMP2INMSEL_A {
    type Ux = u8;
}
impl COMP2INMSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<COMP2INMSEL_A> {
        match self.bits {
            0 => Some(COMP2INMSEL_A::ONE_QUARTER_VREF),
            1 => Some(COMP2INMSEL_A::ONE_HALF_VREF),
            2 => Some(COMP2INMSEL_A::THREE_QUARTER_VREF),
            3 => Some(COMP2INMSEL_A::VREF),
            4 => Some(COMP2INMSEL_A::PA4_DAC1_CH1),
            5 => Some(COMP2INMSEL_A::DAC1_CH2),
            6 => Some(COMP2INMSEL_A::PA2),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `ONE_QUARTER_VREF`"]
    #[inline(always)]
    pub fn is_one_quarter_vref(&self) -> bool {
        *self == COMP2INMSEL_A::ONE_QUARTER_VREF
    }
    #[doc = "Checks if the value of the field is `ONE_HALF_VREF`"]
    #[inline(always)]
    pub fn is_one_half_vref(&self) -> bool {
        *self == COMP2INMSEL_A::ONE_HALF_VREF
    }
    #[doc = "Checks if the value of the field is `THREE_QUARTER_VREF`"]
    #[inline(always)]
    pub fn is_three_quarter_vref(&self) -> bool {
        *self == COMP2INMSEL_A::THREE_QUARTER_VREF
    }
    #[doc = "Checks if the value of the field is `VREF`"]
    #[inline(always)]
    pub fn is_vref(&self) -> bool {
        *self == COMP2INMSEL_A::VREF
    }
    #[doc = "Checks if the value of the field is `PA4_DAC1_CH1`"]
    #[inline(always)]
    pub fn is_pa4_dac1_ch1(&self) -> bool {
        *self == COMP2INMSEL_A::PA4_DAC1_CH1
    }
    #[doc = "Checks if the value of the field is `DAC1_CH2`"]
    #[inline(always)]
    pub fn is_dac1_ch2(&self) -> bool {
        *self == COMP2INMSEL_A::DAC1_CH2
    }
    #[doc = "Checks if the value of the field is `PA2`"]
    #[inline(always)]
    pub fn is_pa2(&self) -> bool {
        *self == COMP2INMSEL_A::PA2
    }
}
#[doc = "Field `COMP2INMSEL` writer - Comparator 2 inverting input selection"]
pub type COMP2INMSEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, COMP2_CSR_SPEC, 3, O, COMP2INMSEL_A>;
impl<'a, const O: u8> COMP2INMSEL_W<'a, O> {
    #[doc = "1/4 of VRefint"]
    #[inline(always)]
    pub fn one_quarter_vref(self) -> &'a mut W {
        self.variant(COMP2INMSEL_A::ONE_QUARTER_VREF)
    }
    #[doc = "1/2 of VRefint"]
    #[inline(always)]
    pub fn one_half_vref(self) -> &'a mut W {
        self.variant(COMP2INMSEL_A::ONE_HALF_VREF)
    }
    #[doc = "3/4 of VRefint"]
    #[inline(always)]
    pub fn three_quarter_vref(self) -> &'a mut W {
        self.variant(COMP2INMSEL_A::THREE_QUARTER_VREF)
    }
    #[doc = "VRefint"]
    #[inline(always)]
    pub fn vref(self) -> &'a mut W {
        self.variant(COMP2INMSEL_A::VREF)
    }
    #[doc = "PA4 or DAC1_CH1 output if enabled"]
    #[inline(always)]
    pub fn pa4_dac1_ch1(self) -> &'a mut W {
        self.variant(COMP2INMSEL_A::PA4_DAC1_CH1)
    }
    #[doc = "DAC1_CH2"]
    #[inline(always)]
    pub fn dac1_ch2(self) -> &'a mut W {
        self.variant(COMP2INMSEL_A::DAC1_CH2)
    }
    #[doc = "PA2"]
    #[inline(always)]
    pub fn pa2(self) -> &'a mut W {
        self.variant(COMP2INMSEL_A::PA2)
    }
}
#[doc = "Field `COMP2INPSEL` reader - Comparator 2 non inverted input"]
pub type COMP2INPSEL_R = crate::BitReader;
#[doc = "Field `COMP2INPSEL` writer - Comparator 2 non inverted input"]
pub type COMP2INPSEL_W<'a, const O: u8> = crate::BitWriter<'a, COMP2_CSR_SPEC, O>;
#[doc = "Field `COMP2WINMODE` reader - Comparator 2 window mode"]
pub type COMP2WINMODE_R = crate::BitReader;
#[doc = "Field `COMP2WINMODE` writer - Comparator 2 window mode"]
pub type COMP2WINMODE_W<'a, const O: u8> = crate::BitWriter<'a, COMP2_CSR_SPEC, O>;
#[doc = "Field `COMP2OUTSEL` reader - Comparator 2 output selection"]
pub type COMP2OUTSEL_R = crate::FieldReader<COMP2OUTSEL_A>;
#[doc = "Comparator 2 output selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum COMP2OUTSEL_A {
    #[doc = "0: No selection"]
    NO_SELECTION = 0,
    #[doc = "1: Timer 1 break input"]
    TIMER1BREAK_INPUT = 1,
    #[doc = "2: Timer 1 break input 2"]
    TIMER1BREAK_INPUT2 = 2,
    #[doc = "6: Timer 1 OCREF_CLR input"]
    TIMER1OCREF_CLEAR_INPUT = 6,
    #[doc = "7: Timer 1 input capture 1"]
    TIMER1INPUT_CAPTURE1 = 7,
    #[doc = "8: Timer 2 input capture 4"]
    TIMER2INPUT_CAPTURE4 = 8,
    #[doc = "9: Timer 2 OCREF_CLR input"]
    TIMER2OCREF_CLEAR_INPUT = 9,
    #[doc = "10: Timer 3 input capture 1"]
    TIMER3INPUT_CAPTURE1 = 10,
    #[doc = "11: Timer 3 OCREF_CLR input"]
    TIMER3OCREF_CLEAR_INPUT = 11,
}
impl From<COMP2OUTSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: COMP2OUTSEL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for COMP2OUTSEL_A {
    type Ux = u8;
}
impl COMP2OUTSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<COMP2OUTSEL_A> {
        match self.bits {
            0 => Some(COMP2OUTSEL_A::NO_SELECTION),
            1 => Some(COMP2OUTSEL_A::TIMER1BREAK_INPUT),
            2 => Some(COMP2OUTSEL_A::TIMER1BREAK_INPUT2),
            6 => Some(COMP2OUTSEL_A::TIMER1OCREF_CLEAR_INPUT),
            7 => Some(COMP2OUTSEL_A::TIMER1INPUT_CAPTURE1),
            8 => Some(COMP2OUTSEL_A::TIMER2INPUT_CAPTURE4),
            9 => Some(COMP2OUTSEL_A::TIMER2OCREF_CLEAR_INPUT),
            10 => Some(COMP2OUTSEL_A::TIMER3INPUT_CAPTURE1),
            11 => Some(COMP2OUTSEL_A::TIMER3OCREF_CLEAR_INPUT),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NO_SELECTION`"]
    #[inline(always)]
    pub fn is_no_selection(&self) -> bool {
        *self == COMP2OUTSEL_A::NO_SELECTION
    }
    #[doc = "Checks if the value of the field is `TIMER1BREAK_INPUT`"]
    #[inline(always)]
    pub fn is_timer1break_input(&self) -> bool {
        *self == COMP2OUTSEL_A::TIMER1BREAK_INPUT
    }
    #[doc = "Checks if the value of the field is `TIMER1BREAK_INPUT2`"]
    #[inline(always)]
    pub fn is_timer1break_input2(&self) -> bool {
        *self == COMP2OUTSEL_A::TIMER1BREAK_INPUT2
    }
    #[doc = "Checks if the value of the field is `TIMER1OCREF_CLEAR_INPUT`"]
    #[inline(always)]
    pub fn is_timer1ocref_clear_input(&self) -> bool {
        *self == COMP2OUTSEL_A::TIMER1OCREF_CLEAR_INPUT
    }
    #[doc = "Checks if the value of the field is `TIMER1INPUT_CAPTURE1`"]
    #[inline(always)]
    pub fn is_timer1input_capture1(&self) -> bool {
        *self == COMP2OUTSEL_A::TIMER1INPUT_CAPTURE1
    }
    #[doc = "Checks if the value of the field is `TIMER2INPUT_CAPTURE4`"]
    #[inline(always)]
    pub fn is_timer2input_capture4(&self) -> bool {
        *self == COMP2OUTSEL_A::TIMER2INPUT_CAPTURE4
    }
    #[doc = "Checks if the value of the field is `TIMER2OCREF_CLEAR_INPUT`"]
    #[inline(always)]
    pub fn is_timer2ocref_clear_input(&self) -> bool {
        *self == COMP2OUTSEL_A::TIMER2OCREF_CLEAR_INPUT
    }
    #[doc = "Checks if the value of the field is `TIMER3INPUT_CAPTURE1`"]
    #[inline(always)]
    pub fn is_timer3input_capture1(&self) -> bool {
        *self == COMP2OUTSEL_A::TIMER3INPUT_CAPTURE1
    }
    #[doc = "Checks if the value of the field is `TIMER3OCREF_CLEAR_INPUT`"]
    #[inline(always)]
    pub fn is_timer3ocref_clear_input(&self) -> bool {
        *self == COMP2OUTSEL_A::TIMER3OCREF_CLEAR_INPUT
    }
}
#[doc = "Field `COMP2OUTSEL` writer - Comparator 2 output selection"]
pub type COMP2OUTSEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, COMP2_CSR_SPEC, 4, O, COMP2OUTSEL_A>;
impl<'a, const O: u8> COMP2OUTSEL_W<'a, O> {
    #[doc = "No selection"]
    #[inline(always)]
    pub fn no_selection(self) -> &'a mut W {
        self.variant(COMP2OUTSEL_A::NO_SELECTION)
    }
    #[doc = "Timer 1 break input"]
    #[inline(always)]
    pub fn timer1break_input(self) -> &'a mut W {
        self.variant(COMP2OUTSEL_A::TIMER1BREAK_INPUT)
    }
    #[doc = "Timer 1 break input 2"]
    #[inline(always)]
    pub fn timer1break_input2(self) -> &'a mut W {
        self.variant(COMP2OUTSEL_A::TIMER1BREAK_INPUT2)
    }
    #[doc = "Timer 1 OCREF_CLR input"]
    #[inline(always)]
    pub fn timer1ocref_clear_input(self) -> &'a mut W {
        self.variant(COMP2OUTSEL_A::TIMER1OCREF_CLEAR_INPUT)
    }
    #[doc = "Timer 1 input capture 1"]
    #[inline(always)]
    pub fn timer1input_capture1(self) -> &'a mut W {
        self.variant(COMP2OUTSEL_A::TIMER1INPUT_CAPTURE1)
    }
    #[doc = "Timer 2 input capture 4"]
    #[inline(always)]
    pub fn timer2input_capture4(self) -> &'a mut W {
        self.variant(COMP2OUTSEL_A::TIMER2INPUT_CAPTURE4)
    }
    #[doc = "Timer 2 OCREF_CLR input"]
    #[inline(always)]
    pub fn timer2ocref_clear_input(self) -> &'a mut W {
        self.variant(COMP2OUTSEL_A::TIMER2OCREF_CLEAR_INPUT)
    }
    #[doc = "Timer 3 input capture 1"]
    #[inline(always)]
    pub fn timer3input_capture1(self) -> &'a mut W {
        self.variant(COMP2OUTSEL_A::TIMER3INPUT_CAPTURE1)
    }
    #[doc = "Timer 3 OCREF_CLR input"]
    #[inline(always)]
    pub fn timer3ocref_clear_input(self) -> &'a mut W {
        self.variant(COMP2OUTSEL_A::TIMER3OCREF_CLEAR_INPUT)
    }
}
#[doc = "Field `COMP2POL` reader - Comparator 2 output polarity"]
pub type COMP2POL_R = crate::BitReader<COMP2POL_A>;
#[doc = "Comparator 2 output polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum COMP2POL_A {
    #[doc = "0: Output is not inverted"]
    NOT_INVERTED = 0,
    #[doc = "1: Output is inverted"]
    INVERTED = 1,
}
impl From<COMP2POL_A> for bool {
    #[inline(always)]
    fn from(variant: COMP2POL_A) -> Self {
        variant as u8 != 0
    }
}
impl COMP2POL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COMP2POL_A {
        match self.bits {
            false => COMP2POL_A::NOT_INVERTED,
            true => COMP2POL_A::INVERTED,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_INVERTED`"]
    #[inline(always)]
    pub fn is_not_inverted(&self) -> bool {
        *self == COMP2POL_A::NOT_INVERTED
    }
    #[doc = "Checks if the value of the field is `INVERTED`"]
    #[inline(always)]
    pub fn is_inverted(&self) -> bool {
        *self == COMP2POL_A::INVERTED
    }
}
#[doc = "Field `COMP2POL` writer - Comparator 2 output polarity"]
pub type COMP2POL_W<'a, const O: u8> = crate::BitWriter<'a, COMP2_CSR_SPEC, O, COMP2POL_A>;
impl<'a, const O: u8> COMP2POL_W<'a, O> {
    #[doc = "Output is not inverted"]
    #[inline(always)]
    pub fn not_inverted(self) -> &'a mut W {
        self.variant(COMP2POL_A::NOT_INVERTED)
    }
    #[doc = "Output is inverted"]
    #[inline(always)]
    pub fn inverted(self) -> &'a mut W {
        self.variant(COMP2POL_A::INVERTED)
    }
}
#[doc = "Field `COMP2HYST` reader - Comparator 2 hysteresis"]
pub type COMP2HYST_R = crate::FieldReader;
#[doc = "Field `COMP2HYST` writer - Comparator 2 hysteresis"]
pub type COMP2HYST_W<'a, const O: u8> = crate::FieldWriter<'a, COMP2_CSR_SPEC, 2, O>;
#[doc = "Field `COMP2_BLANKING` reader - Comparator 2 blanking source"]
pub type COMP2_BLANKING_R = crate::FieldReader<COMP2_BLANKING_A>;
#[doc = "Comparator 2 blanking source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum COMP2_BLANKING_A {
    #[doc = "0: No blanking"]
    NO_BLANKING = 0,
    #[doc = "1: TIM1 OC5 selected as blanking source"]
    TIM1OC5 = 1,
    #[doc = "2: TIM2 OC3 selected as blanking source"]
    TIM2OC3 = 2,
    #[doc = "3: TIM3 OC3 selected as blanking source"]
    TIM3OC3 = 3,
}
impl From<COMP2_BLANKING_A> for u8 {
    #[inline(always)]
    fn from(variant: COMP2_BLANKING_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for COMP2_BLANKING_A {
    type Ux = u8;
}
impl COMP2_BLANKING_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<COMP2_BLANKING_A> {
        match self.bits {
            0 => Some(COMP2_BLANKING_A::NO_BLANKING),
            1 => Some(COMP2_BLANKING_A::TIM1OC5),
            2 => Some(COMP2_BLANKING_A::TIM2OC3),
            3 => Some(COMP2_BLANKING_A::TIM3OC3),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NO_BLANKING`"]
    #[inline(always)]
    pub fn is_no_blanking(&self) -> bool {
        *self == COMP2_BLANKING_A::NO_BLANKING
    }
    #[doc = "Checks if the value of the field is `TIM1OC5`"]
    #[inline(always)]
    pub fn is_tim1oc5(&self) -> bool {
        *self == COMP2_BLANKING_A::TIM1OC5
    }
    #[doc = "Checks if the value of the field is `TIM2OC3`"]
    #[inline(always)]
    pub fn is_tim2oc3(&self) -> bool {
        *self == COMP2_BLANKING_A::TIM2OC3
    }
    #[doc = "Checks if the value of the field is `TIM3OC3`"]
    #[inline(always)]
    pub fn is_tim3oc3(&self) -> bool {
        *self == COMP2_BLANKING_A::TIM3OC3
    }
}
#[doc = "Field `COMP2_BLANKING` writer - Comparator 2 blanking source"]
pub type COMP2_BLANKING_W<'a, const O: u8> =
    crate::FieldWriter<'a, COMP2_CSR_SPEC, 3, O, COMP2_BLANKING_A>;
impl<'a, const O: u8> COMP2_BLANKING_W<'a, O> {
    #[doc = "No blanking"]
    #[inline(always)]
    pub fn no_blanking(self) -> &'a mut W {
        self.variant(COMP2_BLANKING_A::NO_BLANKING)
    }
    #[doc = "TIM1 OC5 selected as blanking source"]
    #[inline(always)]
    pub fn tim1oc5(self) -> &'a mut W {
        self.variant(COMP2_BLANKING_A::TIM1OC5)
    }
    #[doc = "TIM2 OC3 selected as blanking source"]
    #[inline(always)]
    pub fn tim2oc3(self) -> &'a mut W {
        self.variant(COMP2_BLANKING_A::TIM2OC3)
    }
    #[doc = "TIM3 OC3 selected as blanking source"]
    #[inline(always)]
    pub fn tim3oc3(self) -> &'a mut W {
        self.variant(COMP2_BLANKING_A::TIM3OC3)
    }
}
#[doc = "Field `COMP2INMSEL3` reader - Comparator 2 inverting input selection"]
pub type COMP2INMSEL3_R = crate::BitReader;
#[doc = "Field `COMP2INMSEL3` writer - Comparator 2 inverting input selection"]
pub type COMP2INMSEL3_W<'a, const O: u8> = crate::BitWriter<'a, COMP2_CSR_SPEC, O>;
#[doc = "Field `COMP2OUT` reader - Comparator 2 output"]
pub type COMP2OUT_R = crate::BitReader<COMP2OUT_A>;
#[doc = "Comparator 2 output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum COMP2OUT_A {
    #[doc = "0: Non-inverting input below inverting input"]
    LOW = 0,
    #[doc = "1: Non-inverting input above inverting input"]
    HIGH = 1,
}
impl From<COMP2OUT_A> for bool {
    #[inline(always)]
    fn from(variant: COMP2OUT_A) -> Self {
        variant as u8 != 0
    }
}
impl COMP2OUT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COMP2OUT_A {
        match self.bits {
            false => COMP2OUT_A::LOW,
            true => COMP2OUT_A::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == COMP2OUT_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == COMP2OUT_A::HIGH
    }
}
#[doc = "Field `COMP2LOCK` reader - Comparator 2 lock"]
pub type COMP2LOCK_R = crate::BitReader<COMP2LOCK_A>;
#[doc = "Comparator 2 lock\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum COMP2LOCK_A {
    #[doc = "0: Comparator CSR bits are read-write"]
    UNLOCKED = 0,
    #[doc = "1: Comparator CSR bits are read-only"]
    LOCKED = 1,
}
impl From<COMP2LOCK_A> for bool {
    #[inline(always)]
    fn from(variant: COMP2LOCK_A) -> Self {
        variant as u8 != 0
    }
}
impl COMP2LOCK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COMP2LOCK_A {
        match self.bits {
            false => COMP2LOCK_A::UNLOCKED,
            true => COMP2LOCK_A::LOCKED,
        }
    }
    #[doc = "Checks if the value of the field is `UNLOCKED`"]
    #[inline(always)]
    pub fn is_unlocked(&self) -> bool {
        *self == COMP2LOCK_A::UNLOCKED
    }
    #[doc = "Checks if the value of the field is `LOCKED`"]
    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        *self == COMP2LOCK_A::LOCKED
    }
}
#[doc = "Field `COMP2LOCK` writer - Comparator 2 lock"]
pub type COMP2LOCK_W<'a, const O: u8> = crate::BitWriter<'a, COMP2_CSR_SPEC, O, COMP2LOCK_A>;
impl<'a, const O: u8> COMP2LOCK_W<'a, O> {
    #[doc = "Comparator CSR bits are read-write"]
    #[inline(always)]
    pub fn unlocked(self) -> &'a mut W {
        self.variant(COMP2LOCK_A::UNLOCKED)
    }
    #[doc = "Comparator CSR bits are read-only"]
    #[inline(always)]
    pub fn locked(self) -> &'a mut W {
        self.variant(COMP2LOCK_A::LOCKED)
    }
}
impl R {
    #[doc = "Bit 0 - Comparator 2 enable"]
    #[inline(always)]
    pub fn comp2en(&self) -> COMP2EN_R {
        COMP2EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 2:3 - Comparator 2 mode"]
    #[inline(always)]
    pub fn comp2mode(&self) -> COMP2MODE_R {
        COMP2MODE_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:6 - Comparator 2 inverting input selection"]
    #[inline(always)]
    pub fn comp2inmsel(&self) -> COMP2INMSEL_R {
        COMP2INMSEL_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - Comparator 2 non inverted input"]
    #[inline(always)]
    pub fn comp2inpsel(&self) -> COMP2INPSEL_R {
        COMP2INPSEL_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 9 - Comparator 2 window mode"]
    #[inline(always)]
    pub fn comp2winmode(&self) -> COMP2WINMODE_R {
        COMP2WINMODE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:13 - Comparator 2 output selection"]
    #[inline(always)]
    pub fn comp2outsel(&self) -> COMP2OUTSEL_R {
        COMP2OUTSEL_R::new(((self.bits >> 10) & 0x0f) as u8)
    }
    #[doc = "Bit 15 - Comparator 2 output polarity"]
    #[inline(always)]
    pub fn comp2pol(&self) -> COMP2POL_R {
        COMP2POL_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:17 - Comparator 2 hysteresis"]
    #[inline(always)]
    pub fn comp2hyst(&self) -> COMP2HYST_R {
        COMP2HYST_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:20 - Comparator 2 blanking source"]
    #[inline(always)]
    pub fn comp2_blanking(&self) -> COMP2_BLANKING_R {
        COMP2_BLANKING_R::new(((self.bits >> 18) & 7) as u8)
    }
    #[doc = "Bit 22 - Comparator 2 inverting input selection"]
    #[inline(always)]
    pub fn comp2inmsel3(&self) -> COMP2INMSEL3_R {
        COMP2INMSEL3_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 30 - Comparator 2 output"]
    #[inline(always)]
    pub fn comp2out(&self) -> COMP2OUT_R {
        COMP2OUT_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Comparator 2 lock"]
    #[inline(always)]
    pub fn comp2lock(&self) -> COMP2LOCK_R {
        COMP2LOCK_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Comparator 2 enable"]
    #[inline(always)]
    #[must_use]
    pub fn comp2en(&mut self) -> COMP2EN_W<0> {
        COMP2EN_W::new(self)
    }
    #[doc = "Bits 2:3 - Comparator 2 mode"]
    #[inline(always)]
    #[must_use]
    pub fn comp2mode(&mut self) -> COMP2MODE_W<2> {
        COMP2MODE_W::new(self)
    }
    #[doc = "Bits 4:6 - Comparator 2 inverting input selection"]
    #[inline(always)]
    #[must_use]
    pub fn comp2inmsel(&mut self) -> COMP2INMSEL_W<4> {
        COMP2INMSEL_W::new(self)
    }
    #[doc = "Bit 7 - Comparator 2 non inverted input"]
    #[inline(always)]
    #[must_use]
    pub fn comp2inpsel(&mut self) -> COMP2INPSEL_W<7> {
        COMP2INPSEL_W::new(self)
    }
    #[doc = "Bit 9 - Comparator 2 window mode"]
    #[inline(always)]
    #[must_use]
    pub fn comp2winmode(&mut self) -> COMP2WINMODE_W<9> {
        COMP2WINMODE_W::new(self)
    }
    #[doc = "Bits 10:13 - Comparator 2 output selection"]
    #[inline(always)]
    #[must_use]
    pub fn comp2outsel(&mut self) -> COMP2OUTSEL_W<10> {
        COMP2OUTSEL_W::new(self)
    }
    #[doc = "Bit 15 - Comparator 2 output polarity"]
    #[inline(always)]
    #[must_use]
    pub fn comp2pol(&mut self) -> COMP2POL_W<15> {
        COMP2POL_W::new(self)
    }
    #[doc = "Bits 16:17 - Comparator 2 hysteresis"]
    #[inline(always)]
    #[must_use]
    pub fn comp2hyst(&mut self) -> COMP2HYST_W<16> {
        COMP2HYST_W::new(self)
    }
    #[doc = "Bits 18:20 - Comparator 2 blanking source"]
    #[inline(always)]
    #[must_use]
    pub fn comp2_blanking(&mut self) -> COMP2_BLANKING_W<18> {
        COMP2_BLANKING_W::new(self)
    }
    #[doc = "Bit 22 - Comparator 2 inverting input selection"]
    #[inline(always)]
    #[must_use]
    pub fn comp2inmsel3(&mut self) -> COMP2INMSEL3_W<22> {
        COMP2INMSEL3_W::new(self)
    }
    #[doc = "Bit 31 - Comparator 2 lock"]
    #[inline(always)]
    #[must_use]
    pub fn comp2lock(&mut self) -> COMP2LOCK_W<31> {
        COMP2LOCK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "control and status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [comp2_csr](index.html) module"]
pub struct COMP2_CSR_SPEC;
impl crate::RegisterSpec for COMP2_CSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [comp2_csr::R](R) reader structure"]
impl crate::Readable for COMP2_CSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [comp2_csr::W](W) writer structure"]
impl crate::Writable for COMP2_CSR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets COMP2_CSR to value 0"]
impl crate::Resettable for COMP2_CSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

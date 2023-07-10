#[doc = "Register `COMP6_CSR` reader"]
pub struct R(crate::R<COMP6_CSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<COMP6_CSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<COMP6_CSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<COMP6_CSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `COMP6_CSR` writer"]
pub struct W(crate::W<COMP6_CSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<COMP6_CSR_SPEC>;
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
impl From<crate::W<COMP6_CSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<COMP6_CSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `COMP6EN` reader - Comparator 6 enable"]
pub type COMP6EN_R = crate::BitReader<COMP6EN_A>;
#[doc = "Comparator 6 enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum COMP6EN_A {
    #[doc = "0: Comparator disabled"]
    DISABLED = 0,
    #[doc = "1: Comparator enabled"]
    ENABLED = 1,
}
impl From<COMP6EN_A> for bool {
    #[inline(always)]
    fn from(variant: COMP6EN_A) -> Self {
        variant as u8 != 0
    }
}
impl COMP6EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COMP6EN_A {
        match self.bits {
            false => COMP6EN_A::DISABLED,
            true => COMP6EN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == COMP6EN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == COMP6EN_A::ENABLED
    }
}
#[doc = "Field `COMP6EN` writer - Comparator 6 enable"]
pub type COMP6EN_W<'a, const O: u8> = crate::BitWriter<'a, COMP6_CSR_SPEC, O, COMP6EN_A>;
impl<'a, const O: u8> COMP6EN_W<'a, O> {
    #[doc = "Comparator disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(COMP6EN_A::DISABLED)
    }
    #[doc = "Comparator enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(COMP6EN_A::ENABLED)
    }
}
#[doc = "Field `COMP6MODE` reader - Comparator 6 mode"]
pub type COMP6MODE_R = crate::FieldReader;
#[doc = "Field `COMP6MODE` writer - Comparator 6 mode"]
pub type COMP6MODE_W<'a, const O: u8> = crate::FieldWriter<'a, COMP6_CSR_SPEC, 2, O>;
#[doc = "Field `COMP6INMSEL` reader - Comparator 6 inverting input selection"]
pub type COMP6INMSEL_R = crate::FieldReader<COMP6INMSEL_A>;
#[doc = "Comparator 6 inverting input selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum COMP6INMSEL_A {
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
    #[doc = "7: PB15"]
    PB15 = 7,
}
impl From<COMP6INMSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: COMP6INMSEL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for COMP6INMSEL_A {
    type Ux = u8;
}
impl COMP6INMSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<COMP6INMSEL_A> {
        match self.bits {
            0 => Some(COMP6INMSEL_A::ONE_QUARTER_VREF),
            1 => Some(COMP6INMSEL_A::ONE_HALF_VREF),
            2 => Some(COMP6INMSEL_A::THREE_QUARTER_VREF),
            3 => Some(COMP6INMSEL_A::VREF),
            4 => Some(COMP6INMSEL_A::PA4_DAC1_CH1),
            5 => Some(COMP6INMSEL_A::DAC1_CH2),
            7 => Some(COMP6INMSEL_A::PB15),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `ONE_QUARTER_VREF`"]
    #[inline(always)]
    pub fn is_one_quarter_vref(&self) -> bool {
        *self == COMP6INMSEL_A::ONE_QUARTER_VREF
    }
    #[doc = "Checks if the value of the field is `ONE_HALF_VREF`"]
    #[inline(always)]
    pub fn is_one_half_vref(&self) -> bool {
        *self == COMP6INMSEL_A::ONE_HALF_VREF
    }
    #[doc = "Checks if the value of the field is `THREE_QUARTER_VREF`"]
    #[inline(always)]
    pub fn is_three_quarter_vref(&self) -> bool {
        *self == COMP6INMSEL_A::THREE_QUARTER_VREF
    }
    #[doc = "Checks if the value of the field is `VREF`"]
    #[inline(always)]
    pub fn is_vref(&self) -> bool {
        *self == COMP6INMSEL_A::VREF
    }
    #[doc = "Checks if the value of the field is `PA4_DAC1_CH1`"]
    #[inline(always)]
    pub fn is_pa4_dac1_ch1(&self) -> bool {
        *self == COMP6INMSEL_A::PA4_DAC1_CH1
    }
    #[doc = "Checks if the value of the field is `DAC1_CH2`"]
    #[inline(always)]
    pub fn is_dac1_ch2(&self) -> bool {
        *self == COMP6INMSEL_A::DAC1_CH2
    }
    #[doc = "Checks if the value of the field is `PB15`"]
    #[inline(always)]
    pub fn is_pb15(&self) -> bool {
        *self == COMP6INMSEL_A::PB15
    }
}
#[doc = "Field `COMP6INMSEL` writer - Comparator 6 inverting input selection"]
pub type COMP6INMSEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, COMP6_CSR_SPEC, 3, O, COMP6INMSEL_A>;
impl<'a, const O: u8> COMP6INMSEL_W<'a, O> {
    #[doc = "1/4 of VRefint"]
    #[inline(always)]
    pub fn one_quarter_vref(self) -> &'a mut W {
        self.variant(COMP6INMSEL_A::ONE_QUARTER_VREF)
    }
    #[doc = "1/2 of VRefint"]
    #[inline(always)]
    pub fn one_half_vref(self) -> &'a mut W {
        self.variant(COMP6INMSEL_A::ONE_HALF_VREF)
    }
    #[doc = "3/4 of VRefint"]
    #[inline(always)]
    pub fn three_quarter_vref(self) -> &'a mut W {
        self.variant(COMP6INMSEL_A::THREE_QUARTER_VREF)
    }
    #[doc = "VRefint"]
    #[inline(always)]
    pub fn vref(self) -> &'a mut W {
        self.variant(COMP6INMSEL_A::VREF)
    }
    #[doc = "PA4 or DAC1_CH1 output if enabled"]
    #[inline(always)]
    pub fn pa4_dac1_ch1(self) -> &'a mut W {
        self.variant(COMP6INMSEL_A::PA4_DAC1_CH1)
    }
    #[doc = "DAC1_CH2"]
    #[inline(always)]
    pub fn dac1_ch2(self) -> &'a mut W {
        self.variant(COMP6INMSEL_A::DAC1_CH2)
    }
    #[doc = "PB15"]
    #[inline(always)]
    pub fn pb15(self) -> &'a mut W {
        self.variant(COMP6INMSEL_A::PB15)
    }
}
#[doc = "Field `COMP6INPSEL` reader - Comparator 6 non inverted input"]
pub type COMP6INPSEL_R = crate::BitReader;
#[doc = "Field `COMP6INPSEL` writer - Comparator 6 non inverted input"]
pub type COMP6INPSEL_W<'a, const O: u8> = crate::BitWriter<'a, COMP6_CSR_SPEC, O>;
#[doc = "Field `COMP6WINMODE` reader - Comparator 6 window mode"]
pub type COMP6WINMODE_R = crate::BitReader;
#[doc = "Field `COMP6WINMODE` writer - Comparator 6 window mode"]
pub type COMP6WINMODE_W<'a, const O: u8> = crate::BitWriter<'a, COMP6_CSR_SPEC, O>;
#[doc = "Field `COMP6OUTSEL` reader - Comparator 6 output selection"]
pub type COMP6OUTSEL_R = crate::FieldReader<COMP6OUTSEL_A>;
#[doc = "Comparator 6 output selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum COMP6OUTSEL_A {
    #[doc = "0: No selection"]
    NO_SELECTION = 0,
    #[doc = "1: Timer 1 break input"]
    TIMER1BREAK_INPUT = 1,
    #[doc = "2: Timer 1 break input 2"]
    TIMER1BREAK_INPUT2 = 2,
    #[doc = "6: Timer 2 input capture 2"]
    TIMER2INPUT_CAPTURE2 = 6,
    #[doc = "8: Timer 2 OCREF_CLR input"]
    TIMER2OCREF_CLEAR_INPUT = 8,
    #[doc = "9: Timer 16 OCREF_CLR input"]
    TIMER16OCREF_CLEAR_INPUT = 9,
    #[doc = "10: Timer 16 input capture 1"]
    TIMER16INPUT_CAPTURE1 = 10,
}
impl From<COMP6OUTSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: COMP6OUTSEL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for COMP6OUTSEL_A {
    type Ux = u8;
}
impl COMP6OUTSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<COMP6OUTSEL_A> {
        match self.bits {
            0 => Some(COMP6OUTSEL_A::NO_SELECTION),
            1 => Some(COMP6OUTSEL_A::TIMER1BREAK_INPUT),
            2 => Some(COMP6OUTSEL_A::TIMER1BREAK_INPUT2),
            6 => Some(COMP6OUTSEL_A::TIMER2INPUT_CAPTURE2),
            8 => Some(COMP6OUTSEL_A::TIMER2OCREF_CLEAR_INPUT),
            9 => Some(COMP6OUTSEL_A::TIMER16OCREF_CLEAR_INPUT),
            10 => Some(COMP6OUTSEL_A::TIMER16INPUT_CAPTURE1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NO_SELECTION`"]
    #[inline(always)]
    pub fn is_no_selection(&self) -> bool {
        *self == COMP6OUTSEL_A::NO_SELECTION
    }
    #[doc = "Checks if the value of the field is `TIMER1BREAK_INPUT`"]
    #[inline(always)]
    pub fn is_timer1break_input(&self) -> bool {
        *self == COMP6OUTSEL_A::TIMER1BREAK_INPUT
    }
    #[doc = "Checks if the value of the field is `TIMER1BREAK_INPUT2`"]
    #[inline(always)]
    pub fn is_timer1break_input2(&self) -> bool {
        *self == COMP6OUTSEL_A::TIMER1BREAK_INPUT2
    }
    #[doc = "Checks if the value of the field is `TIMER2INPUT_CAPTURE2`"]
    #[inline(always)]
    pub fn is_timer2input_capture2(&self) -> bool {
        *self == COMP6OUTSEL_A::TIMER2INPUT_CAPTURE2
    }
    #[doc = "Checks if the value of the field is `TIMER2OCREF_CLEAR_INPUT`"]
    #[inline(always)]
    pub fn is_timer2ocref_clear_input(&self) -> bool {
        *self == COMP6OUTSEL_A::TIMER2OCREF_CLEAR_INPUT
    }
    #[doc = "Checks if the value of the field is `TIMER16OCREF_CLEAR_INPUT`"]
    #[inline(always)]
    pub fn is_timer16ocref_clear_input(&self) -> bool {
        *self == COMP6OUTSEL_A::TIMER16OCREF_CLEAR_INPUT
    }
    #[doc = "Checks if the value of the field is `TIMER16INPUT_CAPTURE1`"]
    #[inline(always)]
    pub fn is_timer16input_capture1(&self) -> bool {
        *self == COMP6OUTSEL_A::TIMER16INPUT_CAPTURE1
    }
}
#[doc = "Field `COMP6OUTSEL` writer - Comparator 6 output selection"]
pub type COMP6OUTSEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, COMP6_CSR_SPEC, 4, O, COMP6OUTSEL_A>;
impl<'a, const O: u8> COMP6OUTSEL_W<'a, O> {
    #[doc = "No selection"]
    #[inline(always)]
    pub fn no_selection(self) -> &'a mut W {
        self.variant(COMP6OUTSEL_A::NO_SELECTION)
    }
    #[doc = "Timer 1 break input"]
    #[inline(always)]
    pub fn timer1break_input(self) -> &'a mut W {
        self.variant(COMP6OUTSEL_A::TIMER1BREAK_INPUT)
    }
    #[doc = "Timer 1 break input 2"]
    #[inline(always)]
    pub fn timer1break_input2(self) -> &'a mut W {
        self.variant(COMP6OUTSEL_A::TIMER1BREAK_INPUT2)
    }
    #[doc = "Timer 2 input capture 2"]
    #[inline(always)]
    pub fn timer2input_capture2(self) -> &'a mut W {
        self.variant(COMP6OUTSEL_A::TIMER2INPUT_CAPTURE2)
    }
    #[doc = "Timer 2 OCREF_CLR input"]
    #[inline(always)]
    pub fn timer2ocref_clear_input(self) -> &'a mut W {
        self.variant(COMP6OUTSEL_A::TIMER2OCREF_CLEAR_INPUT)
    }
    #[doc = "Timer 16 OCREF_CLR input"]
    #[inline(always)]
    pub fn timer16ocref_clear_input(self) -> &'a mut W {
        self.variant(COMP6OUTSEL_A::TIMER16OCREF_CLEAR_INPUT)
    }
    #[doc = "Timer 16 input capture 1"]
    #[inline(always)]
    pub fn timer16input_capture1(self) -> &'a mut W {
        self.variant(COMP6OUTSEL_A::TIMER16INPUT_CAPTURE1)
    }
}
#[doc = "Field `COMP6POL` reader - Comparator 6 output polarity"]
pub type COMP6POL_R = crate::BitReader<COMP6POL_A>;
#[doc = "Comparator 6 output polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum COMP6POL_A {
    #[doc = "0: Output is not inverted"]
    NOT_INVERTED = 0,
    #[doc = "1: Output is inverted"]
    INVERTED = 1,
}
impl From<COMP6POL_A> for bool {
    #[inline(always)]
    fn from(variant: COMP6POL_A) -> Self {
        variant as u8 != 0
    }
}
impl COMP6POL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COMP6POL_A {
        match self.bits {
            false => COMP6POL_A::NOT_INVERTED,
            true => COMP6POL_A::INVERTED,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_INVERTED`"]
    #[inline(always)]
    pub fn is_not_inverted(&self) -> bool {
        *self == COMP6POL_A::NOT_INVERTED
    }
    #[doc = "Checks if the value of the field is `INVERTED`"]
    #[inline(always)]
    pub fn is_inverted(&self) -> bool {
        *self == COMP6POL_A::INVERTED
    }
}
#[doc = "Field `COMP6POL` writer - Comparator 6 output polarity"]
pub type COMP6POL_W<'a, const O: u8> = crate::BitWriter<'a, COMP6_CSR_SPEC, O, COMP6POL_A>;
impl<'a, const O: u8> COMP6POL_W<'a, O> {
    #[doc = "Output is not inverted"]
    #[inline(always)]
    pub fn not_inverted(self) -> &'a mut W {
        self.variant(COMP6POL_A::NOT_INVERTED)
    }
    #[doc = "Output is inverted"]
    #[inline(always)]
    pub fn inverted(self) -> &'a mut W {
        self.variant(COMP6POL_A::INVERTED)
    }
}
#[doc = "Field `COMP6HYST` reader - Comparator 6 hysteresis"]
pub type COMP6HYST_R = crate::FieldReader;
#[doc = "Field `COMP6HYST` writer - Comparator 6 hysteresis"]
pub type COMP6HYST_W<'a, const O: u8> = crate::FieldWriter<'a, COMP6_CSR_SPEC, 2, O>;
#[doc = "Field `COMP6_BLANKING` reader - Comparator 6 blanking source"]
pub type COMP6_BLANKING_R = crate::FieldReader<COMP6_BLANKING_A>;
#[doc = "Comparator 6 blanking source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum COMP6_BLANKING_A {
    #[doc = "0: No blanking"]
    NO_BLANKING = 0,
    #[doc = "3: TIM2 OC4 selected as blanking source"]
    TIM2OC4 = 3,
    #[doc = "4: TIM15 OC2 selected as blanking source"]
    TIM15OC2 = 4,
}
impl From<COMP6_BLANKING_A> for u8 {
    #[inline(always)]
    fn from(variant: COMP6_BLANKING_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for COMP6_BLANKING_A {
    type Ux = u8;
}
impl COMP6_BLANKING_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<COMP6_BLANKING_A> {
        match self.bits {
            0 => Some(COMP6_BLANKING_A::NO_BLANKING),
            3 => Some(COMP6_BLANKING_A::TIM2OC4),
            4 => Some(COMP6_BLANKING_A::TIM15OC2),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NO_BLANKING`"]
    #[inline(always)]
    pub fn is_no_blanking(&self) -> bool {
        *self == COMP6_BLANKING_A::NO_BLANKING
    }
    #[doc = "Checks if the value of the field is `TIM2OC4`"]
    #[inline(always)]
    pub fn is_tim2oc4(&self) -> bool {
        *self == COMP6_BLANKING_A::TIM2OC4
    }
    #[doc = "Checks if the value of the field is `TIM15OC2`"]
    #[inline(always)]
    pub fn is_tim15oc2(&self) -> bool {
        *self == COMP6_BLANKING_A::TIM15OC2
    }
}
#[doc = "Field `COMP6_BLANKING` writer - Comparator 6 blanking source"]
pub type COMP6_BLANKING_W<'a, const O: u8> =
    crate::FieldWriter<'a, COMP6_CSR_SPEC, 3, O, COMP6_BLANKING_A>;
impl<'a, const O: u8> COMP6_BLANKING_W<'a, O> {
    #[doc = "No blanking"]
    #[inline(always)]
    pub fn no_blanking(self) -> &'a mut W {
        self.variant(COMP6_BLANKING_A::NO_BLANKING)
    }
    #[doc = "TIM2 OC4 selected as blanking source"]
    #[inline(always)]
    pub fn tim2oc4(self) -> &'a mut W {
        self.variant(COMP6_BLANKING_A::TIM2OC4)
    }
    #[doc = "TIM15 OC2 selected as blanking source"]
    #[inline(always)]
    pub fn tim15oc2(self) -> &'a mut W {
        self.variant(COMP6_BLANKING_A::TIM15OC2)
    }
}
#[doc = "Field `COMP6INMSEL3` reader - Comparator 6 inverting input selection"]
pub type COMP6INMSEL3_R = crate::BitReader;
#[doc = "Field `COMP6INMSEL3` writer - Comparator 6 inverting input selection"]
pub type COMP6INMSEL3_W<'a, const O: u8> = crate::BitWriter<'a, COMP6_CSR_SPEC, O>;
#[doc = "Field `COMP6OUT` reader - Comparator 6 output"]
pub type COMP6OUT_R = crate::BitReader<COMP6OUT_A>;
#[doc = "Comparator 6 output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum COMP6OUT_A {
    #[doc = "0: Non-inverting input below inverting input"]
    LOW = 0,
    #[doc = "1: Non-inverting input above inverting input"]
    HIGH = 1,
}
impl From<COMP6OUT_A> for bool {
    #[inline(always)]
    fn from(variant: COMP6OUT_A) -> Self {
        variant as u8 != 0
    }
}
impl COMP6OUT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COMP6OUT_A {
        match self.bits {
            false => COMP6OUT_A::LOW,
            true => COMP6OUT_A::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == COMP6OUT_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == COMP6OUT_A::HIGH
    }
}
#[doc = "Field `COMP6LOCK` reader - Comparator 6 lock"]
pub type COMP6LOCK_R = crate::BitReader<COMP6LOCK_A>;
#[doc = "Comparator 6 lock\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum COMP6LOCK_A {
    #[doc = "0: Comparator CSR bits are read-write"]
    UNLOCKED = 0,
    #[doc = "1: Comparator CSR bits are read-only"]
    LOCKED = 1,
}
impl From<COMP6LOCK_A> for bool {
    #[inline(always)]
    fn from(variant: COMP6LOCK_A) -> Self {
        variant as u8 != 0
    }
}
impl COMP6LOCK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COMP6LOCK_A {
        match self.bits {
            false => COMP6LOCK_A::UNLOCKED,
            true => COMP6LOCK_A::LOCKED,
        }
    }
    #[doc = "Checks if the value of the field is `UNLOCKED`"]
    #[inline(always)]
    pub fn is_unlocked(&self) -> bool {
        *self == COMP6LOCK_A::UNLOCKED
    }
    #[doc = "Checks if the value of the field is `LOCKED`"]
    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        *self == COMP6LOCK_A::LOCKED
    }
}
#[doc = "Field `COMP6LOCK` writer - Comparator 6 lock"]
pub type COMP6LOCK_W<'a, const O: u8> = crate::BitWriter<'a, COMP6_CSR_SPEC, O, COMP6LOCK_A>;
impl<'a, const O: u8> COMP6LOCK_W<'a, O> {
    #[doc = "Comparator CSR bits are read-write"]
    #[inline(always)]
    pub fn unlocked(self) -> &'a mut W {
        self.variant(COMP6LOCK_A::UNLOCKED)
    }
    #[doc = "Comparator CSR bits are read-only"]
    #[inline(always)]
    pub fn locked(self) -> &'a mut W {
        self.variant(COMP6LOCK_A::LOCKED)
    }
}
impl R {
    #[doc = "Bit 0 - Comparator 6 enable"]
    #[inline(always)]
    pub fn comp6en(&self) -> COMP6EN_R {
        COMP6EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 2:3 - Comparator 6 mode"]
    #[inline(always)]
    pub fn comp6mode(&self) -> COMP6MODE_R {
        COMP6MODE_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:6 - Comparator 6 inverting input selection"]
    #[inline(always)]
    pub fn comp6inmsel(&self) -> COMP6INMSEL_R {
        COMP6INMSEL_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - Comparator 6 non inverted input"]
    #[inline(always)]
    pub fn comp6inpsel(&self) -> COMP6INPSEL_R {
        COMP6INPSEL_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 9 - Comparator 6 window mode"]
    #[inline(always)]
    pub fn comp6winmode(&self) -> COMP6WINMODE_R {
        COMP6WINMODE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:13 - Comparator 6 output selection"]
    #[inline(always)]
    pub fn comp6outsel(&self) -> COMP6OUTSEL_R {
        COMP6OUTSEL_R::new(((self.bits >> 10) & 0x0f) as u8)
    }
    #[doc = "Bit 15 - Comparator 6 output polarity"]
    #[inline(always)]
    pub fn comp6pol(&self) -> COMP6POL_R {
        COMP6POL_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:17 - Comparator 6 hysteresis"]
    #[inline(always)]
    pub fn comp6hyst(&self) -> COMP6HYST_R {
        COMP6HYST_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:20 - Comparator 6 blanking source"]
    #[inline(always)]
    pub fn comp6_blanking(&self) -> COMP6_BLANKING_R {
        COMP6_BLANKING_R::new(((self.bits >> 18) & 7) as u8)
    }
    #[doc = "Bit 22 - Comparator 6 inverting input selection"]
    #[inline(always)]
    pub fn comp6inmsel3(&self) -> COMP6INMSEL3_R {
        COMP6INMSEL3_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 30 - Comparator 6 output"]
    #[inline(always)]
    pub fn comp6out(&self) -> COMP6OUT_R {
        COMP6OUT_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Comparator 6 lock"]
    #[inline(always)]
    pub fn comp6lock(&self) -> COMP6LOCK_R {
        COMP6LOCK_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Comparator 6 enable"]
    #[inline(always)]
    #[must_use]
    pub fn comp6en(&mut self) -> COMP6EN_W<0> {
        COMP6EN_W::new(self)
    }
    #[doc = "Bits 2:3 - Comparator 6 mode"]
    #[inline(always)]
    #[must_use]
    pub fn comp6mode(&mut self) -> COMP6MODE_W<2> {
        COMP6MODE_W::new(self)
    }
    #[doc = "Bits 4:6 - Comparator 6 inverting input selection"]
    #[inline(always)]
    #[must_use]
    pub fn comp6inmsel(&mut self) -> COMP6INMSEL_W<4> {
        COMP6INMSEL_W::new(self)
    }
    #[doc = "Bit 7 - Comparator 6 non inverted input"]
    #[inline(always)]
    #[must_use]
    pub fn comp6inpsel(&mut self) -> COMP6INPSEL_W<7> {
        COMP6INPSEL_W::new(self)
    }
    #[doc = "Bit 9 - Comparator 6 window mode"]
    #[inline(always)]
    #[must_use]
    pub fn comp6winmode(&mut self) -> COMP6WINMODE_W<9> {
        COMP6WINMODE_W::new(self)
    }
    #[doc = "Bits 10:13 - Comparator 6 output selection"]
    #[inline(always)]
    #[must_use]
    pub fn comp6outsel(&mut self) -> COMP6OUTSEL_W<10> {
        COMP6OUTSEL_W::new(self)
    }
    #[doc = "Bit 15 - Comparator 6 output polarity"]
    #[inline(always)]
    #[must_use]
    pub fn comp6pol(&mut self) -> COMP6POL_W<15> {
        COMP6POL_W::new(self)
    }
    #[doc = "Bits 16:17 - Comparator 6 hysteresis"]
    #[inline(always)]
    #[must_use]
    pub fn comp6hyst(&mut self) -> COMP6HYST_W<16> {
        COMP6HYST_W::new(self)
    }
    #[doc = "Bits 18:20 - Comparator 6 blanking source"]
    #[inline(always)]
    #[must_use]
    pub fn comp6_blanking(&mut self) -> COMP6_BLANKING_W<18> {
        COMP6_BLANKING_W::new(self)
    }
    #[doc = "Bit 22 - Comparator 6 inverting input selection"]
    #[inline(always)]
    #[must_use]
    pub fn comp6inmsel3(&mut self) -> COMP6INMSEL3_W<22> {
        COMP6INMSEL3_W::new(self)
    }
    #[doc = "Bit 31 - Comparator 6 lock"]
    #[inline(always)]
    #[must_use]
    pub fn comp6lock(&mut self) -> COMP6LOCK_W<31> {
        COMP6LOCK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "control and status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [comp6_csr](index.html) module"]
pub struct COMP6_CSR_SPEC;
impl crate::RegisterSpec for COMP6_CSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [comp6_csr::R](R) reader structure"]
impl crate::Readable for COMP6_CSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [comp6_csr::W](W) writer structure"]
impl crate::Writable for COMP6_CSR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets COMP6_CSR to value 0"]
impl crate::Resettable for COMP6_CSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

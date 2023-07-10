#[doc = "Register `CFGR` reader"]
pub struct R(crate::R<CFGR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFGR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFGR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFGR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CFGR` writer"]
pub struct W(crate::W<CFGR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFGR_SPEC>;
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
impl From<crate::W<CFGR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFGR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DMAEN` reader - DMAEN"]
pub type DMAEN_R = crate::BitReader<DMAEN_A>;
#[doc = "DMAEN\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMAEN_A {
    #[doc = "0: DMA disabled"]
    DISABLED = 0,
    #[doc = "1: DMA enabled"]
    ENABLED = 1,
}
impl From<DMAEN_A> for bool {
    #[inline(always)]
    fn from(variant: DMAEN_A) -> Self {
        variant as u8 != 0
    }
}
impl DMAEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMAEN_A {
        match self.bits {
            false => DMAEN_A::DISABLED,
            true => DMAEN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DMAEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DMAEN_A::ENABLED
    }
}
#[doc = "Field `DMAEN` writer - DMAEN"]
pub type DMAEN_W<'a, const O: u8> = crate::BitWriter<'a, CFGR_SPEC, O, DMAEN_A>;
impl<'a, const O: u8> DMAEN_W<'a, O> {
    #[doc = "DMA disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DMAEN_A::DISABLED)
    }
    #[doc = "DMA enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DMAEN_A::ENABLED)
    }
}
#[doc = "Field `DMACFG` reader - DMACFG"]
pub type DMACFG_R = crate::BitReader<DMACFG_A>;
#[doc = "DMACFG\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMACFG_A {
    #[doc = "0: DMA One Shot Mode selected"]
    ONE_SHOT = 0,
    #[doc = "1: DMA circular mode selected"]
    CIRCULAR = 1,
}
impl From<DMACFG_A> for bool {
    #[inline(always)]
    fn from(variant: DMACFG_A) -> Self {
        variant as u8 != 0
    }
}
impl DMACFG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMACFG_A {
        match self.bits {
            false => DMACFG_A::ONE_SHOT,
            true => DMACFG_A::CIRCULAR,
        }
    }
    #[doc = "Checks if the value of the field is `ONE_SHOT`"]
    #[inline(always)]
    pub fn is_one_shot(&self) -> bool {
        *self == DMACFG_A::ONE_SHOT
    }
    #[doc = "Checks if the value of the field is `CIRCULAR`"]
    #[inline(always)]
    pub fn is_circular(&self) -> bool {
        *self == DMACFG_A::CIRCULAR
    }
}
#[doc = "Field `DMACFG` writer - DMACFG"]
pub type DMACFG_W<'a, const O: u8> = crate::BitWriter<'a, CFGR_SPEC, O, DMACFG_A>;
impl<'a, const O: u8> DMACFG_W<'a, O> {
    #[doc = "DMA One Shot Mode selected"]
    #[inline(always)]
    pub fn one_shot(self) -> &'a mut W {
        self.variant(DMACFG_A::ONE_SHOT)
    }
    #[doc = "DMA circular mode selected"]
    #[inline(always)]
    pub fn circular(self) -> &'a mut W {
        self.variant(DMACFG_A::CIRCULAR)
    }
}
#[doc = "Field `RES` reader - RES"]
pub type RES_R = crate::FieldReader<RES_A>;
#[doc = "RES\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RES_A {
    #[doc = "0: 12-bit"]
    BITS12 = 0,
    #[doc = "1: 10-bit"]
    BITS10 = 1,
    #[doc = "2: 8-bit"]
    BITS8 = 2,
    #[doc = "3: 6-bit"]
    BITS6 = 3,
}
impl From<RES_A> for u8 {
    #[inline(always)]
    fn from(variant: RES_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for RES_A {
    type Ux = u8;
}
impl RES_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RES_A {
        match self.bits {
            0 => RES_A::BITS12,
            1 => RES_A::BITS10,
            2 => RES_A::BITS8,
            3 => RES_A::BITS6,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `BITS12`"]
    #[inline(always)]
    pub fn is_bits12(&self) -> bool {
        *self == RES_A::BITS12
    }
    #[doc = "Checks if the value of the field is `BITS10`"]
    #[inline(always)]
    pub fn is_bits10(&self) -> bool {
        *self == RES_A::BITS10
    }
    #[doc = "Checks if the value of the field is `BITS8`"]
    #[inline(always)]
    pub fn is_bits8(&self) -> bool {
        *self == RES_A::BITS8
    }
    #[doc = "Checks if the value of the field is `BITS6`"]
    #[inline(always)]
    pub fn is_bits6(&self) -> bool {
        *self == RES_A::BITS6
    }
}
#[doc = "Field `RES` writer - RES"]
pub type RES_W<'a, const O: u8> = crate::FieldWriterSafe<'a, CFGR_SPEC, 2, O, RES_A>;
impl<'a, const O: u8> RES_W<'a, O> {
    #[doc = "12-bit"]
    #[inline(always)]
    pub fn bits12(self) -> &'a mut W {
        self.variant(RES_A::BITS12)
    }
    #[doc = "10-bit"]
    #[inline(always)]
    pub fn bits10(self) -> &'a mut W {
        self.variant(RES_A::BITS10)
    }
    #[doc = "8-bit"]
    #[inline(always)]
    pub fn bits8(self) -> &'a mut W {
        self.variant(RES_A::BITS8)
    }
    #[doc = "6-bit"]
    #[inline(always)]
    pub fn bits6(self) -> &'a mut W {
        self.variant(RES_A::BITS6)
    }
}
#[doc = "Field `ALIGN` reader - ALIGN"]
pub type ALIGN_R = crate::BitReader<ALIGN_A>;
#[doc = "ALIGN\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ALIGN_A {
    #[doc = "0: Right alignment"]
    RIGHT = 0,
    #[doc = "1: Left alignment"]
    LEFT = 1,
}
impl From<ALIGN_A> for bool {
    #[inline(always)]
    fn from(variant: ALIGN_A) -> Self {
        variant as u8 != 0
    }
}
impl ALIGN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ALIGN_A {
        match self.bits {
            false => ALIGN_A::RIGHT,
            true => ALIGN_A::LEFT,
        }
    }
    #[doc = "Checks if the value of the field is `RIGHT`"]
    #[inline(always)]
    pub fn is_right(&self) -> bool {
        *self == ALIGN_A::RIGHT
    }
    #[doc = "Checks if the value of the field is `LEFT`"]
    #[inline(always)]
    pub fn is_left(&self) -> bool {
        *self == ALIGN_A::LEFT
    }
}
#[doc = "Field `ALIGN` writer - ALIGN"]
pub type ALIGN_W<'a, const O: u8> = crate::BitWriter<'a, CFGR_SPEC, O, ALIGN_A>;
impl<'a, const O: u8> ALIGN_W<'a, O> {
    #[doc = "Right alignment"]
    #[inline(always)]
    pub fn right(self) -> &'a mut W {
        self.variant(ALIGN_A::RIGHT)
    }
    #[doc = "Left alignment"]
    #[inline(always)]
    pub fn left(self) -> &'a mut W {
        self.variant(ALIGN_A::LEFT)
    }
}
#[doc = "Field `EXTSEL` reader - EXTSEL"]
pub type EXTSEL_R = crate::FieldReader<EXTSEL_A>;
#[doc = "EXTSEL\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EXTSEL_A {
    #[doc = "7: HRTIM_ADCTRG1 event"]
    HRTIM_ADCTRG1 = 7,
    #[doc = "8: HRTIM_ADCTRG3 event"]
    HRTIM_ADCTRG3 = 8,
    #[doc = "0: Timer 1 CC1 event"]
    TIM1_CC1 = 0,
    #[doc = "1: Timer 1 CC2 event"]
    TIM1_CC2 = 1,
    #[doc = "2: Timer 1 CC3 event"]
    TIM1_CC3 = 2,
    #[doc = "3: Timer 2 CC2 event"]
    TIM2_CC2 = 3,
    #[doc = "4: Timer 3 TRGO event"]
    TIM3_TRGO = 4,
    #[doc = "6: EXTI line 11"]
    EXTI11 = 6,
    #[doc = "9: Timer 1 TRGO event"]
    TIM1_TRGO = 9,
    #[doc = "10: Timer 1 TRGO2 event"]
    TIM1_TRGO2 = 10,
    #[doc = "11: Timer 2 TRGO event"]
    TIM2_TRGO = 11,
    #[doc = "13: Timer 6 TRGO event"]
    TIM6_TRGO = 13,
    #[doc = "14: Timer 15 TRGO event"]
    TIM15_TRGO = 14,
    #[doc = "15: Timer 3 CC4 event"]
    TIM3_CC4 = 15,
}
impl From<EXTSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: EXTSEL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for EXTSEL_A {
    type Ux = u8;
}
impl EXTSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<EXTSEL_A> {
        match self.bits {
            7 => Some(EXTSEL_A::HRTIM_ADCTRG1),
            8 => Some(EXTSEL_A::HRTIM_ADCTRG3),
            0 => Some(EXTSEL_A::TIM1_CC1),
            1 => Some(EXTSEL_A::TIM1_CC2),
            2 => Some(EXTSEL_A::TIM1_CC3),
            3 => Some(EXTSEL_A::TIM2_CC2),
            4 => Some(EXTSEL_A::TIM3_TRGO),
            6 => Some(EXTSEL_A::EXTI11),
            9 => Some(EXTSEL_A::TIM1_TRGO),
            10 => Some(EXTSEL_A::TIM1_TRGO2),
            11 => Some(EXTSEL_A::TIM2_TRGO),
            13 => Some(EXTSEL_A::TIM6_TRGO),
            14 => Some(EXTSEL_A::TIM15_TRGO),
            15 => Some(EXTSEL_A::TIM3_CC4),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `HRTIM_ADCTRG1`"]
    #[inline(always)]
    pub fn is_hrtim_adctrg1(&self) -> bool {
        *self == EXTSEL_A::HRTIM_ADCTRG1
    }
    #[doc = "Checks if the value of the field is `HRTIM_ADCTRG3`"]
    #[inline(always)]
    pub fn is_hrtim_adctrg3(&self) -> bool {
        *self == EXTSEL_A::HRTIM_ADCTRG3
    }
    #[doc = "Checks if the value of the field is `TIM1_CC1`"]
    #[inline(always)]
    pub fn is_tim1_cc1(&self) -> bool {
        *self == EXTSEL_A::TIM1_CC1
    }
    #[doc = "Checks if the value of the field is `TIM1_CC2`"]
    #[inline(always)]
    pub fn is_tim1_cc2(&self) -> bool {
        *self == EXTSEL_A::TIM1_CC2
    }
    #[doc = "Checks if the value of the field is `TIM1_CC3`"]
    #[inline(always)]
    pub fn is_tim1_cc3(&self) -> bool {
        *self == EXTSEL_A::TIM1_CC3
    }
    #[doc = "Checks if the value of the field is `TIM2_CC2`"]
    #[inline(always)]
    pub fn is_tim2_cc2(&self) -> bool {
        *self == EXTSEL_A::TIM2_CC2
    }
    #[doc = "Checks if the value of the field is `TIM3_TRGO`"]
    #[inline(always)]
    pub fn is_tim3_trgo(&self) -> bool {
        *self == EXTSEL_A::TIM3_TRGO
    }
    #[doc = "Checks if the value of the field is `EXTI11`"]
    #[inline(always)]
    pub fn is_exti11(&self) -> bool {
        *self == EXTSEL_A::EXTI11
    }
    #[doc = "Checks if the value of the field is `TIM1_TRGO`"]
    #[inline(always)]
    pub fn is_tim1_trgo(&self) -> bool {
        *self == EXTSEL_A::TIM1_TRGO
    }
    #[doc = "Checks if the value of the field is `TIM1_TRGO2`"]
    #[inline(always)]
    pub fn is_tim1_trgo2(&self) -> bool {
        *self == EXTSEL_A::TIM1_TRGO2
    }
    #[doc = "Checks if the value of the field is `TIM2_TRGO`"]
    #[inline(always)]
    pub fn is_tim2_trgo(&self) -> bool {
        *self == EXTSEL_A::TIM2_TRGO
    }
    #[doc = "Checks if the value of the field is `TIM6_TRGO`"]
    #[inline(always)]
    pub fn is_tim6_trgo(&self) -> bool {
        *self == EXTSEL_A::TIM6_TRGO
    }
    #[doc = "Checks if the value of the field is `TIM15_TRGO`"]
    #[inline(always)]
    pub fn is_tim15_trgo(&self) -> bool {
        *self == EXTSEL_A::TIM15_TRGO
    }
    #[doc = "Checks if the value of the field is `TIM3_CC4`"]
    #[inline(always)]
    pub fn is_tim3_cc4(&self) -> bool {
        *self == EXTSEL_A::TIM3_CC4
    }
}
#[doc = "Field `EXTSEL` writer - EXTSEL"]
pub type EXTSEL_W<'a, const O: u8> = crate::FieldWriter<'a, CFGR_SPEC, 4, O, EXTSEL_A>;
impl<'a, const O: u8> EXTSEL_W<'a, O> {
    #[doc = "HRTIM_ADCTRG1 event"]
    #[inline(always)]
    pub fn hrtim_adctrg1(self) -> &'a mut W {
        self.variant(EXTSEL_A::HRTIM_ADCTRG1)
    }
    #[doc = "HRTIM_ADCTRG3 event"]
    #[inline(always)]
    pub fn hrtim_adctrg3(self) -> &'a mut W {
        self.variant(EXTSEL_A::HRTIM_ADCTRG3)
    }
    #[doc = "Timer 1 CC1 event"]
    #[inline(always)]
    pub fn tim1_cc1(self) -> &'a mut W {
        self.variant(EXTSEL_A::TIM1_CC1)
    }
    #[doc = "Timer 1 CC2 event"]
    #[inline(always)]
    pub fn tim1_cc2(self) -> &'a mut W {
        self.variant(EXTSEL_A::TIM1_CC2)
    }
    #[doc = "Timer 1 CC3 event"]
    #[inline(always)]
    pub fn tim1_cc3(self) -> &'a mut W {
        self.variant(EXTSEL_A::TIM1_CC3)
    }
    #[doc = "Timer 2 CC2 event"]
    #[inline(always)]
    pub fn tim2_cc2(self) -> &'a mut W {
        self.variant(EXTSEL_A::TIM2_CC2)
    }
    #[doc = "Timer 3 TRGO event"]
    #[inline(always)]
    pub fn tim3_trgo(self) -> &'a mut W {
        self.variant(EXTSEL_A::TIM3_TRGO)
    }
    #[doc = "EXTI line 11"]
    #[inline(always)]
    pub fn exti11(self) -> &'a mut W {
        self.variant(EXTSEL_A::EXTI11)
    }
    #[doc = "Timer 1 TRGO event"]
    #[inline(always)]
    pub fn tim1_trgo(self) -> &'a mut W {
        self.variant(EXTSEL_A::TIM1_TRGO)
    }
    #[doc = "Timer 1 TRGO2 event"]
    #[inline(always)]
    pub fn tim1_trgo2(self) -> &'a mut W {
        self.variant(EXTSEL_A::TIM1_TRGO2)
    }
    #[doc = "Timer 2 TRGO event"]
    #[inline(always)]
    pub fn tim2_trgo(self) -> &'a mut W {
        self.variant(EXTSEL_A::TIM2_TRGO)
    }
    #[doc = "Timer 6 TRGO event"]
    #[inline(always)]
    pub fn tim6_trgo(self) -> &'a mut W {
        self.variant(EXTSEL_A::TIM6_TRGO)
    }
    #[doc = "Timer 15 TRGO event"]
    #[inline(always)]
    pub fn tim15_trgo(self) -> &'a mut W {
        self.variant(EXTSEL_A::TIM15_TRGO)
    }
    #[doc = "Timer 3 CC4 event"]
    #[inline(always)]
    pub fn tim3_cc4(self) -> &'a mut W {
        self.variant(EXTSEL_A::TIM3_CC4)
    }
}
#[doc = "Field `EXTEN` reader - EXTEN"]
pub type EXTEN_R = crate::FieldReader<EXTEN_A>;
#[doc = "EXTEN\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EXTEN_A {
    #[doc = "0: Trigger detection disabled"]
    DISABLED = 0,
    #[doc = "1: Trigger detection on the rising edge"]
    RISING_EDGE = 1,
    #[doc = "2: Trigger detection on the falling edge"]
    FALLING_EDGE = 2,
    #[doc = "3: Trigger detection on both the rising and falling edges"]
    BOTH_EDGES = 3,
}
impl From<EXTEN_A> for u8 {
    #[inline(always)]
    fn from(variant: EXTEN_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for EXTEN_A {
    type Ux = u8;
}
impl EXTEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EXTEN_A {
        match self.bits {
            0 => EXTEN_A::DISABLED,
            1 => EXTEN_A::RISING_EDGE,
            2 => EXTEN_A::FALLING_EDGE,
            3 => EXTEN_A::BOTH_EDGES,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == EXTEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `RISING_EDGE`"]
    #[inline(always)]
    pub fn is_rising_edge(&self) -> bool {
        *self == EXTEN_A::RISING_EDGE
    }
    #[doc = "Checks if the value of the field is `FALLING_EDGE`"]
    #[inline(always)]
    pub fn is_falling_edge(&self) -> bool {
        *self == EXTEN_A::FALLING_EDGE
    }
    #[doc = "Checks if the value of the field is `BOTH_EDGES`"]
    #[inline(always)]
    pub fn is_both_edges(&self) -> bool {
        *self == EXTEN_A::BOTH_EDGES
    }
}
#[doc = "Field `EXTEN` writer - EXTEN"]
pub type EXTEN_W<'a, const O: u8> = crate::FieldWriterSafe<'a, CFGR_SPEC, 2, O, EXTEN_A>;
impl<'a, const O: u8> EXTEN_W<'a, O> {
    #[doc = "Trigger detection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(EXTEN_A::DISABLED)
    }
    #[doc = "Trigger detection on the rising edge"]
    #[inline(always)]
    pub fn rising_edge(self) -> &'a mut W {
        self.variant(EXTEN_A::RISING_EDGE)
    }
    #[doc = "Trigger detection on the falling edge"]
    #[inline(always)]
    pub fn falling_edge(self) -> &'a mut W {
        self.variant(EXTEN_A::FALLING_EDGE)
    }
    #[doc = "Trigger detection on both the rising and falling edges"]
    #[inline(always)]
    pub fn both_edges(self) -> &'a mut W {
        self.variant(EXTEN_A::BOTH_EDGES)
    }
}
#[doc = "Field `OVRMOD` reader - OVRMOD"]
pub type OVRMOD_R = crate::BitReader<OVRMOD_A>;
#[doc = "OVRMOD\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OVRMOD_A {
    #[doc = "0: Preserve DR register when an overrun is detected"]
    PRESERVE = 0,
    #[doc = "1: Overwrite DR register when an overrun is detected"]
    OVERWRITE = 1,
}
impl From<OVRMOD_A> for bool {
    #[inline(always)]
    fn from(variant: OVRMOD_A) -> Self {
        variant as u8 != 0
    }
}
impl OVRMOD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OVRMOD_A {
        match self.bits {
            false => OVRMOD_A::PRESERVE,
            true => OVRMOD_A::OVERWRITE,
        }
    }
    #[doc = "Checks if the value of the field is `PRESERVE`"]
    #[inline(always)]
    pub fn is_preserve(&self) -> bool {
        *self == OVRMOD_A::PRESERVE
    }
    #[doc = "Checks if the value of the field is `OVERWRITE`"]
    #[inline(always)]
    pub fn is_overwrite(&self) -> bool {
        *self == OVRMOD_A::OVERWRITE
    }
}
#[doc = "Field `OVRMOD` writer - OVRMOD"]
pub type OVRMOD_W<'a, const O: u8> = crate::BitWriter<'a, CFGR_SPEC, O, OVRMOD_A>;
impl<'a, const O: u8> OVRMOD_W<'a, O> {
    #[doc = "Preserve DR register when an overrun is detected"]
    #[inline(always)]
    pub fn preserve(self) -> &'a mut W {
        self.variant(OVRMOD_A::PRESERVE)
    }
    #[doc = "Overwrite DR register when an overrun is detected"]
    #[inline(always)]
    pub fn overwrite(self) -> &'a mut W {
        self.variant(OVRMOD_A::OVERWRITE)
    }
}
#[doc = "Field `CONT` reader - CONT"]
pub type CONT_R = crate::BitReader<CONT_A>;
#[doc = "CONT\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CONT_A {
    #[doc = "0: Single conversion mode"]
    SINGLE = 0,
    #[doc = "1: Continuous conversion mode"]
    CONTINUOUS = 1,
}
impl From<CONT_A> for bool {
    #[inline(always)]
    fn from(variant: CONT_A) -> Self {
        variant as u8 != 0
    }
}
impl CONT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CONT_A {
        match self.bits {
            false => CONT_A::SINGLE,
            true => CONT_A::CONTINUOUS,
        }
    }
    #[doc = "Checks if the value of the field is `SINGLE`"]
    #[inline(always)]
    pub fn is_single(&self) -> bool {
        *self == CONT_A::SINGLE
    }
    #[doc = "Checks if the value of the field is `CONTINUOUS`"]
    #[inline(always)]
    pub fn is_continuous(&self) -> bool {
        *self == CONT_A::CONTINUOUS
    }
}
#[doc = "Field `CONT` writer - CONT"]
pub type CONT_W<'a, const O: u8> = crate::BitWriter<'a, CFGR_SPEC, O, CONT_A>;
impl<'a, const O: u8> CONT_W<'a, O> {
    #[doc = "Single conversion mode"]
    #[inline(always)]
    pub fn single(self) -> &'a mut W {
        self.variant(CONT_A::SINGLE)
    }
    #[doc = "Continuous conversion mode"]
    #[inline(always)]
    pub fn continuous(self) -> &'a mut W {
        self.variant(CONT_A::CONTINUOUS)
    }
}
#[doc = "Field `AUTDLY` reader - AUTDLY"]
pub type AUTDLY_R = crate::BitReader<AUTDLY_A>;
#[doc = "AUTDLY\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AUTDLY_A {
    #[doc = "0: Auto delayed conversion mode off"]
    OFF = 0,
    #[doc = "1: Auto delayed conversion mode on"]
    ON = 1,
}
impl From<AUTDLY_A> for bool {
    #[inline(always)]
    fn from(variant: AUTDLY_A) -> Self {
        variant as u8 != 0
    }
}
impl AUTDLY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AUTDLY_A {
        match self.bits {
            false => AUTDLY_A::OFF,
            true => AUTDLY_A::ON,
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == AUTDLY_A::OFF
    }
    #[doc = "Checks if the value of the field is `ON`"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == AUTDLY_A::ON
    }
}
#[doc = "Field `AUTDLY` writer - AUTDLY"]
pub type AUTDLY_W<'a, const O: u8> = crate::BitWriter<'a, CFGR_SPEC, O, AUTDLY_A>;
impl<'a, const O: u8> AUTDLY_W<'a, O> {
    #[doc = "Auto delayed conversion mode off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(AUTDLY_A::OFF)
    }
    #[doc = "Auto delayed conversion mode on"]
    #[inline(always)]
    pub fn on(self) -> &'a mut W {
        self.variant(AUTDLY_A::ON)
    }
}
#[doc = "Field `DISCEN` reader - DISCEN"]
pub type DISCEN_R = crate::BitReader<DISCEN_A>;
#[doc = "DISCEN\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DISCEN_A {
    #[doc = "0: Discontinuous mode on regular channels disabled"]
    DISABLED = 0,
    #[doc = "1: Discontinuous mode on regular channels enabled"]
    ENABLED = 1,
}
impl From<DISCEN_A> for bool {
    #[inline(always)]
    fn from(variant: DISCEN_A) -> Self {
        variant as u8 != 0
    }
}
impl DISCEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DISCEN_A {
        match self.bits {
            false => DISCEN_A::DISABLED,
            true => DISCEN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DISCEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DISCEN_A::ENABLED
    }
}
#[doc = "Field `DISCEN` writer - DISCEN"]
pub type DISCEN_W<'a, const O: u8> = crate::BitWriter<'a, CFGR_SPEC, O, DISCEN_A>;
impl<'a, const O: u8> DISCEN_W<'a, O> {
    #[doc = "Discontinuous mode on regular channels disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DISCEN_A::DISABLED)
    }
    #[doc = "Discontinuous mode on regular channels enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DISCEN_A::ENABLED)
    }
}
#[doc = "Field `DISCNUM` reader - DISCNUM"]
pub type DISCNUM_R = crate::FieldReader;
#[doc = "Field `DISCNUM` writer - DISCNUM"]
pub type DISCNUM_W<'a, const O: u8> = crate::FieldWriterSafe<'a, CFGR_SPEC, 3, O>;
#[doc = "Field `JDISCEN` reader - JDISCEN"]
pub type JDISCEN_R = crate::BitReader<JDISCEN_A>;
#[doc = "JDISCEN\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JDISCEN_A {
    #[doc = "0: Discontinuous mode on injected channels disabled"]
    DISABLED = 0,
    #[doc = "1: Discontinuous mode on injected channels enabled"]
    ENABLED = 1,
}
impl From<JDISCEN_A> for bool {
    #[inline(always)]
    fn from(variant: JDISCEN_A) -> Self {
        variant as u8 != 0
    }
}
impl JDISCEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> JDISCEN_A {
        match self.bits {
            false => JDISCEN_A::DISABLED,
            true => JDISCEN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == JDISCEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == JDISCEN_A::ENABLED
    }
}
#[doc = "Field `JDISCEN` writer - JDISCEN"]
pub type JDISCEN_W<'a, const O: u8> = crate::BitWriter<'a, CFGR_SPEC, O, JDISCEN_A>;
impl<'a, const O: u8> JDISCEN_W<'a, O> {
    #[doc = "Discontinuous mode on injected channels disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(JDISCEN_A::DISABLED)
    }
    #[doc = "Discontinuous mode on injected channels enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(JDISCEN_A::ENABLED)
    }
}
#[doc = "Field `JQM` reader - JQM"]
pub type JQM_R = crate::BitReader<JQM_A>;
#[doc = "JQM\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JQM_A {
    #[doc = "0: JSQR Mode 0: Queue maintains the last written configuration into JSQR"]
    MODE0 = 0,
    #[doc = "1: JSQR Mode 1: An empty queue disables software and hardware triggers of the injected sequence"]
    MODE1 = 1,
}
impl From<JQM_A> for bool {
    #[inline(always)]
    fn from(variant: JQM_A) -> Self {
        variant as u8 != 0
    }
}
impl JQM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> JQM_A {
        match self.bits {
            false => JQM_A::MODE0,
            true => JQM_A::MODE1,
        }
    }
    #[doc = "Checks if the value of the field is `MODE0`"]
    #[inline(always)]
    pub fn is_mode0(&self) -> bool {
        *self == JQM_A::MODE0
    }
    #[doc = "Checks if the value of the field is `MODE1`"]
    #[inline(always)]
    pub fn is_mode1(&self) -> bool {
        *self == JQM_A::MODE1
    }
}
#[doc = "Field `JQM` writer - JQM"]
pub type JQM_W<'a, const O: u8> = crate::BitWriter<'a, CFGR_SPEC, O, JQM_A>;
impl<'a, const O: u8> JQM_W<'a, O> {
    #[doc = "JSQR Mode 0: Queue maintains the last written configuration into JSQR"]
    #[inline(always)]
    pub fn mode0(self) -> &'a mut W {
        self.variant(JQM_A::MODE0)
    }
    #[doc = "JSQR Mode 1: An empty queue disables software and hardware triggers of the injected sequence"]
    #[inline(always)]
    pub fn mode1(self) -> &'a mut W {
        self.variant(JQM_A::MODE1)
    }
}
#[doc = "Field `AWD1SGL` reader - AWD1SGL"]
pub type AWD1SGL_R = crate::BitReader<AWD1SGL_A>;
#[doc = "AWD1SGL\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AWD1SGL_A {
    #[doc = "0: Analog watchdog 1 enabled on all channels"]
    ALL = 0,
    #[doc = "1: Analog watchdog 1 enabled on single channel selected in AWD1CH"]
    SINGLE = 1,
}
impl From<AWD1SGL_A> for bool {
    #[inline(always)]
    fn from(variant: AWD1SGL_A) -> Self {
        variant as u8 != 0
    }
}
impl AWD1SGL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AWD1SGL_A {
        match self.bits {
            false => AWD1SGL_A::ALL,
            true => AWD1SGL_A::SINGLE,
        }
    }
    #[doc = "Checks if the value of the field is `ALL`"]
    #[inline(always)]
    pub fn is_all(&self) -> bool {
        *self == AWD1SGL_A::ALL
    }
    #[doc = "Checks if the value of the field is `SINGLE`"]
    #[inline(always)]
    pub fn is_single(&self) -> bool {
        *self == AWD1SGL_A::SINGLE
    }
}
#[doc = "Field `AWD1SGL` writer - AWD1SGL"]
pub type AWD1SGL_W<'a, const O: u8> = crate::BitWriter<'a, CFGR_SPEC, O, AWD1SGL_A>;
impl<'a, const O: u8> AWD1SGL_W<'a, O> {
    #[doc = "Analog watchdog 1 enabled on all channels"]
    #[inline(always)]
    pub fn all(self) -> &'a mut W {
        self.variant(AWD1SGL_A::ALL)
    }
    #[doc = "Analog watchdog 1 enabled on single channel selected in AWD1CH"]
    #[inline(always)]
    pub fn single(self) -> &'a mut W {
        self.variant(AWD1SGL_A::SINGLE)
    }
}
#[doc = "Field `AWD1EN` reader - AWD1EN"]
pub type AWD1EN_R = crate::BitReader<AWD1EN_A>;
#[doc = "AWD1EN\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AWD1EN_A {
    #[doc = "0: Analog watchdog 1 disabled on regular channels"]
    DISABLED = 0,
    #[doc = "1: Analog watchdog 1 enabled on regular channels"]
    ENABLED = 1,
}
impl From<AWD1EN_A> for bool {
    #[inline(always)]
    fn from(variant: AWD1EN_A) -> Self {
        variant as u8 != 0
    }
}
impl AWD1EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AWD1EN_A {
        match self.bits {
            false => AWD1EN_A::DISABLED,
            true => AWD1EN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == AWD1EN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == AWD1EN_A::ENABLED
    }
}
#[doc = "Field `AWD1EN` writer - AWD1EN"]
pub type AWD1EN_W<'a, const O: u8> = crate::BitWriter<'a, CFGR_SPEC, O, AWD1EN_A>;
impl<'a, const O: u8> AWD1EN_W<'a, O> {
    #[doc = "Analog watchdog 1 disabled on regular channels"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(AWD1EN_A::DISABLED)
    }
    #[doc = "Analog watchdog 1 enabled on regular channels"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(AWD1EN_A::ENABLED)
    }
}
#[doc = "Field `JAWD1EN` reader - JAWD1EN"]
pub type JAWD1EN_R = crate::BitReader<JAWD1EN_A>;
#[doc = "JAWD1EN\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JAWD1EN_A {
    #[doc = "0: Analog watchdog 1 disabled on injected channels"]
    DISABLED = 0,
    #[doc = "1: Analog watchdog 1 enabled on injected channels"]
    ENABLED = 1,
}
impl From<JAWD1EN_A> for bool {
    #[inline(always)]
    fn from(variant: JAWD1EN_A) -> Self {
        variant as u8 != 0
    }
}
impl JAWD1EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> JAWD1EN_A {
        match self.bits {
            false => JAWD1EN_A::DISABLED,
            true => JAWD1EN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == JAWD1EN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == JAWD1EN_A::ENABLED
    }
}
#[doc = "Field `JAWD1EN` writer - JAWD1EN"]
pub type JAWD1EN_W<'a, const O: u8> = crate::BitWriter<'a, CFGR_SPEC, O, JAWD1EN_A>;
impl<'a, const O: u8> JAWD1EN_W<'a, O> {
    #[doc = "Analog watchdog 1 disabled on injected channels"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(JAWD1EN_A::DISABLED)
    }
    #[doc = "Analog watchdog 1 enabled on injected channels"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(JAWD1EN_A::ENABLED)
    }
}
#[doc = "Field `JAUTO` reader - JAUTO"]
pub type JAUTO_R = crate::BitReader<JAUTO_A>;
#[doc = "JAUTO\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JAUTO_A {
    #[doc = "0: Automatic injected group conversion disabled"]
    DISABLED = 0,
    #[doc = "1: Automatic injected group conversion enabled"]
    ENABLED = 1,
}
impl From<JAUTO_A> for bool {
    #[inline(always)]
    fn from(variant: JAUTO_A) -> Self {
        variant as u8 != 0
    }
}
impl JAUTO_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> JAUTO_A {
        match self.bits {
            false => JAUTO_A::DISABLED,
            true => JAUTO_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == JAUTO_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == JAUTO_A::ENABLED
    }
}
#[doc = "Field `JAUTO` writer - JAUTO"]
pub type JAUTO_W<'a, const O: u8> = crate::BitWriter<'a, CFGR_SPEC, O, JAUTO_A>;
impl<'a, const O: u8> JAUTO_W<'a, O> {
    #[doc = "Automatic injected group conversion disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(JAUTO_A::DISABLED)
    }
    #[doc = "Automatic injected group conversion enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(JAUTO_A::ENABLED)
    }
}
#[doc = "Field `AWD1CH` reader - AWDCH1CH"]
pub type AWD1CH_R = crate::FieldReader;
#[doc = "Field `AWD1CH` writer - AWDCH1CH"]
pub type AWD1CH_W<'a, const O: u8> = crate::FieldWriter<'a, CFGR_SPEC, 5, O>;
impl R {
    #[doc = "Bit 0 - DMAEN"]
    #[inline(always)]
    pub fn dmaen(&self) -> DMAEN_R {
        DMAEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DMACFG"]
    #[inline(always)]
    pub fn dmacfg(&self) -> DMACFG_R {
        DMACFG_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 3:4 - RES"]
    #[inline(always)]
    pub fn res(&self) -> RES_R {
        RES_R::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bit 5 - ALIGN"]
    #[inline(always)]
    pub fn align(&self) -> ALIGN_R {
        ALIGN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:9 - EXTSEL"]
    #[inline(always)]
    pub fn extsel(&self) -> EXTSEL_R {
        EXTSEL_R::new(((self.bits >> 6) & 0x0f) as u8)
    }
    #[doc = "Bits 10:11 - EXTEN"]
    #[inline(always)]
    pub fn exten(&self) -> EXTEN_R {
        EXTEN_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bit 12 - OVRMOD"]
    #[inline(always)]
    pub fn ovrmod(&self) -> OVRMOD_R {
        OVRMOD_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - CONT"]
    #[inline(always)]
    pub fn cont(&self) -> CONT_R {
        CONT_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - AUTDLY"]
    #[inline(always)]
    pub fn autdly(&self) -> AUTDLY_R {
        AUTDLY_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - DISCEN"]
    #[inline(always)]
    pub fn discen(&self) -> DISCEN_R {
        DISCEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:19 - DISCNUM"]
    #[inline(always)]
    pub fn discnum(&self) -> DISCNUM_R {
        DISCNUM_R::new(((self.bits >> 17) & 7) as u8)
    }
    #[doc = "Bit 20 - JDISCEN"]
    #[inline(always)]
    pub fn jdiscen(&self) -> JDISCEN_R {
        JDISCEN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - JQM"]
    #[inline(always)]
    pub fn jqm(&self) -> JQM_R {
        JQM_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - AWD1SGL"]
    #[inline(always)]
    pub fn awd1sgl(&self) -> AWD1SGL_R {
        AWD1SGL_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - AWD1EN"]
    #[inline(always)]
    pub fn awd1en(&self) -> AWD1EN_R {
        AWD1EN_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - JAWD1EN"]
    #[inline(always)]
    pub fn jawd1en(&self) -> JAWD1EN_R {
        JAWD1EN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - JAUTO"]
    #[inline(always)]
    pub fn jauto(&self) -> JAUTO_R {
        JAUTO_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bits 26:30 - AWDCH1CH"]
    #[inline(always)]
    pub fn awd1ch(&self) -> AWD1CH_R {
        AWD1CH_R::new(((self.bits >> 26) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - DMAEN"]
    #[inline(always)]
    #[must_use]
    pub fn dmaen(&mut self) -> DMAEN_W<0> {
        DMAEN_W::new(self)
    }
    #[doc = "Bit 1 - DMACFG"]
    #[inline(always)]
    #[must_use]
    pub fn dmacfg(&mut self) -> DMACFG_W<1> {
        DMACFG_W::new(self)
    }
    #[doc = "Bits 3:4 - RES"]
    #[inline(always)]
    #[must_use]
    pub fn res(&mut self) -> RES_W<3> {
        RES_W::new(self)
    }
    #[doc = "Bit 5 - ALIGN"]
    #[inline(always)]
    #[must_use]
    pub fn align(&mut self) -> ALIGN_W<5> {
        ALIGN_W::new(self)
    }
    #[doc = "Bits 6:9 - EXTSEL"]
    #[inline(always)]
    #[must_use]
    pub fn extsel(&mut self) -> EXTSEL_W<6> {
        EXTSEL_W::new(self)
    }
    #[doc = "Bits 10:11 - EXTEN"]
    #[inline(always)]
    #[must_use]
    pub fn exten(&mut self) -> EXTEN_W<10> {
        EXTEN_W::new(self)
    }
    #[doc = "Bit 12 - OVRMOD"]
    #[inline(always)]
    #[must_use]
    pub fn ovrmod(&mut self) -> OVRMOD_W<12> {
        OVRMOD_W::new(self)
    }
    #[doc = "Bit 13 - CONT"]
    #[inline(always)]
    #[must_use]
    pub fn cont(&mut self) -> CONT_W<13> {
        CONT_W::new(self)
    }
    #[doc = "Bit 14 - AUTDLY"]
    #[inline(always)]
    #[must_use]
    pub fn autdly(&mut self) -> AUTDLY_W<14> {
        AUTDLY_W::new(self)
    }
    #[doc = "Bit 16 - DISCEN"]
    #[inline(always)]
    #[must_use]
    pub fn discen(&mut self) -> DISCEN_W<16> {
        DISCEN_W::new(self)
    }
    #[doc = "Bits 17:19 - DISCNUM"]
    #[inline(always)]
    #[must_use]
    pub fn discnum(&mut self) -> DISCNUM_W<17> {
        DISCNUM_W::new(self)
    }
    #[doc = "Bit 20 - JDISCEN"]
    #[inline(always)]
    #[must_use]
    pub fn jdiscen(&mut self) -> JDISCEN_W<20> {
        JDISCEN_W::new(self)
    }
    #[doc = "Bit 21 - JQM"]
    #[inline(always)]
    #[must_use]
    pub fn jqm(&mut self) -> JQM_W<21> {
        JQM_W::new(self)
    }
    #[doc = "Bit 22 - AWD1SGL"]
    #[inline(always)]
    #[must_use]
    pub fn awd1sgl(&mut self) -> AWD1SGL_W<22> {
        AWD1SGL_W::new(self)
    }
    #[doc = "Bit 23 - AWD1EN"]
    #[inline(always)]
    #[must_use]
    pub fn awd1en(&mut self) -> AWD1EN_W<23> {
        AWD1EN_W::new(self)
    }
    #[doc = "Bit 24 - JAWD1EN"]
    #[inline(always)]
    #[must_use]
    pub fn jawd1en(&mut self) -> JAWD1EN_W<24> {
        JAWD1EN_W::new(self)
    }
    #[doc = "Bit 25 - JAUTO"]
    #[inline(always)]
    #[must_use]
    pub fn jauto(&mut self) -> JAUTO_W<25> {
        JAUTO_W::new(self)
    }
    #[doc = "Bits 26:30 - AWDCH1CH"]
    #[inline(always)]
    #[must_use]
    pub fn awd1ch(&mut self) -> AWD1CH_W<26> {
        AWD1CH_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfgr](index.html) module"]
pub struct CFGR_SPEC;
impl crate::RegisterSpec for CFGR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfgr::R](R) reader structure"]
impl crate::Readable for CFGR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cfgr::W](W) writer structure"]
impl crate::Writable for CFGR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CFGR to value 0"]
impl crate::Resettable for CFGR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

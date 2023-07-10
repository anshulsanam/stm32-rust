#[doc = "Register `JSQR` reader"]
pub struct R(crate::R<JSQR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<JSQR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<JSQR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<JSQR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `JSQR` writer"]
pub struct W(crate::W<JSQR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<JSQR_SPEC>;
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
impl From<crate::W<JSQR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<JSQR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `JL` reader - JL"]
pub type JL_R = crate::FieldReader;
#[doc = "Field `JL` writer - JL"]
pub type JL_W<'a, const O: u8> = crate::FieldWriterSafe<'a, JSQR_SPEC, 2, O>;
#[doc = "Field `JEXTSEL` reader - JEXTSEL"]
pub type JEXTSEL_R = crate::FieldReader<JEXTSEL_A>;
#[doc = "JEXTSEL\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum JEXTSEL_A {
    #[doc = "9: HRTIM_ADCTRG2 event"]
    HRTIM_ADCTRG2 = 9,
    #[doc = "10: HRTIM_ADCTRG4 event"]
    HRTIM_ADCTRG4 = 10,
    #[doc = "0: Timer 1 TRGO event"]
    TIM1_TRGO = 0,
    #[doc = "1: Timer 1 CC4 event"]
    TIM1_CC4 = 1,
    #[doc = "2: Timer 2 TRGO event"]
    TIM2_TRGO = 2,
    #[doc = "3: Timer 2 CC1 event"]
    TIM2_CC1 = 3,
    #[doc = "4: Timer 3 CC4 event"]
    TIM3_CC4 = 4,
    #[doc = "6: EXTI line 15"]
    EXTI15 = 6,
    #[doc = "8: Timer 1 TRGO2 event"]
    TIM1_TRGO2 = 8,
    #[doc = "11: Timer 3 CC3 event"]
    TIM3_CC3 = 11,
    #[doc = "12: Timer 3 TRGO event"]
    TIM3_TRGO = 12,
    #[doc = "13: Timer 3 CC1 event"]
    TIM3_CC1 = 13,
    #[doc = "14: Timer 6 TRGO event"]
    TIM6_TRGO = 14,
    #[doc = "15: Timer 15 TRGO event"]
    TIM15_TRGO = 15,
}
impl From<JEXTSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: JEXTSEL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for JEXTSEL_A {
    type Ux = u8;
}
impl JEXTSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<JEXTSEL_A> {
        match self.bits {
            9 => Some(JEXTSEL_A::HRTIM_ADCTRG2),
            10 => Some(JEXTSEL_A::HRTIM_ADCTRG4),
            0 => Some(JEXTSEL_A::TIM1_TRGO),
            1 => Some(JEXTSEL_A::TIM1_CC4),
            2 => Some(JEXTSEL_A::TIM2_TRGO),
            3 => Some(JEXTSEL_A::TIM2_CC1),
            4 => Some(JEXTSEL_A::TIM3_CC4),
            6 => Some(JEXTSEL_A::EXTI15),
            8 => Some(JEXTSEL_A::TIM1_TRGO2),
            11 => Some(JEXTSEL_A::TIM3_CC3),
            12 => Some(JEXTSEL_A::TIM3_TRGO),
            13 => Some(JEXTSEL_A::TIM3_CC1),
            14 => Some(JEXTSEL_A::TIM6_TRGO),
            15 => Some(JEXTSEL_A::TIM15_TRGO),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `HRTIM_ADCTRG2`"]
    #[inline(always)]
    pub fn is_hrtim_adctrg2(&self) -> bool {
        *self == JEXTSEL_A::HRTIM_ADCTRG2
    }
    #[doc = "Checks if the value of the field is `HRTIM_ADCTRG4`"]
    #[inline(always)]
    pub fn is_hrtim_adctrg4(&self) -> bool {
        *self == JEXTSEL_A::HRTIM_ADCTRG4
    }
    #[doc = "Checks if the value of the field is `TIM1_TRGO`"]
    #[inline(always)]
    pub fn is_tim1_trgo(&self) -> bool {
        *self == JEXTSEL_A::TIM1_TRGO
    }
    #[doc = "Checks if the value of the field is `TIM1_CC4`"]
    #[inline(always)]
    pub fn is_tim1_cc4(&self) -> bool {
        *self == JEXTSEL_A::TIM1_CC4
    }
    #[doc = "Checks if the value of the field is `TIM2_TRGO`"]
    #[inline(always)]
    pub fn is_tim2_trgo(&self) -> bool {
        *self == JEXTSEL_A::TIM2_TRGO
    }
    #[doc = "Checks if the value of the field is `TIM2_CC1`"]
    #[inline(always)]
    pub fn is_tim2_cc1(&self) -> bool {
        *self == JEXTSEL_A::TIM2_CC1
    }
    #[doc = "Checks if the value of the field is `TIM3_CC4`"]
    #[inline(always)]
    pub fn is_tim3_cc4(&self) -> bool {
        *self == JEXTSEL_A::TIM3_CC4
    }
    #[doc = "Checks if the value of the field is `EXTI15`"]
    #[inline(always)]
    pub fn is_exti15(&self) -> bool {
        *self == JEXTSEL_A::EXTI15
    }
    #[doc = "Checks if the value of the field is `TIM1_TRGO2`"]
    #[inline(always)]
    pub fn is_tim1_trgo2(&self) -> bool {
        *self == JEXTSEL_A::TIM1_TRGO2
    }
    #[doc = "Checks if the value of the field is `TIM3_CC3`"]
    #[inline(always)]
    pub fn is_tim3_cc3(&self) -> bool {
        *self == JEXTSEL_A::TIM3_CC3
    }
    #[doc = "Checks if the value of the field is `TIM3_TRGO`"]
    #[inline(always)]
    pub fn is_tim3_trgo(&self) -> bool {
        *self == JEXTSEL_A::TIM3_TRGO
    }
    #[doc = "Checks if the value of the field is `TIM3_CC1`"]
    #[inline(always)]
    pub fn is_tim3_cc1(&self) -> bool {
        *self == JEXTSEL_A::TIM3_CC1
    }
    #[doc = "Checks if the value of the field is `TIM6_TRGO`"]
    #[inline(always)]
    pub fn is_tim6_trgo(&self) -> bool {
        *self == JEXTSEL_A::TIM6_TRGO
    }
    #[doc = "Checks if the value of the field is `TIM15_TRGO`"]
    #[inline(always)]
    pub fn is_tim15_trgo(&self) -> bool {
        *self == JEXTSEL_A::TIM15_TRGO
    }
}
#[doc = "Field `JEXTSEL` writer - JEXTSEL"]
pub type JEXTSEL_W<'a, const O: u8> = crate::FieldWriter<'a, JSQR_SPEC, 4, O, JEXTSEL_A>;
impl<'a, const O: u8> JEXTSEL_W<'a, O> {
    #[doc = "HRTIM_ADCTRG2 event"]
    #[inline(always)]
    pub fn hrtim_adctrg2(self) -> &'a mut W {
        self.variant(JEXTSEL_A::HRTIM_ADCTRG2)
    }
    #[doc = "HRTIM_ADCTRG4 event"]
    #[inline(always)]
    pub fn hrtim_adctrg4(self) -> &'a mut W {
        self.variant(JEXTSEL_A::HRTIM_ADCTRG4)
    }
    #[doc = "Timer 1 TRGO event"]
    #[inline(always)]
    pub fn tim1_trgo(self) -> &'a mut W {
        self.variant(JEXTSEL_A::TIM1_TRGO)
    }
    #[doc = "Timer 1 CC4 event"]
    #[inline(always)]
    pub fn tim1_cc4(self) -> &'a mut W {
        self.variant(JEXTSEL_A::TIM1_CC4)
    }
    #[doc = "Timer 2 TRGO event"]
    #[inline(always)]
    pub fn tim2_trgo(self) -> &'a mut W {
        self.variant(JEXTSEL_A::TIM2_TRGO)
    }
    #[doc = "Timer 2 CC1 event"]
    #[inline(always)]
    pub fn tim2_cc1(self) -> &'a mut W {
        self.variant(JEXTSEL_A::TIM2_CC1)
    }
    #[doc = "Timer 3 CC4 event"]
    #[inline(always)]
    pub fn tim3_cc4(self) -> &'a mut W {
        self.variant(JEXTSEL_A::TIM3_CC4)
    }
    #[doc = "EXTI line 15"]
    #[inline(always)]
    pub fn exti15(self) -> &'a mut W {
        self.variant(JEXTSEL_A::EXTI15)
    }
    #[doc = "Timer 1 TRGO2 event"]
    #[inline(always)]
    pub fn tim1_trgo2(self) -> &'a mut W {
        self.variant(JEXTSEL_A::TIM1_TRGO2)
    }
    #[doc = "Timer 3 CC3 event"]
    #[inline(always)]
    pub fn tim3_cc3(self) -> &'a mut W {
        self.variant(JEXTSEL_A::TIM3_CC3)
    }
    #[doc = "Timer 3 TRGO event"]
    #[inline(always)]
    pub fn tim3_trgo(self) -> &'a mut W {
        self.variant(JEXTSEL_A::TIM3_TRGO)
    }
    #[doc = "Timer 3 CC1 event"]
    #[inline(always)]
    pub fn tim3_cc1(self) -> &'a mut W {
        self.variant(JEXTSEL_A::TIM3_CC1)
    }
    #[doc = "Timer 6 TRGO event"]
    #[inline(always)]
    pub fn tim6_trgo(self) -> &'a mut W {
        self.variant(JEXTSEL_A::TIM6_TRGO)
    }
    #[doc = "Timer 15 TRGO event"]
    #[inline(always)]
    pub fn tim15_trgo(self) -> &'a mut W {
        self.variant(JEXTSEL_A::TIM15_TRGO)
    }
}
#[doc = "Field `JEXTEN` reader - JEXTEN"]
pub type JEXTEN_R = crate::FieldReader<JEXTEN_A>;
#[doc = "JEXTEN\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum JEXTEN_A {
    #[doc = "0: Trigger detection disabled"]
    DISABLED = 0,
    #[doc = "1: Trigger detection on the rising edge"]
    RISING_EDGE = 1,
    #[doc = "2: Trigger detection on the falling edge"]
    FALLING_EDGE = 2,
    #[doc = "3: Trigger detection on both the rising and falling edges"]
    BOTH_EDGES = 3,
}
impl From<JEXTEN_A> for u8 {
    #[inline(always)]
    fn from(variant: JEXTEN_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for JEXTEN_A {
    type Ux = u8;
}
impl JEXTEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> JEXTEN_A {
        match self.bits {
            0 => JEXTEN_A::DISABLED,
            1 => JEXTEN_A::RISING_EDGE,
            2 => JEXTEN_A::FALLING_EDGE,
            3 => JEXTEN_A::BOTH_EDGES,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == JEXTEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `RISING_EDGE`"]
    #[inline(always)]
    pub fn is_rising_edge(&self) -> bool {
        *self == JEXTEN_A::RISING_EDGE
    }
    #[doc = "Checks if the value of the field is `FALLING_EDGE`"]
    #[inline(always)]
    pub fn is_falling_edge(&self) -> bool {
        *self == JEXTEN_A::FALLING_EDGE
    }
    #[doc = "Checks if the value of the field is `BOTH_EDGES`"]
    #[inline(always)]
    pub fn is_both_edges(&self) -> bool {
        *self == JEXTEN_A::BOTH_EDGES
    }
}
#[doc = "Field `JEXTEN` writer - JEXTEN"]
pub type JEXTEN_W<'a, const O: u8> = crate::FieldWriterSafe<'a, JSQR_SPEC, 2, O, JEXTEN_A>;
impl<'a, const O: u8> JEXTEN_W<'a, O> {
    #[doc = "Trigger detection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(JEXTEN_A::DISABLED)
    }
    #[doc = "Trigger detection on the rising edge"]
    #[inline(always)]
    pub fn rising_edge(self) -> &'a mut W {
        self.variant(JEXTEN_A::RISING_EDGE)
    }
    #[doc = "Trigger detection on the falling edge"]
    #[inline(always)]
    pub fn falling_edge(self) -> &'a mut W {
        self.variant(JEXTEN_A::FALLING_EDGE)
    }
    #[doc = "Trigger detection on both the rising and falling edges"]
    #[inline(always)]
    pub fn both_edges(self) -> &'a mut W {
        self.variant(JEXTEN_A::BOTH_EDGES)
    }
}
#[doc = "Field `JSQ1` reader - JSQ1"]
pub type JSQ1_R = crate::FieldReader;
#[doc = "Field `JSQ1` writer - JSQ1"]
pub type JSQ1_W<'a, const O: u8> = crate::FieldWriter<'a, JSQR_SPEC, 5, O>;
#[doc = "Field `JSQ2` reader - JSQ2"]
pub type JSQ2_R = crate::FieldReader;
#[doc = "Field `JSQ2` writer - JSQ2"]
pub type JSQ2_W<'a, const O: u8> = crate::FieldWriter<'a, JSQR_SPEC, 5, O>;
#[doc = "Field `JSQ3` reader - JSQ3"]
pub type JSQ3_R = crate::FieldReader;
#[doc = "Field `JSQ3` writer - JSQ3"]
pub type JSQ3_W<'a, const O: u8> = crate::FieldWriter<'a, JSQR_SPEC, 5, O>;
#[doc = "Field `JSQ4` reader - JSQ4"]
pub type JSQ4_R = crate::FieldReader;
#[doc = "Field `JSQ4` writer - JSQ4"]
pub type JSQ4_W<'a, const O: u8> = crate::FieldWriter<'a, JSQR_SPEC, 5, O>;
impl R {
    #[doc = "Bits 0:1 - JL"]
    #[inline(always)]
    pub fn jl(&self) -> JL_R {
        JL_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:5 - JEXTSEL"]
    #[inline(always)]
    pub fn jextsel(&self) -> JEXTSEL_R {
        JEXTSEL_R::new(((self.bits >> 2) & 0x0f) as u8)
    }
    #[doc = "Bits 6:7 - JEXTEN"]
    #[inline(always)]
    pub fn jexten(&self) -> JEXTEN_R {
        JEXTEN_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:12 - JSQ1"]
    #[inline(always)]
    pub fn jsq1(&self) -> JSQ1_R {
        JSQ1_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 14:18 - JSQ2"]
    #[inline(always)]
    pub fn jsq2(&self) -> JSQ2_R {
        JSQ2_R::new(((self.bits >> 14) & 0x1f) as u8)
    }
    #[doc = "Bits 20:24 - JSQ3"]
    #[inline(always)]
    pub fn jsq3(&self) -> JSQ3_R {
        JSQ3_R::new(((self.bits >> 20) & 0x1f) as u8)
    }
    #[doc = "Bits 26:30 - JSQ4"]
    #[inline(always)]
    pub fn jsq4(&self) -> JSQ4_R {
        JSQ4_R::new(((self.bits >> 26) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - JL"]
    #[inline(always)]
    #[must_use]
    pub fn jl(&mut self) -> JL_W<0> {
        JL_W::new(self)
    }
    #[doc = "Bits 2:5 - JEXTSEL"]
    #[inline(always)]
    #[must_use]
    pub fn jextsel(&mut self) -> JEXTSEL_W<2> {
        JEXTSEL_W::new(self)
    }
    #[doc = "Bits 6:7 - JEXTEN"]
    #[inline(always)]
    #[must_use]
    pub fn jexten(&mut self) -> JEXTEN_W<6> {
        JEXTEN_W::new(self)
    }
    #[doc = "Bits 8:12 - JSQ1"]
    #[inline(always)]
    #[must_use]
    pub fn jsq1(&mut self) -> JSQ1_W<8> {
        JSQ1_W::new(self)
    }
    #[doc = "Bits 14:18 - JSQ2"]
    #[inline(always)]
    #[must_use]
    pub fn jsq2(&mut self) -> JSQ2_W<14> {
        JSQ2_W::new(self)
    }
    #[doc = "Bits 20:24 - JSQ3"]
    #[inline(always)]
    #[must_use]
    pub fn jsq3(&mut self) -> JSQ3_W<20> {
        JSQ3_W::new(self)
    }
    #[doc = "Bits 26:30 - JSQ4"]
    #[inline(always)]
    #[must_use]
    pub fn jsq4(&mut self) -> JSQ4_W<26> {
        JSQ4_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "injected sequence register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [jsqr](index.html) module"]
pub struct JSQR_SPEC;
impl crate::RegisterSpec for JSQR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [jsqr::R](R) reader structure"]
impl crate::Readable for JSQR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [jsqr::W](W) writer structure"]
impl crate::Writable for JSQR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets JSQR to value 0"]
impl crate::Resettable for JSQR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

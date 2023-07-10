#[doc = "Register `EXTICR1` reader"]
pub struct R(crate::R<EXTICR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EXTICR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EXTICR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EXTICR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EXTICR1` writer"]
pub struct W(crate::W<EXTICR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EXTICR1_SPEC>;
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
impl From<crate::W<EXTICR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EXTICR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EXTI0` reader - EXTI 0 configuration bits"]
pub type EXTI0_R = crate::FieldReader<EXTI0_A>;
#[doc = "EXTI 0 configuration bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EXTI0_A {
    #[doc = "0: Select PA0 as the source input for the EXTI0 external interrupt"]
    PA0 = 0,
    #[doc = "1: Select PB0 as the source input for the EXTI0 external interrupt"]
    PB0 = 1,
    #[doc = "2: Select PC0 as the source input for the EXTI0 external interrupt"]
    PC0 = 2,
    #[doc = "3: Select PD0 as the source input for the EXTI0 external interrupt"]
    PD0 = 3,
    #[doc = "4: Select PE0 as the source input for the EXTI0 external interrupt"]
    PE0 = 4,
    #[doc = "5: Select PF0 as the source input for the EXTI0 external interrupt"]
    PF0 = 5,
    #[doc = "6: Select PG0 as the source input for the EXTI0 external interrupt"]
    PG0 = 6,
    #[doc = "7: Select PH0 as the source input for the EXTI0 external interrupt"]
    PH0 = 7,
}
impl From<EXTI0_A> for u8 {
    #[inline(always)]
    fn from(variant: EXTI0_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for EXTI0_A {
    type Ux = u8;
}
impl EXTI0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<EXTI0_A> {
        match self.bits {
            0 => Some(EXTI0_A::PA0),
            1 => Some(EXTI0_A::PB0),
            2 => Some(EXTI0_A::PC0),
            3 => Some(EXTI0_A::PD0),
            4 => Some(EXTI0_A::PE0),
            5 => Some(EXTI0_A::PF0),
            6 => Some(EXTI0_A::PG0),
            7 => Some(EXTI0_A::PH0),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `PA0`"]
    #[inline(always)]
    pub fn is_pa0(&self) -> bool {
        *self == EXTI0_A::PA0
    }
    #[doc = "Checks if the value of the field is `PB0`"]
    #[inline(always)]
    pub fn is_pb0(&self) -> bool {
        *self == EXTI0_A::PB0
    }
    #[doc = "Checks if the value of the field is `PC0`"]
    #[inline(always)]
    pub fn is_pc0(&self) -> bool {
        *self == EXTI0_A::PC0
    }
    #[doc = "Checks if the value of the field is `PD0`"]
    #[inline(always)]
    pub fn is_pd0(&self) -> bool {
        *self == EXTI0_A::PD0
    }
    #[doc = "Checks if the value of the field is `PE0`"]
    #[inline(always)]
    pub fn is_pe0(&self) -> bool {
        *self == EXTI0_A::PE0
    }
    #[doc = "Checks if the value of the field is `PF0`"]
    #[inline(always)]
    pub fn is_pf0(&self) -> bool {
        *self == EXTI0_A::PF0
    }
    #[doc = "Checks if the value of the field is `PG0`"]
    #[inline(always)]
    pub fn is_pg0(&self) -> bool {
        *self == EXTI0_A::PG0
    }
    #[doc = "Checks if the value of the field is `PH0`"]
    #[inline(always)]
    pub fn is_ph0(&self) -> bool {
        *self == EXTI0_A::PH0
    }
}
#[doc = "Field `EXTI0` writer - EXTI 0 configuration bits"]
pub type EXTI0_W<'a, const O: u8> = crate::FieldWriter<'a, EXTICR1_SPEC, 4, O, EXTI0_A>;
impl<'a, const O: u8> EXTI0_W<'a, O> {
    #[doc = "Select PA0 as the source input for the EXTI0 external interrupt"]
    #[inline(always)]
    pub fn pa0(self) -> &'a mut W {
        self.variant(EXTI0_A::PA0)
    }
    #[doc = "Select PB0 as the source input for the EXTI0 external interrupt"]
    #[inline(always)]
    pub fn pb0(self) -> &'a mut W {
        self.variant(EXTI0_A::PB0)
    }
    #[doc = "Select PC0 as the source input for the EXTI0 external interrupt"]
    #[inline(always)]
    pub fn pc0(self) -> &'a mut W {
        self.variant(EXTI0_A::PC0)
    }
    #[doc = "Select PD0 as the source input for the EXTI0 external interrupt"]
    #[inline(always)]
    pub fn pd0(self) -> &'a mut W {
        self.variant(EXTI0_A::PD0)
    }
    #[doc = "Select PE0 as the source input for the EXTI0 external interrupt"]
    #[inline(always)]
    pub fn pe0(self) -> &'a mut W {
        self.variant(EXTI0_A::PE0)
    }
    #[doc = "Select PF0 as the source input for the EXTI0 external interrupt"]
    #[inline(always)]
    pub fn pf0(self) -> &'a mut W {
        self.variant(EXTI0_A::PF0)
    }
    #[doc = "Select PG0 as the source input for the EXTI0 external interrupt"]
    #[inline(always)]
    pub fn pg0(self) -> &'a mut W {
        self.variant(EXTI0_A::PG0)
    }
    #[doc = "Select PH0 as the source input for the EXTI0 external interrupt"]
    #[inline(always)]
    pub fn ph0(self) -> &'a mut W {
        self.variant(EXTI0_A::PH0)
    }
}
#[doc = "Field `EXTI1` reader - EXTI 1 configuration bits"]
pub type EXTI1_R = crate::FieldReader<EXTI1_A>;
#[doc = "EXTI 1 configuration bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EXTI1_A {
    #[doc = "0: Select PA1 as the source input for the EXTI1 external interrupt"]
    PA1 = 0,
    #[doc = "1: Select PB1 as the source input for the EXTI1 external interrupt"]
    PB1 = 1,
    #[doc = "2: Select PC1 as the source input for the EXTI1 external interrupt"]
    PC1 = 2,
    #[doc = "3: Select PD1 as the source input for the EXTI1 external interrupt"]
    PD1 = 3,
    #[doc = "4: Select PE1 as the source input for the EXTI1 external interrupt"]
    PE1 = 4,
    #[doc = "5: Select PF1 as the source input for the EXTI1 external interrupt"]
    PF1 = 5,
    #[doc = "6: Select PG1 as the source input for the EXTI1 external interrupt"]
    PG1 = 6,
    #[doc = "7: Select PH1 as the source input for the EXTI1 external interrupt"]
    PH1 = 7,
}
impl From<EXTI1_A> for u8 {
    #[inline(always)]
    fn from(variant: EXTI1_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for EXTI1_A {
    type Ux = u8;
}
impl EXTI1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<EXTI1_A> {
        match self.bits {
            0 => Some(EXTI1_A::PA1),
            1 => Some(EXTI1_A::PB1),
            2 => Some(EXTI1_A::PC1),
            3 => Some(EXTI1_A::PD1),
            4 => Some(EXTI1_A::PE1),
            5 => Some(EXTI1_A::PF1),
            6 => Some(EXTI1_A::PG1),
            7 => Some(EXTI1_A::PH1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `PA1`"]
    #[inline(always)]
    pub fn is_pa1(&self) -> bool {
        *self == EXTI1_A::PA1
    }
    #[doc = "Checks if the value of the field is `PB1`"]
    #[inline(always)]
    pub fn is_pb1(&self) -> bool {
        *self == EXTI1_A::PB1
    }
    #[doc = "Checks if the value of the field is `PC1`"]
    #[inline(always)]
    pub fn is_pc1(&self) -> bool {
        *self == EXTI1_A::PC1
    }
    #[doc = "Checks if the value of the field is `PD1`"]
    #[inline(always)]
    pub fn is_pd1(&self) -> bool {
        *self == EXTI1_A::PD1
    }
    #[doc = "Checks if the value of the field is `PE1`"]
    #[inline(always)]
    pub fn is_pe1(&self) -> bool {
        *self == EXTI1_A::PE1
    }
    #[doc = "Checks if the value of the field is `PF1`"]
    #[inline(always)]
    pub fn is_pf1(&self) -> bool {
        *self == EXTI1_A::PF1
    }
    #[doc = "Checks if the value of the field is `PG1`"]
    #[inline(always)]
    pub fn is_pg1(&self) -> bool {
        *self == EXTI1_A::PG1
    }
    #[doc = "Checks if the value of the field is `PH1`"]
    #[inline(always)]
    pub fn is_ph1(&self) -> bool {
        *self == EXTI1_A::PH1
    }
}
#[doc = "Field `EXTI1` writer - EXTI 1 configuration bits"]
pub type EXTI1_W<'a, const O: u8> = crate::FieldWriter<'a, EXTICR1_SPEC, 4, O, EXTI1_A>;
impl<'a, const O: u8> EXTI1_W<'a, O> {
    #[doc = "Select PA1 as the source input for the EXTI1 external interrupt"]
    #[inline(always)]
    pub fn pa1(self) -> &'a mut W {
        self.variant(EXTI1_A::PA1)
    }
    #[doc = "Select PB1 as the source input for the EXTI1 external interrupt"]
    #[inline(always)]
    pub fn pb1(self) -> &'a mut W {
        self.variant(EXTI1_A::PB1)
    }
    #[doc = "Select PC1 as the source input for the EXTI1 external interrupt"]
    #[inline(always)]
    pub fn pc1(self) -> &'a mut W {
        self.variant(EXTI1_A::PC1)
    }
    #[doc = "Select PD1 as the source input for the EXTI1 external interrupt"]
    #[inline(always)]
    pub fn pd1(self) -> &'a mut W {
        self.variant(EXTI1_A::PD1)
    }
    #[doc = "Select PE1 as the source input for the EXTI1 external interrupt"]
    #[inline(always)]
    pub fn pe1(self) -> &'a mut W {
        self.variant(EXTI1_A::PE1)
    }
    #[doc = "Select PF1 as the source input for the EXTI1 external interrupt"]
    #[inline(always)]
    pub fn pf1(self) -> &'a mut W {
        self.variant(EXTI1_A::PF1)
    }
    #[doc = "Select PG1 as the source input for the EXTI1 external interrupt"]
    #[inline(always)]
    pub fn pg1(self) -> &'a mut W {
        self.variant(EXTI1_A::PG1)
    }
    #[doc = "Select PH1 as the source input for the EXTI1 external interrupt"]
    #[inline(always)]
    pub fn ph1(self) -> &'a mut W {
        self.variant(EXTI1_A::PH1)
    }
}
#[doc = "Field `EXTI2` reader - EXTI 2 configuration bits"]
pub type EXTI2_R = crate::FieldReader<EXTI2_A>;
#[doc = "EXTI 2 configuration bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EXTI2_A {
    #[doc = "0: Select PA2 as the source input for the EXTI2 external interrupt"]
    PA2 = 0,
    #[doc = "1: Select PB2 as the source input for the EXTI2 external interrupt"]
    PB2 = 1,
    #[doc = "2: Select PC2 as the source input for the EXTI2 external interrupt"]
    PC2 = 2,
    #[doc = "3: Select PD2 as the source input for the EXTI2 external interrupt"]
    PD2 = 3,
    #[doc = "4: Select PE2 as the source input for the EXTI2 external interrupt"]
    PE2 = 4,
    #[doc = "5: Select PF2 as the source input for the EXTI2 external interrupt"]
    PF2 = 5,
    #[doc = "6: Select PG2 as the source input for the EXTI2 external interrupt"]
    PG2 = 6,
    #[doc = "7: Select PH2 as the source input for the EXTI2 external interrupt"]
    PH2 = 7,
}
impl From<EXTI2_A> for u8 {
    #[inline(always)]
    fn from(variant: EXTI2_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for EXTI2_A {
    type Ux = u8;
}
impl EXTI2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<EXTI2_A> {
        match self.bits {
            0 => Some(EXTI2_A::PA2),
            1 => Some(EXTI2_A::PB2),
            2 => Some(EXTI2_A::PC2),
            3 => Some(EXTI2_A::PD2),
            4 => Some(EXTI2_A::PE2),
            5 => Some(EXTI2_A::PF2),
            6 => Some(EXTI2_A::PG2),
            7 => Some(EXTI2_A::PH2),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `PA2`"]
    #[inline(always)]
    pub fn is_pa2(&self) -> bool {
        *self == EXTI2_A::PA2
    }
    #[doc = "Checks if the value of the field is `PB2`"]
    #[inline(always)]
    pub fn is_pb2(&self) -> bool {
        *self == EXTI2_A::PB2
    }
    #[doc = "Checks if the value of the field is `PC2`"]
    #[inline(always)]
    pub fn is_pc2(&self) -> bool {
        *self == EXTI2_A::PC2
    }
    #[doc = "Checks if the value of the field is `PD2`"]
    #[inline(always)]
    pub fn is_pd2(&self) -> bool {
        *self == EXTI2_A::PD2
    }
    #[doc = "Checks if the value of the field is `PE2`"]
    #[inline(always)]
    pub fn is_pe2(&self) -> bool {
        *self == EXTI2_A::PE2
    }
    #[doc = "Checks if the value of the field is `PF2`"]
    #[inline(always)]
    pub fn is_pf2(&self) -> bool {
        *self == EXTI2_A::PF2
    }
    #[doc = "Checks if the value of the field is `PG2`"]
    #[inline(always)]
    pub fn is_pg2(&self) -> bool {
        *self == EXTI2_A::PG2
    }
    #[doc = "Checks if the value of the field is `PH2`"]
    #[inline(always)]
    pub fn is_ph2(&self) -> bool {
        *self == EXTI2_A::PH2
    }
}
#[doc = "Field `EXTI2` writer - EXTI 2 configuration bits"]
pub type EXTI2_W<'a, const O: u8> = crate::FieldWriter<'a, EXTICR1_SPEC, 4, O, EXTI2_A>;
impl<'a, const O: u8> EXTI2_W<'a, O> {
    #[doc = "Select PA2 as the source input for the EXTI2 external interrupt"]
    #[inline(always)]
    pub fn pa2(self) -> &'a mut W {
        self.variant(EXTI2_A::PA2)
    }
    #[doc = "Select PB2 as the source input for the EXTI2 external interrupt"]
    #[inline(always)]
    pub fn pb2(self) -> &'a mut W {
        self.variant(EXTI2_A::PB2)
    }
    #[doc = "Select PC2 as the source input for the EXTI2 external interrupt"]
    #[inline(always)]
    pub fn pc2(self) -> &'a mut W {
        self.variant(EXTI2_A::PC2)
    }
    #[doc = "Select PD2 as the source input for the EXTI2 external interrupt"]
    #[inline(always)]
    pub fn pd2(self) -> &'a mut W {
        self.variant(EXTI2_A::PD2)
    }
    #[doc = "Select PE2 as the source input for the EXTI2 external interrupt"]
    #[inline(always)]
    pub fn pe2(self) -> &'a mut W {
        self.variant(EXTI2_A::PE2)
    }
    #[doc = "Select PF2 as the source input for the EXTI2 external interrupt"]
    #[inline(always)]
    pub fn pf2(self) -> &'a mut W {
        self.variant(EXTI2_A::PF2)
    }
    #[doc = "Select PG2 as the source input for the EXTI2 external interrupt"]
    #[inline(always)]
    pub fn pg2(self) -> &'a mut W {
        self.variant(EXTI2_A::PG2)
    }
    #[doc = "Select PH2 as the source input for the EXTI2 external interrupt"]
    #[inline(always)]
    pub fn ph2(self) -> &'a mut W {
        self.variant(EXTI2_A::PH2)
    }
}
#[doc = "Field `EXTI3` reader - EXTI 3 configuration bits"]
pub type EXTI3_R = crate::FieldReader<EXTI3_A>;
#[doc = "EXTI 3 configuration bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EXTI3_A {
    #[doc = "0: Select PA3 as the source input for the EXTI3 external interrupt"]
    PA3 = 0,
    #[doc = "1: Select PB3 as the source input for the EXTI3 external interrupt"]
    PB3 = 1,
    #[doc = "2: Select PC3 as the source input for the EXTI3 external interrupt"]
    PC3 = 2,
    #[doc = "3: Select PD3 as the source input for the EXTI3 external interrupt"]
    PD3 = 3,
    #[doc = "4: Select PE3 as the source input for the EXTI3 external interrupt"]
    PE3 = 4,
    #[doc = "5: Select PF3 as the source input for the EXTI3 external interrupt"]
    PF3 = 5,
    #[doc = "6: Select PG3 as the source input for the EXTI3 external interrupt"]
    PG3 = 6,
}
impl From<EXTI3_A> for u8 {
    #[inline(always)]
    fn from(variant: EXTI3_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for EXTI3_A {
    type Ux = u8;
}
impl EXTI3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<EXTI3_A> {
        match self.bits {
            0 => Some(EXTI3_A::PA3),
            1 => Some(EXTI3_A::PB3),
            2 => Some(EXTI3_A::PC3),
            3 => Some(EXTI3_A::PD3),
            4 => Some(EXTI3_A::PE3),
            5 => Some(EXTI3_A::PF3),
            6 => Some(EXTI3_A::PG3),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `PA3`"]
    #[inline(always)]
    pub fn is_pa3(&self) -> bool {
        *self == EXTI3_A::PA3
    }
    #[doc = "Checks if the value of the field is `PB3`"]
    #[inline(always)]
    pub fn is_pb3(&self) -> bool {
        *self == EXTI3_A::PB3
    }
    #[doc = "Checks if the value of the field is `PC3`"]
    #[inline(always)]
    pub fn is_pc3(&self) -> bool {
        *self == EXTI3_A::PC3
    }
    #[doc = "Checks if the value of the field is `PD3`"]
    #[inline(always)]
    pub fn is_pd3(&self) -> bool {
        *self == EXTI3_A::PD3
    }
    #[doc = "Checks if the value of the field is `PE3`"]
    #[inline(always)]
    pub fn is_pe3(&self) -> bool {
        *self == EXTI3_A::PE3
    }
    #[doc = "Checks if the value of the field is `PF3`"]
    #[inline(always)]
    pub fn is_pf3(&self) -> bool {
        *self == EXTI3_A::PF3
    }
    #[doc = "Checks if the value of the field is `PG3`"]
    #[inline(always)]
    pub fn is_pg3(&self) -> bool {
        *self == EXTI3_A::PG3
    }
}
#[doc = "Field `EXTI3` writer - EXTI 3 configuration bits"]
pub type EXTI3_W<'a, const O: u8> = crate::FieldWriter<'a, EXTICR1_SPEC, 4, O, EXTI3_A>;
impl<'a, const O: u8> EXTI3_W<'a, O> {
    #[doc = "Select PA3 as the source input for the EXTI3 external interrupt"]
    #[inline(always)]
    pub fn pa3(self) -> &'a mut W {
        self.variant(EXTI3_A::PA3)
    }
    #[doc = "Select PB3 as the source input for the EXTI3 external interrupt"]
    #[inline(always)]
    pub fn pb3(self) -> &'a mut W {
        self.variant(EXTI3_A::PB3)
    }
    #[doc = "Select PC3 as the source input for the EXTI3 external interrupt"]
    #[inline(always)]
    pub fn pc3(self) -> &'a mut W {
        self.variant(EXTI3_A::PC3)
    }
    #[doc = "Select PD3 as the source input for the EXTI3 external interrupt"]
    #[inline(always)]
    pub fn pd3(self) -> &'a mut W {
        self.variant(EXTI3_A::PD3)
    }
    #[doc = "Select PE3 as the source input for the EXTI3 external interrupt"]
    #[inline(always)]
    pub fn pe3(self) -> &'a mut W {
        self.variant(EXTI3_A::PE3)
    }
    #[doc = "Select PF3 as the source input for the EXTI3 external interrupt"]
    #[inline(always)]
    pub fn pf3(self) -> &'a mut W {
        self.variant(EXTI3_A::PF3)
    }
    #[doc = "Select PG3 as the source input for the EXTI3 external interrupt"]
    #[inline(always)]
    pub fn pg3(self) -> &'a mut W {
        self.variant(EXTI3_A::PG3)
    }
}
impl R {
    #[doc = "Bits 0:3 - EXTI 0 configuration bits"]
    #[inline(always)]
    pub fn exti0(&self) -> EXTI0_R {
        EXTI0_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - EXTI 1 configuration bits"]
    #[inline(always)]
    pub fn exti1(&self) -> EXTI1_R {
        EXTI1_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - EXTI 2 configuration bits"]
    #[inline(always)]
    pub fn exti2(&self) -> EXTI2_R {
        EXTI2_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - EXTI 3 configuration bits"]
    #[inline(always)]
    pub fn exti3(&self) -> EXTI3_R {
        EXTI3_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - EXTI 0 configuration bits"]
    #[inline(always)]
    #[must_use]
    pub fn exti0(&mut self) -> EXTI0_W<0> {
        EXTI0_W::new(self)
    }
    #[doc = "Bits 4:7 - EXTI 1 configuration bits"]
    #[inline(always)]
    #[must_use]
    pub fn exti1(&mut self) -> EXTI1_W<4> {
        EXTI1_W::new(self)
    }
    #[doc = "Bits 8:11 - EXTI 2 configuration bits"]
    #[inline(always)]
    #[must_use]
    pub fn exti2(&mut self) -> EXTI2_W<8> {
        EXTI2_W::new(self)
    }
    #[doc = "Bits 12:15 - EXTI 3 configuration bits"]
    #[inline(always)]
    #[must_use]
    pub fn exti3(&mut self) -> EXTI3_W<12> {
        EXTI3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "external interrupt configuration register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [exticr1](index.html) module"]
pub struct EXTICR1_SPEC;
impl crate::RegisterSpec for EXTICR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [exticr1::R](R) reader structure"]
impl crate::Readable for EXTICR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [exticr1::W](W) writer structure"]
impl crate::Writable for EXTICR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EXTICR1 to value 0"]
impl crate::Resettable for EXTICR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

#[doc = "Register `EXTICR4` reader"]
pub struct R(crate::R<EXTICR4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EXTICR4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EXTICR4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EXTICR4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EXTICR4` writer"]
pub struct W(crate::W<EXTICR4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EXTICR4_SPEC>;
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
impl From<crate::W<EXTICR4_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EXTICR4_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EXTI12` reader - EXTI 12 configuration bits"]
pub type EXTI12_R = crate::FieldReader<EXTI12_A>;
#[doc = "EXTI 12 configuration bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EXTI12_A {
    #[doc = "0: Select PA12 as the source input for the EXTI12 external interrupt"]
    PA12 = 0,
    #[doc = "1: Select PB12 as the source input for the EXTI12 external interrupt"]
    PB12 = 1,
    #[doc = "2: Select PC12 as the source input for the EXTI12 external interrupt"]
    PC12 = 2,
    #[doc = "3: Select PD12 as the source input for the EXTI12 external interrupt"]
    PD12 = 3,
    #[doc = "4: Select PE12 as the source input for the EXTI12 external interrupt"]
    PE12 = 4,
    #[doc = "5: Select PF12 as the source input for the EXTI12 external interrupt"]
    PF12 = 5,
    #[doc = "6: Select PG12 as the source input for the EXTI12 external interrupt"]
    PG12 = 6,
}
impl From<EXTI12_A> for u8 {
    #[inline(always)]
    fn from(variant: EXTI12_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for EXTI12_A {
    type Ux = u8;
}
impl EXTI12_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<EXTI12_A> {
        match self.bits {
            0 => Some(EXTI12_A::PA12),
            1 => Some(EXTI12_A::PB12),
            2 => Some(EXTI12_A::PC12),
            3 => Some(EXTI12_A::PD12),
            4 => Some(EXTI12_A::PE12),
            5 => Some(EXTI12_A::PF12),
            6 => Some(EXTI12_A::PG12),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `PA12`"]
    #[inline(always)]
    pub fn is_pa12(&self) -> bool {
        *self == EXTI12_A::PA12
    }
    #[doc = "Checks if the value of the field is `PB12`"]
    #[inline(always)]
    pub fn is_pb12(&self) -> bool {
        *self == EXTI12_A::PB12
    }
    #[doc = "Checks if the value of the field is `PC12`"]
    #[inline(always)]
    pub fn is_pc12(&self) -> bool {
        *self == EXTI12_A::PC12
    }
    #[doc = "Checks if the value of the field is `PD12`"]
    #[inline(always)]
    pub fn is_pd12(&self) -> bool {
        *self == EXTI12_A::PD12
    }
    #[doc = "Checks if the value of the field is `PE12`"]
    #[inline(always)]
    pub fn is_pe12(&self) -> bool {
        *self == EXTI12_A::PE12
    }
    #[doc = "Checks if the value of the field is `PF12`"]
    #[inline(always)]
    pub fn is_pf12(&self) -> bool {
        *self == EXTI12_A::PF12
    }
    #[doc = "Checks if the value of the field is `PG12`"]
    #[inline(always)]
    pub fn is_pg12(&self) -> bool {
        *self == EXTI12_A::PG12
    }
}
#[doc = "Field `EXTI12` writer - EXTI 12 configuration bits"]
pub type EXTI12_W<'a, const O: u8> = crate::FieldWriter<'a, EXTICR4_SPEC, 4, O, EXTI12_A>;
impl<'a, const O: u8> EXTI12_W<'a, O> {
    #[doc = "Select PA12 as the source input for the EXTI12 external interrupt"]
    #[inline(always)]
    pub fn pa12(self) -> &'a mut W {
        self.variant(EXTI12_A::PA12)
    }
    #[doc = "Select PB12 as the source input for the EXTI12 external interrupt"]
    #[inline(always)]
    pub fn pb12(self) -> &'a mut W {
        self.variant(EXTI12_A::PB12)
    }
    #[doc = "Select PC12 as the source input for the EXTI12 external interrupt"]
    #[inline(always)]
    pub fn pc12(self) -> &'a mut W {
        self.variant(EXTI12_A::PC12)
    }
    #[doc = "Select PD12 as the source input for the EXTI12 external interrupt"]
    #[inline(always)]
    pub fn pd12(self) -> &'a mut W {
        self.variant(EXTI12_A::PD12)
    }
    #[doc = "Select PE12 as the source input for the EXTI12 external interrupt"]
    #[inline(always)]
    pub fn pe12(self) -> &'a mut W {
        self.variant(EXTI12_A::PE12)
    }
    #[doc = "Select PF12 as the source input for the EXTI12 external interrupt"]
    #[inline(always)]
    pub fn pf12(self) -> &'a mut W {
        self.variant(EXTI12_A::PF12)
    }
    #[doc = "Select PG12 as the source input for the EXTI12 external interrupt"]
    #[inline(always)]
    pub fn pg12(self) -> &'a mut W {
        self.variant(EXTI12_A::PG12)
    }
}
#[doc = "Field `EXTI13` reader - EXTI 13 configuration bits"]
pub type EXTI13_R = crate::FieldReader<EXTI13_A>;
#[doc = "EXTI 13 configuration bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EXTI13_A {
    #[doc = "0: Select PA13 as the source input for the EXTI13 external interrupt"]
    PA13 = 0,
    #[doc = "1: Select PB13 as the source input for the EXTI13 external interrupt"]
    PB13 = 1,
    #[doc = "2: Select PC13 as the source input for the EXTI13 external interrupt"]
    PC13 = 2,
    #[doc = "3: Select PD13 as the source input for the EXTI13 external interrupt"]
    PD13 = 3,
    #[doc = "4: Select PE13 as the source input for the EXTI13 external interrupt"]
    PE13 = 4,
    #[doc = "5: Select PF13 as the source input for the EXTI13 external interrupt"]
    PF13 = 5,
    #[doc = "6: Select PG13 as the source input for the EXTI13 external interrupt"]
    PG13 = 6,
}
impl From<EXTI13_A> for u8 {
    #[inline(always)]
    fn from(variant: EXTI13_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for EXTI13_A {
    type Ux = u8;
}
impl EXTI13_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<EXTI13_A> {
        match self.bits {
            0 => Some(EXTI13_A::PA13),
            1 => Some(EXTI13_A::PB13),
            2 => Some(EXTI13_A::PC13),
            3 => Some(EXTI13_A::PD13),
            4 => Some(EXTI13_A::PE13),
            5 => Some(EXTI13_A::PF13),
            6 => Some(EXTI13_A::PG13),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `PA13`"]
    #[inline(always)]
    pub fn is_pa13(&self) -> bool {
        *self == EXTI13_A::PA13
    }
    #[doc = "Checks if the value of the field is `PB13`"]
    #[inline(always)]
    pub fn is_pb13(&self) -> bool {
        *self == EXTI13_A::PB13
    }
    #[doc = "Checks if the value of the field is `PC13`"]
    #[inline(always)]
    pub fn is_pc13(&self) -> bool {
        *self == EXTI13_A::PC13
    }
    #[doc = "Checks if the value of the field is `PD13`"]
    #[inline(always)]
    pub fn is_pd13(&self) -> bool {
        *self == EXTI13_A::PD13
    }
    #[doc = "Checks if the value of the field is `PE13`"]
    #[inline(always)]
    pub fn is_pe13(&self) -> bool {
        *self == EXTI13_A::PE13
    }
    #[doc = "Checks if the value of the field is `PF13`"]
    #[inline(always)]
    pub fn is_pf13(&self) -> bool {
        *self == EXTI13_A::PF13
    }
    #[doc = "Checks if the value of the field is `PG13`"]
    #[inline(always)]
    pub fn is_pg13(&self) -> bool {
        *self == EXTI13_A::PG13
    }
}
#[doc = "Field `EXTI13` writer - EXTI 13 configuration bits"]
pub type EXTI13_W<'a, const O: u8> = crate::FieldWriter<'a, EXTICR4_SPEC, 4, O, EXTI13_A>;
impl<'a, const O: u8> EXTI13_W<'a, O> {
    #[doc = "Select PA13 as the source input for the EXTI13 external interrupt"]
    #[inline(always)]
    pub fn pa13(self) -> &'a mut W {
        self.variant(EXTI13_A::PA13)
    }
    #[doc = "Select PB13 as the source input for the EXTI13 external interrupt"]
    #[inline(always)]
    pub fn pb13(self) -> &'a mut W {
        self.variant(EXTI13_A::PB13)
    }
    #[doc = "Select PC13 as the source input for the EXTI13 external interrupt"]
    #[inline(always)]
    pub fn pc13(self) -> &'a mut W {
        self.variant(EXTI13_A::PC13)
    }
    #[doc = "Select PD13 as the source input for the EXTI13 external interrupt"]
    #[inline(always)]
    pub fn pd13(self) -> &'a mut W {
        self.variant(EXTI13_A::PD13)
    }
    #[doc = "Select PE13 as the source input for the EXTI13 external interrupt"]
    #[inline(always)]
    pub fn pe13(self) -> &'a mut W {
        self.variant(EXTI13_A::PE13)
    }
    #[doc = "Select PF13 as the source input for the EXTI13 external interrupt"]
    #[inline(always)]
    pub fn pf13(self) -> &'a mut W {
        self.variant(EXTI13_A::PF13)
    }
    #[doc = "Select PG13 as the source input for the EXTI13 external interrupt"]
    #[inline(always)]
    pub fn pg13(self) -> &'a mut W {
        self.variant(EXTI13_A::PG13)
    }
}
#[doc = "Field `EXTI14` reader - EXTI 14 configuration bits"]
pub type EXTI14_R = crate::FieldReader<EXTI14_A>;
#[doc = "EXTI 14 configuration bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EXTI14_A {
    #[doc = "0: Select PA14 as the source input for the EXTI14 external interrupt"]
    PA14 = 0,
    #[doc = "1: Select PB14 as the source input for the EXTI14 external interrupt"]
    PB14 = 1,
    #[doc = "2: Select PC14 as the source input for the EXTI14 external interrupt"]
    PC14 = 2,
    #[doc = "3: Select PD14 as the source input for the EXTI14 external interrupt"]
    PD14 = 3,
    #[doc = "4: Select PE14 as the source input for the EXTI14 external interrupt"]
    PE14 = 4,
    #[doc = "5: Select PF14 as the source input for the EXTI14 external interrupt"]
    PF14 = 5,
    #[doc = "6: Select PG14 as the source input for the EXTI14 external interrupt"]
    PG14 = 6,
}
impl From<EXTI14_A> for u8 {
    #[inline(always)]
    fn from(variant: EXTI14_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for EXTI14_A {
    type Ux = u8;
}
impl EXTI14_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<EXTI14_A> {
        match self.bits {
            0 => Some(EXTI14_A::PA14),
            1 => Some(EXTI14_A::PB14),
            2 => Some(EXTI14_A::PC14),
            3 => Some(EXTI14_A::PD14),
            4 => Some(EXTI14_A::PE14),
            5 => Some(EXTI14_A::PF14),
            6 => Some(EXTI14_A::PG14),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `PA14`"]
    #[inline(always)]
    pub fn is_pa14(&self) -> bool {
        *self == EXTI14_A::PA14
    }
    #[doc = "Checks if the value of the field is `PB14`"]
    #[inline(always)]
    pub fn is_pb14(&self) -> bool {
        *self == EXTI14_A::PB14
    }
    #[doc = "Checks if the value of the field is `PC14`"]
    #[inline(always)]
    pub fn is_pc14(&self) -> bool {
        *self == EXTI14_A::PC14
    }
    #[doc = "Checks if the value of the field is `PD14`"]
    #[inline(always)]
    pub fn is_pd14(&self) -> bool {
        *self == EXTI14_A::PD14
    }
    #[doc = "Checks if the value of the field is `PE14`"]
    #[inline(always)]
    pub fn is_pe14(&self) -> bool {
        *self == EXTI14_A::PE14
    }
    #[doc = "Checks if the value of the field is `PF14`"]
    #[inline(always)]
    pub fn is_pf14(&self) -> bool {
        *self == EXTI14_A::PF14
    }
    #[doc = "Checks if the value of the field is `PG14`"]
    #[inline(always)]
    pub fn is_pg14(&self) -> bool {
        *self == EXTI14_A::PG14
    }
}
#[doc = "Field `EXTI14` writer - EXTI 14 configuration bits"]
pub type EXTI14_W<'a, const O: u8> = crate::FieldWriter<'a, EXTICR4_SPEC, 4, O, EXTI14_A>;
impl<'a, const O: u8> EXTI14_W<'a, O> {
    #[doc = "Select PA14 as the source input for the EXTI14 external interrupt"]
    #[inline(always)]
    pub fn pa14(self) -> &'a mut W {
        self.variant(EXTI14_A::PA14)
    }
    #[doc = "Select PB14 as the source input for the EXTI14 external interrupt"]
    #[inline(always)]
    pub fn pb14(self) -> &'a mut W {
        self.variant(EXTI14_A::PB14)
    }
    #[doc = "Select PC14 as the source input for the EXTI14 external interrupt"]
    #[inline(always)]
    pub fn pc14(self) -> &'a mut W {
        self.variant(EXTI14_A::PC14)
    }
    #[doc = "Select PD14 as the source input for the EXTI14 external interrupt"]
    #[inline(always)]
    pub fn pd14(self) -> &'a mut W {
        self.variant(EXTI14_A::PD14)
    }
    #[doc = "Select PE14 as the source input for the EXTI14 external interrupt"]
    #[inline(always)]
    pub fn pe14(self) -> &'a mut W {
        self.variant(EXTI14_A::PE14)
    }
    #[doc = "Select PF14 as the source input for the EXTI14 external interrupt"]
    #[inline(always)]
    pub fn pf14(self) -> &'a mut W {
        self.variant(EXTI14_A::PF14)
    }
    #[doc = "Select PG14 as the source input for the EXTI14 external interrupt"]
    #[inline(always)]
    pub fn pg14(self) -> &'a mut W {
        self.variant(EXTI14_A::PG14)
    }
}
#[doc = "Field `EXTI15` reader - EXTI 15 configuration bits"]
pub type EXTI15_R = crate::FieldReader<EXTI15_A>;
#[doc = "EXTI 15 configuration bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EXTI15_A {
    #[doc = "0: Select PA15 as the source input for the EXTI15 external interrupt"]
    PA15 = 0,
    #[doc = "1: Select PB15 as the source input for the EXTI15 external interrupt"]
    PB15 = 1,
    #[doc = "2: Select PC15 as the source input for the EXTI15 external interrupt"]
    PC15 = 2,
    #[doc = "3: Select PD15 as the source input for the EXTI15 external interrupt"]
    PD15 = 3,
    #[doc = "4: Select PE15 as the source input for the EXTI15 external interrupt"]
    PE15 = 4,
    #[doc = "5: Select PF15 as the source input for the EXTI15 external interrupt"]
    PF15 = 5,
    #[doc = "6: Select PG15 as the source input for the EXTI15 external interrupt"]
    PG15 = 6,
}
impl From<EXTI15_A> for u8 {
    #[inline(always)]
    fn from(variant: EXTI15_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for EXTI15_A {
    type Ux = u8;
}
impl EXTI15_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<EXTI15_A> {
        match self.bits {
            0 => Some(EXTI15_A::PA15),
            1 => Some(EXTI15_A::PB15),
            2 => Some(EXTI15_A::PC15),
            3 => Some(EXTI15_A::PD15),
            4 => Some(EXTI15_A::PE15),
            5 => Some(EXTI15_A::PF15),
            6 => Some(EXTI15_A::PG15),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `PA15`"]
    #[inline(always)]
    pub fn is_pa15(&self) -> bool {
        *self == EXTI15_A::PA15
    }
    #[doc = "Checks if the value of the field is `PB15`"]
    #[inline(always)]
    pub fn is_pb15(&self) -> bool {
        *self == EXTI15_A::PB15
    }
    #[doc = "Checks if the value of the field is `PC15`"]
    #[inline(always)]
    pub fn is_pc15(&self) -> bool {
        *self == EXTI15_A::PC15
    }
    #[doc = "Checks if the value of the field is `PD15`"]
    #[inline(always)]
    pub fn is_pd15(&self) -> bool {
        *self == EXTI15_A::PD15
    }
    #[doc = "Checks if the value of the field is `PE15`"]
    #[inline(always)]
    pub fn is_pe15(&self) -> bool {
        *self == EXTI15_A::PE15
    }
    #[doc = "Checks if the value of the field is `PF15`"]
    #[inline(always)]
    pub fn is_pf15(&self) -> bool {
        *self == EXTI15_A::PF15
    }
    #[doc = "Checks if the value of the field is `PG15`"]
    #[inline(always)]
    pub fn is_pg15(&self) -> bool {
        *self == EXTI15_A::PG15
    }
}
#[doc = "Field `EXTI15` writer - EXTI 15 configuration bits"]
pub type EXTI15_W<'a, const O: u8> = crate::FieldWriter<'a, EXTICR4_SPEC, 4, O, EXTI15_A>;
impl<'a, const O: u8> EXTI15_W<'a, O> {
    #[doc = "Select PA15 as the source input for the EXTI15 external interrupt"]
    #[inline(always)]
    pub fn pa15(self) -> &'a mut W {
        self.variant(EXTI15_A::PA15)
    }
    #[doc = "Select PB15 as the source input for the EXTI15 external interrupt"]
    #[inline(always)]
    pub fn pb15(self) -> &'a mut W {
        self.variant(EXTI15_A::PB15)
    }
    #[doc = "Select PC15 as the source input for the EXTI15 external interrupt"]
    #[inline(always)]
    pub fn pc15(self) -> &'a mut W {
        self.variant(EXTI15_A::PC15)
    }
    #[doc = "Select PD15 as the source input for the EXTI15 external interrupt"]
    #[inline(always)]
    pub fn pd15(self) -> &'a mut W {
        self.variant(EXTI15_A::PD15)
    }
    #[doc = "Select PE15 as the source input for the EXTI15 external interrupt"]
    #[inline(always)]
    pub fn pe15(self) -> &'a mut W {
        self.variant(EXTI15_A::PE15)
    }
    #[doc = "Select PF15 as the source input for the EXTI15 external interrupt"]
    #[inline(always)]
    pub fn pf15(self) -> &'a mut W {
        self.variant(EXTI15_A::PF15)
    }
    #[doc = "Select PG15 as the source input for the EXTI15 external interrupt"]
    #[inline(always)]
    pub fn pg15(self) -> &'a mut W {
        self.variant(EXTI15_A::PG15)
    }
}
impl R {
    #[doc = "Bits 0:3 - EXTI 12 configuration bits"]
    #[inline(always)]
    pub fn exti12(&self) -> EXTI12_R {
        EXTI12_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - EXTI 13 configuration bits"]
    #[inline(always)]
    pub fn exti13(&self) -> EXTI13_R {
        EXTI13_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - EXTI 14 configuration bits"]
    #[inline(always)]
    pub fn exti14(&self) -> EXTI14_R {
        EXTI14_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - EXTI 15 configuration bits"]
    #[inline(always)]
    pub fn exti15(&self) -> EXTI15_R {
        EXTI15_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - EXTI 12 configuration bits"]
    #[inline(always)]
    #[must_use]
    pub fn exti12(&mut self) -> EXTI12_W<0> {
        EXTI12_W::new(self)
    }
    #[doc = "Bits 4:7 - EXTI 13 configuration bits"]
    #[inline(always)]
    #[must_use]
    pub fn exti13(&mut self) -> EXTI13_W<4> {
        EXTI13_W::new(self)
    }
    #[doc = "Bits 8:11 - EXTI 14 configuration bits"]
    #[inline(always)]
    #[must_use]
    pub fn exti14(&mut self) -> EXTI14_W<8> {
        EXTI14_W::new(self)
    }
    #[doc = "Bits 12:15 - EXTI 15 configuration bits"]
    #[inline(always)]
    #[must_use]
    pub fn exti15(&mut self) -> EXTI15_W<12> {
        EXTI15_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "external interrupt configuration register 4\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [exticr4](index.html) module"]
pub struct EXTICR4_SPEC;
impl crate::RegisterSpec for EXTICR4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [exticr4::R](R) reader structure"]
impl crate::Readable for EXTICR4_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [exticr4::W](W) writer structure"]
impl crate::Writable for EXTICR4_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EXTICR4 to value 0"]
impl crate::Resettable for EXTICR4_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

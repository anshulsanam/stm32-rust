#[doc = "Register `EXTICR3` reader"]
pub struct R(crate::R<EXTICR3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EXTICR3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EXTICR3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EXTICR3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EXTICR3` writer"]
pub struct W(crate::W<EXTICR3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EXTICR3_SPEC>;
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
impl From<crate::W<EXTICR3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EXTICR3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EXTI8` reader - EXTI 8 configuration bits"]
pub type EXTI8_R = crate::FieldReader<EXTI8_A>;
#[doc = "EXTI 8 configuration bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EXTI8_A {
    #[doc = "0: Select PA8 as the source input for the EXTI8 external interrupt"]
    PA8 = 0,
    #[doc = "1: Select PB8 as the source input for the EXTI8 external interrupt"]
    PB8 = 1,
    #[doc = "2: Select PC8 as the source input for the EXTI8 external interrupt"]
    PC8 = 2,
    #[doc = "3: Select PD8 as the source input for the EXTI8 external interrupt"]
    PD8 = 3,
    #[doc = "4: Select PE8 as the source input for the EXTI8 external interrupt"]
    PE8 = 4,
    #[doc = "5: Select PF8 as the source input for the EXTI8 external interrupt"]
    PF8 = 5,
    #[doc = "6: Select PG8 as the source input for the EXTI8 external interrupt"]
    PG8 = 6,
}
impl From<EXTI8_A> for u8 {
    #[inline(always)]
    fn from(variant: EXTI8_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for EXTI8_A {
    type Ux = u8;
}
impl EXTI8_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<EXTI8_A> {
        match self.bits {
            0 => Some(EXTI8_A::PA8),
            1 => Some(EXTI8_A::PB8),
            2 => Some(EXTI8_A::PC8),
            3 => Some(EXTI8_A::PD8),
            4 => Some(EXTI8_A::PE8),
            5 => Some(EXTI8_A::PF8),
            6 => Some(EXTI8_A::PG8),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `PA8`"]
    #[inline(always)]
    pub fn is_pa8(&self) -> bool {
        *self == EXTI8_A::PA8
    }
    #[doc = "Checks if the value of the field is `PB8`"]
    #[inline(always)]
    pub fn is_pb8(&self) -> bool {
        *self == EXTI8_A::PB8
    }
    #[doc = "Checks if the value of the field is `PC8`"]
    #[inline(always)]
    pub fn is_pc8(&self) -> bool {
        *self == EXTI8_A::PC8
    }
    #[doc = "Checks if the value of the field is `PD8`"]
    #[inline(always)]
    pub fn is_pd8(&self) -> bool {
        *self == EXTI8_A::PD8
    }
    #[doc = "Checks if the value of the field is `PE8`"]
    #[inline(always)]
    pub fn is_pe8(&self) -> bool {
        *self == EXTI8_A::PE8
    }
    #[doc = "Checks if the value of the field is `PF8`"]
    #[inline(always)]
    pub fn is_pf8(&self) -> bool {
        *self == EXTI8_A::PF8
    }
    #[doc = "Checks if the value of the field is `PG8`"]
    #[inline(always)]
    pub fn is_pg8(&self) -> bool {
        *self == EXTI8_A::PG8
    }
}
#[doc = "Field `EXTI8` writer - EXTI 8 configuration bits"]
pub type EXTI8_W<'a, const O: u8> = crate::FieldWriter<'a, EXTICR3_SPEC, 4, O, EXTI8_A>;
impl<'a, const O: u8> EXTI8_W<'a, O> {
    #[doc = "Select PA8 as the source input for the EXTI8 external interrupt"]
    #[inline(always)]
    pub fn pa8(self) -> &'a mut W {
        self.variant(EXTI8_A::PA8)
    }
    #[doc = "Select PB8 as the source input for the EXTI8 external interrupt"]
    #[inline(always)]
    pub fn pb8(self) -> &'a mut W {
        self.variant(EXTI8_A::PB8)
    }
    #[doc = "Select PC8 as the source input for the EXTI8 external interrupt"]
    #[inline(always)]
    pub fn pc8(self) -> &'a mut W {
        self.variant(EXTI8_A::PC8)
    }
    #[doc = "Select PD8 as the source input for the EXTI8 external interrupt"]
    #[inline(always)]
    pub fn pd8(self) -> &'a mut W {
        self.variant(EXTI8_A::PD8)
    }
    #[doc = "Select PE8 as the source input for the EXTI8 external interrupt"]
    #[inline(always)]
    pub fn pe8(self) -> &'a mut W {
        self.variant(EXTI8_A::PE8)
    }
    #[doc = "Select PF8 as the source input for the EXTI8 external interrupt"]
    #[inline(always)]
    pub fn pf8(self) -> &'a mut W {
        self.variant(EXTI8_A::PF8)
    }
    #[doc = "Select PG8 as the source input for the EXTI8 external interrupt"]
    #[inline(always)]
    pub fn pg8(self) -> &'a mut W {
        self.variant(EXTI8_A::PG8)
    }
}
#[doc = "Field `EXTI9` reader - EXTI 9 configuration bits"]
pub type EXTI9_R = crate::FieldReader<EXTI9_A>;
#[doc = "EXTI 9 configuration bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EXTI9_A {
    #[doc = "0: Select PA9 as the source input for the EXTI9 external interrupt"]
    PA9 = 0,
    #[doc = "1: Select PB9 as the source input for the EXTI9 external interrupt"]
    PB9 = 1,
    #[doc = "2: Select PC9 as the source input for the EXTI9 external interrupt"]
    PC9 = 2,
    #[doc = "3: Select PD9 as the source input for the EXTI9 external interrupt"]
    PD9 = 3,
    #[doc = "4: Select PE9 as the source input for the EXTI9 external interrupt"]
    PE9 = 4,
    #[doc = "5: Select PF9 as the source input for the EXTI9 external interrupt"]
    PF9 = 5,
    #[doc = "6: Select PG9 as the source input for the EXTI9 external interrupt"]
    PG9 = 6,
}
impl From<EXTI9_A> for u8 {
    #[inline(always)]
    fn from(variant: EXTI9_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for EXTI9_A {
    type Ux = u8;
}
impl EXTI9_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<EXTI9_A> {
        match self.bits {
            0 => Some(EXTI9_A::PA9),
            1 => Some(EXTI9_A::PB9),
            2 => Some(EXTI9_A::PC9),
            3 => Some(EXTI9_A::PD9),
            4 => Some(EXTI9_A::PE9),
            5 => Some(EXTI9_A::PF9),
            6 => Some(EXTI9_A::PG9),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `PA9`"]
    #[inline(always)]
    pub fn is_pa9(&self) -> bool {
        *self == EXTI9_A::PA9
    }
    #[doc = "Checks if the value of the field is `PB9`"]
    #[inline(always)]
    pub fn is_pb9(&self) -> bool {
        *self == EXTI9_A::PB9
    }
    #[doc = "Checks if the value of the field is `PC9`"]
    #[inline(always)]
    pub fn is_pc9(&self) -> bool {
        *self == EXTI9_A::PC9
    }
    #[doc = "Checks if the value of the field is `PD9`"]
    #[inline(always)]
    pub fn is_pd9(&self) -> bool {
        *self == EXTI9_A::PD9
    }
    #[doc = "Checks if the value of the field is `PE9`"]
    #[inline(always)]
    pub fn is_pe9(&self) -> bool {
        *self == EXTI9_A::PE9
    }
    #[doc = "Checks if the value of the field is `PF9`"]
    #[inline(always)]
    pub fn is_pf9(&self) -> bool {
        *self == EXTI9_A::PF9
    }
    #[doc = "Checks if the value of the field is `PG9`"]
    #[inline(always)]
    pub fn is_pg9(&self) -> bool {
        *self == EXTI9_A::PG9
    }
}
#[doc = "Field `EXTI9` writer - EXTI 9 configuration bits"]
pub type EXTI9_W<'a, const O: u8> = crate::FieldWriter<'a, EXTICR3_SPEC, 4, O, EXTI9_A>;
impl<'a, const O: u8> EXTI9_W<'a, O> {
    #[doc = "Select PA9 as the source input for the EXTI9 external interrupt"]
    #[inline(always)]
    pub fn pa9(self) -> &'a mut W {
        self.variant(EXTI9_A::PA9)
    }
    #[doc = "Select PB9 as the source input for the EXTI9 external interrupt"]
    #[inline(always)]
    pub fn pb9(self) -> &'a mut W {
        self.variant(EXTI9_A::PB9)
    }
    #[doc = "Select PC9 as the source input for the EXTI9 external interrupt"]
    #[inline(always)]
    pub fn pc9(self) -> &'a mut W {
        self.variant(EXTI9_A::PC9)
    }
    #[doc = "Select PD9 as the source input for the EXTI9 external interrupt"]
    #[inline(always)]
    pub fn pd9(self) -> &'a mut W {
        self.variant(EXTI9_A::PD9)
    }
    #[doc = "Select PE9 as the source input for the EXTI9 external interrupt"]
    #[inline(always)]
    pub fn pe9(self) -> &'a mut W {
        self.variant(EXTI9_A::PE9)
    }
    #[doc = "Select PF9 as the source input for the EXTI9 external interrupt"]
    #[inline(always)]
    pub fn pf9(self) -> &'a mut W {
        self.variant(EXTI9_A::PF9)
    }
    #[doc = "Select PG9 as the source input for the EXTI9 external interrupt"]
    #[inline(always)]
    pub fn pg9(self) -> &'a mut W {
        self.variant(EXTI9_A::PG9)
    }
}
#[doc = "Field `EXTI10` reader - EXTI 10 configuration bits"]
pub type EXTI10_R = crate::FieldReader<EXTI10_A>;
#[doc = "EXTI 10 configuration bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EXTI10_A {
    #[doc = "0: Select PA10 as the source input for the EXTI10 external interrupt"]
    PA10 = 0,
    #[doc = "1: Select PB10 as the source input for the EXTI10 external interrupt"]
    PB10 = 1,
    #[doc = "2: Select PC10 as the source input for the EXTI10 external interrupt"]
    PC10 = 2,
    #[doc = "3: Select PD10 as the source input for the EXTI10 external interrupt"]
    PD10 = 3,
    #[doc = "4: Select PE10 as the source input for the EXTI10 external interrupt"]
    PE10 = 4,
    #[doc = "5: Select PF10 as the source input for the EXTI10 external interrupt"]
    PF10 = 5,
    #[doc = "6: Select PG10 as the source input for the EXTI10 external interrupt"]
    PG10 = 6,
}
impl From<EXTI10_A> for u8 {
    #[inline(always)]
    fn from(variant: EXTI10_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for EXTI10_A {
    type Ux = u8;
}
impl EXTI10_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<EXTI10_A> {
        match self.bits {
            0 => Some(EXTI10_A::PA10),
            1 => Some(EXTI10_A::PB10),
            2 => Some(EXTI10_A::PC10),
            3 => Some(EXTI10_A::PD10),
            4 => Some(EXTI10_A::PE10),
            5 => Some(EXTI10_A::PF10),
            6 => Some(EXTI10_A::PG10),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `PA10`"]
    #[inline(always)]
    pub fn is_pa10(&self) -> bool {
        *self == EXTI10_A::PA10
    }
    #[doc = "Checks if the value of the field is `PB10`"]
    #[inline(always)]
    pub fn is_pb10(&self) -> bool {
        *self == EXTI10_A::PB10
    }
    #[doc = "Checks if the value of the field is `PC10`"]
    #[inline(always)]
    pub fn is_pc10(&self) -> bool {
        *self == EXTI10_A::PC10
    }
    #[doc = "Checks if the value of the field is `PD10`"]
    #[inline(always)]
    pub fn is_pd10(&self) -> bool {
        *self == EXTI10_A::PD10
    }
    #[doc = "Checks if the value of the field is `PE10`"]
    #[inline(always)]
    pub fn is_pe10(&self) -> bool {
        *self == EXTI10_A::PE10
    }
    #[doc = "Checks if the value of the field is `PF10`"]
    #[inline(always)]
    pub fn is_pf10(&self) -> bool {
        *self == EXTI10_A::PF10
    }
    #[doc = "Checks if the value of the field is `PG10`"]
    #[inline(always)]
    pub fn is_pg10(&self) -> bool {
        *self == EXTI10_A::PG10
    }
}
#[doc = "Field `EXTI10` writer - EXTI 10 configuration bits"]
pub type EXTI10_W<'a, const O: u8> = crate::FieldWriter<'a, EXTICR3_SPEC, 4, O, EXTI10_A>;
impl<'a, const O: u8> EXTI10_W<'a, O> {
    #[doc = "Select PA10 as the source input for the EXTI10 external interrupt"]
    #[inline(always)]
    pub fn pa10(self) -> &'a mut W {
        self.variant(EXTI10_A::PA10)
    }
    #[doc = "Select PB10 as the source input for the EXTI10 external interrupt"]
    #[inline(always)]
    pub fn pb10(self) -> &'a mut W {
        self.variant(EXTI10_A::PB10)
    }
    #[doc = "Select PC10 as the source input for the EXTI10 external interrupt"]
    #[inline(always)]
    pub fn pc10(self) -> &'a mut W {
        self.variant(EXTI10_A::PC10)
    }
    #[doc = "Select PD10 as the source input for the EXTI10 external interrupt"]
    #[inline(always)]
    pub fn pd10(self) -> &'a mut W {
        self.variant(EXTI10_A::PD10)
    }
    #[doc = "Select PE10 as the source input for the EXTI10 external interrupt"]
    #[inline(always)]
    pub fn pe10(self) -> &'a mut W {
        self.variant(EXTI10_A::PE10)
    }
    #[doc = "Select PF10 as the source input for the EXTI10 external interrupt"]
    #[inline(always)]
    pub fn pf10(self) -> &'a mut W {
        self.variant(EXTI10_A::PF10)
    }
    #[doc = "Select PG10 as the source input for the EXTI10 external interrupt"]
    #[inline(always)]
    pub fn pg10(self) -> &'a mut W {
        self.variant(EXTI10_A::PG10)
    }
}
#[doc = "Field `EXTI11` reader - EXTI 11 configuration bits"]
pub type EXTI11_R = crate::FieldReader<EXTI11_A>;
#[doc = "EXTI 11 configuration bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EXTI11_A {
    #[doc = "0: Select PA11 as the source input for the EXTI11 external interrupt"]
    PA11 = 0,
    #[doc = "1: Select PB11 as the source input for the EXTI11 external interrupt"]
    PB11 = 1,
    #[doc = "2: Select PC11 as the source input for the EXTI11 external interrupt"]
    PC11 = 2,
    #[doc = "3: Select PD11 as the source input for the EXTI11 external interrupt"]
    PD11 = 3,
    #[doc = "4: Select PE11 as the source input for the EXTI11 external interrupt"]
    PE11 = 4,
    #[doc = "5: Select PF11 as the source input for the EXTI11 external interrupt"]
    PF11 = 5,
    #[doc = "6: Select PG11 as the source input for the EXTI11 external interrupt"]
    PG11 = 6,
}
impl From<EXTI11_A> for u8 {
    #[inline(always)]
    fn from(variant: EXTI11_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for EXTI11_A {
    type Ux = u8;
}
impl EXTI11_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<EXTI11_A> {
        match self.bits {
            0 => Some(EXTI11_A::PA11),
            1 => Some(EXTI11_A::PB11),
            2 => Some(EXTI11_A::PC11),
            3 => Some(EXTI11_A::PD11),
            4 => Some(EXTI11_A::PE11),
            5 => Some(EXTI11_A::PF11),
            6 => Some(EXTI11_A::PG11),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `PA11`"]
    #[inline(always)]
    pub fn is_pa11(&self) -> bool {
        *self == EXTI11_A::PA11
    }
    #[doc = "Checks if the value of the field is `PB11`"]
    #[inline(always)]
    pub fn is_pb11(&self) -> bool {
        *self == EXTI11_A::PB11
    }
    #[doc = "Checks if the value of the field is `PC11`"]
    #[inline(always)]
    pub fn is_pc11(&self) -> bool {
        *self == EXTI11_A::PC11
    }
    #[doc = "Checks if the value of the field is `PD11`"]
    #[inline(always)]
    pub fn is_pd11(&self) -> bool {
        *self == EXTI11_A::PD11
    }
    #[doc = "Checks if the value of the field is `PE11`"]
    #[inline(always)]
    pub fn is_pe11(&self) -> bool {
        *self == EXTI11_A::PE11
    }
    #[doc = "Checks if the value of the field is `PF11`"]
    #[inline(always)]
    pub fn is_pf11(&self) -> bool {
        *self == EXTI11_A::PF11
    }
    #[doc = "Checks if the value of the field is `PG11`"]
    #[inline(always)]
    pub fn is_pg11(&self) -> bool {
        *self == EXTI11_A::PG11
    }
}
#[doc = "Field `EXTI11` writer - EXTI 11 configuration bits"]
pub type EXTI11_W<'a, const O: u8> = crate::FieldWriter<'a, EXTICR3_SPEC, 4, O, EXTI11_A>;
impl<'a, const O: u8> EXTI11_W<'a, O> {
    #[doc = "Select PA11 as the source input for the EXTI11 external interrupt"]
    #[inline(always)]
    pub fn pa11(self) -> &'a mut W {
        self.variant(EXTI11_A::PA11)
    }
    #[doc = "Select PB11 as the source input for the EXTI11 external interrupt"]
    #[inline(always)]
    pub fn pb11(self) -> &'a mut W {
        self.variant(EXTI11_A::PB11)
    }
    #[doc = "Select PC11 as the source input for the EXTI11 external interrupt"]
    #[inline(always)]
    pub fn pc11(self) -> &'a mut W {
        self.variant(EXTI11_A::PC11)
    }
    #[doc = "Select PD11 as the source input for the EXTI11 external interrupt"]
    #[inline(always)]
    pub fn pd11(self) -> &'a mut W {
        self.variant(EXTI11_A::PD11)
    }
    #[doc = "Select PE11 as the source input for the EXTI11 external interrupt"]
    #[inline(always)]
    pub fn pe11(self) -> &'a mut W {
        self.variant(EXTI11_A::PE11)
    }
    #[doc = "Select PF11 as the source input for the EXTI11 external interrupt"]
    #[inline(always)]
    pub fn pf11(self) -> &'a mut W {
        self.variant(EXTI11_A::PF11)
    }
    #[doc = "Select PG11 as the source input for the EXTI11 external interrupt"]
    #[inline(always)]
    pub fn pg11(self) -> &'a mut W {
        self.variant(EXTI11_A::PG11)
    }
}
impl R {
    #[doc = "Bits 0:3 - EXTI 8 configuration bits"]
    #[inline(always)]
    pub fn exti8(&self) -> EXTI8_R {
        EXTI8_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - EXTI 9 configuration bits"]
    #[inline(always)]
    pub fn exti9(&self) -> EXTI9_R {
        EXTI9_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - EXTI 10 configuration bits"]
    #[inline(always)]
    pub fn exti10(&self) -> EXTI10_R {
        EXTI10_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - EXTI 11 configuration bits"]
    #[inline(always)]
    pub fn exti11(&self) -> EXTI11_R {
        EXTI11_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - EXTI 8 configuration bits"]
    #[inline(always)]
    #[must_use]
    pub fn exti8(&mut self) -> EXTI8_W<0> {
        EXTI8_W::new(self)
    }
    #[doc = "Bits 4:7 - EXTI 9 configuration bits"]
    #[inline(always)]
    #[must_use]
    pub fn exti9(&mut self) -> EXTI9_W<4> {
        EXTI9_W::new(self)
    }
    #[doc = "Bits 8:11 - EXTI 10 configuration bits"]
    #[inline(always)]
    #[must_use]
    pub fn exti10(&mut self) -> EXTI10_W<8> {
        EXTI10_W::new(self)
    }
    #[doc = "Bits 12:15 - EXTI 11 configuration bits"]
    #[inline(always)]
    #[must_use]
    pub fn exti11(&mut self) -> EXTI11_W<12> {
        EXTI11_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "external interrupt configuration register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [exticr3](index.html) module"]
pub struct EXTICR3_SPEC;
impl crate::RegisterSpec for EXTICR3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [exticr3::R](R) reader structure"]
impl crate::Readable for EXTICR3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [exticr3::W](W) writer structure"]
impl crate::Writable for EXTICR3_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EXTICR3 to value 0"]
impl crate::Resettable for EXTICR3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

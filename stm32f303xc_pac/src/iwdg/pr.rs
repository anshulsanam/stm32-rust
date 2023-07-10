#[doc = "Register `PR` reader"]
pub struct R(crate::R<PR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PR` writer"]
pub struct W(crate::W<PR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PR_SPEC>;
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
impl From<crate::W<PR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PR` reader - Prescaler divider"]
pub type PR_R = crate::FieldReader<PR_A>;
#[doc = "Prescaler divider\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PR_A {
    #[doc = "0: Divider /4"]
    DIVIDE_BY4 = 0,
    #[doc = "1: Divider /8"]
    DIVIDE_BY8 = 1,
    #[doc = "2: Divider /16"]
    DIVIDE_BY16 = 2,
    #[doc = "3: Divider /32"]
    DIVIDE_BY32 = 3,
    #[doc = "4: Divider /64"]
    DIVIDE_BY64 = 4,
    #[doc = "5: Divider /128"]
    DIVIDE_BY128 = 5,
    #[doc = "6: Divider /256"]
    DIVIDE_BY256 = 6,
    #[doc = "7: Divider /256"]
    DIVIDE_BY256BIS = 7,
}
impl From<PR_A> for u8 {
    #[inline(always)]
    fn from(variant: PR_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PR_A {
    type Ux = u8;
}
impl PR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PR_A {
        match self.bits {
            0 => PR_A::DIVIDE_BY4,
            1 => PR_A::DIVIDE_BY8,
            2 => PR_A::DIVIDE_BY16,
            3 => PR_A::DIVIDE_BY32,
            4 => PR_A::DIVIDE_BY64,
            5 => PR_A::DIVIDE_BY128,
            6 => PR_A::DIVIDE_BY256,
            7 => PR_A::DIVIDE_BY256BIS,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIVIDE_BY4`"]
    #[inline(always)]
    pub fn is_divide_by4(&self) -> bool {
        *self == PR_A::DIVIDE_BY4
    }
    #[doc = "Checks if the value of the field is `DIVIDE_BY8`"]
    #[inline(always)]
    pub fn is_divide_by8(&self) -> bool {
        *self == PR_A::DIVIDE_BY8
    }
    #[doc = "Checks if the value of the field is `DIVIDE_BY16`"]
    #[inline(always)]
    pub fn is_divide_by16(&self) -> bool {
        *self == PR_A::DIVIDE_BY16
    }
    #[doc = "Checks if the value of the field is `DIVIDE_BY32`"]
    #[inline(always)]
    pub fn is_divide_by32(&self) -> bool {
        *self == PR_A::DIVIDE_BY32
    }
    #[doc = "Checks if the value of the field is `DIVIDE_BY64`"]
    #[inline(always)]
    pub fn is_divide_by64(&self) -> bool {
        *self == PR_A::DIVIDE_BY64
    }
    #[doc = "Checks if the value of the field is `DIVIDE_BY128`"]
    #[inline(always)]
    pub fn is_divide_by128(&self) -> bool {
        *self == PR_A::DIVIDE_BY128
    }
    #[doc = "Checks if the value of the field is `DIVIDE_BY256`"]
    #[inline(always)]
    pub fn is_divide_by256(&self) -> bool {
        *self == PR_A::DIVIDE_BY256
    }
    #[doc = "Checks if the value of the field is `DIVIDE_BY256BIS`"]
    #[inline(always)]
    pub fn is_divide_by256bis(&self) -> bool {
        *self == PR_A::DIVIDE_BY256BIS
    }
}
#[doc = "Field `PR` writer - Prescaler divider"]
pub type PR_W<'a, const O: u8> = crate::FieldWriterSafe<'a, PR_SPEC, 3, O, PR_A>;
impl<'a, const O: u8> PR_W<'a, O> {
    #[doc = "Divider /4"]
    #[inline(always)]
    pub fn divide_by4(self) -> &'a mut W {
        self.variant(PR_A::DIVIDE_BY4)
    }
    #[doc = "Divider /8"]
    #[inline(always)]
    pub fn divide_by8(self) -> &'a mut W {
        self.variant(PR_A::DIVIDE_BY8)
    }
    #[doc = "Divider /16"]
    #[inline(always)]
    pub fn divide_by16(self) -> &'a mut W {
        self.variant(PR_A::DIVIDE_BY16)
    }
    #[doc = "Divider /32"]
    #[inline(always)]
    pub fn divide_by32(self) -> &'a mut W {
        self.variant(PR_A::DIVIDE_BY32)
    }
    #[doc = "Divider /64"]
    #[inline(always)]
    pub fn divide_by64(self) -> &'a mut W {
        self.variant(PR_A::DIVIDE_BY64)
    }
    #[doc = "Divider /128"]
    #[inline(always)]
    pub fn divide_by128(self) -> &'a mut W {
        self.variant(PR_A::DIVIDE_BY128)
    }
    #[doc = "Divider /256"]
    #[inline(always)]
    pub fn divide_by256(self) -> &'a mut W {
        self.variant(PR_A::DIVIDE_BY256)
    }
    #[doc = "Divider /256"]
    #[inline(always)]
    pub fn divide_by256bis(self) -> &'a mut W {
        self.variant(PR_A::DIVIDE_BY256BIS)
    }
}
impl R {
    #[doc = "Bits 0:2 - Prescaler divider"]
    #[inline(always)]
    pub fn pr(&self) -> PR_R {
        PR_R::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Prescaler divider"]
    #[inline(always)]
    #[must_use]
    pub fn pr(&mut self) -> PR_W<0> {
        PR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Prescaler register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pr](index.html) module"]
pub struct PR_SPEC;
impl crate::RegisterSpec for PR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pr::R](R) reader structure"]
impl crate::Readable for PR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pr::W](W) writer structure"]
impl crate::Writable for PR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PR to value 0"]
impl crate::Resettable for PR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

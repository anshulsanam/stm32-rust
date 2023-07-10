#[doc = "Register `SYSCFG_EXTICR2` reader"]
pub struct R(crate::R<SYSCFG_EXTICR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYSCFG_EXTICR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SYSCFG_EXTICR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SYSCFG_EXTICR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SYSCFG_EXTICR2` writer"]
pub struct W(crate::W<SYSCFG_EXTICR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SYSCFG_EXTICR2_SPEC>;
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
impl From<crate::W<SYSCFG_EXTICR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SYSCFG_EXTICR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EXTI4` reader - EXTI 4 configuration bits"]
pub type EXTI4_R = crate::FieldReader;
#[doc = "Field `EXTI4` writer - EXTI 4 configuration bits"]
pub type EXTI4_W<'a, const O: u8> = crate::FieldWriter<'a, SYSCFG_EXTICR2_SPEC, 4, O>;
#[doc = "Field `EXTI5` reader - EXTI 5 configuration bits"]
pub type EXTI5_R = crate::FieldReader;
#[doc = "Field `EXTI5` writer - EXTI 5 configuration bits"]
pub type EXTI5_W<'a, const O: u8> = crate::FieldWriter<'a, SYSCFG_EXTICR2_SPEC, 4, O>;
#[doc = "Field `EXTI6` reader - EXTI 6 configuration bits"]
pub type EXTI6_R = crate::FieldReader;
#[doc = "Field `EXTI6` writer - EXTI 6 configuration bits"]
pub type EXTI6_W<'a, const O: u8> = crate::FieldWriter<'a, SYSCFG_EXTICR2_SPEC, 4, O>;
#[doc = "Field `EXTI7` reader - EXTI 7 configuration bits"]
pub type EXTI7_R = crate::FieldReader;
#[doc = "Field `EXTI7` writer - EXTI 7 configuration bits"]
pub type EXTI7_W<'a, const O: u8> = crate::FieldWriter<'a, SYSCFG_EXTICR2_SPEC, 4, O>;
impl R {
    #[doc = "Bits 0:3 - EXTI 4 configuration bits"]
    #[inline(always)]
    pub fn exti4(&self) -> EXTI4_R {
        EXTI4_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - EXTI 5 configuration bits"]
    #[inline(always)]
    pub fn exti5(&self) -> EXTI5_R {
        EXTI5_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - EXTI 6 configuration bits"]
    #[inline(always)]
    pub fn exti6(&self) -> EXTI6_R {
        EXTI6_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - EXTI 7 configuration bits"]
    #[inline(always)]
    pub fn exti7(&self) -> EXTI7_R {
        EXTI7_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - EXTI 4 configuration bits"]
    #[inline(always)]
    #[must_use]
    pub fn exti4(&mut self) -> EXTI4_W<0> {
        EXTI4_W::new(self)
    }
    #[doc = "Bits 4:7 - EXTI 5 configuration bits"]
    #[inline(always)]
    #[must_use]
    pub fn exti5(&mut self) -> EXTI5_W<4> {
        EXTI5_W::new(self)
    }
    #[doc = "Bits 8:11 - EXTI 6 configuration bits"]
    #[inline(always)]
    #[must_use]
    pub fn exti6(&mut self) -> EXTI6_W<8> {
        EXTI6_W::new(self)
    }
    #[doc = "Bits 12:15 - EXTI 7 configuration bits"]
    #[inline(always)]
    #[must_use]
    pub fn exti7(&mut self) -> EXTI7_W<12> {
        EXTI7_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "external interrupt configuration register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [syscfg_exticr2](index.html) module"]
pub struct SYSCFG_EXTICR2_SPEC;
impl crate::RegisterSpec for SYSCFG_EXTICR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [syscfg_exticr2::R](R) reader structure"]
impl crate::Readable for SYSCFG_EXTICR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [syscfg_exticr2::W](W) writer structure"]
impl crate::Writable for SYSCFG_EXTICR2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SYSCFG_EXTICR2 to value 0"]
impl crate::Resettable for SYSCFG_EXTICR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

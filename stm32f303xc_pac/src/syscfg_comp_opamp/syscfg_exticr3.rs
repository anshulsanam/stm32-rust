#[doc = "Register `SYSCFG_EXTICR3` reader"]
pub struct R(crate::R<SYSCFG_EXTICR3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYSCFG_EXTICR3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SYSCFG_EXTICR3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SYSCFG_EXTICR3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SYSCFG_EXTICR3` writer"]
pub struct W(crate::W<SYSCFG_EXTICR3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SYSCFG_EXTICR3_SPEC>;
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
impl From<crate::W<SYSCFG_EXTICR3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SYSCFG_EXTICR3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EXTI8` reader - EXTI 8 configuration bits"]
pub type EXTI8_R = crate::FieldReader;
#[doc = "Field `EXTI8` writer - EXTI 8 configuration bits"]
pub type EXTI8_W<'a, const O: u8> = crate::FieldWriter<'a, SYSCFG_EXTICR3_SPEC, 4, O>;
#[doc = "Field `EXTI9` reader - EXTI 9 configuration bits"]
pub type EXTI9_R = crate::FieldReader;
#[doc = "Field `EXTI9` writer - EXTI 9 configuration bits"]
pub type EXTI9_W<'a, const O: u8> = crate::FieldWriter<'a, SYSCFG_EXTICR3_SPEC, 4, O>;
#[doc = "Field `EXTI10` reader - EXTI 10 configuration bits"]
pub type EXTI10_R = crate::FieldReader;
#[doc = "Field `EXTI10` writer - EXTI 10 configuration bits"]
pub type EXTI10_W<'a, const O: u8> = crate::FieldWriter<'a, SYSCFG_EXTICR3_SPEC, 4, O>;
#[doc = "Field `EXTI11` reader - EXTI 11 configuration bits"]
pub type EXTI11_R = crate::FieldReader;
#[doc = "Field `EXTI11` writer - EXTI 11 configuration bits"]
pub type EXTI11_W<'a, const O: u8> = crate::FieldWriter<'a, SYSCFG_EXTICR3_SPEC, 4, O>;
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
#[doc = "external interrupt configuration register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [syscfg_exticr3](index.html) module"]
pub struct SYSCFG_EXTICR3_SPEC;
impl crate::RegisterSpec for SYSCFG_EXTICR3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [syscfg_exticr3::R](R) reader structure"]
impl crate::Readable for SYSCFG_EXTICR3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [syscfg_exticr3::W](W) writer structure"]
impl crate::Writable for SYSCFG_EXTICR3_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SYSCFG_EXTICR3 to value 0"]
impl crate::Resettable for SYSCFG_EXTICR3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

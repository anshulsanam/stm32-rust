#[doc = "Register `CCR4` reader"]
pub struct R(crate::R<CCR4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CCR4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CCR4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CCR4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CCR4` writer"]
pub struct W(crate::W<CCR4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CCR4_SPEC>;
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
impl From<crate::W<CCR4_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CCR4_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CCR4L` reader - Low Capture/Compare value"]
pub type CCR4L_R = crate::FieldReader<u16>;
#[doc = "Field `CCR4L` writer - Low Capture/Compare value"]
pub type CCR4L_W<'a, const O: u8> = crate::FieldWriter<'a, CCR4_SPEC, 16, O, u16>;
#[doc = "Field `CCR4H` reader - High Capture/Compare value (on TIM2)"]
pub type CCR4H_R = crate::FieldReader<u16>;
#[doc = "Field `CCR4H` writer - High Capture/Compare value (on TIM2)"]
pub type CCR4H_W<'a, const O: u8> = crate::FieldWriter<'a, CCR4_SPEC, 16, O, u16>;
impl R {
    #[doc = "Bits 0:15 - Low Capture/Compare value"]
    #[inline(always)]
    pub fn ccr4l(&self) -> CCR4L_R {
        CCR4L_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - High Capture/Compare value (on TIM2)"]
    #[inline(always)]
    pub fn ccr4h(&self) -> CCR4H_R {
        CCR4H_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Low Capture/Compare value"]
    #[inline(always)]
    #[must_use]
    pub fn ccr4l(&mut self) -> CCR4L_W<0> {
        CCR4L_W::new(self)
    }
    #[doc = "Bits 16:31 - High Capture/Compare value (on TIM2)"]
    #[inline(always)]
    #[must_use]
    pub fn ccr4h(&mut self) -> CCR4H_W<16> {
        CCR4H_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "capture/compare register 4\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ccr4](index.html) module"]
pub struct CCR4_SPEC;
impl crate::RegisterSpec for CCR4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ccr4::R](R) reader structure"]
impl crate::Readable for CCR4_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ccr4::W](W) writer structure"]
impl crate::Writable for CCR4_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CCR4 to value 0"]
impl crate::Resettable for CCR4_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

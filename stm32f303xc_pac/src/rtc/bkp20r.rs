#[doc = "Register `BKP20R` reader"]
pub struct R(crate::R<BKP20R_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BKP20R_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BKP20R_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BKP20R_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BKP20R` writer"]
pub struct W(crate::W<BKP20R_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BKP20R_SPEC>;
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
impl From<crate::W<BKP20R_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BKP20R_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BKP` reader - BKP"]
pub type BKP_R = crate::FieldReader<u32>;
#[doc = "Field `BKP` writer - BKP"]
pub type BKP_W<'a, const O: u8> = crate::FieldWriter<'a, BKP20R_SPEC, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - BKP"]
    #[inline(always)]
    pub fn bkp(&self) -> BKP_R {
        BKP_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - BKP"]
    #[inline(always)]
    #[must_use]
    pub fn bkp(&mut self) -> BKP_W<0> {
        BKP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "backup register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bkp20r](index.html) module"]
pub struct BKP20R_SPEC;
impl crate::RegisterSpec for BKP20R_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bkp20r::R](R) reader structure"]
impl crate::Readable for BKP20R_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bkp20r::W](W) writer structure"]
impl crate::Writable for BKP20R_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BKP20R to value 0"]
impl crate::Resettable for BKP20R_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

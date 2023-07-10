#[doc = "Register `IOG%sCR` reader"]
pub struct R(crate::R<IOGCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IOGCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IOGCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IOGCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CNT` reader - Counter value"]
pub type CNT_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:13 - Counter value"]
    #[inline(always)]
    pub fn cnt(&self) -> CNT_R {
        CNT_R::new((self.bits & 0x3fff) as u16)
    }
}
#[doc = "I/O group x counter register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iogcr](index.html) module"]
pub struct IOGCR_SPEC;
impl crate::RegisterSpec for IOGCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [iogcr::R](R) reader structure"]
impl crate::Readable for IOGCR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets IOG%sCR to value 0"]
impl crate::Resettable for IOGCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

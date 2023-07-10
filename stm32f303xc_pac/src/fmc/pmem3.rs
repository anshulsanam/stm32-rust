#[doc = "Register `PMEM3` reader"]
pub struct R(crate::R<PMEM3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PMEM3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PMEM3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PMEM3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PMEM3` writer"]
pub struct W(crate::W<PMEM3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PMEM3_SPEC>;
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
impl From<crate::W<PMEM3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PMEM3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MEMSETx` reader - MEMSETx"]
pub type MEMSETX_R = crate::FieldReader;
#[doc = "Field `MEMSETx` writer - MEMSETx"]
pub type MEMSETX_W<'a, const O: u8> = crate::FieldWriter<'a, PMEM3_SPEC, 8, O>;
#[doc = "Field `MEMWAITx` reader - MEMWAITx"]
pub type MEMWAITX_R = crate::FieldReader;
#[doc = "Field `MEMWAITx` writer - MEMWAITx"]
pub type MEMWAITX_W<'a, const O: u8> = crate::FieldWriter<'a, PMEM3_SPEC, 8, O>;
#[doc = "Field `MEMHOLDx` reader - MEMHOLDx"]
pub type MEMHOLDX_R = crate::FieldReader;
#[doc = "Field `MEMHOLDx` writer - MEMHOLDx"]
pub type MEMHOLDX_W<'a, const O: u8> = crate::FieldWriter<'a, PMEM3_SPEC, 8, O>;
#[doc = "Field `MEMHIZx` reader - MEMHIZx"]
pub type MEMHIZX_R = crate::FieldReader;
#[doc = "Field `MEMHIZx` writer - MEMHIZx"]
pub type MEMHIZX_W<'a, const O: u8> = crate::FieldWriter<'a, PMEM3_SPEC, 8, O>;
impl R {
    #[doc = "Bits 0:7 - MEMSETx"]
    #[inline(always)]
    pub fn memsetx(&self) -> MEMSETX_R {
        MEMSETX_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - MEMWAITx"]
    #[inline(always)]
    pub fn memwaitx(&self) -> MEMWAITX_R {
        MEMWAITX_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - MEMHOLDx"]
    #[inline(always)]
    pub fn memholdx(&self) -> MEMHOLDX_R {
        MEMHOLDX_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - MEMHIZx"]
    #[inline(always)]
    pub fn memhizx(&self) -> MEMHIZX_R {
        MEMHIZX_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - MEMSETx"]
    #[inline(always)]
    #[must_use]
    pub fn memsetx(&mut self) -> MEMSETX_W<0> {
        MEMSETX_W::new(self)
    }
    #[doc = "Bits 8:15 - MEMWAITx"]
    #[inline(always)]
    #[must_use]
    pub fn memwaitx(&mut self) -> MEMWAITX_W<8> {
        MEMWAITX_W::new(self)
    }
    #[doc = "Bits 16:23 - MEMHOLDx"]
    #[inline(always)]
    #[must_use]
    pub fn memholdx(&mut self) -> MEMHOLDX_W<16> {
        MEMHOLDX_W::new(self)
    }
    #[doc = "Bits 24:31 - MEMHIZx"]
    #[inline(always)]
    #[must_use]
    pub fn memhizx(&mut self) -> MEMHIZX_W<24> {
        MEMHIZX_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Common memory space timing register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pmem3](index.html) module"]
pub struct PMEM3_SPEC;
impl crate::RegisterSpec for PMEM3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pmem3::R](R) reader structure"]
impl crate::Readable for PMEM3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pmem3::W](W) writer structure"]
impl crate::Writable for PMEM3_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PMEM3 to value 0xfcfc_fcfc"]
impl crate::Resettable for PMEM3_SPEC {
    const RESET_VALUE: Self::Ux = 0xfcfc_fcfc;
}

#[doc = "Register `SHIFTR` writer"]
pub struct W(crate::W<SHIFTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SHIFTR_SPEC>;
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
impl From<crate::W<SHIFTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SHIFTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SUBFS` writer - Subtract a fraction of a second"]
pub type SUBFS_W<'a, const O: u8> = crate::FieldWriterSafe<'a, SHIFTR_SPEC, 15, O, u16>;
#[doc = "Add one second\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADD1SW_AW {
    #[doc = "1: Add one second to the clock/calendar"]
    ADD1 = 1,
}
impl From<ADD1SW_AW> for bool {
    #[inline(always)]
    fn from(variant: ADD1SW_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADD1S` writer - Add one second"]
pub type ADD1S_W<'a, const O: u8> = crate::BitWriter<'a, SHIFTR_SPEC, O, ADD1SW_AW>;
impl<'a, const O: u8> ADD1S_W<'a, O> {
    #[doc = "Add one second to the clock/calendar"]
    #[inline(always)]
    pub fn add1(self) -> &'a mut W {
        self.variant(ADD1SW_AW::ADD1)
    }
}
impl W {
    #[doc = "Bits 0:14 - Subtract a fraction of a second"]
    #[inline(always)]
    #[must_use]
    pub fn subfs(&mut self) -> SUBFS_W<0> {
        SUBFS_W::new(self)
    }
    #[doc = "Bit 31 - Add one second"]
    #[inline(always)]
    #[must_use]
    pub fn add1s(&mut self) -> ADD1S_W<31> {
        ADD1S_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "shift control register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [shiftr](index.html) module"]
pub struct SHIFTR_SPEC;
impl crate::RegisterSpec for SHIFTR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [shiftr::W](W) writer structure"]
impl crate::Writable for SHIFTR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SHIFTR to value 0"]
impl crate::Resettable for SHIFTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

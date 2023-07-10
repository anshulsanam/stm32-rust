#[doc = "Register `SYSCFG_RCR` reader"]
pub struct R(crate::R<SYSCFG_RCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYSCFG_RCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SYSCFG_RCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SYSCFG_RCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SYSCFG_RCR` writer"]
pub struct W(crate::W<SYSCFG_RCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SYSCFG_RCR_SPEC>;
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
impl From<crate::W<SYSCFG_RCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SYSCFG_RCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PAGE0_WP` reader - CCM SRAM page write protection bit"]
pub type PAGE0_WP_R = crate::BitReader;
#[doc = "Field `PAGE0_WP` writer - CCM SRAM page write protection bit"]
pub type PAGE0_WP_W<'a, const O: u8> = crate::BitWriter<'a, SYSCFG_RCR_SPEC, O>;
#[doc = "Field `PAGE1_WP` reader - CCM SRAM page write protection bit"]
pub type PAGE1_WP_R = crate::BitReader;
#[doc = "Field `PAGE1_WP` writer - CCM SRAM page write protection bit"]
pub type PAGE1_WP_W<'a, const O: u8> = crate::BitWriter<'a, SYSCFG_RCR_SPEC, O>;
#[doc = "Field `PAGE2_WP` reader - CCM SRAM page write protection bit"]
pub type PAGE2_WP_R = crate::BitReader;
#[doc = "Field `PAGE2_WP` writer - CCM SRAM page write protection bit"]
pub type PAGE2_WP_W<'a, const O: u8> = crate::BitWriter<'a, SYSCFG_RCR_SPEC, O>;
#[doc = "Field `PAGE3_WP` reader - CCM SRAM page write protection bit"]
pub type PAGE3_WP_R = crate::BitReader;
#[doc = "Field `PAGE3_WP` writer - CCM SRAM page write protection bit"]
pub type PAGE3_WP_W<'a, const O: u8> = crate::BitWriter<'a, SYSCFG_RCR_SPEC, O>;
#[doc = "Field `PAGE4_WP` reader - CCM SRAM page write protection bit"]
pub type PAGE4_WP_R = crate::BitReader;
#[doc = "Field `PAGE4_WP` writer - CCM SRAM page write protection bit"]
pub type PAGE4_WP_W<'a, const O: u8> = crate::BitWriter<'a, SYSCFG_RCR_SPEC, O>;
#[doc = "Field `PAGE5_WP` reader - CCM SRAM page write protection bit"]
pub type PAGE5_WP_R = crate::BitReader;
#[doc = "Field `PAGE5_WP` writer - CCM SRAM page write protection bit"]
pub type PAGE5_WP_W<'a, const O: u8> = crate::BitWriter<'a, SYSCFG_RCR_SPEC, O>;
#[doc = "Field `PAGE6_WP` reader - CCM SRAM page write protection bit"]
pub type PAGE6_WP_R = crate::BitReader;
#[doc = "Field `PAGE6_WP` writer - CCM SRAM page write protection bit"]
pub type PAGE6_WP_W<'a, const O: u8> = crate::BitWriter<'a, SYSCFG_RCR_SPEC, O>;
#[doc = "Field `PAGE7_WP` reader - CCM SRAM page write protection bit"]
pub type PAGE7_WP_R = crate::BitReader;
#[doc = "Field `PAGE7_WP` writer - CCM SRAM page write protection bit"]
pub type PAGE7_WP_W<'a, const O: u8> = crate::BitWriter<'a, SYSCFG_RCR_SPEC, O>;
impl R {
    #[doc = "Bit 0 - CCM SRAM page write protection bit"]
    #[inline(always)]
    pub fn page0_wp(&self) -> PAGE0_WP_R {
        PAGE0_WP_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CCM SRAM page write protection bit"]
    #[inline(always)]
    pub fn page1_wp(&self) -> PAGE1_WP_R {
        PAGE1_WP_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - CCM SRAM page write protection bit"]
    #[inline(always)]
    pub fn page2_wp(&self) -> PAGE2_WP_R {
        PAGE2_WP_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - CCM SRAM page write protection bit"]
    #[inline(always)]
    pub fn page3_wp(&self) -> PAGE3_WP_R {
        PAGE3_WP_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - CCM SRAM page write protection bit"]
    #[inline(always)]
    pub fn page4_wp(&self) -> PAGE4_WP_R {
        PAGE4_WP_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - CCM SRAM page write protection bit"]
    #[inline(always)]
    pub fn page5_wp(&self) -> PAGE5_WP_R {
        PAGE5_WP_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - CCM SRAM page write protection bit"]
    #[inline(always)]
    pub fn page6_wp(&self) -> PAGE6_WP_R {
        PAGE6_WP_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - CCM SRAM page write protection bit"]
    #[inline(always)]
    pub fn page7_wp(&self) -> PAGE7_WP_R {
        PAGE7_WP_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CCM SRAM page write protection bit"]
    #[inline(always)]
    #[must_use]
    pub fn page0_wp(&mut self) -> PAGE0_WP_W<0> {
        PAGE0_WP_W::new(self)
    }
    #[doc = "Bit 1 - CCM SRAM page write protection bit"]
    #[inline(always)]
    #[must_use]
    pub fn page1_wp(&mut self) -> PAGE1_WP_W<1> {
        PAGE1_WP_W::new(self)
    }
    #[doc = "Bit 2 - CCM SRAM page write protection bit"]
    #[inline(always)]
    #[must_use]
    pub fn page2_wp(&mut self) -> PAGE2_WP_W<2> {
        PAGE2_WP_W::new(self)
    }
    #[doc = "Bit 3 - CCM SRAM page write protection bit"]
    #[inline(always)]
    #[must_use]
    pub fn page3_wp(&mut self) -> PAGE3_WP_W<3> {
        PAGE3_WP_W::new(self)
    }
    #[doc = "Bit 4 - CCM SRAM page write protection bit"]
    #[inline(always)]
    #[must_use]
    pub fn page4_wp(&mut self) -> PAGE4_WP_W<4> {
        PAGE4_WP_W::new(self)
    }
    #[doc = "Bit 5 - CCM SRAM page write protection bit"]
    #[inline(always)]
    #[must_use]
    pub fn page5_wp(&mut self) -> PAGE5_WP_W<5> {
        PAGE5_WP_W::new(self)
    }
    #[doc = "Bit 6 - CCM SRAM page write protection bit"]
    #[inline(always)]
    #[must_use]
    pub fn page6_wp(&mut self) -> PAGE6_WP_W<6> {
        PAGE6_WP_W::new(self)
    }
    #[doc = "Bit 7 - CCM SRAM page write protection bit"]
    #[inline(always)]
    #[must_use]
    pub fn page7_wp(&mut self) -> PAGE7_WP_W<7> {
        PAGE7_WP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CCM SRAM protection register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [syscfg_rcr](index.html) module"]
pub struct SYSCFG_RCR_SPEC;
impl crate::RegisterSpec for SYSCFG_RCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [syscfg_rcr::R](R) reader structure"]
impl crate::Readable for SYSCFG_RCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [syscfg_rcr::W](W) writer structure"]
impl crate::Writable for SYSCFG_RCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SYSCFG_RCR to value 0"]
impl crate::Resettable for SYSCFG_RCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

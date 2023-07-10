#[doc = "Register `ACR` reader"]
pub struct R(crate::R<ACR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ACR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ACR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ACR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ACR` writer"]
pub struct W(crate::W<ACR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ACR_SPEC>;
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
impl From<crate::W<ACR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ACR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LATENCY` reader - LATENCY"]
pub type LATENCY_R = crate::FieldReader<LATENCY_A>;
#[doc = "LATENCY\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LATENCY_A {
    #[doc = "0: 0 wait states, if 0 &lt; HCLK &lt;= 24 MHz"]
    WS0 = 0,
    #[doc = "1: 1 wait state, if 24 &lt; HCLK &lt;= 48 MHz"]
    WS1 = 1,
    #[doc = "2: 2 wait states, if 48 &lt; HCLK &lt;= 72 MHz"]
    WS2 = 2,
}
impl From<LATENCY_A> for u8 {
    #[inline(always)]
    fn from(variant: LATENCY_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for LATENCY_A {
    type Ux = u8;
}
impl LATENCY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<LATENCY_A> {
        match self.bits {
            0 => Some(LATENCY_A::WS0),
            1 => Some(LATENCY_A::WS1),
            2 => Some(LATENCY_A::WS2),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `WS0`"]
    #[inline(always)]
    pub fn is_ws0(&self) -> bool {
        *self == LATENCY_A::WS0
    }
    #[doc = "Checks if the value of the field is `WS1`"]
    #[inline(always)]
    pub fn is_ws1(&self) -> bool {
        *self == LATENCY_A::WS1
    }
    #[doc = "Checks if the value of the field is `WS2`"]
    #[inline(always)]
    pub fn is_ws2(&self) -> bool {
        *self == LATENCY_A::WS2
    }
}
#[doc = "Field `LATENCY` writer - LATENCY"]
pub type LATENCY_W<'a, const O: u8> = crate::FieldWriter<'a, ACR_SPEC, 3, O, LATENCY_A>;
impl<'a, const O: u8> LATENCY_W<'a, O> {
    #[doc = "0 wait states, if 0 &lt; HCLK &lt;= 24 MHz"]
    #[inline(always)]
    pub fn ws0(self) -> &'a mut W {
        self.variant(LATENCY_A::WS0)
    }
    #[doc = "1 wait state, if 24 &lt; HCLK &lt;= 48 MHz"]
    #[inline(always)]
    pub fn ws1(self) -> &'a mut W {
        self.variant(LATENCY_A::WS1)
    }
    #[doc = "2 wait states, if 48 &lt; HCLK &lt;= 72 MHz"]
    #[inline(always)]
    pub fn ws2(self) -> &'a mut W {
        self.variant(LATENCY_A::WS2)
    }
}
#[doc = "Field `HLFCYA` reader - Flash half cycle access enable"]
pub type HLFCYA_R = crate::BitReader<HLFCYA_A>;
#[doc = "Flash half cycle access enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HLFCYA_A {
    #[doc = "0: Half cycle is disabled"]
    DISABLED = 0,
    #[doc = "1: Half cycle is enabled"]
    ENABLED = 1,
}
impl From<HLFCYA_A> for bool {
    #[inline(always)]
    fn from(variant: HLFCYA_A) -> Self {
        variant as u8 != 0
    }
}
impl HLFCYA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HLFCYA_A {
        match self.bits {
            false => HLFCYA_A::DISABLED,
            true => HLFCYA_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == HLFCYA_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == HLFCYA_A::ENABLED
    }
}
#[doc = "Field `HLFCYA` writer - Flash half cycle access enable"]
pub type HLFCYA_W<'a, const O: u8> = crate::BitWriter<'a, ACR_SPEC, O, HLFCYA_A>;
impl<'a, const O: u8> HLFCYA_W<'a, O> {
    #[doc = "Half cycle is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(HLFCYA_A::DISABLED)
    }
    #[doc = "Half cycle is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(HLFCYA_A::ENABLED)
    }
}
#[doc = "Field `PRFTBE` reader - PRFTBE"]
pub type PRFTBE_R = crate::BitReader<PRFTBE_A>;
#[doc = "PRFTBE\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PRFTBE_A {
    #[doc = "0: Prefetch is disabled"]
    DISABLED = 0,
    #[doc = "1: Prefetch is enabled"]
    ENABLED = 1,
}
impl From<PRFTBE_A> for bool {
    #[inline(always)]
    fn from(variant: PRFTBE_A) -> Self {
        variant as u8 != 0
    }
}
impl PRFTBE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PRFTBE_A {
        match self.bits {
            false => PRFTBE_A::DISABLED,
            true => PRFTBE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PRFTBE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PRFTBE_A::ENABLED
    }
}
#[doc = "Field `PRFTBE` writer - PRFTBE"]
pub type PRFTBE_W<'a, const O: u8> = crate::BitWriter<'a, ACR_SPEC, O, PRFTBE_A>;
impl<'a, const O: u8> PRFTBE_W<'a, O> {
    #[doc = "Prefetch is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PRFTBE_A::DISABLED)
    }
    #[doc = "Prefetch is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PRFTBE_A::ENABLED)
    }
}
#[doc = "Field `PRFTBS` reader - PRFTBS"]
pub type PRFTBS_R = crate::BitReader<PRFTBS_A>;
#[doc = "PRFTBS\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PRFTBS_A {
    #[doc = "0: Prefetch buffer is disabled"]
    DISABLED = 0,
    #[doc = "1: Prefetch buffer is enabled"]
    ENABLED = 1,
}
impl From<PRFTBS_A> for bool {
    #[inline(always)]
    fn from(variant: PRFTBS_A) -> Self {
        variant as u8 != 0
    }
}
impl PRFTBS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PRFTBS_A {
        match self.bits {
            false => PRFTBS_A::DISABLED,
            true => PRFTBS_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PRFTBS_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PRFTBS_A::ENABLED
    }
}
impl R {
    #[doc = "Bits 0:2 - LATENCY"]
    #[inline(always)]
    pub fn latency(&self) -> LATENCY_R {
        LATENCY_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - Flash half cycle access enable"]
    #[inline(always)]
    pub fn hlfcya(&self) -> HLFCYA_R {
        HLFCYA_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - PRFTBE"]
    #[inline(always)]
    pub fn prftbe(&self) -> PRFTBE_R {
        PRFTBE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - PRFTBS"]
    #[inline(always)]
    pub fn prftbs(&self) -> PRFTBS_R {
        PRFTBS_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - LATENCY"]
    #[inline(always)]
    #[must_use]
    pub fn latency(&mut self) -> LATENCY_W<0> {
        LATENCY_W::new(self)
    }
    #[doc = "Bit 3 - Flash half cycle access enable"]
    #[inline(always)]
    #[must_use]
    pub fn hlfcya(&mut self) -> HLFCYA_W<3> {
        HLFCYA_W::new(self)
    }
    #[doc = "Bit 4 - PRFTBE"]
    #[inline(always)]
    #[must_use]
    pub fn prftbe(&mut self) -> PRFTBE_W<4> {
        PRFTBE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Flash access control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [acr](index.html) module"]
pub struct ACR_SPEC;
impl crate::RegisterSpec for ACR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [acr::R](R) reader structure"]
impl crate::Readable for ACR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [acr::W](W) writer structure"]
impl crate::Writable for ACR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ACR to value 0x30"]
impl crate::Resettable for ACR_SPEC {
    const RESET_VALUE: Self::Ux = 0x30;
}

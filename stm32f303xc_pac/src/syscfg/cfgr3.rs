#[doc = "Register `CFGR3` reader"]
pub struct R(crate::R<CFGR3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFGR3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFGR3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFGR3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CFGR3` writer"]
pub struct W(crate::W<CFGR3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFGR3_SPEC>;
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
impl From<crate::W<CFGR3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFGR3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SPI1_RX_DMA_RMP` reader - SPI1_RX DMA remapping bit"]
pub type SPI1_RX_DMA_RMP_R = crate::FieldReader<SPI1_RX_DMA_RMP_A>;
#[doc = "SPI1_RX DMA remapping bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SPI1_RX_DMA_RMP_A {
    #[doc = "0: SPI1_RX mapped on DMA1 CH2"]
    MAP_DMA1CH3 = 0,
    #[doc = "1: SPI1_RX mapped on DMA1 CH4"]
    MAP_DMA1CH5 = 1,
    #[doc = "2: SPI1_RX mapped on DMA1 CH6"]
    MAP_DMA1CH7 = 2,
}
impl From<SPI1_RX_DMA_RMP_A> for u8 {
    #[inline(always)]
    fn from(variant: SPI1_RX_DMA_RMP_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SPI1_RX_DMA_RMP_A {
    type Ux = u8;
}
impl SPI1_RX_DMA_RMP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SPI1_RX_DMA_RMP_A> {
        match self.bits {
            0 => Some(SPI1_RX_DMA_RMP_A::MAP_DMA1CH3),
            1 => Some(SPI1_RX_DMA_RMP_A::MAP_DMA1CH5),
            2 => Some(SPI1_RX_DMA_RMP_A::MAP_DMA1CH7),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `MAP_DMA1CH3`"]
    #[inline(always)]
    pub fn is_map_dma1ch3(&self) -> bool {
        *self == SPI1_RX_DMA_RMP_A::MAP_DMA1CH3
    }
    #[doc = "Checks if the value of the field is `MAP_DMA1CH5`"]
    #[inline(always)]
    pub fn is_map_dma1ch5(&self) -> bool {
        *self == SPI1_RX_DMA_RMP_A::MAP_DMA1CH5
    }
    #[doc = "Checks if the value of the field is `MAP_DMA1CH7`"]
    #[inline(always)]
    pub fn is_map_dma1ch7(&self) -> bool {
        *self == SPI1_RX_DMA_RMP_A::MAP_DMA1CH7
    }
}
#[doc = "Field `SPI1_RX_DMA_RMP` writer - SPI1_RX DMA remapping bit"]
pub type SPI1_RX_DMA_RMP_W<'a, const O: u8> =
    crate::FieldWriter<'a, CFGR3_SPEC, 2, O, SPI1_RX_DMA_RMP_A>;
impl<'a, const O: u8> SPI1_RX_DMA_RMP_W<'a, O> {
    #[doc = "SPI1_RX mapped on DMA1 CH2"]
    #[inline(always)]
    pub fn map_dma1ch3(self) -> &'a mut W {
        self.variant(SPI1_RX_DMA_RMP_A::MAP_DMA1CH3)
    }
    #[doc = "SPI1_RX mapped on DMA1 CH4"]
    #[inline(always)]
    pub fn map_dma1ch5(self) -> &'a mut W {
        self.variant(SPI1_RX_DMA_RMP_A::MAP_DMA1CH5)
    }
    #[doc = "SPI1_RX mapped on DMA1 CH6"]
    #[inline(always)]
    pub fn map_dma1ch7(self) -> &'a mut W {
        self.variant(SPI1_RX_DMA_RMP_A::MAP_DMA1CH7)
    }
}
#[doc = "Field `SPI1_TX_DMA_RMP` reader - SPI1_TX DMA remapping bit"]
pub type SPI1_TX_DMA_RMP_R = crate::FieldReader<SPI1_TX_DMA_RMP_A>;
#[doc = "SPI1_TX DMA remapping bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SPI1_TX_DMA_RMP_A {
    #[doc = "0: SPI1_TX mapped on DMA1 CH3"]
    MAP_DMA1CH3 = 0,
    #[doc = "1: SPI1_TX mapped on DMA1 CH5"]
    MAP_DMA1CH5 = 1,
    #[doc = "2: SPI1_TX mapped on DMA1 CH7"]
    MAP_DMA1CH7 = 2,
}
impl From<SPI1_TX_DMA_RMP_A> for u8 {
    #[inline(always)]
    fn from(variant: SPI1_TX_DMA_RMP_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SPI1_TX_DMA_RMP_A {
    type Ux = u8;
}
impl SPI1_TX_DMA_RMP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SPI1_TX_DMA_RMP_A> {
        match self.bits {
            0 => Some(SPI1_TX_DMA_RMP_A::MAP_DMA1CH3),
            1 => Some(SPI1_TX_DMA_RMP_A::MAP_DMA1CH5),
            2 => Some(SPI1_TX_DMA_RMP_A::MAP_DMA1CH7),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `MAP_DMA1CH3`"]
    #[inline(always)]
    pub fn is_map_dma1ch3(&self) -> bool {
        *self == SPI1_TX_DMA_RMP_A::MAP_DMA1CH3
    }
    #[doc = "Checks if the value of the field is `MAP_DMA1CH5`"]
    #[inline(always)]
    pub fn is_map_dma1ch5(&self) -> bool {
        *self == SPI1_TX_DMA_RMP_A::MAP_DMA1CH5
    }
    #[doc = "Checks if the value of the field is `MAP_DMA1CH7`"]
    #[inline(always)]
    pub fn is_map_dma1ch7(&self) -> bool {
        *self == SPI1_TX_DMA_RMP_A::MAP_DMA1CH7
    }
}
#[doc = "Field `SPI1_TX_DMA_RMP` writer - SPI1_TX DMA remapping bit"]
pub type SPI1_TX_DMA_RMP_W<'a, const O: u8> =
    crate::FieldWriter<'a, CFGR3_SPEC, 2, O, SPI1_TX_DMA_RMP_A>;
impl<'a, const O: u8> SPI1_TX_DMA_RMP_W<'a, O> {
    #[doc = "SPI1_TX mapped on DMA1 CH3"]
    #[inline(always)]
    pub fn map_dma1ch3(self) -> &'a mut W {
        self.variant(SPI1_TX_DMA_RMP_A::MAP_DMA1CH3)
    }
    #[doc = "SPI1_TX mapped on DMA1 CH5"]
    #[inline(always)]
    pub fn map_dma1ch5(self) -> &'a mut W {
        self.variant(SPI1_TX_DMA_RMP_A::MAP_DMA1CH5)
    }
    #[doc = "SPI1_TX mapped on DMA1 CH7"]
    #[inline(always)]
    pub fn map_dma1ch7(self) -> &'a mut W {
        self.variant(SPI1_TX_DMA_RMP_A::MAP_DMA1CH7)
    }
}
#[doc = "Field `I2C1_RX_DMA_RMP` reader - I2C1_RX DMA remapping bit"]
pub type I2C1_RX_DMA_RMP_R = crate::FieldReader<I2C1_RX_DMA_RMP_A>;
#[doc = "I2C1_RX DMA remapping bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum I2C1_RX_DMA_RMP_A {
    #[doc = "0: I2C1_RX mapped on DMA1 CH7"]
    MAP_DMA1CH7 = 0,
    #[doc = "1: I2C1_RX mapped on DMA1 CH3"]
    MAP_DMA1CH3 = 1,
    #[doc = "2: I2C1_RX mapped on DMA1 CH5"]
    MAP_DMA1CH5 = 2,
}
impl From<I2C1_RX_DMA_RMP_A> for u8 {
    #[inline(always)]
    fn from(variant: I2C1_RX_DMA_RMP_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for I2C1_RX_DMA_RMP_A {
    type Ux = u8;
}
impl I2C1_RX_DMA_RMP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<I2C1_RX_DMA_RMP_A> {
        match self.bits {
            0 => Some(I2C1_RX_DMA_RMP_A::MAP_DMA1CH7),
            1 => Some(I2C1_RX_DMA_RMP_A::MAP_DMA1CH3),
            2 => Some(I2C1_RX_DMA_RMP_A::MAP_DMA1CH5),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `MAP_DMA1CH7`"]
    #[inline(always)]
    pub fn is_map_dma1ch7(&self) -> bool {
        *self == I2C1_RX_DMA_RMP_A::MAP_DMA1CH7
    }
    #[doc = "Checks if the value of the field is `MAP_DMA1CH3`"]
    #[inline(always)]
    pub fn is_map_dma1ch3(&self) -> bool {
        *self == I2C1_RX_DMA_RMP_A::MAP_DMA1CH3
    }
    #[doc = "Checks if the value of the field is `MAP_DMA1CH5`"]
    #[inline(always)]
    pub fn is_map_dma1ch5(&self) -> bool {
        *self == I2C1_RX_DMA_RMP_A::MAP_DMA1CH5
    }
}
#[doc = "Field `I2C1_RX_DMA_RMP` writer - I2C1_RX DMA remapping bit"]
pub type I2C1_RX_DMA_RMP_W<'a, const O: u8> =
    crate::FieldWriter<'a, CFGR3_SPEC, 2, O, I2C1_RX_DMA_RMP_A>;
impl<'a, const O: u8> I2C1_RX_DMA_RMP_W<'a, O> {
    #[doc = "I2C1_RX mapped on DMA1 CH7"]
    #[inline(always)]
    pub fn map_dma1ch7(self) -> &'a mut W {
        self.variant(I2C1_RX_DMA_RMP_A::MAP_DMA1CH7)
    }
    #[doc = "I2C1_RX mapped on DMA1 CH3"]
    #[inline(always)]
    pub fn map_dma1ch3(self) -> &'a mut W {
        self.variant(I2C1_RX_DMA_RMP_A::MAP_DMA1CH3)
    }
    #[doc = "I2C1_RX mapped on DMA1 CH5"]
    #[inline(always)]
    pub fn map_dma1ch5(self) -> &'a mut W {
        self.variant(I2C1_RX_DMA_RMP_A::MAP_DMA1CH5)
    }
}
#[doc = "Field `I2C1_TX_DMA_RMP` reader - I2C1_TX DMA remapping bit"]
pub type I2C1_TX_DMA_RMP_R = crate::FieldReader<I2C1_TX_DMA_RMP_A>;
#[doc = "I2C1_TX DMA remapping bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum I2C1_TX_DMA_RMP_A {
    #[doc = "0: I2C1_TX mapped on DMA1 CH6"]
    MAP_DMA1CH6 = 0,
    #[doc = "1: I2C1_TX mapped on DMA1 CH2"]
    MAP_DMA1CH2 = 1,
    #[doc = "2: I2C1_TX mapped on DMA1 CH4"]
    MAP_DMA1CH4 = 2,
}
impl From<I2C1_TX_DMA_RMP_A> for u8 {
    #[inline(always)]
    fn from(variant: I2C1_TX_DMA_RMP_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for I2C1_TX_DMA_RMP_A {
    type Ux = u8;
}
impl I2C1_TX_DMA_RMP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<I2C1_TX_DMA_RMP_A> {
        match self.bits {
            0 => Some(I2C1_TX_DMA_RMP_A::MAP_DMA1CH6),
            1 => Some(I2C1_TX_DMA_RMP_A::MAP_DMA1CH2),
            2 => Some(I2C1_TX_DMA_RMP_A::MAP_DMA1CH4),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `MAP_DMA1CH6`"]
    #[inline(always)]
    pub fn is_map_dma1ch6(&self) -> bool {
        *self == I2C1_TX_DMA_RMP_A::MAP_DMA1CH6
    }
    #[doc = "Checks if the value of the field is `MAP_DMA1CH2`"]
    #[inline(always)]
    pub fn is_map_dma1ch2(&self) -> bool {
        *self == I2C1_TX_DMA_RMP_A::MAP_DMA1CH2
    }
    #[doc = "Checks if the value of the field is `MAP_DMA1CH4`"]
    #[inline(always)]
    pub fn is_map_dma1ch4(&self) -> bool {
        *self == I2C1_TX_DMA_RMP_A::MAP_DMA1CH4
    }
}
#[doc = "Field `I2C1_TX_DMA_RMP` writer - I2C1_TX DMA remapping bit"]
pub type I2C1_TX_DMA_RMP_W<'a, const O: u8> =
    crate::FieldWriter<'a, CFGR3_SPEC, 2, O, I2C1_TX_DMA_RMP_A>;
impl<'a, const O: u8> I2C1_TX_DMA_RMP_W<'a, O> {
    #[doc = "I2C1_TX mapped on DMA1 CH6"]
    #[inline(always)]
    pub fn map_dma1ch6(self) -> &'a mut W {
        self.variant(I2C1_TX_DMA_RMP_A::MAP_DMA1CH6)
    }
    #[doc = "I2C1_TX mapped on DMA1 CH2"]
    #[inline(always)]
    pub fn map_dma1ch2(self) -> &'a mut W {
        self.variant(I2C1_TX_DMA_RMP_A::MAP_DMA1CH2)
    }
    #[doc = "I2C1_TX mapped on DMA1 CH4"]
    #[inline(always)]
    pub fn map_dma1ch4(self) -> &'a mut W {
        self.variant(I2C1_TX_DMA_RMP_A::MAP_DMA1CH4)
    }
}
#[doc = "Field `ADC2_DMA_RMP` reader - ADC2 DMA remapping bit"]
pub type ADC2_DMA_RMP_R = crate::FieldReader<ADC2_DMA_RMP_A>;
#[doc = "ADC2 DMA remapping bit\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ADC2_DMA_RMP_A {
    #[doc = "0: ADC2 mapped on DMA2"]
    MAP_DMA2 = 0,
    #[doc = "2: ADC2 mapped on DMA1 channel 2"]
    MAP_DMA1CH2 = 2,
    #[doc = "3: ADC2 mapped on DMA1 channel 4"]
    MAP_DMA1CH4 = 3,
}
impl From<ADC2_DMA_RMP_A> for u8 {
    #[inline(always)]
    fn from(variant: ADC2_DMA_RMP_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ADC2_DMA_RMP_A {
    type Ux = u8;
}
impl ADC2_DMA_RMP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<ADC2_DMA_RMP_A> {
        match self.bits {
            0 => Some(ADC2_DMA_RMP_A::MAP_DMA2),
            2 => Some(ADC2_DMA_RMP_A::MAP_DMA1CH2),
            3 => Some(ADC2_DMA_RMP_A::MAP_DMA1CH4),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `MAP_DMA2`"]
    #[inline(always)]
    pub fn is_map_dma2(&self) -> bool {
        *self == ADC2_DMA_RMP_A::MAP_DMA2
    }
    #[doc = "Checks if the value of the field is `MAP_DMA1CH2`"]
    #[inline(always)]
    pub fn is_map_dma1ch2(&self) -> bool {
        *self == ADC2_DMA_RMP_A::MAP_DMA1CH2
    }
    #[doc = "Checks if the value of the field is `MAP_DMA1CH4`"]
    #[inline(always)]
    pub fn is_map_dma1ch4(&self) -> bool {
        *self == ADC2_DMA_RMP_A::MAP_DMA1CH4
    }
}
#[doc = "Field `ADC2_DMA_RMP` writer - ADC2 DMA remapping bit"]
pub type ADC2_DMA_RMP_W<'a, const O: u8> = crate::FieldWriter<'a, CFGR3_SPEC, 2, O, ADC2_DMA_RMP_A>;
impl<'a, const O: u8> ADC2_DMA_RMP_W<'a, O> {
    #[doc = "ADC2 mapped on DMA2"]
    #[inline(always)]
    pub fn map_dma2(self) -> &'a mut W {
        self.variant(ADC2_DMA_RMP_A::MAP_DMA2)
    }
    #[doc = "ADC2 mapped on DMA1 channel 2"]
    #[inline(always)]
    pub fn map_dma1ch2(self) -> &'a mut W {
        self.variant(ADC2_DMA_RMP_A::MAP_DMA1CH2)
    }
    #[doc = "ADC2 mapped on DMA1 channel 4"]
    #[inline(always)]
    pub fn map_dma1ch4(self) -> &'a mut W {
        self.variant(ADC2_DMA_RMP_A::MAP_DMA1CH4)
    }
}
impl R {
    #[doc = "Bits 0:1 - SPI1_RX DMA remapping bit"]
    #[inline(always)]
    pub fn spi1_rx_dma_rmp(&self) -> SPI1_RX_DMA_RMP_R {
        SPI1_RX_DMA_RMP_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - SPI1_TX DMA remapping bit"]
    #[inline(always)]
    pub fn spi1_tx_dma_rmp(&self) -> SPI1_TX_DMA_RMP_R {
        SPI1_TX_DMA_RMP_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - I2C1_RX DMA remapping bit"]
    #[inline(always)]
    pub fn i2c1_rx_dma_rmp(&self) -> I2C1_RX_DMA_RMP_R {
        I2C1_RX_DMA_RMP_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - I2C1_TX DMA remapping bit"]
    #[inline(always)]
    pub fn i2c1_tx_dma_rmp(&self) -> I2C1_TX_DMA_RMP_R {
        I2C1_TX_DMA_RMP_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - ADC2 DMA remapping bit"]
    #[inline(always)]
    pub fn adc2_dma_rmp(&self) -> ADC2_DMA_RMP_R {
        ADC2_DMA_RMP_R::new(((self.bits >> 8) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - SPI1_RX DMA remapping bit"]
    #[inline(always)]
    #[must_use]
    pub fn spi1_rx_dma_rmp(&mut self) -> SPI1_RX_DMA_RMP_W<0> {
        SPI1_RX_DMA_RMP_W::new(self)
    }
    #[doc = "Bits 2:3 - SPI1_TX DMA remapping bit"]
    #[inline(always)]
    #[must_use]
    pub fn spi1_tx_dma_rmp(&mut self) -> SPI1_TX_DMA_RMP_W<2> {
        SPI1_TX_DMA_RMP_W::new(self)
    }
    #[doc = "Bits 4:5 - I2C1_RX DMA remapping bit"]
    #[inline(always)]
    #[must_use]
    pub fn i2c1_rx_dma_rmp(&mut self) -> I2C1_RX_DMA_RMP_W<4> {
        I2C1_RX_DMA_RMP_W::new(self)
    }
    #[doc = "Bits 6:7 - I2C1_TX DMA remapping bit"]
    #[inline(always)]
    #[must_use]
    pub fn i2c1_tx_dma_rmp(&mut self) -> I2C1_TX_DMA_RMP_W<6> {
        I2C1_TX_DMA_RMP_W::new(self)
    }
    #[doc = "Bits 8:9 - ADC2 DMA remapping bit"]
    #[inline(always)]
    #[must_use]
    pub fn adc2_dma_rmp(&mut self) -> ADC2_DMA_RMP_W<8> {
        ADC2_DMA_RMP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "configuration register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfgr3](index.html) module"]
pub struct CFGR3_SPEC;
impl crate::RegisterSpec for CFGR3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfgr3::R](R) reader structure"]
impl crate::Readable for CFGR3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cfgr3::W](W) writer structure"]
impl crate::Writable for CFGR3_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CFGR3 to value 0x0200"]
impl crate::Resettable for CFGR3_SPEC {
    const RESET_VALUE: Self::Ux = 0x0200;
}

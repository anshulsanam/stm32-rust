#[doc = "Register `I2SCFGR` reader"]
pub struct R(crate::R<I2SCFGR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<I2SCFGR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<I2SCFGR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<I2SCFGR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `I2SCFGR` writer"]
pub struct W(crate::W<I2SCFGR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<I2SCFGR_SPEC>;
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
impl From<crate::W<I2SCFGR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<I2SCFGR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CHLEN` reader - Channel length (number of bits per audio channel)"]
pub type CHLEN_R = crate::BitReader<CHLEN_A>;
#[doc = "Channel length (number of bits per audio channel)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CHLEN_A {
    #[doc = "0: 16-bit wide"]
    SIXTEEN_BIT = 0,
    #[doc = "1: 32-bit wide"]
    THIRTY_TWO_BIT = 1,
}
impl From<CHLEN_A> for bool {
    #[inline(always)]
    fn from(variant: CHLEN_A) -> Self {
        variant as u8 != 0
    }
}
impl CHLEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHLEN_A {
        match self.bits {
            false => CHLEN_A::SIXTEEN_BIT,
            true => CHLEN_A::THIRTY_TWO_BIT,
        }
    }
    #[doc = "Checks if the value of the field is `SIXTEEN_BIT`"]
    #[inline(always)]
    pub fn is_sixteen_bit(&self) -> bool {
        *self == CHLEN_A::SIXTEEN_BIT
    }
    #[doc = "Checks if the value of the field is `THIRTY_TWO_BIT`"]
    #[inline(always)]
    pub fn is_thirty_two_bit(&self) -> bool {
        *self == CHLEN_A::THIRTY_TWO_BIT
    }
}
#[doc = "Field `CHLEN` writer - Channel length (number of bits per audio channel)"]
pub type CHLEN_W<'a, const O: u8> = crate::BitWriter<'a, I2SCFGR_SPEC, O, CHLEN_A>;
impl<'a, const O: u8> CHLEN_W<'a, O> {
    #[doc = "16-bit wide"]
    #[inline(always)]
    pub fn sixteen_bit(self) -> &'a mut W {
        self.variant(CHLEN_A::SIXTEEN_BIT)
    }
    #[doc = "32-bit wide"]
    #[inline(always)]
    pub fn thirty_two_bit(self) -> &'a mut W {
        self.variant(CHLEN_A::THIRTY_TWO_BIT)
    }
}
#[doc = "Field `DATLEN` reader - Data length to be transferred"]
pub type DATLEN_R = crate::FieldReader<DATLEN_A>;
#[doc = "Data length to be transferred\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DATLEN_A {
    #[doc = "0: 16-bit data length"]
    SIXTEEN_BIT = 0,
    #[doc = "1: 24-bit data length"]
    TWENTY_FOUR_BIT = 1,
    #[doc = "2: 32-bit data length"]
    THIRTY_TWO_BIT = 2,
}
impl From<DATLEN_A> for u8 {
    #[inline(always)]
    fn from(variant: DATLEN_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DATLEN_A {
    type Ux = u8;
}
impl DATLEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<DATLEN_A> {
        match self.bits {
            0 => Some(DATLEN_A::SIXTEEN_BIT),
            1 => Some(DATLEN_A::TWENTY_FOUR_BIT),
            2 => Some(DATLEN_A::THIRTY_TWO_BIT),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `SIXTEEN_BIT`"]
    #[inline(always)]
    pub fn is_sixteen_bit(&self) -> bool {
        *self == DATLEN_A::SIXTEEN_BIT
    }
    #[doc = "Checks if the value of the field is `TWENTY_FOUR_BIT`"]
    #[inline(always)]
    pub fn is_twenty_four_bit(&self) -> bool {
        *self == DATLEN_A::TWENTY_FOUR_BIT
    }
    #[doc = "Checks if the value of the field is `THIRTY_TWO_BIT`"]
    #[inline(always)]
    pub fn is_thirty_two_bit(&self) -> bool {
        *self == DATLEN_A::THIRTY_TWO_BIT
    }
}
#[doc = "Field `DATLEN` writer - Data length to be transferred"]
pub type DATLEN_W<'a, const O: u8> = crate::FieldWriter<'a, I2SCFGR_SPEC, 2, O, DATLEN_A>;
impl<'a, const O: u8> DATLEN_W<'a, O> {
    #[doc = "16-bit data length"]
    #[inline(always)]
    pub fn sixteen_bit(self) -> &'a mut W {
        self.variant(DATLEN_A::SIXTEEN_BIT)
    }
    #[doc = "24-bit data length"]
    #[inline(always)]
    pub fn twenty_four_bit(self) -> &'a mut W {
        self.variant(DATLEN_A::TWENTY_FOUR_BIT)
    }
    #[doc = "32-bit data length"]
    #[inline(always)]
    pub fn thirty_two_bit(self) -> &'a mut W {
        self.variant(DATLEN_A::THIRTY_TWO_BIT)
    }
}
#[doc = "Field `CKPOL` reader - Steady state clock polarity"]
pub type CKPOL_R = crate::BitReader<CKPOL_A>;
#[doc = "Steady state clock polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CKPOL_A {
    #[doc = "0: I2S clock inactive state is low level"]
    IDLE_LOW = 0,
    #[doc = "1: I2S clock inactive state is high level"]
    IDLE_HIGH = 1,
}
impl From<CKPOL_A> for bool {
    #[inline(always)]
    fn from(variant: CKPOL_A) -> Self {
        variant as u8 != 0
    }
}
impl CKPOL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CKPOL_A {
        match self.bits {
            false => CKPOL_A::IDLE_LOW,
            true => CKPOL_A::IDLE_HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `IDLE_LOW`"]
    #[inline(always)]
    pub fn is_idle_low(&self) -> bool {
        *self == CKPOL_A::IDLE_LOW
    }
    #[doc = "Checks if the value of the field is `IDLE_HIGH`"]
    #[inline(always)]
    pub fn is_idle_high(&self) -> bool {
        *self == CKPOL_A::IDLE_HIGH
    }
}
#[doc = "Field `CKPOL` writer - Steady state clock polarity"]
pub type CKPOL_W<'a, const O: u8> = crate::BitWriter<'a, I2SCFGR_SPEC, O, CKPOL_A>;
impl<'a, const O: u8> CKPOL_W<'a, O> {
    #[doc = "I2S clock inactive state is low level"]
    #[inline(always)]
    pub fn idle_low(self) -> &'a mut W {
        self.variant(CKPOL_A::IDLE_LOW)
    }
    #[doc = "I2S clock inactive state is high level"]
    #[inline(always)]
    pub fn idle_high(self) -> &'a mut W {
        self.variant(CKPOL_A::IDLE_HIGH)
    }
}
#[doc = "Field `I2SSTD` reader - I2S standard selection"]
pub type I2SSTD_R = crate::FieldReader<I2SSTD_A>;
#[doc = "I2S standard selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum I2SSTD_A {
    #[doc = "0: I2S Philips standard"]
    PHILIPS = 0,
    #[doc = "1: MSB justified standard"]
    MSB = 1,
    #[doc = "2: LSB justified standard"]
    LSB = 2,
    #[doc = "3: PCM standard"]
    PCM = 3,
}
impl From<I2SSTD_A> for u8 {
    #[inline(always)]
    fn from(variant: I2SSTD_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for I2SSTD_A {
    type Ux = u8;
}
impl I2SSTD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> I2SSTD_A {
        match self.bits {
            0 => I2SSTD_A::PHILIPS,
            1 => I2SSTD_A::MSB,
            2 => I2SSTD_A::LSB,
            3 => I2SSTD_A::PCM,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PHILIPS`"]
    #[inline(always)]
    pub fn is_philips(&self) -> bool {
        *self == I2SSTD_A::PHILIPS
    }
    #[doc = "Checks if the value of the field is `MSB`"]
    #[inline(always)]
    pub fn is_msb(&self) -> bool {
        *self == I2SSTD_A::MSB
    }
    #[doc = "Checks if the value of the field is `LSB`"]
    #[inline(always)]
    pub fn is_lsb(&self) -> bool {
        *self == I2SSTD_A::LSB
    }
    #[doc = "Checks if the value of the field is `PCM`"]
    #[inline(always)]
    pub fn is_pcm(&self) -> bool {
        *self == I2SSTD_A::PCM
    }
}
#[doc = "Field `I2SSTD` writer - I2S standard selection"]
pub type I2SSTD_W<'a, const O: u8> = crate::FieldWriterSafe<'a, I2SCFGR_SPEC, 2, O, I2SSTD_A>;
impl<'a, const O: u8> I2SSTD_W<'a, O> {
    #[doc = "I2S Philips standard"]
    #[inline(always)]
    pub fn philips(self) -> &'a mut W {
        self.variant(I2SSTD_A::PHILIPS)
    }
    #[doc = "MSB justified standard"]
    #[inline(always)]
    pub fn msb(self) -> &'a mut W {
        self.variant(I2SSTD_A::MSB)
    }
    #[doc = "LSB justified standard"]
    #[inline(always)]
    pub fn lsb(self) -> &'a mut W {
        self.variant(I2SSTD_A::LSB)
    }
    #[doc = "PCM standard"]
    #[inline(always)]
    pub fn pcm(self) -> &'a mut W {
        self.variant(I2SSTD_A::PCM)
    }
}
#[doc = "Field `PCMSYNC` reader - PCM frame synchronization"]
pub type PCMSYNC_R = crate::BitReader<PCMSYNC_A>;
#[doc = "PCM frame synchronization\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PCMSYNC_A {
    #[doc = "0: Short frame synchronisation"]
    SHORT = 0,
    #[doc = "1: Long frame synchronisation"]
    LONG = 1,
}
impl From<PCMSYNC_A> for bool {
    #[inline(always)]
    fn from(variant: PCMSYNC_A) -> Self {
        variant as u8 != 0
    }
}
impl PCMSYNC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PCMSYNC_A {
        match self.bits {
            false => PCMSYNC_A::SHORT,
            true => PCMSYNC_A::LONG,
        }
    }
    #[doc = "Checks if the value of the field is `SHORT`"]
    #[inline(always)]
    pub fn is_short(&self) -> bool {
        *self == PCMSYNC_A::SHORT
    }
    #[doc = "Checks if the value of the field is `LONG`"]
    #[inline(always)]
    pub fn is_long(&self) -> bool {
        *self == PCMSYNC_A::LONG
    }
}
#[doc = "Field `PCMSYNC` writer - PCM frame synchronization"]
pub type PCMSYNC_W<'a, const O: u8> = crate::BitWriter<'a, I2SCFGR_SPEC, O, PCMSYNC_A>;
impl<'a, const O: u8> PCMSYNC_W<'a, O> {
    #[doc = "Short frame synchronisation"]
    #[inline(always)]
    pub fn short(self) -> &'a mut W {
        self.variant(PCMSYNC_A::SHORT)
    }
    #[doc = "Long frame synchronisation"]
    #[inline(always)]
    pub fn long(self) -> &'a mut W {
        self.variant(PCMSYNC_A::LONG)
    }
}
#[doc = "Field `I2SCFG` reader - I2S configuration mode"]
pub type I2SCFG_R = crate::FieldReader<I2SCFG_A>;
#[doc = "I2S configuration mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum I2SCFG_A {
    #[doc = "0: Slave - transmit"]
    SLAVE_TX = 0,
    #[doc = "1: Slave - receive"]
    SLAVE_RX = 1,
    #[doc = "2: Master - transmit"]
    MASTER_TX = 2,
    #[doc = "3: Master - receive"]
    MASTER_RX = 3,
}
impl From<I2SCFG_A> for u8 {
    #[inline(always)]
    fn from(variant: I2SCFG_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for I2SCFG_A {
    type Ux = u8;
}
impl I2SCFG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> I2SCFG_A {
        match self.bits {
            0 => I2SCFG_A::SLAVE_TX,
            1 => I2SCFG_A::SLAVE_RX,
            2 => I2SCFG_A::MASTER_TX,
            3 => I2SCFG_A::MASTER_RX,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SLAVE_TX`"]
    #[inline(always)]
    pub fn is_slave_tx(&self) -> bool {
        *self == I2SCFG_A::SLAVE_TX
    }
    #[doc = "Checks if the value of the field is `SLAVE_RX`"]
    #[inline(always)]
    pub fn is_slave_rx(&self) -> bool {
        *self == I2SCFG_A::SLAVE_RX
    }
    #[doc = "Checks if the value of the field is `MASTER_TX`"]
    #[inline(always)]
    pub fn is_master_tx(&self) -> bool {
        *self == I2SCFG_A::MASTER_TX
    }
    #[doc = "Checks if the value of the field is `MASTER_RX`"]
    #[inline(always)]
    pub fn is_master_rx(&self) -> bool {
        *self == I2SCFG_A::MASTER_RX
    }
}
#[doc = "Field `I2SCFG` writer - I2S configuration mode"]
pub type I2SCFG_W<'a, const O: u8> = crate::FieldWriterSafe<'a, I2SCFGR_SPEC, 2, O, I2SCFG_A>;
impl<'a, const O: u8> I2SCFG_W<'a, O> {
    #[doc = "Slave - transmit"]
    #[inline(always)]
    pub fn slave_tx(self) -> &'a mut W {
        self.variant(I2SCFG_A::SLAVE_TX)
    }
    #[doc = "Slave - receive"]
    #[inline(always)]
    pub fn slave_rx(self) -> &'a mut W {
        self.variant(I2SCFG_A::SLAVE_RX)
    }
    #[doc = "Master - transmit"]
    #[inline(always)]
    pub fn master_tx(self) -> &'a mut W {
        self.variant(I2SCFG_A::MASTER_TX)
    }
    #[doc = "Master - receive"]
    #[inline(always)]
    pub fn master_rx(self) -> &'a mut W {
        self.variant(I2SCFG_A::MASTER_RX)
    }
}
#[doc = "Field `I2SE` reader - I2S Enable"]
pub type I2SE_R = crate::BitReader<I2SE_A>;
#[doc = "I2S Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum I2SE_A {
    #[doc = "0: I2S peripheral is disabled"]
    DISABLED = 0,
    #[doc = "1: I2S peripheral is enabled"]
    ENABLED = 1,
}
impl From<I2SE_A> for bool {
    #[inline(always)]
    fn from(variant: I2SE_A) -> Self {
        variant as u8 != 0
    }
}
impl I2SE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> I2SE_A {
        match self.bits {
            false => I2SE_A::DISABLED,
            true => I2SE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == I2SE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == I2SE_A::ENABLED
    }
}
#[doc = "Field `I2SE` writer - I2S Enable"]
pub type I2SE_W<'a, const O: u8> = crate::BitWriter<'a, I2SCFGR_SPEC, O, I2SE_A>;
impl<'a, const O: u8> I2SE_W<'a, O> {
    #[doc = "I2S peripheral is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(I2SE_A::DISABLED)
    }
    #[doc = "I2S peripheral is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(I2SE_A::ENABLED)
    }
}
#[doc = "Field `I2SMOD` reader - I2S mode selection"]
pub type I2SMOD_R = crate::BitReader<I2SMOD_A>;
#[doc = "I2S mode selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum I2SMOD_A {
    #[doc = "0: SPI mode is selected"]
    SPIMODE = 0,
    #[doc = "1: I2S mode is selected"]
    I2SMODE = 1,
}
impl From<I2SMOD_A> for bool {
    #[inline(always)]
    fn from(variant: I2SMOD_A) -> Self {
        variant as u8 != 0
    }
}
impl I2SMOD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> I2SMOD_A {
        match self.bits {
            false => I2SMOD_A::SPIMODE,
            true => I2SMOD_A::I2SMODE,
        }
    }
    #[doc = "Checks if the value of the field is `SPIMODE`"]
    #[inline(always)]
    pub fn is_spimode(&self) -> bool {
        *self == I2SMOD_A::SPIMODE
    }
    #[doc = "Checks if the value of the field is `I2SMODE`"]
    #[inline(always)]
    pub fn is_i2smode(&self) -> bool {
        *self == I2SMOD_A::I2SMODE
    }
}
#[doc = "Field `I2SMOD` writer - I2S mode selection"]
pub type I2SMOD_W<'a, const O: u8> = crate::BitWriter<'a, I2SCFGR_SPEC, O, I2SMOD_A>;
impl<'a, const O: u8> I2SMOD_W<'a, O> {
    #[doc = "SPI mode is selected"]
    #[inline(always)]
    pub fn spimode(self) -> &'a mut W {
        self.variant(I2SMOD_A::SPIMODE)
    }
    #[doc = "I2S mode is selected"]
    #[inline(always)]
    pub fn i2smode(self) -> &'a mut W {
        self.variant(I2SMOD_A::I2SMODE)
    }
}
impl R {
    #[doc = "Bit 0 - Channel length (number of bits per audio channel)"]
    #[inline(always)]
    pub fn chlen(&self) -> CHLEN_R {
        CHLEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - Data length to be transferred"]
    #[inline(always)]
    pub fn datlen(&self) -> DATLEN_R {
        DATLEN_R::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bit 3 - Steady state clock polarity"]
    #[inline(always)]
    pub fn ckpol(&self) -> CKPOL_R {
        CKPOL_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - I2S standard selection"]
    #[inline(always)]
    pub fn i2sstd(&self) -> I2SSTD_R {
        I2SSTD_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 7 - PCM frame synchronization"]
    #[inline(always)]
    pub fn pcmsync(&self) -> PCMSYNC_R {
        PCMSYNC_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - I2S configuration mode"]
    #[inline(always)]
    pub fn i2scfg(&self) -> I2SCFG_R {
        I2SCFG_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 10 - I2S Enable"]
    #[inline(always)]
    pub fn i2se(&self) -> I2SE_R {
        I2SE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - I2S mode selection"]
    #[inline(always)]
    pub fn i2smod(&self) -> I2SMOD_R {
        I2SMOD_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Channel length (number of bits per audio channel)"]
    #[inline(always)]
    #[must_use]
    pub fn chlen(&mut self) -> CHLEN_W<0> {
        CHLEN_W::new(self)
    }
    #[doc = "Bits 1:2 - Data length to be transferred"]
    #[inline(always)]
    #[must_use]
    pub fn datlen(&mut self) -> DATLEN_W<1> {
        DATLEN_W::new(self)
    }
    #[doc = "Bit 3 - Steady state clock polarity"]
    #[inline(always)]
    #[must_use]
    pub fn ckpol(&mut self) -> CKPOL_W<3> {
        CKPOL_W::new(self)
    }
    #[doc = "Bits 4:5 - I2S standard selection"]
    #[inline(always)]
    #[must_use]
    pub fn i2sstd(&mut self) -> I2SSTD_W<4> {
        I2SSTD_W::new(self)
    }
    #[doc = "Bit 7 - PCM frame synchronization"]
    #[inline(always)]
    #[must_use]
    pub fn pcmsync(&mut self) -> PCMSYNC_W<7> {
        PCMSYNC_W::new(self)
    }
    #[doc = "Bits 8:9 - I2S configuration mode"]
    #[inline(always)]
    #[must_use]
    pub fn i2scfg(&mut self) -> I2SCFG_W<8> {
        I2SCFG_W::new(self)
    }
    #[doc = "Bit 10 - I2S Enable"]
    #[inline(always)]
    #[must_use]
    pub fn i2se(&mut self) -> I2SE_W<10> {
        I2SE_W::new(self)
    }
    #[doc = "Bit 11 - I2S mode selection"]
    #[inline(always)]
    #[must_use]
    pub fn i2smod(&mut self) -> I2SMOD_W<11> {
        I2SMOD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I2S configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2scfgr](index.html) module"]
pub struct I2SCFGR_SPEC;
impl crate::RegisterSpec for I2SCFGR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [i2scfgr::R](R) reader structure"]
impl crate::Readable for I2SCFGR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [i2scfgr::W](W) writer structure"]
impl crate::Writable for I2SCFGR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets I2SCFGR to value 0"]
impl crate::Resettable for I2SCFGR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}

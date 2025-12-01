#[doc = "Register `IER` reader"]
pub type R = crate::R<IER_SPEC>;
#[doc = "Register `IER` writer"]
pub type W = crate::W<IER_SPEC>;
#[doc = "Tamper 1 interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TAMP1IE_A {
    #[doc = "0: Tamper 1 interrupt disabled."]
    B_0x0 = 0,
    #[doc = "1: Tamper 1 interrupt enabled."]
    B_0x1 = 1,
}
impl From<TAMP1IE_A> for bool {
    #[inline(always)]
    fn from(variant: TAMP1IE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TAMP1IE` reader - Tamper 1 interrupt enable"]
pub type TAMP1IE_R = crate::BitReader<TAMP1IE_A>;
impl TAMP1IE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TAMP1IE_A {
        match self.bits {
            false => TAMP1IE_A::B_0x0,
            true => TAMP1IE_A::B_0x1,
        }
    }
    #[doc = "Tamper 1 interrupt disabled."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == TAMP1IE_A::B_0x0
    }
    #[doc = "Tamper 1 interrupt enabled."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == TAMP1IE_A::B_0x1
    }
}
#[doc = "Field `TAMP1IE` writer - Tamper 1 interrupt enable"]
pub type TAMP1IE_W<'a, REG> = crate::BitWriter<'a, REG, TAMP1IE_A>;
impl<'a, REG> TAMP1IE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Tamper 1 interrupt disabled."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(TAMP1IE_A::B_0x0)
    }
    #[doc = "Tamper 1 interrupt enabled."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(TAMP1IE_A::B_0x1)
    }
}
#[doc = "Tamper 2 interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TAMP2IE_A {
    #[doc = "0: Tamper 2 interrupt disabled."]
    B_0x0 = 0,
    #[doc = "1: Tamper 2 interrupt enabled."]
    B_0x1 = 1,
}
impl From<TAMP2IE_A> for bool {
    #[inline(always)]
    fn from(variant: TAMP2IE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TAMP2IE` reader - Tamper 2 interrupt enable"]
pub type TAMP2IE_R = crate::BitReader<TAMP2IE_A>;
impl TAMP2IE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TAMP2IE_A {
        match self.bits {
            false => TAMP2IE_A::B_0x0,
            true => TAMP2IE_A::B_0x1,
        }
    }
    #[doc = "Tamper 2 interrupt disabled."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == TAMP2IE_A::B_0x0
    }
    #[doc = "Tamper 2 interrupt enabled."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == TAMP2IE_A::B_0x1
    }
}
#[doc = "Field `TAMP2IE` writer - Tamper 2 interrupt enable"]
pub type TAMP2IE_W<'a, REG> = crate::BitWriter<'a, REG, TAMP2IE_A>;
impl<'a, REG> TAMP2IE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Tamper 2 interrupt disabled."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(TAMP2IE_A::B_0x0)
    }
    #[doc = "Tamper 2 interrupt enabled."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(TAMP2IE_A::B_0x1)
    }
}
#[doc = "Tamper 3 interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TAMP3IE_A {
    #[doc = "0: Tamper 3 interrupt disabled."]
    B_0x0 = 0,
    #[doc = "1: Tamper 3 interrupt enabled.."]
    B_0x1 = 1,
}
impl From<TAMP3IE_A> for bool {
    #[inline(always)]
    fn from(variant: TAMP3IE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TAMP3IE` reader - Tamper 3 interrupt enable"]
pub type TAMP3IE_R = crate::BitReader<TAMP3IE_A>;
impl TAMP3IE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TAMP3IE_A {
        match self.bits {
            false => TAMP3IE_A::B_0x0,
            true => TAMP3IE_A::B_0x1,
        }
    }
    #[doc = "Tamper 3 interrupt disabled."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == TAMP3IE_A::B_0x0
    }
    #[doc = "Tamper 3 interrupt enabled.."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == TAMP3IE_A::B_0x1
    }
}
#[doc = "Field `TAMP3IE` writer - Tamper 3 interrupt enable"]
pub type TAMP3IE_W<'a, REG> = crate::BitWriter<'a, REG, TAMP3IE_A>;
impl<'a, REG> TAMP3IE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Tamper 3 interrupt disabled."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(TAMP3IE_A::B_0x0)
    }
    #[doc = "Tamper 3 interrupt enabled.."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(TAMP3IE_A::B_0x1)
    }
}
#[doc = "Internal tamper 3 interrupt enable: LSE monitoring\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ITAMP3IE_A {
    #[doc = "0: Internal tamper 3 interrupt disabled."]
    B_0x0 = 0,
    #[doc = "1: Internal tamper 3 interrupt enabled."]
    B_0x1 = 1,
}
impl From<ITAMP3IE_A> for bool {
    #[inline(always)]
    fn from(variant: ITAMP3IE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ITAMP3IE` reader - Internal tamper 3 interrupt enable: LSE monitoring"]
pub type ITAMP3IE_R = crate::BitReader<ITAMP3IE_A>;
impl ITAMP3IE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ITAMP3IE_A {
        match self.bits {
            false => ITAMP3IE_A::B_0x0,
            true => ITAMP3IE_A::B_0x1,
        }
    }
    #[doc = "Internal tamper 3 interrupt disabled."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == ITAMP3IE_A::B_0x0
    }
    #[doc = "Internal tamper 3 interrupt enabled."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == ITAMP3IE_A::B_0x1
    }
}
#[doc = "Field `ITAMP3IE` writer - Internal tamper 3 interrupt enable: LSE monitoring"]
pub type ITAMP3IE_W<'a, REG> = crate::BitWriter<'a, REG, ITAMP3IE_A>;
impl<'a, REG> ITAMP3IE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Internal tamper 3 interrupt disabled."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(ITAMP3IE_A::B_0x0)
    }
    #[doc = "Internal tamper 3 interrupt enabled."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(ITAMP3IE_A::B_0x1)
    }
}
#[doc = "Internal tamper 4 interrupt enable: HSE monitoring\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ITAMP4IE_A {
    #[doc = "0: Internal tamper 4 interrupt disabled."]
    B_0x0 = 0,
    #[doc = "1: Internal tamper 4 interrupt enabled."]
    B_0x1 = 1,
}
impl From<ITAMP4IE_A> for bool {
    #[inline(always)]
    fn from(variant: ITAMP4IE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ITAMP4IE` reader - Internal tamper 4 interrupt enable: HSE monitoring"]
pub type ITAMP4IE_R = crate::BitReader<ITAMP4IE_A>;
impl ITAMP4IE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ITAMP4IE_A {
        match self.bits {
            false => ITAMP4IE_A::B_0x0,
            true => ITAMP4IE_A::B_0x1,
        }
    }
    #[doc = "Internal tamper 4 interrupt disabled."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == ITAMP4IE_A::B_0x0
    }
    #[doc = "Internal tamper 4 interrupt enabled."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == ITAMP4IE_A::B_0x1
    }
}
#[doc = "Field `ITAMP4IE` writer - Internal tamper 4 interrupt enable: HSE monitoring"]
pub type ITAMP4IE_W<'a, REG> = crate::BitWriter<'a, REG, ITAMP4IE_A>;
impl<'a, REG> ITAMP4IE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Internal tamper 4 interrupt disabled."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(ITAMP4IE_A::B_0x0)
    }
    #[doc = "Internal tamper 4 interrupt enabled."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(ITAMP4IE_A::B_0x1)
    }
}
#[doc = "Internal tamper 5 interrupt enable: RTC calendar overflow\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ITAMP5IE_A {
    #[doc = "0: Internal tamper 5 interrupt disabled."]
    B_0x0 = 0,
    #[doc = "1: Internal tamper 5 interrupt enabled."]
    B_0x1 = 1,
}
impl From<ITAMP5IE_A> for bool {
    #[inline(always)]
    fn from(variant: ITAMP5IE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ITAMP5IE` reader - Internal tamper 5 interrupt enable: RTC calendar overflow"]
pub type ITAMP5IE_R = crate::BitReader<ITAMP5IE_A>;
impl ITAMP5IE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ITAMP5IE_A {
        match self.bits {
            false => ITAMP5IE_A::B_0x0,
            true => ITAMP5IE_A::B_0x1,
        }
    }
    #[doc = "Internal tamper 5 interrupt disabled."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == ITAMP5IE_A::B_0x0
    }
    #[doc = "Internal tamper 5 interrupt enabled."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == ITAMP5IE_A::B_0x1
    }
}
#[doc = "Field `ITAMP5IE` writer - Internal tamper 5 interrupt enable: RTC calendar overflow"]
pub type ITAMP5IE_W<'a, REG> = crate::BitWriter<'a, REG, ITAMP5IE_A>;
impl<'a, REG> ITAMP5IE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Internal tamper 5 interrupt disabled."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(ITAMP5IE_A::B_0x0)
    }
    #[doc = "Internal tamper 5 interrupt enabled."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(ITAMP5IE_A::B_0x1)
    }
}
#[doc = "Internal tamper 6 interrupt enable: ST manufacturer readout\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ITAMP6IE_A {
    #[doc = "0: Internal tamper 6 interrupt disabled."]
    B_0x0 = 0,
    #[doc = "1: Internal tamper 6 interrupt enabled."]
    B_0x1 = 1,
}
impl From<ITAMP6IE_A> for bool {
    #[inline(always)]
    fn from(variant: ITAMP6IE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ITAMP6IE` reader - Internal tamper 6 interrupt enable: ST manufacturer readout"]
pub type ITAMP6IE_R = crate::BitReader<ITAMP6IE_A>;
impl ITAMP6IE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ITAMP6IE_A {
        match self.bits {
            false => ITAMP6IE_A::B_0x0,
            true => ITAMP6IE_A::B_0x1,
        }
    }
    #[doc = "Internal tamper 6 interrupt disabled."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == ITAMP6IE_A::B_0x0
    }
    #[doc = "Internal tamper 6 interrupt enabled."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == ITAMP6IE_A::B_0x1
    }
}
#[doc = "Field `ITAMP6IE` writer - Internal tamper 6 interrupt enable: ST manufacturer readout"]
pub type ITAMP6IE_W<'a, REG> = crate::BitWriter<'a, REG, ITAMP6IE_A>;
impl<'a, REG> ITAMP6IE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Internal tamper 6 interrupt disabled."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(ITAMP6IE_A::B_0x0)
    }
    #[doc = "Internal tamper 6 interrupt enabled."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(ITAMP6IE_A::B_0x1)
    }
}
impl R {
    #[doc = "Bit 0 - Tamper 1 interrupt enable"]
    #[inline(always)]
    pub fn TAMP1IE(&self) -> TAMP1IE_R {
        TAMP1IE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Tamper 2 interrupt enable"]
    #[inline(always)]
    pub fn TAMP2IE(&self) -> TAMP2IE_R {
        TAMP2IE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Tamper 3 interrupt enable"]
    #[inline(always)]
    pub fn TAMP3IE(&self) -> TAMP3IE_R {
        TAMP3IE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 18 - Internal tamper 3 interrupt enable: LSE monitoring"]
    #[inline(always)]
    pub fn ITAMP3IE(&self) -> ITAMP3IE_R {
        ITAMP3IE_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Internal tamper 4 interrupt enable: HSE monitoring"]
    #[inline(always)]
    pub fn ITAMP4IE(&self) -> ITAMP4IE_R {
        ITAMP4IE_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Internal tamper 5 interrupt enable: RTC calendar overflow"]
    #[inline(always)]
    pub fn ITAMP5IE(&self) -> ITAMP5IE_R {
        ITAMP5IE_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Internal tamper 6 interrupt enable: ST manufacturer readout"]
    #[inline(always)]
    pub fn ITAMP6IE(&self) -> ITAMP6IE_R {
        ITAMP6IE_R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Tamper 1 interrupt enable"]
    #[inline(always)]
    pub fn TAMP1IE(&mut self) -> TAMP1IE_W<'_, IER_SPEC> {
        TAMP1IE_W::new(self, 0)
    }
    #[doc = "Bit 1 - Tamper 2 interrupt enable"]
    #[inline(always)]
    pub fn TAMP2IE(&mut self) -> TAMP2IE_W<'_, IER_SPEC> {
        TAMP2IE_W::new(self, 1)
    }
    #[doc = "Bit 2 - Tamper 3 interrupt enable"]
    #[inline(always)]
    pub fn TAMP3IE(&mut self) -> TAMP3IE_W<'_, IER_SPEC> {
        TAMP3IE_W::new(self, 2)
    }
    #[doc = "Bit 18 - Internal tamper 3 interrupt enable: LSE monitoring"]
    #[inline(always)]
    pub fn ITAMP3IE(&mut self) -> ITAMP3IE_W<'_, IER_SPEC> {
        ITAMP3IE_W::new(self, 18)
    }
    #[doc = "Bit 19 - Internal tamper 4 interrupt enable: HSE monitoring"]
    #[inline(always)]
    pub fn ITAMP4IE(&mut self) -> ITAMP4IE_W<'_, IER_SPEC> {
        ITAMP4IE_W::new(self, 19)
    }
    #[doc = "Bit 20 - Internal tamper 5 interrupt enable: RTC calendar overflow"]
    #[inline(always)]
    pub fn ITAMP5IE(&mut self) -> ITAMP5IE_W<'_, IER_SPEC> {
        ITAMP5IE_W::new(self, 20)
    }
    #[doc = "Bit 21 - Internal tamper 6 interrupt enable: ST manufacturer readout"]
    #[inline(always)]
    pub fn ITAMP6IE(&mut self) -> ITAMP6IE_W<'_, IER_SPEC> {
        ITAMP6IE_W::new(self, 21)
    }
}
#[doc = "TAMP interrupt enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`ier::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ier::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IER_SPEC;
impl crate::RegisterSpec for IER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ier::R`](R) reader structure"]
impl crate::Readable for IER_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ier::W`](W) writer structure"]
impl crate::Writable for IER_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets IER to value 0"]
impl crate::Resettable for IER_SPEC {}

#[doc = "Reader of register RADIO"]
pub type R = crate::R<u32, super::RADIO>;
#[doc = "Writer for register RADIO"]
pub type W = crate::W<u32, super::RADIO>;
#[doc = "Register RADIO `reset()`'s with value 0"]
impl crate::ResetValue for super::RADIO {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Configuration field for RAM block 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RAM0_A {
    #[doc = "0: Priority 0."]
    PRI0,
    #[doc = "2: Priority 2."]
    PRI2,
    #[doc = "4: Priority 4."]
    PRI4,
    #[doc = "6: Priority 6."]
    PRI6,
    #[doc = "8: Priority 8."]
    PRI8,
    #[doc = "10: Priority 10."]
    PRI10,
    #[doc = "12: Priority 12."]
    PRI12,
    #[doc = "14: Priority 14."]
    PRI14,
}
impl From<RAM0_A> for u8 {
    #[inline(always)]
    fn from(variant: RAM0_A) -> Self {
        match variant {
            RAM0_A::PRI0 => 0,
            RAM0_A::PRI2 => 2,
            RAM0_A::PRI4 => 4,
            RAM0_A::PRI6 => 6,
            RAM0_A::PRI8 => 8,
            RAM0_A::PRI10 => 10,
            RAM0_A::PRI12 => 12,
            RAM0_A::PRI14 => 14,
        }
    }
}
#[doc = "Reader of field `RAM0`"]
pub type RAM0_R = crate::R<u8, RAM0_A>;
impl RAM0_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, RAM0_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(RAM0_A::PRI0),
            2 => Val(RAM0_A::PRI2),
            4 => Val(RAM0_A::PRI4),
            6 => Val(RAM0_A::PRI6),
            8 => Val(RAM0_A::PRI8),
            10 => Val(RAM0_A::PRI10),
            12 => Val(RAM0_A::PRI12),
            14 => Val(RAM0_A::PRI14),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `PRI0`"]
    #[inline(always)]
    pub fn is_pri0(&self) -> bool {
        *self == RAM0_A::PRI0
    }
    #[doc = "Checks if the value of the field is `PRI2`"]
    #[inline(always)]
    pub fn is_pri2(&self) -> bool {
        *self == RAM0_A::PRI2
    }
    #[doc = "Checks if the value of the field is `PRI4`"]
    #[inline(always)]
    pub fn is_pri4(&self) -> bool {
        *self == RAM0_A::PRI4
    }
    #[doc = "Checks if the value of the field is `PRI6`"]
    #[inline(always)]
    pub fn is_pri6(&self) -> bool {
        *self == RAM0_A::PRI6
    }
    #[doc = "Checks if the value of the field is `PRI8`"]
    #[inline(always)]
    pub fn is_pri8(&self) -> bool {
        *self == RAM0_A::PRI8
    }
    #[doc = "Checks if the value of the field is `PRI10`"]
    #[inline(always)]
    pub fn is_pri10(&self) -> bool {
        *self == RAM0_A::PRI10
    }
    #[doc = "Checks if the value of the field is `PRI12`"]
    #[inline(always)]
    pub fn is_pri12(&self) -> bool {
        *self == RAM0_A::PRI12
    }
    #[doc = "Checks if the value of the field is `PRI14`"]
    #[inline(always)]
    pub fn is_pri14(&self) -> bool {
        *self == RAM0_A::PRI14
    }
}
#[doc = "Write proxy for field `RAM0`"]
pub struct RAM0_W<'a> {
    w: &'a mut W,
}
impl<'a> RAM0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RAM0_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Priority 0."]
    #[inline(always)]
    pub fn pri0(self) -> &'a mut W {
        self.variant(RAM0_A::PRI0)
    }
    #[doc = "Priority 2."]
    #[inline(always)]
    pub fn pri2(self) -> &'a mut W {
        self.variant(RAM0_A::PRI2)
    }
    #[doc = "Priority 4."]
    #[inline(always)]
    pub fn pri4(self) -> &'a mut W {
        self.variant(RAM0_A::PRI4)
    }
    #[doc = "Priority 6."]
    #[inline(always)]
    pub fn pri6(self) -> &'a mut W {
        self.variant(RAM0_A::PRI6)
    }
    #[doc = "Priority 8."]
    #[inline(always)]
    pub fn pri8(self) -> &'a mut W {
        self.variant(RAM0_A::PRI8)
    }
    #[doc = "Priority 10."]
    #[inline(always)]
    pub fn pri10(self) -> &'a mut W {
        self.variant(RAM0_A::PRI10)
    }
    #[doc = "Priority 12."]
    #[inline(always)]
    pub fn pri12(self) -> &'a mut W {
        self.variant(RAM0_A::PRI12)
    }
    #[doc = "Priority 14."]
    #[inline(always)]
    pub fn pri14(self) -> &'a mut W {
        self.variant(RAM0_A::PRI14)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
#[doc = "Configuration field for RAM block 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RAM1_A {
    #[doc = "0: Priority 0."]
    PRI0,
    #[doc = "2: Priority 2."]
    PRI2,
    #[doc = "4: Priority 4."]
    PRI4,
    #[doc = "6: Priority 6."]
    PRI6,
    #[doc = "8: Priority 8."]
    PRI8,
    #[doc = "10: Priority 10."]
    PRI10,
    #[doc = "12: Priority 12."]
    PRI12,
    #[doc = "14: Priority 14."]
    PRI14,
}
impl From<RAM1_A> for u8 {
    #[inline(always)]
    fn from(variant: RAM1_A) -> Self {
        match variant {
            RAM1_A::PRI0 => 0,
            RAM1_A::PRI2 => 2,
            RAM1_A::PRI4 => 4,
            RAM1_A::PRI6 => 6,
            RAM1_A::PRI8 => 8,
            RAM1_A::PRI10 => 10,
            RAM1_A::PRI12 => 12,
            RAM1_A::PRI14 => 14,
        }
    }
}
#[doc = "Reader of field `RAM1`"]
pub type RAM1_R = crate::R<u8, RAM1_A>;
impl RAM1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, RAM1_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(RAM1_A::PRI0),
            2 => Val(RAM1_A::PRI2),
            4 => Val(RAM1_A::PRI4),
            6 => Val(RAM1_A::PRI6),
            8 => Val(RAM1_A::PRI8),
            10 => Val(RAM1_A::PRI10),
            12 => Val(RAM1_A::PRI12),
            14 => Val(RAM1_A::PRI14),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `PRI0`"]
    #[inline(always)]
    pub fn is_pri0(&self) -> bool {
        *self == RAM1_A::PRI0
    }
    #[doc = "Checks if the value of the field is `PRI2`"]
    #[inline(always)]
    pub fn is_pri2(&self) -> bool {
        *self == RAM1_A::PRI2
    }
    #[doc = "Checks if the value of the field is `PRI4`"]
    #[inline(always)]
    pub fn is_pri4(&self) -> bool {
        *self == RAM1_A::PRI4
    }
    #[doc = "Checks if the value of the field is `PRI6`"]
    #[inline(always)]
    pub fn is_pri6(&self) -> bool {
        *self == RAM1_A::PRI6
    }
    #[doc = "Checks if the value of the field is `PRI8`"]
    #[inline(always)]
    pub fn is_pri8(&self) -> bool {
        *self == RAM1_A::PRI8
    }
    #[doc = "Checks if the value of the field is `PRI10`"]
    #[inline(always)]
    pub fn is_pri10(&self) -> bool {
        *self == RAM1_A::PRI10
    }
    #[doc = "Checks if the value of the field is `PRI12`"]
    #[inline(always)]
    pub fn is_pri12(&self) -> bool {
        *self == RAM1_A::PRI12
    }
    #[doc = "Checks if the value of the field is `PRI14`"]
    #[inline(always)]
    pub fn is_pri14(&self) -> bool {
        *self == RAM1_A::PRI14
    }
}
#[doc = "Write proxy for field `RAM1`"]
pub struct RAM1_W<'a> {
    w: &'a mut W,
}
impl<'a> RAM1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RAM1_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Priority 0."]
    #[inline(always)]
    pub fn pri0(self) -> &'a mut W {
        self.variant(RAM1_A::PRI0)
    }
    #[doc = "Priority 2."]
    #[inline(always)]
    pub fn pri2(self) -> &'a mut W {
        self.variant(RAM1_A::PRI2)
    }
    #[doc = "Priority 4."]
    #[inline(always)]
    pub fn pri4(self) -> &'a mut W {
        self.variant(RAM1_A::PRI4)
    }
    #[doc = "Priority 6."]
    #[inline(always)]
    pub fn pri6(self) -> &'a mut W {
        self.variant(RAM1_A::PRI6)
    }
    #[doc = "Priority 8."]
    #[inline(always)]
    pub fn pri8(self) -> &'a mut W {
        self.variant(RAM1_A::PRI8)
    }
    #[doc = "Priority 10."]
    #[inline(always)]
    pub fn pri10(self) -> &'a mut W {
        self.variant(RAM1_A::PRI10)
    }
    #[doc = "Priority 12."]
    #[inline(always)]
    pub fn pri12(self) -> &'a mut W {
        self.variant(RAM1_A::PRI12)
    }
    #[doc = "Priority 14."]
    #[inline(always)]
    pub fn pri14(self) -> &'a mut W {
        self.variant(RAM1_A::PRI14)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | (((value as u32) & 0x0f) << 4);
        self.w
    }
}
#[doc = "Configuration field for RAM block 2.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RAM2_A {
    #[doc = "0: Priority 0."]
    PRI0,
    #[doc = "2: Priority 2."]
    PRI2,
    #[doc = "4: Priority 4."]
    PRI4,
    #[doc = "6: Priority 6."]
    PRI6,
    #[doc = "8: Priority 8."]
    PRI8,
    #[doc = "10: Priority 10."]
    PRI10,
    #[doc = "12: Priority 12."]
    PRI12,
    #[doc = "14: Priority 14."]
    PRI14,
}
impl From<RAM2_A> for u8 {
    #[inline(always)]
    fn from(variant: RAM2_A) -> Self {
        match variant {
            RAM2_A::PRI0 => 0,
            RAM2_A::PRI2 => 2,
            RAM2_A::PRI4 => 4,
            RAM2_A::PRI6 => 6,
            RAM2_A::PRI8 => 8,
            RAM2_A::PRI10 => 10,
            RAM2_A::PRI12 => 12,
            RAM2_A::PRI14 => 14,
        }
    }
}
#[doc = "Reader of field `RAM2`"]
pub type RAM2_R = crate::R<u8, RAM2_A>;
impl RAM2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, RAM2_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(RAM2_A::PRI0),
            2 => Val(RAM2_A::PRI2),
            4 => Val(RAM2_A::PRI4),
            6 => Val(RAM2_A::PRI6),
            8 => Val(RAM2_A::PRI8),
            10 => Val(RAM2_A::PRI10),
            12 => Val(RAM2_A::PRI12),
            14 => Val(RAM2_A::PRI14),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `PRI0`"]
    #[inline(always)]
    pub fn is_pri0(&self) -> bool {
        *self == RAM2_A::PRI0
    }
    #[doc = "Checks if the value of the field is `PRI2`"]
    #[inline(always)]
    pub fn is_pri2(&self) -> bool {
        *self == RAM2_A::PRI2
    }
    #[doc = "Checks if the value of the field is `PRI4`"]
    #[inline(always)]
    pub fn is_pri4(&self) -> bool {
        *self == RAM2_A::PRI4
    }
    #[doc = "Checks if the value of the field is `PRI6`"]
    #[inline(always)]
    pub fn is_pri6(&self) -> bool {
        *self == RAM2_A::PRI6
    }
    #[doc = "Checks if the value of the field is `PRI8`"]
    #[inline(always)]
    pub fn is_pri8(&self) -> bool {
        *self == RAM2_A::PRI8
    }
    #[doc = "Checks if the value of the field is `PRI10`"]
    #[inline(always)]
    pub fn is_pri10(&self) -> bool {
        *self == RAM2_A::PRI10
    }
    #[doc = "Checks if the value of the field is `PRI12`"]
    #[inline(always)]
    pub fn is_pri12(&self) -> bool {
        *self == RAM2_A::PRI12
    }
    #[doc = "Checks if the value of the field is `PRI14`"]
    #[inline(always)]
    pub fn is_pri14(&self) -> bool {
        *self == RAM2_A::PRI14
    }
}
#[doc = "Write proxy for field `RAM2`"]
pub struct RAM2_W<'a> {
    w: &'a mut W,
}
impl<'a> RAM2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RAM2_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Priority 0."]
    #[inline(always)]
    pub fn pri0(self) -> &'a mut W {
        self.variant(RAM2_A::PRI0)
    }
    #[doc = "Priority 2."]
    #[inline(always)]
    pub fn pri2(self) -> &'a mut W {
        self.variant(RAM2_A::PRI2)
    }
    #[doc = "Priority 4."]
    #[inline(always)]
    pub fn pri4(self) -> &'a mut W {
        self.variant(RAM2_A::PRI4)
    }
    #[doc = "Priority 6."]
    #[inline(always)]
    pub fn pri6(self) -> &'a mut W {
        self.variant(RAM2_A::PRI6)
    }
    #[doc = "Priority 8."]
    #[inline(always)]
    pub fn pri8(self) -> &'a mut W {
        self.variant(RAM2_A::PRI8)
    }
    #[doc = "Priority 10."]
    #[inline(always)]
    pub fn pri10(self) -> &'a mut W {
        self.variant(RAM2_A::PRI10)
    }
    #[doc = "Priority 12."]
    #[inline(always)]
    pub fn pri12(self) -> &'a mut W {
        self.variant(RAM2_A::PRI12)
    }
    #[doc = "Priority 14."]
    #[inline(always)]
    pub fn pri14(self) -> &'a mut W {
        self.variant(RAM2_A::PRI14)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
#[doc = "Configuration field for RAM block 3.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RAM3_A {
    #[doc = "0: Priority 0."]
    PRI0,
    #[doc = "2: Priority 2."]
    PRI2,
    #[doc = "4: Priority 4."]
    PRI4,
    #[doc = "6: Priority 6."]
    PRI6,
    #[doc = "8: Priority 8."]
    PRI8,
    #[doc = "10: Priority 10."]
    PRI10,
    #[doc = "12: Priority 12."]
    PRI12,
    #[doc = "14: Priority 14."]
    PRI14,
}
impl From<RAM3_A> for u8 {
    #[inline(always)]
    fn from(variant: RAM3_A) -> Self {
        match variant {
            RAM3_A::PRI0 => 0,
            RAM3_A::PRI2 => 2,
            RAM3_A::PRI4 => 4,
            RAM3_A::PRI6 => 6,
            RAM3_A::PRI8 => 8,
            RAM3_A::PRI10 => 10,
            RAM3_A::PRI12 => 12,
            RAM3_A::PRI14 => 14,
        }
    }
}
#[doc = "Reader of field `RAM3`"]
pub type RAM3_R = crate::R<u8, RAM3_A>;
impl RAM3_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, RAM3_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(RAM3_A::PRI0),
            2 => Val(RAM3_A::PRI2),
            4 => Val(RAM3_A::PRI4),
            6 => Val(RAM3_A::PRI6),
            8 => Val(RAM3_A::PRI8),
            10 => Val(RAM3_A::PRI10),
            12 => Val(RAM3_A::PRI12),
            14 => Val(RAM3_A::PRI14),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `PRI0`"]
    #[inline(always)]
    pub fn is_pri0(&self) -> bool {
        *self == RAM3_A::PRI0
    }
    #[doc = "Checks if the value of the field is `PRI2`"]
    #[inline(always)]
    pub fn is_pri2(&self) -> bool {
        *self == RAM3_A::PRI2
    }
    #[doc = "Checks if the value of the field is `PRI4`"]
    #[inline(always)]
    pub fn is_pri4(&self) -> bool {
        *self == RAM3_A::PRI4
    }
    #[doc = "Checks if the value of the field is `PRI6`"]
    #[inline(always)]
    pub fn is_pri6(&self) -> bool {
        *self == RAM3_A::PRI6
    }
    #[doc = "Checks if the value of the field is `PRI8`"]
    #[inline(always)]
    pub fn is_pri8(&self) -> bool {
        *self == RAM3_A::PRI8
    }
    #[doc = "Checks if the value of the field is `PRI10`"]
    #[inline(always)]
    pub fn is_pri10(&self) -> bool {
        *self == RAM3_A::PRI10
    }
    #[doc = "Checks if the value of the field is `PRI12`"]
    #[inline(always)]
    pub fn is_pri12(&self) -> bool {
        *self == RAM3_A::PRI12
    }
    #[doc = "Checks if the value of the field is `PRI14`"]
    #[inline(always)]
    pub fn is_pri14(&self) -> bool {
        *self == RAM3_A::PRI14
    }
}
#[doc = "Write proxy for field `RAM3`"]
pub struct RAM3_W<'a> {
    w: &'a mut W,
}
impl<'a> RAM3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RAM3_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Priority 0."]
    #[inline(always)]
    pub fn pri0(self) -> &'a mut W {
        self.variant(RAM3_A::PRI0)
    }
    #[doc = "Priority 2."]
    #[inline(always)]
    pub fn pri2(self) -> &'a mut W {
        self.variant(RAM3_A::PRI2)
    }
    #[doc = "Priority 4."]
    #[inline(always)]
    pub fn pri4(self) -> &'a mut W {
        self.variant(RAM3_A::PRI4)
    }
    #[doc = "Priority 6."]
    #[inline(always)]
    pub fn pri6(self) -> &'a mut W {
        self.variant(RAM3_A::PRI6)
    }
    #[doc = "Priority 8."]
    #[inline(always)]
    pub fn pri8(self) -> &'a mut W {
        self.variant(RAM3_A::PRI8)
    }
    #[doc = "Priority 10."]
    #[inline(always)]
    pub fn pri10(self) -> &'a mut W {
        self.variant(RAM3_A::PRI10)
    }
    #[doc = "Priority 12."]
    #[inline(always)]
    pub fn pri12(self) -> &'a mut W {
        self.variant(RAM3_A::PRI12)
    }
    #[doc = "Priority 14."]
    #[inline(always)]
    pub fn pri14(self) -> &'a mut W {
        self.variant(RAM3_A::PRI14)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | (((value as u32) & 0x0f) << 12);
        self.w
    }
}
#[doc = "Configuration field for RAM block 4.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RAM4_A {
    #[doc = "0: Priority 0."]
    PRI0,
    #[doc = "2: Priority 2."]
    PRI2,
    #[doc = "4: Priority 4."]
    PRI4,
    #[doc = "6: Priority 6."]
    PRI6,
    #[doc = "8: Priority 8."]
    PRI8,
    #[doc = "10: Priority 10."]
    PRI10,
    #[doc = "12: Priority 12."]
    PRI12,
    #[doc = "14: Priority 14."]
    PRI14,
}
impl From<RAM4_A> for u8 {
    #[inline(always)]
    fn from(variant: RAM4_A) -> Self {
        match variant {
            RAM4_A::PRI0 => 0,
            RAM4_A::PRI2 => 2,
            RAM4_A::PRI4 => 4,
            RAM4_A::PRI6 => 6,
            RAM4_A::PRI8 => 8,
            RAM4_A::PRI10 => 10,
            RAM4_A::PRI12 => 12,
            RAM4_A::PRI14 => 14,
        }
    }
}
#[doc = "Reader of field `RAM4`"]
pub type RAM4_R = crate::R<u8, RAM4_A>;
impl RAM4_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, RAM4_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(RAM4_A::PRI0),
            2 => Val(RAM4_A::PRI2),
            4 => Val(RAM4_A::PRI4),
            6 => Val(RAM4_A::PRI6),
            8 => Val(RAM4_A::PRI8),
            10 => Val(RAM4_A::PRI10),
            12 => Val(RAM4_A::PRI12),
            14 => Val(RAM4_A::PRI14),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `PRI0`"]
    #[inline(always)]
    pub fn is_pri0(&self) -> bool {
        *self == RAM4_A::PRI0
    }
    #[doc = "Checks if the value of the field is `PRI2`"]
    #[inline(always)]
    pub fn is_pri2(&self) -> bool {
        *self == RAM4_A::PRI2
    }
    #[doc = "Checks if the value of the field is `PRI4`"]
    #[inline(always)]
    pub fn is_pri4(&self) -> bool {
        *self == RAM4_A::PRI4
    }
    #[doc = "Checks if the value of the field is `PRI6`"]
    #[inline(always)]
    pub fn is_pri6(&self) -> bool {
        *self == RAM4_A::PRI6
    }
    #[doc = "Checks if the value of the field is `PRI8`"]
    #[inline(always)]
    pub fn is_pri8(&self) -> bool {
        *self == RAM4_A::PRI8
    }
    #[doc = "Checks if the value of the field is `PRI10`"]
    #[inline(always)]
    pub fn is_pri10(&self) -> bool {
        *self == RAM4_A::PRI10
    }
    #[doc = "Checks if the value of the field is `PRI12`"]
    #[inline(always)]
    pub fn is_pri12(&self) -> bool {
        *self == RAM4_A::PRI12
    }
    #[doc = "Checks if the value of the field is `PRI14`"]
    #[inline(always)]
    pub fn is_pri14(&self) -> bool {
        *self == RAM4_A::PRI14
    }
}
#[doc = "Write proxy for field `RAM4`"]
pub struct RAM4_W<'a> {
    w: &'a mut W,
}
impl<'a> RAM4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RAM4_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Priority 0."]
    #[inline(always)]
    pub fn pri0(self) -> &'a mut W {
        self.variant(RAM4_A::PRI0)
    }
    #[doc = "Priority 2."]
    #[inline(always)]
    pub fn pri2(self) -> &'a mut W {
        self.variant(RAM4_A::PRI2)
    }
    #[doc = "Priority 4."]
    #[inline(always)]
    pub fn pri4(self) -> &'a mut W {
        self.variant(RAM4_A::PRI4)
    }
    #[doc = "Priority 6."]
    #[inline(always)]
    pub fn pri6(self) -> &'a mut W {
        self.variant(RAM4_A::PRI6)
    }
    #[doc = "Priority 8."]
    #[inline(always)]
    pub fn pri8(self) -> &'a mut W {
        self.variant(RAM4_A::PRI8)
    }
    #[doc = "Priority 10."]
    #[inline(always)]
    pub fn pri10(self) -> &'a mut W {
        self.variant(RAM4_A::PRI10)
    }
    #[doc = "Priority 12."]
    #[inline(always)]
    pub fn pri12(self) -> &'a mut W {
        self.variant(RAM4_A::PRI12)
    }
    #[doc = "Priority 14."]
    #[inline(always)]
    pub fn pri14(self) -> &'a mut W {
        self.variant(RAM4_A::PRI14)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
        self.w
    }
}
#[doc = "Configuration field for RAM block 5.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RAM5_A {
    #[doc = "0: Priority 0."]
    PRI0,
    #[doc = "2: Priority 2."]
    PRI2,
    #[doc = "4: Priority 4."]
    PRI4,
    #[doc = "6: Priority 6."]
    PRI6,
    #[doc = "8: Priority 8."]
    PRI8,
    #[doc = "10: Priority 10."]
    PRI10,
    #[doc = "12: Priority 12."]
    PRI12,
    #[doc = "14: Priority 14."]
    PRI14,
}
impl From<RAM5_A> for u8 {
    #[inline(always)]
    fn from(variant: RAM5_A) -> Self {
        match variant {
            RAM5_A::PRI0 => 0,
            RAM5_A::PRI2 => 2,
            RAM5_A::PRI4 => 4,
            RAM5_A::PRI6 => 6,
            RAM5_A::PRI8 => 8,
            RAM5_A::PRI10 => 10,
            RAM5_A::PRI12 => 12,
            RAM5_A::PRI14 => 14,
        }
    }
}
#[doc = "Reader of field `RAM5`"]
pub type RAM5_R = crate::R<u8, RAM5_A>;
impl RAM5_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, RAM5_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(RAM5_A::PRI0),
            2 => Val(RAM5_A::PRI2),
            4 => Val(RAM5_A::PRI4),
            6 => Val(RAM5_A::PRI6),
            8 => Val(RAM5_A::PRI8),
            10 => Val(RAM5_A::PRI10),
            12 => Val(RAM5_A::PRI12),
            14 => Val(RAM5_A::PRI14),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `PRI0`"]
    #[inline(always)]
    pub fn is_pri0(&self) -> bool {
        *self == RAM5_A::PRI0
    }
    #[doc = "Checks if the value of the field is `PRI2`"]
    #[inline(always)]
    pub fn is_pri2(&self) -> bool {
        *self == RAM5_A::PRI2
    }
    #[doc = "Checks if the value of the field is `PRI4`"]
    #[inline(always)]
    pub fn is_pri4(&self) -> bool {
        *self == RAM5_A::PRI4
    }
    #[doc = "Checks if the value of the field is `PRI6`"]
    #[inline(always)]
    pub fn is_pri6(&self) -> bool {
        *self == RAM5_A::PRI6
    }
    #[doc = "Checks if the value of the field is `PRI8`"]
    #[inline(always)]
    pub fn is_pri8(&self) -> bool {
        *self == RAM5_A::PRI8
    }
    #[doc = "Checks if the value of the field is `PRI10`"]
    #[inline(always)]
    pub fn is_pri10(&self) -> bool {
        *self == RAM5_A::PRI10
    }
    #[doc = "Checks if the value of the field is `PRI12`"]
    #[inline(always)]
    pub fn is_pri12(&self) -> bool {
        *self == RAM5_A::PRI12
    }
    #[doc = "Checks if the value of the field is `PRI14`"]
    #[inline(always)]
    pub fn is_pri14(&self) -> bool {
        *self == RAM5_A::PRI14
    }
}
#[doc = "Write proxy for field `RAM5`"]
pub struct RAM5_W<'a> {
    w: &'a mut W,
}
impl<'a> RAM5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RAM5_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Priority 0."]
    #[inline(always)]
    pub fn pri0(self) -> &'a mut W {
        self.variant(RAM5_A::PRI0)
    }
    #[doc = "Priority 2."]
    #[inline(always)]
    pub fn pri2(self) -> &'a mut W {
        self.variant(RAM5_A::PRI2)
    }
    #[doc = "Priority 4."]
    #[inline(always)]
    pub fn pri4(self) -> &'a mut W {
        self.variant(RAM5_A::PRI4)
    }
    #[doc = "Priority 6."]
    #[inline(always)]
    pub fn pri6(self) -> &'a mut W {
        self.variant(RAM5_A::PRI6)
    }
    #[doc = "Priority 8."]
    #[inline(always)]
    pub fn pri8(self) -> &'a mut W {
        self.variant(RAM5_A::PRI8)
    }
    #[doc = "Priority 10."]
    #[inline(always)]
    pub fn pri10(self) -> &'a mut W {
        self.variant(RAM5_A::PRI10)
    }
    #[doc = "Priority 12."]
    #[inline(always)]
    pub fn pri12(self) -> &'a mut W {
        self.variant(RAM5_A::PRI12)
    }
    #[doc = "Priority 14."]
    #[inline(always)]
    pub fn pri14(self) -> &'a mut W {
        self.variant(RAM5_A::PRI14)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 20)) | (((value as u32) & 0x0f) << 20);
        self.w
    }
}
#[doc = "Configuration field for RAM block 6.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RAM6_A {
    #[doc = "0: Priority 0."]
    PRI0,
    #[doc = "2: Priority 2."]
    PRI2,
    #[doc = "4: Priority 4."]
    PRI4,
    #[doc = "6: Priority 6."]
    PRI6,
    #[doc = "8: Priority 8."]
    PRI8,
    #[doc = "10: Priority 10."]
    PRI10,
    #[doc = "12: Priority 12."]
    PRI12,
    #[doc = "14: Priority 14."]
    PRI14,
}
impl From<RAM6_A> for u8 {
    #[inline(always)]
    fn from(variant: RAM6_A) -> Self {
        match variant {
            RAM6_A::PRI0 => 0,
            RAM6_A::PRI2 => 2,
            RAM6_A::PRI4 => 4,
            RAM6_A::PRI6 => 6,
            RAM6_A::PRI8 => 8,
            RAM6_A::PRI10 => 10,
            RAM6_A::PRI12 => 12,
            RAM6_A::PRI14 => 14,
        }
    }
}
#[doc = "Reader of field `RAM6`"]
pub type RAM6_R = crate::R<u8, RAM6_A>;
impl RAM6_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, RAM6_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(RAM6_A::PRI0),
            2 => Val(RAM6_A::PRI2),
            4 => Val(RAM6_A::PRI4),
            6 => Val(RAM6_A::PRI6),
            8 => Val(RAM6_A::PRI8),
            10 => Val(RAM6_A::PRI10),
            12 => Val(RAM6_A::PRI12),
            14 => Val(RAM6_A::PRI14),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `PRI0`"]
    #[inline(always)]
    pub fn is_pri0(&self) -> bool {
        *self == RAM6_A::PRI0
    }
    #[doc = "Checks if the value of the field is `PRI2`"]
    #[inline(always)]
    pub fn is_pri2(&self) -> bool {
        *self == RAM6_A::PRI2
    }
    #[doc = "Checks if the value of the field is `PRI4`"]
    #[inline(always)]
    pub fn is_pri4(&self) -> bool {
        *self == RAM6_A::PRI4
    }
    #[doc = "Checks if the value of the field is `PRI6`"]
    #[inline(always)]
    pub fn is_pri6(&self) -> bool {
        *self == RAM6_A::PRI6
    }
    #[doc = "Checks if the value of the field is `PRI8`"]
    #[inline(always)]
    pub fn is_pri8(&self) -> bool {
        *self == RAM6_A::PRI8
    }
    #[doc = "Checks if the value of the field is `PRI10`"]
    #[inline(always)]
    pub fn is_pri10(&self) -> bool {
        *self == RAM6_A::PRI10
    }
    #[doc = "Checks if the value of the field is `PRI12`"]
    #[inline(always)]
    pub fn is_pri12(&self) -> bool {
        *self == RAM6_A::PRI12
    }
    #[doc = "Checks if the value of the field is `PRI14`"]
    #[inline(always)]
    pub fn is_pri14(&self) -> bool {
        *self == RAM6_A::PRI14
    }
}
#[doc = "Write proxy for field `RAM6`"]
pub struct RAM6_W<'a> {
    w: &'a mut W,
}
impl<'a> RAM6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RAM6_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Priority 0."]
    #[inline(always)]
    pub fn pri0(self) -> &'a mut W {
        self.variant(RAM6_A::PRI0)
    }
    #[doc = "Priority 2."]
    #[inline(always)]
    pub fn pri2(self) -> &'a mut W {
        self.variant(RAM6_A::PRI2)
    }
    #[doc = "Priority 4."]
    #[inline(always)]
    pub fn pri4(self) -> &'a mut W {
        self.variant(RAM6_A::PRI4)
    }
    #[doc = "Priority 6."]
    #[inline(always)]
    pub fn pri6(self) -> &'a mut W {
        self.variant(RAM6_A::PRI6)
    }
    #[doc = "Priority 8."]
    #[inline(always)]
    pub fn pri8(self) -> &'a mut W {
        self.variant(RAM6_A::PRI8)
    }
    #[doc = "Priority 10."]
    #[inline(always)]
    pub fn pri10(self) -> &'a mut W {
        self.variant(RAM6_A::PRI10)
    }
    #[doc = "Priority 12."]
    #[inline(always)]
    pub fn pri12(self) -> &'a mut W {
        self.variant(RAM6_A::PRI12)
    }
    #[doc = "Priority 14."]
    #[inline(always)]
    pub fn pri14(self) -> &'a mut W {
        self.variant(RAM6_A::PRI14)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | (((value as u32) & 0x0f) << 24);
        self.w
    }
}
#[doc = "Configuration field for RAM block 7.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RAM7_A {
    #[doc = "0: Priority 0."]
    PRI0,
    #[doc = "2: Priority 2."]
    PRI2,
    #[doc = "4: Priority 4."]
    PRI4,
    #[doc = "6: Priority 6."]
    PRI6,
    #[doc = "8: Priority 8."]
    PRI8,
    #[doc = "10: Priority 10."]
    PRI10,
    #[doc = "12: Priority 12."]
    PRI12,
    #[doc = "14: Priority 14."]
    PRI14,
}
impl From<RAM7_A> for u8 {
    #[inline(always)]
    fn from(variant: RAM7_A) -> Self {
        match variant {
            RAM7_A::PRI0 => 0,
            RAM7_A::PRI2 => 2,
            RAM7_A::PRI4 => 4,
            RAM7_A::PRI6 => 6,
            RAM7_A::PRI8 => 8,
            RAM7_A::PRI10 => 10,
            RAM7_A::PRI12 => 12,
            RAM7_A::PRI14 => 14,
        }
    }
}
#[doc = "Reader of field `RAM7`"]
pub type RAM7_R = crate::R<u8, RAM7_A>;
impl RAM7_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, RAM7_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(RAM7_A::PRI0),
            2 => Val(RAM7_A::PRI2),
            4 => Val(RAM7_A::PRI4),
            6 => Val(RAM7_A::PRI6),
            8 => Val(RAM7_A::PRI8),
            10 => Val(RAM7_A::PRI10),
            12 => Val(RAM7_A::PRI12),
            14 => Val(RAM7_A::PRI14),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `PRI0`"]
    #[inline(always)]
    pub fn is_pri0(&self) -> bool {
        *self == RAM7_A::PRI0
    }
    #[doc = "Checks if the value of the field is `PRI2`"]
    #[inline(always)]
    pub fn is_pri2(&self) -> bool {
        *self == RAM7_A::PRI2
    }
    #[doc = "Checks if the value of the field is `PRI4`"]
    #[inline(always)]
    pub fn is_pri4(&self) -> bool {
        *self == RAM7_A::PRI4
    }
    #[doc = "Checks if the value of the field is `PRI6`"]
    #[inline(always)]
    pub fn is_pri6(&self) -> bool {
        *self == RAM7_A::PRI6
    }
    #[doc = "Checks if the value of the field is `PRI8`"]
    #[inline(always)]
    pub fn is_pri8(&self) -> bool {
        *self == RAM7_A::PRI8
    }
    #[doc = "Checks if the value of the field is `PRI10`"]
    #[inline(always)]
    pub fn is_pri10(&self) -> bool {
        *self == RAM7_A::PRI10
    }
    #[doc = "Checks if the value of the field is `PRI12`"]
    #[inline(always)]
    pub fn is_pri12(&self) -> bool {
        *self == RAM7_A::PRI12
    }
    #[doc = "Checks if the value of the field is `PRI14`"]
    #[inline(always)]
    pub fn is_pri14(&self) -> bool {
        *self == RAM7_A::PRI14
    }
}
#[doc = "Write proxy for field `RAM7`"]
pub struct RAM7_W<'a> {
    w: &'a mut W,
}
impl<'a> RAM7_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RAM7_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Priority 0."]
    #[inline(always)]
    pub fn pri0(self) -> &'a mut W {
        self.variant(RAM7_A::PRI0)
    }
    #[doc = "Priority 2."]
    #[inline(always)]
    pub fn pri2(self) -> &'a mut W {
        self.variant(RAM7_A::PRI2)
    }
    #[doc = "Priority 4."]
    #[inline(always)]
    pub fn pri4(self) -> &'a mut W {
        self.variant(RAM7_A::PRI4)
    }
    #[doc = "Priority 6."]
    #[inline(always)]
    pub fn pri6(self) -> &'a mut W {
        self.variant(RAM7_A::PRI6)
    }
    #[doc = "Priority 8."]
    #[inline(always)]
    pub fn pri8(self) -> &'a mut W {
        self.variant(RAM7_A::PRI8)
    }
    #[doc = "Priority 10."]
    #[inline(always)]
    pub fn pri10(self) -> &'a mut W {
        self.variant(RAM7_A::PRI10)
    }
    #[doc = "Priority 12."]
    #[inline(always)]
    pub fn pri12(self) -> &'a mut W {
        self.variant(RAM7_A::PRI12)
    }
    #[doc = "Priority 14."]
    #[inline(always)]
    pub fn pri14(self) -> &'a mut W {
        self.variant(RAM7_A::PRI14)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 28)) | (((value as u32) & 0x0f) << 28);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Configuration field for RAM block 0."]
    #[inline(always)]
    pub fn ram0(&self) -> RAM0_R {
        RAM0_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Configuration field for RAM block 1."]
    #[inline(always)]
    pub fn ram1(&self) -> RAM1_R {
        RAM1_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Configuration field for RAM block 2."]
    #[inline(always)]
    pub fn ram2(&self) -> RAM2_R {
        RAM2_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Configuration field for RAM block 3."]
    #[inline(always)]
    pub fn ram3(&self) -> RAM3_R {
        RAM3_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Configuration field for RAM block 4."]
    #[inline(always)]
    pub fn ram4(&self) -> RAM4_R {
        RAM4_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - Configuration field for RAM block 5."]
    #[inline(always)]
    pub fn ram5(&self) -> RAM5_R {
        RAM5_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Configuration field for RAM block 6."]
    #[inline(always)]
    pub fn ram6(&self) -> RAM6_R {
        RAM6_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - Configuration field for RAM block 7."]
    #[inline(always)]
    pub fn ram7(&self) -> RAM7_R {
        RAM7_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Configuration field for RAM block 0."]
    #[inline(always)]
    pub fn ram0(&mut self) -> RAM0_W {
        RAM0_W { w: self }
    }
    #[doc = "Bits 4:7 - Configuration field for RAM block 1."]
    #[inline(always)]
    pub fn ram1(&mut self) -> RAM1_W {
        RAM1_W { w: self }
    }
    #[doc = "Bits 8:11 - Configuration field for RAM block 2."]
    #[inline(always)]
    pub fn ram2(&mut self) -> RAM2_W {
        RAM2_W { w: self }
    }
    #[doc = "Bits 12:15 - Configuration field for RAM block 3."]
    #[inline(always)]
    pub fn ram3(&mut self) -> RAM3_W {
        RAM3_W { w: self }
    }
    #[doc = "Bits 16:19 - Configuration field for RAM block 4."]
    #[inline(always)]
    pub fn ram4(&mut self) -> RAM4_W {
        RAM4_W { w: self }
    }
    #[doc = "Bits 20:23 - Configuration field for RAM block 5."]
    #[inline(always)]
    pub fn ram5(&mut self) -> RAM5_W {
        RAM5_W { w: self }
    }
    #[doc = "Bits 24:27 - Configuration field for RAM block 6."]
    #[inline(always)]
    pub fn ram6(&mut self) -> RAM6_W {
        RAM6_W { w: self }
    }
    #[doc = "Bits 28:31 - Configuration field for RAM block 7."]
    #[inline(always)]
    pub fn ram7(&mut self) -> RAM7_W {
        RAM7_W { w: self }
    }
}

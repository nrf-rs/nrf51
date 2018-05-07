#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::RADIO {
    #[doc = r" Modifies the contents of the register"]
    #[inline]
    pub fn modify<F>(&self, f: F)
    where
        for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
    {
        let bits = self.register.get();
        let r = R { bits: bits };
        let mut w = W { bits: bits };
        f(&r, &mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
    #[doc = r" Writes to the register"]
    #[inline]
    pub fn write<F>(&self, f: F)
    where
        F: FnOnce(&mut W) -> &mut W,
    {
        let mut w = W::reset_value();
        f(&mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Writes the reset value to the register"]
    #[inline]
    pub fn reset(&self) {
        self.write(|w| w)
    }
}
#[doc = "Possible values of the field `RAM0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RAM0R {
    #[doc = "Priority 0."]
    PRI0,
    #[doc = "Priority 2."]
    PRI2,
    #[doc = "Priority 4."]
    PRI4,
    #[doc = "Priority 6."]
    PRI6,
    #[doc = "Priority 8."]
    PRI8,
    #[doc = "Priority 10."]
    PRI10,
    #[doc = "Priority 12."]
    PRI12,
    #[doc = "Priority 14."]
    PRI14,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl RAM0R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            RAM0R::PRI0 => 0,
            RAM0R::PRI2 => 2,
            RAM0R::PRI4 => 4,
            RAM0R::PRI6 => 6,
            RAM0R::PRI8 => 8,
            RAM0R::PRI10 => 10,
            RAM0R::PRI12 => 12,
            RAM0R::PRI14 => 14,
            RAM0R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> RAM0R {
        match value {
            0 => RAM0R::PRI0,
            2 => RAM0R::PRI2,
            4 => RAM0R::PRI4,
            6 => RAM0R::PRI6,
            8 => RAM0R::PRI8,
            10 => RAM0R::PRI10,
            12 => RAM0R::PRI12,
            14 => RAM0R::PRI14,
            i => RAM0R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `PRI0`"]
    #[inline]
    pub fn is_pri0(&self) -> bool {
        *self == RAM0R::PRI0
    }
    #[doc = "Checks if the value of the field is `PRI2`"]
    #[inline]
    pub fn is_pri2(&self) -> bool {
        *self == RAM0R::PRI2
    }
    #[doc = "Checks if the value of the field is `PRI4`"]
    #[inline]
    pub fn is_pri4(&self) -> bool {
        *self == RAM0R::PRI4
    }
    #[doc = "Checks if the value of the field is `PRI6`"]
    #[inline]
    pub fn is_pri6(&self) -> bool {
        *self == RAM0R::PRI6
    }
    #[doc = "Checks if the value of the field is `PRI8`"]
    #[inline]
    pub fn is_pri8(&self) -> bool {
        *self == RAM0R::PRI8
    }
    #[doc = "Checks if the value of the field is `PRI10`"]
    #[inline]
    pub fn is_pri10(&self) -> bool {
        *self == RAM0R::PRI10
    }
    #[doc = "Checks if the value of the field is `PRI12`"]
    #[inline]
    pub fn is_pri12(&self) -> bool {
        *self == RAM0R::PRI12
    }
    #[doc = "Checks if the value of the field is `PRI14`"]
    #[inline]
    pub fn is_pri14(&self) -> bool {
        *self == RAM0R::PRI14
    }
}
#[doc = "Possible values of the field `RAM1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RAM1R {
    #[doc = "Priority 0."]
    PRI0,
    #[doc = "Priority 2."]
    PRI2,
    #[doc = "Priority 4."]
    PRI4,
    #[doc = "Priority 6."]
    PRI6,
    #[doc = "Priority 8."]
    PRI8,
    #[doc = "Priority 10."]
    PRI10,
    #[doc = "Priority 12."]
    PRI12,
    #[doc = "Priority 14."]
    PRI14,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl RAM1R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            RAM1R::PRI0 => 0,
            RAM1R::PRI2 => 2,
            RAM1R::PRI4 => 4,
            RAM1R::PRI6 => 6,
            RAM1R::PRI8 => 8,
            RAM1R::PRI10 => 10,
            RAM1R::PRI12 => 12,
            RAM1R::PRI14 => 14,
            RAM1R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> RAM1R {
        match value {
            0 => RAM1R::PRI0,
            2 => RAM1R::PRI2,
            4 => RAM1R::PRI4,
            6 => RAM1R::PRI6,
            8 => RAM1R::PRI8,
            10 => RAM1R::PRI10,
            12 => RAM1R::PRI12,
            14 => RAM1R::PRI14,
            i => RAM1R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `PRI0`"]
    #[inline]
    pub fn is_pri0(&self) -> bool {
        *self == RAM1R::PRI0
    }
    #[doc = "Checks if the value of the field is `PRI2`"]
    #[inline]
    pub fn is_pri2(&self) -> bool {
        *self == RAM1R::PRI2
    }
    #[doc = "Checks if the value of the field is `PRI4`"]
    #[inline]
    pub fn is_pri4(&self) -> bool {
        *self == RAM1R::PRI4
    }
    #[doc = "Checks if the value of the field is `PRI6`"]
    #[inline]
    pub fn is_pri6(&self) -> bool {
        *self == RAM1R::PRI6
    }
    #[doc = "Checks if the value of the field is `PRI8`"]
    #[inline]
    pub fn is_pri8(&self) -> bool {
        *self == RAM1R::PRI8
    }
    #[doc = "Checks if the value of the field is `PRI10`"]
    #[inline]
    pub fn is_pri10(&self) -> bool {
        *self == RAM1R::PRI10
    }
    #[doc = "Checks if the value of the field is `PRI12`"]
    #[inline]
    pub fn is_pri12(&self) -> bool {
        *self == RAM1R::PRI12
    }
    #[doc = "Checks if the value of the field is `PRI14`"]
    #[inline]
    pub fn is_pri14(&self) -> bool {
        *self == RAM1R::PRI14
    }
}
#[doc = "Possible values of the field `RAM2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RAM2R {
    #[doc = "Priority 0."]
    PRI0,
    #[doc = "Priority 2."]
    PRI2,
    #[doc = "Priority 4."]
    PRI4,
    #[doc = "Priority 6."]
    PRI6,
    #[doc = "Priority 8."]
    PRI8,
    #[doc = "Priority 10."]
    PRI10,
    #[doc = "Priority 12."]
    PRI12,
    #[doc = "Priority 14."]
    PRI14,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl RAM2R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            RAM2R::PRI0 => 0,
            RAM2R::PRI2 => 2,
            RAM2R::PRI4 => 4,
            RAM2R::PRI6 => 6,
            RAM2R::PRI8 => 8,
            RAM2R::PRI10 => 10,
            RAM2R::PRI12 => 12,
            RAM2R::PRI14 => 14,
            RAM2R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> RAM2R {
        match value {
            0 => RAM2R::PRI0,
            2 => RAM2R::PRI2,
            4 => RAM2R::PRI4,
            6 => RAM2R::PRI6,
            8 => RAM2R::PRI8,
            10 => RAM2R::PRI10,
            12 => RAM2R::PRI12,
            14 => RAM2R::PRI14,
            i => RAM2R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `PRI0`"]
    #[inline]
    pub fn is_pri0(&self) -> bool {
        *self == RAM2R::PRI0
    }
    #[doc = "Checks if the value of the field is `PRI2`"]
    #[inline]
    pub fn is_pri2(&self) -> bool {
        *self == RAM2R::PRI2
    }
    #[doc = "Checks if the value of the field is `PRI4`"]
    #[inline]
    pub fn is_pri4(&self) -> bool {
        *self == RAM2R::PRI4
    }
    #[doc = "Checks if the value of the field is `PRI6`"]
    #[inline]
    pub fn is_pri6(&self) -> bool {
        *self == RAM2R::PRI6
    }
    #[doc = "Checks if the value of the field is `PRI8`"]
    #[inline]
    pub fn is_pri8(&self) -> bool {
        *self == RAM2R::PRI8
    }
    #[doc = "Checks if the value of the field is `PRI10`"]
    #[inline]
    pub fn is_pri10(&self) -> bool {
        *self == RAM2R::PRI10
    }
    #[doc = "Checks if the value of the field is `PRI12`"]
    #[inline]
    pub fn is_pri12(&self) -> bool {
        *self == RAM2R::PRI12
    }
    #[doc = "Checks if the value of the field is `PRI14`"]
    #[inline]
    pub fn is_pri14(&self) -> bool {
        *self == RAM2R::PRI14
    }
}
#[doc = "Possible values of the field `RAM3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RAM3R {
    #[doc = "Priority 0."]
    PRI0,
    #[doc = "Priority 2."]
    PRI2,
    #[doc = "Priority 4."]
    PRI4,
    #[doc = "Priority 6."]
    PRI6,
    #[doc = "Priority 8."]
    PRI8,
    #[doc = "Priority 10."]
    PRI10,
    #[doc = "Priority 12."]
    PRI12,
    #[doc = "Priority 14."]
    PRI14,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl RAM3R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            RAM3R::PRI0 => 0,
            RAM3R::PRI2 => 2,
            RAM3R::PRI4 => 4,
            RAM3R::PRI6 => 6,
            RAM3R::PRI8 => 8,
            RAM3R::PRI10 => 10,
            RAM3R::PRI12 => 12,
            RAM3R::PRI14 => 14,
            RAM3R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> RAM3R {
        match value {
            0 => RAM3R::PRI0,
            2 => RAM3R::PRI2,
            4 => RAM3R::PRI4,
            6 => RAM3R::PRI6,
            8 => RAM3R::PRI8,
            10 => RAM3R::PRI10,
            12 => RAM3R::PRI12,
            14 => RAM3R::PRI14,
            i => RAM3R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `PRI0`"]
    #[inline]
    pub fn is_pri0(&self) -> bool {
        *self == RAM3R::PRI0
    }
    #[doc = "Checks if the value of the field is `PRI2`"]
    #[inline]
    pub fn is_pri2(&self) -> bool {
        *self == RAM3R::PRI2
    }
    #[doc = "Checks if the value of the field is `PRI4`"]
    #[inline]
    pub fn is_pri4(&self) -> bool {
        *self == RAM3R::PRI4
    }
    #[doc = "Checks if the value of the field is `PRI6`"]
    #[inline]
    pub fn is_pri6(&self) -> bool {
        *self == RAM3R::PRI6
    }
    #[doc = "Checks if the value of the field is `PRI8`"]
    #[inline]
    pub fn is_pri8(&self) -> bool {
        *self == RAM3R::PRI8
    }
    #[doc = "Checks if the value of the field is `PRI10`"]
    #[inline]
    pub fn is_pri10(&self) -> bool {
        *self == RAM3R::PRI10
    }
    #[doc = "Checks if the value of the field is `PRI12`"]
    #[inline]
    pub fn is_pri12(&self) -> bool {
        *self == RAM3R::PRI12
    }
    #[doc = "Checks if the value of the field is `PRI14`"]
    #[inline]
    pub fn is_pri14(&self) -> bool {
        *self == RAM3R::PRI14
    }
}
#[doc = "Possible values of the field `RAM4`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RAM4R {
    #[doc = "Priority 0."]
    PRI0,
    #[doc = "Priority 2."]
    PRI2,
    #[doc = "Priority 4."]
    PRI4,
    #[doc = "Priority 6."]
    PRI6,
    #[doc = "Priority 8."]
    PRI8,
    #[doc = "Priority 10."]
    PRI10,
    #[doc = "Priority 12."]
    PRI12,
    #[doc = "Priority 14."]
    PRI14,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl RAM4R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            RAM4R::PRI0 => 0,
            RAM4R::PRI2 => 2,
            RAM4R::PRI4 => 4,
            RAM4R::PRI6 => 6,
            RAM4R::PRI8 => 8,
            RAM4R::PRI10 => 10,
            RAM4R::PRI12 => 12,
            RAM4R::PRI14 => 14,
            RAM4R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> RAM4R {
        match value {
            0 => RAM4R::PRI0,
            2 => RAM4R::PRI2,
            4 => RAM4R::PRI4,
            6 => RAM4R::PRI6,
            8 => RAM4R::PRI8,
            10 => RAM4R::PRI10,
            12 => RAM4R::PRI12,
            14 => RAM4R::PRI14,
            i => RAM4R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `PRI0`"]
    #[inline]
    pub fn is_pri0(&self) -> bool {
        *self == RAM4R::PRI0
    }
    #[doc = "Checks if the value of the field is `PRI2`"]
    #[inline]
    pub fn is_pri2(&self) -> bool {
        *self == RAM4R::PRI2
    }
    #[doc = "Checks if the value of the field is `PRI4`"]
    #[inline]
    pub fn is_pri4(&self) -> bool {
        *self == RAM4R::PRI4
    }
    #[doc = "Checks if the value of the field is `PRI6`"]
    #[inline]
    pub fn is_pri6(&self) -> bool {
        *self == RAM4R::PRI6
    }
    #[doc = "Checks if the value of the field is `PRI8`"]
    #[inline]
    pub fn is_pri8(&self) -> bool {
        *self == RAM4R::PRI8
    }
    #[doc = "Checks if the value of the field is `PRI10`"]
    #[inline]
    pub fn is_pri10(&self) -> bool {
        *self == RAM4R::PRI10
    }
    #[doc = "Checks if the value of the field is `PRI12`"]
    #[inline]
    pub fn is_pri12(&self) -> bool {
        *self == RAM4R::PRI12
    }
    #[doc = "Checks if the value of the field is `PRI14`"]
    #[inline]
    pub fn is_pri14(&self) -> bool {
        *self == RAM4R::PRI14
    }
}
#[doc = "Possible values of the field `RAM5`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RAM5R {
    #[doc = "Priority 0."]
    PRI0,
    #[doc = "Priority 2."]
    PRI2,
    #[doc = "Priority 4."]
    PRI4,
    #[doc = "Priority 6."]
    PRI6,
    #[doc = "Priority 8."]
    PRI8,
    #[doc = "Priority 10."]
    PRI10,
    #[doc = "Priority 12."]
    PRI12,
    #[doc = "Priority 14."]
    PRI14,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl RAM5R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            RAM5R::PRI0 => 0,
            RAM5R::PRI2 => 2,
            RAM5R::PRI4 => 4,
            RAM5R::PRI6 => 6,
            RAM5R::PRI8 => 8,
            RAM5R::PRI10 => 10,
            RAM5R::PRI12 => 12,
            RAM5R::PRI14 => 14,
            RAM5R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> RAM5R {
        match value {
            0 => RAM5R::PRI0,
            2 => RAM5R::PRI2,
            4 => RAM5R::PRI4,
            6 => RAM5R::PRI6,
            8 => RAM5R::PRI8,
            10 => RAM5R::PRI10,
            12 => RAM5R::PRI12,
            14 => RAM5R::PRI14,
            i => RAM5R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `PRI0`"]
    #[inline]
    pub fn is_pri0(&self) -> bool {
        *self == RAM5R::PRI0
    }
    #[doc = "Checks if the value of the field is `PRI2`"]
    #[inline]
    pub fn is_pri2(&self) -> bool {
        *self == RAM5R::PRI2
    }
    #[doc = "Checks if the value of the field is `PRI4`"]
    #[inline]
    pub fn is_pri4(&self) -> bool {
        *self == RAM5R::PRI4
    }
    #[doc = "Checks if the value of the field is `PRI6`"]
    #[inline]
    pub fn is_pri6(&self) -> bool {
        *self == RAM5R::PRI6
    }
    #[doc = "Checks if the value of the field is `PRI8`"]
    #[inline]
    pub fn is_pri8(&self) -> bool {
        *self == RAM5R::PRI8
    }
    #[doc = "Checks if the value of the field is `PRI10`"]
    #[inline]
    pub fn is_pri10(&self) -> bool {
        *self == RAM5R::PRI10
    }
    #[doc = "Checks if the value of the field is `PRI12`"]
    #[inline]
    pub fn is_pri12(&self) -> bool {
        *self == RAM5R::PRI12
    }
    #[doc = "Checks if the value of the field is `PRI14`"]
    #[inline]
    pub fn is_pri14(&self) -> bool {
        *self == RAM5R::PRI14
    }
}
#[doc = "Possible values of the field `RAM6`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RAM6R {
    #[doc = "Priority 0."]
    PRI0,
    #[doc = "Priority 2."]
    PRI2,
    #[doc = "Priority 4."]
    PRI4,
    #[doc = "Priority 6."]
    PRI6,
    #[doc = "Priority 8."]
    PRI8,
    #[doc = "Priority 10."]
    PRI10,
    #[doc = "Priority 12."]
    PRI12,
    #[doc = "Priority 14."]
    PRI14,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl RAM6R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            RAM6R::PRI0 => 0,
            RAM6R::PRI2 => 2,
            RAM6R::PRI4 => 4,
            RAM6R::PRI6 => 6,
            RAM6R::PRI8 => 8,
            RAM6R::PRI10 => 10,
            RAM6R::PRI12 => 12,
            RAM6R::PRI14 => 14,
            RAM6R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> RAM6R {
        match value {
            0 => RAM6R::PRI0,
            2 => RAM6R::PRI2,
            4 => RAM6R::PRI4,
            6 => RAM6R::PRI6,
            8 => RAM6R::PRI8,
            10 => RAM6R::PRI10,
            12 => RAM6R::PRI12,
            14 => RAM6R::PRI14,
            i => RAM6R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `PRI0`"]
    #[inline]
    pub fn is_pri0(&self) -> bool {
        *self == RAM6R::PRI0
    }
    #[doc = "Checks if the value of the field is `PRI2`"]
    #[inline]
    pub fn is_pri2(&self) -> bool {
        *self == RAM6R::PRI2
    }
    #[doc = "Checks if the value of the field is `PRI4`"]
    #[inline]
    pub fn is_pri4(&self) -> bool {
        *self == RAM6R::PRI4
    }
    #[doc = "Checks if the value of the field is `PRI6`"]
    #[inline]
    pub fn is_pri6(&self) -> bool {
        *self == RAM6R::PRI6
    }
    #[doc = "Checks if the value of the field is `PRI8`"]
    #[inline]
    pub fn is_pri8(&self) -> bool {
        *self == RAM6R::PRI8
    }
    #[doc = "Checks if the value of the field is `PRI10`"]
    #[inline]
    pub fn is_pri10(&self) -> bool {
        *self == RAM6R::PRI10
    }
    #[doc = "Checks if the value of the field is `PRI12`"]
    #[inline]
    pub fn is_pri12(&self) -> bool {
        *self == RAM6R::PRI12
    }
    #[doc = "Checks if the value of the field is `PRI14`"]
    #[inline]
    pub fn is_pri14(&self) -> bool {
        *self == RAM6R::PRI14
    }
}
#[doc = "Possible values of the field `RAM7`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RAM7R {
    #[doc = "Priority 0."]
    PRI0,
    #[doc = "Priority 2."]
    PRI2,
    #[doc = "Priority 4."]
    PRI4,
    #[doc = "Priority 6."]
    PRI6,
    #[doc = "Priority 8."]
    PRI8,
    #[doc = "Priority 10."]
    PRI10,
    #[doc = "Priority 12."]
    PRI12,
    #[doc = "Priority 14."]
    PRI14,
    #[doc = r" Reserved"]
    _Reserved(u8),
}
impl RAM7R {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        match *self {
            RAM7R::PRI0 => 0,
            RAM7R::PRI2 => 2,
            RAM7R::PRI4 => 4,
            RAM7R::PRI6 => 6,
            RAM7R::PRI8 => 8,
            RAM7R::PRI10 => 10,
            RAM7R::PRI12 => 12,
            RAM7R::PRI14 => 14,
            RAM7R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _from(value: u8) -> RAM7R {
        match value {
            0 => RAM7R::PRI0,
            2 => RAM7R::PRI2,
            4 => RAM7R::PRI4,
            6 => RAM7R::PRI6,
            8 => RAM7R::PRI8,
            10 => RAM7R::PRI10,
            12 => RAM7R::PRI12,
            14 => RAM7R::PRI14,
            i => RAM7R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `PRI0`"]
    #[inline]
    pub fn is_pri0(&self) -> bool {
        *self == RAM7R::PRI0
    }
    #[doc = "Checks if the value of the field is `PRI2`"]
    #[inline]
    pub fn is_pri2(&self) -> bool {
        *self == RAM7R::PRI2
    }
    #[doc = "Checks if the value of the field is `PRI4`"]
    #[inline]
    pub fn is_pri4(&self) -> bool {
        *self == RAM7R::PRI4
    }
    #[doc = "Checks if the value of the field is `PRI6`"]
    #[inline]
    pub fn is_pri6(&self) -> bool {
        *self == RAM7R::PRI6
    }
    #[doc = "Checks if the value of the field is `PRI8`"]
    #[inline]
    pub fn is_pri8(&self) -> bool {
        *self == RAM7R::PRI8
    }
    #[doc = "Checks if the value of the field is `PRI10`"]
    #[inline]
    pub fn is_pri10(&self) -> bool {
        *self == RAM7R::PRI10
    }
    #[doc = "Checks if the value of the field is `PRI12`"]
    #[inline]
    pub fn is_pri12(&self) -> bool {
        *self == RAM7R::PRI12
    }
    #[doc = "Checks if the value of the field is `PRI14`"]
    #[inline]
    pub fn is_pri14(&self) -> bool {
        *self == RAM7R::PRI14
    }
}
#[doc = "Values that can be written to the field `RAM0`"]
pub enum RAM0W {
    #[doc = "Priority 0."]
    PRI0,
    #[doc = "Priority 2."]
    PRI2,
    #[doc = "Priority 4."]
    PRI4,
    #[doc = "Priority 6."]
    PRI6,
    #[doc = "Priority 8."]
    PRI8,
    #[doc = "Priority 10."]
    PRI10,
    #[doc = "Priority 12."]
    PRI12,
    #[doc = "Priority 14."]
    PRI14,
}
impl RAM0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            RAM0W::PRI0 => 0,
            RAM0W::PRI2 => 2,
            RAM0W::PRI4 => 4,
            RAM0W::PRI6 => 6,
            RAM0W::PRI8 => 8,
            RAM0W::PRI10 => 10,
            RAM0W::PRI12 => 12,
            RAM0W::PRI14 => 14,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RAM0W<'a> {
    w: &'a mut W,
}
impl<'a> _RAM0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RAM0W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Priority 0."]
    #[inline]
    pub fn pri0(self) -> &'a mut W {
        self.variant(RAM0W::PRI0)
    }
    #[doc = "Priority 2."]
    #[inline]
    pub fn pri2(self) -> &'a mut W {
        self.variant(RAM0W::PRI2)
    }
    #[doc = "Priority 4."]
    #[inline]
    pub fn pri4(self) -> &'a mut W {
        self.variant(RAM0W::PRI4)
    }
    #[doc = "Priority 6."]
    #[inline]
    pub fn pri6(self) -> &'a mut W {
        self.variant(RAM0W::PRI6)
    }
    #[doc = "Priority 8."]
    #[inline]
    pub fn pri8(self) -> &'a mut W {
        self.variant(RAM0W::PRI8)
    }
    #[doc = "Priority 10."]
    #[inline]
    pub fn pri10(self) -> &'a mut W {
        self.variant(RAM0W::PRI10)
    }
    #[doc = "Priority 12."]
    #[inline]
    pub fn pri12(self) -> &'a mut W {
        self.variant(RAM0W::PRI12)
    }
    #[doc = "Priority 14."]
    #[inline]
    pub fn pri14(self) -> &'a mut W {
        self.variant(RAM0W::PRI14)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RAM1`"]
pub enum RAM1W {
    #[doc = "Priority 0."]
    PRI0,
    #[doc = "Priority 2."]
    PRI2,
    #[doc = "Priority 4."]
    PRI4,
    #[doc = "Priority 6."]
    PRI6,
    #[doc = "Priority 8."]
    PRI8,
    #[doc = "Priority 10."]
    PRI10,
    #[doc = "Priority 12."]
    PRI12,
    #[doc = "Priority 14."]
    PRI14,
}
impl RAM1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            RAM1W::PRI0 => 0,
            RAM1W::PRI2 => 2,
            RAM1W::PRI4 => 4,
            RAM1W::PRI6 => 6,
            RAM1W::PRI8 => 8,
            RAM1W::PRI10 => 10,
            RAM1W::PRI12 => 12,
            RAM1W::PRI14 => 14,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RAM1W<'a> {
    w: &'a mut W,
}
impl<'a> _RAM1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RAM1W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Priority 0."]
    #[inline]
    pub fn pri0(self) -> &'a mut W {
        self.variant(RAM1W::PRI0)
    }
    #[doc = "Priority 2."]
    #[inline]
    pub fn pri2(self) -> &'a mut W {
        self.variant(RAM1W::PRI2)
    }
    #[doc = "Priority 4."]
    #[inline]
    pub fn pri4(self) -> &'a mut W {
        self.variant(RAM1W::PRI4)
    }
    #[doc = "Priority 6."]
    #[inline]
    pub fn pri6(self) -> &'a mut W {
        self.variant(RAM1W::PRI6)
    }
    #[doc = "Priority 8."]
    #[inline]
    pub fn pri8(self) -> &'a mut W {
        self.variant(RAM1W::PRI8)
    }
    #[doc = "Priority 10."]
    #[inline]
    pub fn pri10(self) -> &'a mut W {
        self.variant(RAM1W::PRI10)
    }
    #[doc = "Priority 12."]
    #[inline]
    pub fn pri12(self) -> &'a mut W {
        self.variant(RAM1W::PRI12)
    }
    #[doc = "Priority 14."]
    #[inline]
    pub fn pri14(self) -> &'a mut W {
        self.variant(RAM1W::PRI14)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RAM2`"]
pub enum RAM2W {
    #[doc = "Priority 0."]
    PRI0,
    #[doc = "Priority 2."]
    PRI2,
    #[doc = "Priority 4."]
    PRI4,
    #[doc = "Priority 6."]
    PRI6,
    #[doc = "Priority 8."]
    PRI8,
    #[doc = "Priority 10."]
    PRI10,
    #[doc = "Priority 12."]
    PRI12,
    #[doc = "Priority 14."]
    PRI14,
}
impl RAM2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            RAM2W::PRI0 => 0,
            RAM2W::PRI2 => 2,
            RAM2W::PRI4 => 4,
            RAM2W::PRI6 => 6,
            RAM2W::PRI8 => 8,
            RAM2W::PRI10 => 10,
            RAM2W::PRI12 => 12,
            RAM2W::PRI14 => 14,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RAM2W<'a> {
    w: &'a mut W,
}
impl<'a> _RAM2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RAM2W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Priority 0."]
    #[inline]
    pub fn pri0(self) -> &'a mut W {
        self.variant(RAM2W::PRI0)
    }
    #[doc = "Priority 2."]
    #[inline]
    pub fn pri2(self) -> &'a mut W {
        self.variant(RAM2W::PRI2)
    }
    #[doc = "Priority 4."]
    #[inline]
    pub fn pri4(self) -> &'a mut W {
        self.variant(RAM2W::PRI4)
    }
    #[doc = "Priority 6."]
    #[inline]
    pub fn pri6(self) -> &'a mut W {
        self.variant(RAM2W::PRI6)
    }
    #[doc = "Priority 8."]
    #[inline]
    pub fn pri8(self) -> &'a mut W {
        self.variant(RAM2W::PRI8)
    }
    #[doc = "Priority 10."]
    #[inline]
    pub fn pri10(self) -> &'a mut W {
        self.variant(RAM2W::PRI10)
    }
    #[doc = "Priority 12."]
    #[inline]
    pub fn pri12(self) -> &'a mut W {
        self.variant(RAM2W::PRI12)
    }
    #[doc = "Priority 14."]
    #[inline]
    pub fn pri14(self) -> &'a mut W {
        self.variant(RAM2W::PRI14)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RAM3`"]
pub enum RAM3W {
    #[doc = "Priority 0."]
    PRI0,
    #[doc = "Priority 2."]
    PRI2,
    #[doc = "Priority 4."]
    PRI4,
    #[doc = "Priority 6."]
    PRI6,
    #[doc = "Priority 8."]
    PRI8,
    #[doc = "Priority 10."]
    PRI10,
    #[doc = "Priority 12."]
    PRI12,
    #[doc = "Priority 14."]
    PRI14,
}
impl RAM3W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            RAM3W::PRI0 => 0,
            RAM3W::PRI2 => 2,
            RAM3W::PRI4 => 4,
            RAM3W::PRI6 => 6,
            RAM3W::PRI8 => 8,
            RAM3W::PRI10 => 10,
            RAM3W::PRI12 => 12,
            RAM3W::PRI14 => 14,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RAM3W<'a> {
    w: &'a mut W,
}
impl<'a> _RAM3W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RAM3W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Priority 0."]
    #[inline]
    pub fn pri0(self) -> &'a mut W {
        self.variant(RAM3W::PRI0)
    }
    #[doc = "Priority 2."]
    #[inline]
    pub fn pri2(self) -> &'a mut W {
        self.variant(RAM3W::PRI2)
    }
    #[doc = "Priority 4."]
    #[inline]
    pub fn pri4(self) -> &'a mut W {
        self.variant(RAM3W::PRI4)
    }
    #[doc = "Priority 6."]
    #[inline]
    pub fn pri6(self) -> &'a mut W {
        self.variant(RAM3W::PRI6)
    }
    #[doc = "Priority 8."]
    #[inline]
    pub fn pri8(self) -> &'a mut W {
        self.variant(RAM3W::PRI8)
    }
    #[doc = "Priority 10."]
    #[inline]
    pub fn pri10(self) -> &'a mut W {
        self.variant(RAM3W::PRI10)
    }
    #[doc = "Priority 12."]
    #[inline]
    pub fn pri12(self) -> &'a mut W {
        self.variant(RAM3W::PRI12)
    }
    #[doc = "Priority 14."]
    #[inline]
    pub fn pri14(self) -> &'a mut W {
        self.variant(RAM3W::PRI14)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RAM4`"]
pub enum RAM4W {
    #[doc = "Priority 0."]
    PRI0,
    #[doc = "Priority 2."]
    PRI2,
    #[doc = "Priority 4."]
    PRI4,
    #[doc = "Priority 6."]
    PRI6,
    #[doc = "Priority 8."]
    PRI8,
    #[doc = "Priority 10."]
    PRI10,
    #[doc = "Priority 12."]
    PRI12,
    #[doc = "Priority 14."]
    PRI14,
}
impl RAM4W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            RAM4W::PRI0 => 0,
            RAM4W::PRI2 => 2,
            RAM4W::PRI4 => 4,
            RAM4W::PRI6 => 6,
            RAM4W::PRI8 => 8,
            RAM4W::PRI10 => 10,
            RAM4W::PRI12 => 12,
            RAM4W::PRI14 => 14,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RAM4W<'a> {
    w: &'a mut W,
}
impl<'a> _RAM4W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RAM4W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Priority 0."]
    #[inline]
    pub fn pri0(self) -> &'a mut W {
        self.variant(RAM4W::PRI0)
    }
    #[doc = "Priority 2."]
    #[inline]
    pub fn pri2(self) -> &'a mut W {
        self.variant(RAM4W::PRI2)
    }
    #[doc = "Priority 4."]
    #[inline]
    pub fn pri4(self) -> &'a mut W {
        self.variant(RAM4W::PRI4)
    }
    #[doc = "Priority 6."]
    #[inline]
    pub fn pri6(self) -> &'a mut W {
        self.variant(RAM4W::PRI6)
    }
    #[doc = "Priority 8."]
    #[inline]
    pub fn pri8(self) -> &'a mut W {
        self.variant(RAM4W::PRI8)
    }
    #[doc = "Priority 10."]
    #[inline]
    pub fn pri10(self) -> &'a mut W {
        self.variant(RAM4W::PRI10)
    }
    #[doc = "Priority 12."]
    #[inline]
    pub fn pri12(self) -> &'a mut W {
        self.variant(RAM4W::PRI12)
    }
    #[doc = "Priority 14."]
    #[inline]
    pub fn pri14(self) -> &'a mut W {
        self.variant(RAM4W::PRI14)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RAM5`"]
pub enum RAM5W {
    #[doc = "Priority 0."]
    PRI0,
    #[doc = "Priority 2."]
    PRI2,
    #[doc = "Priority 4."]
    PRI4,
    #[doc = "Priority 6."]
    PRI6,
    #[doc = "Priority 8."]
    PRI8,
    #[doc = "Priority 10."]
    PRI10,
    #[doc = "Priority 12."]
    PRI12,
    #[doc = "Priority 14."]
    PRI14,
}
impl RAM5W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            RAM5W::PRI0 => 0,
            RAM5W::PRI2 => 2,
            RAM5W::PRI4 => 4,
            RAM5W::PRI6 => 6,
            RAM5W::PRI8 => 8,
            RAM5W::PRI10 => 10,
            RAM5W::PRI12 => 12,
            RAM5W::PRI14 => 14,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RAM5W<'a> {
    w: &'a mut W,
}
impl<'a> _RAM5W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RAM5W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Priority 0."]
    #[inline]
    pub fn pri0(self) -> &'a mut W {
        self.variant(RAM5W::PRI0)
    }
    #[doc = "Priority 2."]
    #[inline]
    pub fn pri2(self) -> &'a mut W {
        self.variant(RAM5W::PRI2)
    }
    #[doc = "Priority 4."]
    #[inline]
    pub fn pri4(self) -> &'a mut W {
        self.variant(RAM5W::PRI4)
    }
    #[doc = "Priority 6."]
    #[inline]
    pub fn pri6(self) -> &'a mut W {
        self.variant(RAM5W::PRI6)
    }
    #[doc = "Priority 8."]
    #[inline]
    pub fn pri8(self) -> &'a mut W {
        self.variant(RAM5W::PRI8)
    }
    #[doc = "Priority 10."]
    #[inline]
    pub fn pri10(self) -> &'a mut W {
        self.variant(RAM5W::PRI10)
    }
    #[doc = "Priority 12."]
    #[inline]
    pub fn pri12(self) -> &'a mut W {
        self.variant(RAM5W::PRI12)
    }
    #[doc = "Priority 14."]
    #[inline]
    pub fn pri14(self) -> &'a mut W {
        self.variant(RAM5W::PRI14)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RAM6`"]
pub enum RAM6W {
    #[doc = "Priority 0."]
    PRI0,
    #[doc = "Priority 2."]
    PRI2,
    #[doc = "Priority 4."]
    PRI4,
    #[doc = "Priority 6."]
    PRI6,
    #[doc = "Priority 8."]
    PRI8,
    #[doc = "Priority 10."]
    PRI10,
    #[doc = "Priority 12."]
    PRI12,
    #[doc = "Priority 14."]
    PRI14,
}
impl RAM6W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            RAM6W::PRI0 => 0,
            RAM6W::PRI2 => 2,
            RAM6W::PRI4 => 4,
            RAM6W::PRI6 => 6,
            RAM6W::PRI8 => 8,
            RAM6W::PRI10 => 10,
            RAM6W::PRI12 => 12,
            RAM6W::PRI14 => 14,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RAM6W<'a> {
    w: &'a mut W,
}
impl<'a> _RAM6W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RAM6W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Priority 0."]
    #[inline]
    pub fn pri0(self) -> &'a mut W {
        self.variant(RAM6W::PRI0)
    }
    #[doc = "Priority 2."]
    #[inline]
    pub fn pri2(self) -> &'a mut W {
        self.variant(RAM6W::PRI2)
    }
    #[doc = "Priority 4."]
    #[inline]
    pub fn pri4(self) -> &'a mut W {
        self.variant(RAM6W::PRI4)
    }
    #[doc = "Priority 6."]
    #[inline]
    pub fn pri6(self) -> &'a mut W {
        self.variant(RAM6W::PRI6)
    }
    #[doc = "Priority 8."]
    #[inline]
    pub fn pri8(self) -> &'a mut W {
        self.variant(RAM6W::PRI8)
    }
    #[doc = "Priority 10."]
    #[inline]
    pub fn pri10(self) -> &'a mut W {
        self.variant(RAM6W::PRI10)
    }
    #[doc = "Priority 12."]
    #[inline]
    pub fn pri12(self) -> &'a mut W {
        self.variant(RAM6W::PRI12)
    }
    #[doc = "Priority 14."]
    #[inline]
    pub fn pri14(self) -> &'a mut W {
        self.variant(RAM6W::PRI14)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `RAM7`"]
pub enum RAM7W {
    #[doc = "Priority 0."]
    PRI0,
    #[doc = "Priority 2."]
    PRI2,
    #[doc = "Priority 4."]
    PRI4,
    #[doc = "Priority 6."]
    PRI6,
    #[doc = "Priority 8."]
    PRI8,
    #[doc = "Priority 10."]
    PRI10,
    #[doc = "Priority 12."]
    PRI12,
    #[doc = "Priority 14."]
    PRI14,
}
impl RAM7W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> u8 {
        match *self {
            RAM7W::PRI0 => 0,
            RAM7W::PRI2 => 2,
            RAM7W::PRI4 => 4,
            RAM7W::PRI6 => 6,
            RAM7W::PRI8 => 8,
            RAM7W::PRI10 => 10,
            RAM7W::PRI12 => 12,
            RAM7W::PRI14 => 14,
        }
    }
}
#[doc = r" Proxy"]
pub struct _RAM7W<'a> {
    w: &'a mut W,
}
impl<'a> _RAM7W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: RAM7W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Priority 0."]
    #[inline]
    pub fn pri0(self) -> &'a mut W {
        self.variant(RAM7W::PRI0)
    }
    #[doc = "Priority 2."]
    #[inline]
    pub fn pri2(self) -> &'a mut W {
        self.variant(RAM7W::PRI2)
    }
    #[doc = "Priority 4."]
    #[inline]
    pub fn pri4(self) -> &'a mut W {
        self.variant(RAM7W::PRI4)
    }
    #[doc = "Priority 6."]
    #[inline]
    pub fn pri6(self) -> &'a mut W {
        self.variant(RAM7W::PRI6)
    }
    #[doc = "Priority 8."]
    #[inline]
    pub fn pri8(self) -> &'a mut W {
        self.variant(RAM7W::PRI8)
    }
    #[doc = "Priority 10."]
    #[inline]
    pub fn pri10(self) -> &'a mut W {
        self.variant(RAM7W::PRI10)
    }
    #[doc = "Priority 12."]
    #[inline]
    pub fn pri12(self) -> &'a mut W {
        self.variant(RAM7W::PRI12)
    }
    #[doc = "Priority 14."]
    #[inline]
    pub fn pri14(self) -> &'a mut W {
        self.variant(RAM7W::PRI14)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 28;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:3 - Configuration field for RAM block 0."]
    #[inline]
    pub fn ram0(&self) -> RAM0R {
        RAM0R::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 4:7 - Configuration field for RAM block 1."]
    #[inline]
    pub fn ram1(&self) -> RAM1R {
        RAM1R::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 8:11 - Configuration field for RAM block 2."]
    #[inline]
    pub fn ram2(&self) -> RAM2R {
        RAM2R::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 12:15 - Configuration field for RAM block 3."]
    #[inline]
    pub fn ram3(&self) -> RAM3R {
        RAM3R::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 16:19 - Configuration field for RAM block 4."]
    #[inline]
    pub fn ram4(&self) -> RAM4R {
        RAM4R::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 20:23 - Configuration field for RAM block 5."]
    #[inline]
    pub fn ram5(&self) -> RAM5R {
        RAM5R::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 24:27 - Configuration field for RAM block 6."]
    #[inline]
    pub fn ram6(&self) -> RAM6R {
        RAM6R::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
    #[doc = "Bits 28:31 - Configuration field for RAM block 7."]
    #[inline]
    pub fn ram7(&self) -> RAM7R {
        RAM7R::_from({
            const MASK: u8 = 15;
            const OFFSET: u8 = 28;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        })
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 0 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:3 - Configuration field for RAM block 0."]
    #[inline]
    pub fn ram0(&mut self) -> _RAM0W {
        _RAM0W { w: self }
    }
    #[doc = "Bits 4:7 - Configuration field for RAM block 1."]
    #[inline]
    pub fn ram1(&mut self) -> _RAM1W {
        _RAM1W { w: self }
    }
    #[doc = "Bits 8:11 - Configuration field for RAM block 2."]
    #[inline]
    pub fn ram2(&mut self) -> _RAM2W {
        _RAM2W { w: self }
    }
    #[doc = "Bits 12:15 - Configuration field for RAM block 3."]
    #[inline]
    pub fn ram3(&mut self) -> _RAM3W {
        _RAM3W { w: self }
    }
    #[doc = "Bits 16:19 - Configuration field for RAM block 4."]
    #[inline]
    pub fn ram4(&mut self) -> _RAM4W {
        _RAM4W { w: self }
    }
    #[doc = "Bits 20:23 - Configuration field for RAM block 5."]
    #[inline]
    pub fn ram5(&mut self) -> _RAM5W {
        _RAM5W { w: self }
    }
    #[doc = "Bits 24:27 - Configuration field for RAM block 6."]
    #[inline]
    pub fn ram6(&mut self) -> _RAM6W {
        _RAM6W { w: self }
    }
    #[doc = "Bits 28:31 - Configuration field for RAM block 7."]
    #[inline]
    pub fn ram7(&mut self) -> _RAM7W {
        _RAM7W { w: self }
    }
}

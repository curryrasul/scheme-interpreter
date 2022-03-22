use std::{cmp, fmt::Display, ops};

#[derive(PartialEq, Eq)]
enum TypedNumLevel {
    IntegerLvl,
    FloatLvl,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TypedNum {
    Integer(i64),
    Float(f64),
}

impl TypedNum {
    pub fn to_int(&self) -> i64 {
        match *self {
            Self::Integer(val) => val,
            Self::Float(val) => val as i64,
        }
    }

    pub fn to_float(&self) -> f64 {
        match *self {
            Self::Integer(val) => val as f64,
            Self::Float(val) => val,
        }
    }

    fn to_int_wn(self) -> Self {
        match self {
            Self::Integer(_) => self,
            Self::Float(val) => Self::Integer(val as i64),
        }
    }

    fn to_float_wn(self) -> Self {
        match self {
            Self::Integer(val) => Self::Float(val as f64),
            Self::Float(_) => self,
        }
    }

    fn get_level(&self) -> TypedNumLevel {
        match self {
            Self::Integer(_) => TypedNumLevel::IntegerLvl,
            Self::Float(_) => TypedNumLevel::FloatLvl,
        }
    }

    fn common_val(num1: Self, num2: Self) -> (Self, Self) {
        use TypedNumLevel::*;

        let lvl1 = num1.get_level();
        let lvl2 = num2.get_level();

        if lvl1 == FloatLvl || lvl2 == FloatLvl {
            (num1.to_float_wn(), num2.to_float_wn())
        } else {
            (num1.to_int_wn(), num2.to_int_wn())
        }
    }
}

macro_rules! gen_binary_op {
    ($trait:ident, $func_name:ident, $op:expr) => {
        impl ops::$trait for TypedNum {
            type Output = Self;

            fn $func_name(self, rhs: Self) -> Self::Output {
                let (val1, val2) = Self::common_val(self, rhs);

                match val1 {
                    Self::Integer(v1) => {
                        if let Self::Integer(v2) = val2 {
                            return Self::Integer(($op)(v1, v2));
                        } else {
                            unreachable!();
                        }
                    }

                    Self::Float(v1) => {
                        if let Self::Float(v2) = val2 {
                            return Self::Float(($op)(v1, v2));
                        } else {
                            unreachable!();
                        }
                    }
                }
            }
        }
    };
}

gen_binary_op!(Add, add, |a, b| a + b);
gen_binary_op!(Sub, sub, |a, b| a - b);
gen_binary_op!(Mul, mul, |a, b| a * b);
gen_binary_op!(Div, div, |a, b| a / b);

impl ops::Neg for TypedNum {
    type Output = Self;

    fn neg(self) -> Self::Output {
        match self {
            Self::Integer(val) => Self::Integer(val),
            Self::Float(val) => Self::Float(val),
        }
    }
}

impl TypedNum {
    pub fn abs(self) -> Self {
        match self {
            Self::Integer(val) => Self::Integer(val.abs()),
            Self::Float(val) => Self::Float(val.abs()),
        }
    }
}

impl cmp::PartialOrd for TypedNum {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        let (val1, val2) = Self::common_val(*self, *other);

        if let Self::Integer(v1) = val1 {
            if let Self::Integer(v2) = val2 {
                v1.partial_cmp(&v2)
            } else {
                unreachable!();
            }
        } else if let Self::Float(v1) = val1 {
            if let Self::Float(v2) = val2 {
                v1.partial_cmp(&v2)
            } else {
                unreachable!();
            }
        } else {
            unreachable!();
        }
    }
}

impl Display for TypedNum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TypedNum::Integer(val) => write!(f, "int({})", val),
            TypedNum::Float(val) => write!(f, "float({})", val),
        }
    }
}

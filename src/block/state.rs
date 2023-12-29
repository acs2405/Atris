use std::ops::{Sub, Shl, Shr, BitAnd, BitOr, Not};

/// Represents a generic state that is stored as an unsigned integer. Values can be extrated and set with methods `get` and `set`.
/// Operations without a `State` objects can be performed with functions `get_from` and `set_to`.
#[derive(Default, Copy, Clone, Debug)]
pub struct State<T: num::Unsigned> {
    value: T,
}

impl<T> State<T>
    where T: num::Unsigned + 
            Shl<T, Output=T> + 
            Shr<T, Output=T> + 
            Sub<T, Output=T> + 
            BitAnd<T, Output=T> +
            BitOr<T, Output=T> +
            Not<Output=T> +
            From<u8> + 
            Copy {
    /// Creates a new `State` object, with every bit set to 0.
    pub fn new() -> Self {
        Self { value: T::from(0u8) }
    }

    /// Returns the internal value of the `State` object.
    pub fn value(&self) -> T { self.value }

    /// Extracts some bits from `state`. Those bits are between `range.pos` and `range.pos + range.len` (last one not included).
    /// 
    /// ```
    /// use atris::block::state::{State, BitRange};
    /// 
    /// let state = 0b11_10011_01u64;
    /// assert_eq!(State::get_from(0b11_10011_01u64, BitRange{pos: 2, len: 5}), 0b10011u64);
    /// assert_eq!(State::get_from(0b1110011__01u64, BitRange{pos: 2, len: 0}), 0b____0u64);
    /// assert_eq!(State::get_from(0b11_10_01101u64, BitRange{pos: 5, len: 2}), 0b___10u64);
    /// assert_eq!(State::get_from(0b111001_101_u64, BitRange{pos: 0, len: 3}), 0b__101u64);
    /// ```
    pub fn get_from(state: T, range: BitRange) -> T {
        let rshifted_value = state >> range.pos.into();
        let mask: T = (T::from(1u8) << range.len.into()) - T::from(1u8);
        let masked_value: T = rshifted_value & mask;
        masked_value
    }
    
    /// Inserts some bits from `value` to `state` and returns the result. The first `range.len` bits in `value` are set
    /// between `range.pos` and `range.pos + range.len` (last one not included) into `state`.
    /// 
    /// ```
    /// use atris::block::state::{State, BitRange};
    /// 
    /// let state = 0b11_10011_01u64;
    /// assert_eq!(State::set_to(0b11_10011_01u64, BitRange{pos: 2, len: 5}, 0b01000u64), 0b11_01000_01u64);
    /// assert_eq!(State::set_to(0b1110011__01u64, BitRange{pos: 2, len: 0}, 0b___11u64), 0b1110011__01u64);
    /// assert_eq!(State::set_to(0b11_10_01101u64, BitRange{pos: 5, len: 2}, 0b___00u64), 0b11_00_01101u64);
    /// assert_eq!(State::set_to(0b111001_101_u64, BitRange{pos: 0, len: 3}, 0b__110u64), 0b111001_110_u64);
    /// ```
    pub fn set_to(state: T, range: BitRange, value: T) -> T {
        let mask: T = (T::from(1u8) << range.len.into()) - T::from(1u8);
        let masked_value: T = value & mask;
        let lshifted_value: T = masked_value << range.pos.into();
        let excluded_value = state & !(mask << range.pos.into());
        excluded_value | lshifted_value
    }
    
    /// Extracts some bits from `self.value`. Those bits are between `range.pos` and `range.pos + range.len` (last one not included).
    /// 
    /// This method calls `Self::get_from(T, BitRange) -> T` to do the operations.
    /// 
    /// ```
    /// use atris::block::state::{State, BitRange};
    /// 
    /// let state: State<u32> = State::from(0b1110_0110_1u32);
    /// let alive = BitRange{pos: 0, len: 1};
    /// let age = BitRange{pos: 1, len: 4};
    /// let children = BitRange{pos: 5, len: 4};
    /// assert_eq!(state.get(alive), 0b1u32);
    /// assert_eq!(state.get(age), 0b_0110u32);
    /// assert_eq!(state.get(children), 0b_1110u32);
    /// ```
    pub fn get(&self, range: BitRange) -> T {
        Self::get_from(self.value, range)
    }
    
    /// Inserts some bits from `value` to `self.value`. Those bits (the first `range.len` bits in `value`) are set
    /// between `range.pos` and `range.pos + range.len` (last one not included).
    /// 
    /// This method calls `Self::set_to(T, BitRange, T)` to do the operations.
    /// 
    /// ```
    /// use atris::block::state::{State, BitRange};
    /// 
    /// let mut state: State<u32> = State::from(0b1110_0110_1u32);
    /// let alive = BitRange{pos: 0, len: 1};
    /// let age = BitRange{pos: 1, len: 4};
    /// let children = BitRange{pos: 5, len: 4};
    /// state.set(age, state.get(age) + 2);
    /// state.set(alive, 0u32);
    /// state.set(children, 0100u32);
    /// assert_eq!(state.get(alive), 0b0u32);
    /// assert_eq!(state.get(age), 0b_1000u32);
    /// assert_eq!(state.get(children), 0b_0100u32);
    /// assert_eq!(state.value(), 0b0100_1000_0u32);
    /// ```
    pub fn set(&mut self, range: BitRange, value: T) {
        self.value = Self::set_to(self.value, range, value);
    }
}

impl<T, U> From<T> for State<U>
    where T: num::Integer,
        U: num::Unsigned + From<T> {
    /// Creates a new `State` object from an integer number, setting the `State`'s value to the number.
    fn from(value: T) -> Self {
        State { value: value.into() }
    }
}

// impl<T, U> From<State<T>> for (U,)
//     where T: num::Unsigned,
//         U: From<T> + num::Unsigned {
//     fn from(state: State<T>) -> Self {
//         (state.value.into(),)
//     }
// }

/// This struct represents a range (position of the first bit, length of range) of bits in an unsigned integer. This is
/// used for `State`'s operations.
#[derive(Default, Copy, Clone, Debug)]
pub struct BitRange {
    pub pos: u8,
    pub len: u8,
}
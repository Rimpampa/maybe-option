#![feature(generic_const_exprs)]
use std::ops::Deref;

/// A struct that can be either an [`Option<T>`][Option] or a `T` depending on a
/// const generic boolean
pub struct MaybeOption<T, const IS_OPTION: bool>
where
    [(); IS_OPTION as usize]:,
    [(); !IS_OPTION as usize]:,
{
    option: [Option<T>; IS_OPTION as usize],
    value: [T; !IS_OPTION as usize],
}

impl<T> From<T> for MaybeOption<T, false> {
    fn from(v: T) -> Self {
        Self {
            option: [],
            value: [v],
        }
    }
}

impl<T> From<Option<T>> for MaybeOption<T, true> {
    fn from(v: Option<T>) -> Self {
        Self {
            option: [v],
            value: [],
        }
    }
}

impl<T> From<MaybeOption<T, false>> for MaybeOption<T, true> {
    fn from(from: MaybeOption<T, false>) -> Self {
        let [value] = from.value;
        Self {
            option: [Some(value)],
            value: [],
        }
    }
}

impl<T> TryFrom<MaybeOption<T, true>> for MaybeOption<T, false> {
    type Error = ();

    fn try_from(value: MaybeOption<T, true>) -> Result<Self, Self::Error> {
        match value {
            MaybeOption {
                value: [],
                option: [Some(value)],
            } => Ok(Self {
                option: [],
                value: [value],
            }),
            _ => Err(()),
        }
    }
}

impl<T> Deref for MaybeOption<T, true> {
    type Target = Option<T>;

    fn deref(&self) -> &Self::Target {
        &self.option[0]
    }
}

impl<T> Deref for MaybeOption<T, false> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.value[0]
    }
}

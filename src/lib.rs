#![no_std]
#![allow(unreachable_patterns)]
use core::marker::PhantomData;

trait Is<T>: Sized {
    fn mov(self) -> T;
    fn lend(&self) -> &T;
    fn lend_mut(&mut self) -> &mut T;
}

impl<T> Is<T> for T {
    fn mov(self) -> T {
        self
    }

    fn lend(&self) -> &T {
        self
    }

    fn lend_mut(&mut self) -> &mut T {
        self
    }
}

pub trait VArgs {
    type Tuple: Tuple;
}

pub trait Tuple {
    type VArgs: VArgs;
}

trait Index<const I: usize> {
    type Output;
}

pub struct Indexed<T, const I: usize>(T);

struct PhantomIndex<const I: usize>;

trait VArgsErasedOps<const I: usize>: VArgs {
    type Output;
    fn get(&self) -> Option<&Self::Output>;
    fn get_mut(&mut self) -> Option<&mut Self::Output>;
    fn try_unwrap(self) -> Result<Self::Output, Self> where Self: Sized;
}

macro_rules! index_imp {
    ($Name:ident $($index:literal $T:tt)+) => {
        index_imp!($Name | $($index $T)*);
    };
    ($Name:ident $($Tback:ident)* | $index:literal $T:ident $($ifront:literal $Tfront:tt)*) => {
        impl<$($Tback,)* $T, $($Tfront),*> Index<$index> for ($($Tback,)* $T, $($Tfront,)*) {
            type Output = $T;
        }

        impl<$($Tback,)* $T, $($Tfront),*> From<Indexed<$T, $index>> for $Name<$($Tback,)* $T, $($Tfront),*> {
            fn from(Indexed(value): Indexed<$T, $index>) -> Self {
                Self::$T(value)
            }
        }
        
        impl<$($Tback,)* $T, $($Tfront),*> VArgsErasedOps<$index> for $Name<$($Tback,)* $T, $($Tfront),*> {
            type Output = $T;

            fn get(&self) -> Option<&$T> {
                match self {
                    Self::$T(value) => Some(value),
                    _ => None,
                }
            }

            fn get_mut(&mut self) -> Option<&mut $T> {
                match self {
                    Self::$T(value) => Some(value),
                    _ => None,
                }
            }

            fn try_unwrap(self) -> Result<$T, Self> {
                match self {
                    Self::$T(value) => Ok(value),
                    _ => Err(self),
                }
            }
        }

        index_imp!($Name $($Tback)* $T | $($ifront $Tfront)*);
    };
    ($Name:ident $($Tback:ident)* |) => {};
}

macro_rules! vargs_def {
    ($Name:ident) => {
        pub enum $Name {}

        impl VArgs for $Name {
            type Tuple = ();
        }

        impl Tuple for () {
            type VArgs = $Name;
        }
    };

    ($Name:ident $($index:literal: $T:tt $I:tt)+) => {
        pub enum $Name<$($T),*> {
            $($T($T)),*
        }

        impl<$($T),*> VArgs for $Name<$($T),*> {
            type Tuple = ($($T),*,);
        }
        
        impl<$($T),*> Tuple for ($($T,)*) {
            type VArgs = $Name<$($T),*>;
        }

        index_imp!($Name $($index $T)*);

        impl<$($T,)* V, $(const $I: usize,)*> From<$Name<$($T),*>> for (V, PhantomData<($(PhantomIndex<$I>,)*)>)
        where $(V: From<Indexed<$T, $I>>,)* {
            fn from(vargs: $Name<$($T),*>) -> Self {
                (match vargs {
                    $($Name::$T(t) => Indexed(t).into(),)*
                },
                PhantomData)
            }
        }
    };
}

vargs_def!(VArgs0);
vargs_def!(VArgs1  0:T1 I1);
vargs_def!(VArgs2  0:T1 I1 1:T2 I2);
vargs_def!(VArgs3  0:T1 I1 1:T2 I2 2:T3 I3);
vargs_def!(VArgs4  0:T1 I1 1:T2 I2 2:T3 I3 3:T4 I4);
vargs_def!(VArgs5  0:T1 I1 1:T2 I2 2:T3 I3 3:T4 I4 4:T5 I5);
vargs_def!(VArgs6  0:T1 I1 1:T2 I2 2:T3 I3 3:T4 I4 4:T5 I5 5:T6 I6);
vargs_def!(VArgs7  0:T1 I1 1:T2 I2 2:T3 I3 3:T4 I4 4:T5 I5 5:T6 I6 6:T7 I7);
vargs_def!(VArgs8  0:T1 I1 1:T2 I2 2:T3 I3 3:T4 I4 4:T5 I5 5:T6 I6 6:T7 I7 7:T8 I8);
vargs_def!(VArgs9  0:T1 I1 1:T2 I2 2:T3 I3 3:T4 I4 4:T5 I5 5:T6 I6 6:T7 I7 7:T8 I8 8:T9 I9);
vargs_def!(VArgs10 0:T1 I1 1:T2 I2 2:T3 I3 3:T4 I4 4:T5 I5 5:T6 I6 6:T7 I7 7:T8 I8 8:T9 I9 9:T10 I10);
vargs_def!(VArgs11 0:T1 I1 1:T2 I2 2:T3 I3 3:T4 I4 4:T5 I5 5:T6 I6 6:T7 I7 7:T8 I8 8:T9 I9 9:T10 I10 10:T11 I11);
vargs_def!(VArgs12 0:T1 I1 1:T2 I2 2:T3 I3 3:T4 I4 4:T5 I5 5:T6 I6 6:T7 I7 7:T8 I8 8:T9 I9 9:T10 I10 10:T11 I11 11:T12 I12);

pub struct Variant<Args: Tuple> {
    inner: Args::VArgs,
}

impl<Args: Tuple> Variant<Args> {
    pub fn new<T, const I: usize>(value: T) -> Self where Args::VArgs: From<Indexed<T, I>> {
        Self {
            inner: Indexed(value).into()
        }
    }

    pub fn cast_from<OtherArgs: Tuple, P>(v: Variant<OtherArgs>) -> Self where (Args::VArgs, P): From<OtherArgs::VArgs> {
        let (inner, _) = v.inner.into();
        Self { inner }
    }

    pub fn cast_into<OtherArgs: Tuple, P>(self) -> Variant<OtherArgs> where (OtherArgs::VArgs, P): From<Args::VArgs> {
        Variant::cast_from(self)
    }
}

trait IntoVariant<const I: usize>: Sized {
    fn into_variant<Args: Tuple>(self) -> Variant<Args> where Args::VArgs: From<Indexed<Self, I>> {
        Variant::new(self)
    }
}

impl<T, const I: usize> IntoVariant<I> for T {}

trait VariantErasedOps<const I: usize> {
    type VArgs;
    type Output;
    fn get<T>(&self) -> Option<&T> where Self::VArgs: From<Indexed<T, I>>, Self::Output: Is<T>;
    fn get_mut<T>(&mut self) -> Option<&mut T> where Self::VArgs: From<Indexed<T, I>>, Self::Output: Is<T>;
    fn try_unwrap<T>(self) -> Result<T, Self> where Self::VArgs: From<Indexed<T, I>>, Self::Output: Is<T>, Self: Sized;
}

impl<Args: Tuple, const I: usize> VariantErasedOps<I> for Variant<Args>
where
    Args::VArgs: VArgsErasedOps<I>,
{
    type VArgs = Args::VArgs;
    type Output = <Args::VArgs as VArgsErasedOps<I>>::Output;

    fn get<T>(&self) -> Option<&T> where Self::VArgs: From<Indexed<T, I>>, Self::Output: Is<T> {
        self.inner.get().map(Is::lend)
    }

    fn get_mut<T>(&mut self) -> Option<&mut T> where Self::VArgs: From<Indexed<T, I>>, Self::Output: Is<T> {
        self.inner.get_mut().map(Is::lend_mut)
    }

    fn try_unwrap<T>(self) -> Result<T, Self> where Self::VArgs: From<Indexed<T, I>>, Self::Output: Is<T> {
        self.inner.try_unwrap().map(Is::mov).map_err(|inner| Self { inner })
    }
}
use std::any::Any;

/// Supports conversion to 'Any'. Traits to be extended by 'downcast_impl!' must extend 'Downcast'.
pub trait Downcast: Any {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

impl<T: Any> Downcast for T {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

/// Adds downcasting support to traits that extend 'downcast::Downcast' by defining forwarding
/// methods to the corresponding implementations on 'std::any::Any' in the standard library.
#[macro_export]
macro_rules! impl_downcast {
    (@$trait_:ident) => {
        impl $trait_ {
            /// Returns true if the boxed type is the same as '_T'.
            #[inline]
            pub fn is<_T: $trait_>(&self) -> bool {
                crate::Downcast::as_any(self).is::<_T>()
            }

            /// Return a reference to the boxed value if it is of type '_T',
            /// 'None' if it isn't
            #[inline]
            pub fn downcast_ref<_T: $trait_>(&self) -> Option<&_T> {
                crate::Downcast::as_any(self).downcast_ref::<_T>()
            }

            ///  Return a mutable referce to the boxed value if it is of type '_T',
            /// or 'None' if it isn't
            #[inline]
            pub fn downcast_mut<_T: $trait_>(&mut self) -> Option<&mut _T> {
                crate::Downcast::as_any_mut(self).downcast_mut::<_T>()
            }
        }
    };
    (@$trait_:ident [$($args:ident,)*]) => {
        impl<$($args),*> dyn $trait_<$($args),*> {
            /// Returns true if the boxed type is the same as 'T'.
            #[inline]
            pub fn is<_T: $trait_<$($args), *>>(&self) -> bool {
                crate::Downcast::as_any(self).is::<_T>()
            }

            /// Returns a reference to the boxed value if it is of type'T', or
            /// 'None' if it isn't.
            #[inline]
            pub fn downcast_ref<_T: $trait_<$($args),*>>(&self) -> Option<&_T> {
                crate::Downcast::as_any(self).downcast_ref::<_T>()
            }

            /// Returns a mutable reference to the boxed value if it is of type
            /// 'T', or 'None' if it isn't.
            #[inline]
            pub fn downcast_mut<_T: $trait_<$($args),*>>(&mut self) -> Option<&mut _T> {
                crate::Downcast::as_any_mut(self).downcast_mut::<_T>()
            }
        }
    };

    (concrete @$trait_:ident [$($args:ident),* $(,)*]) => {
        impl $trait_<$($args),*> {

            #[inline]
            pub fn is<_T: $trait_ <$($args),*>>(&self) -> bool {
                crate::Downcast::as_any(self).is::<_T>()
            }

            #[inline]
            pub fn downcast_ref<_T: $trait_<$($args),*>>(&self) -> Option<&_T> {
                crate::Downcast::as_any(self).downcast_ref::<_T>()
            }

            #[inline]
            pub fn downcast_mut<_T: $trait_<$($args),*>>(&mut self) -> Option<&mut _T> {
                crate::Downcast::as_any_mut(self).downcast_mut::<_T>()
            }
        }
    };

    ($trait_:ident <>) => {impl_downcast! {@$trait_ }};
    ($trait_:ident < $($args:ident),* $(,)*>) => {impl_downcast! {@$trait_[$($args,)*]}};
    ($trait_:ident) => {impl_downcast! {@$trait_ }};
    (concrete $trait_:ident <$($args:ident),* $(,)*>) => {impl_downcast! {concrete @$trait_[$($args,)*]}};
}


#[cfg(test)]
mod test {
    mod non_generic {
        use Downcast;

        // A trait that can be downcast.
        trait Base: Downcast {}
        impl_downcast!(Base);

        // Concrete type implementing Base.
        struct Foo(u32);

        impl Base for Foo {}

        // Functions that can work on references to Base trait objects.
        fn get_val(base: &Box<dyn Base>) -> u32 {
            match base.downcast_ref::<Foo>() {
                Some(val) => val.0,
                None => 0
            }
        }

        fn set_val(base: &mut Box<dyn Base>, val: u32) {
            if let Some(foo) = base.downcast_mut::<Foo>() {
                foo.0 = val;
            }
        }

        #[test]
        fn test() {
            let mut base: Box<dyn Base> = Box::new(Foo(42));
            assert_eq!(get_val(&base), 42);

            set_val(&mut base, 6 * 9);
            assert_eq!(get_val(&base), 6 * 9);

            assert!(base.is::<Foo>());
        }
    }

    mod generic {
        use Downcast;

        trait Base<T>: Downcast {}
        impl_downcast!(Base<T>);

        struct Foo(u32);

        impl Base<u32> for Foo {}

        /// Functions that can work on references to Base trait objects.
        fn get_val(base: &Box<Base<u32>>) -> u32 {
            match base.downcast_ref::<Foo>() {
                Some(val) => val.0,
                None => 0
            }
        }

        fn set_val(base: &mut Box<Base<u32>>, val: u32) {
            if let Some(foo) = base.downcast_mut::<Foo>() {
                foo.0 = val;
            }
        }

        #[test]
        fn test() {
            let mut base: Box<Base<u32>> = Box::new(Foo(42));
            assert_eq!(get_val(&base), 42);

            set_val(&mut base, 6 * 9);
            assert_eq!(get_val(&base), 6 * 9);

            assert!(base.is::<Foo>());
        }
    }

    mod concrete {
        use super::super::Downcast;

        // A trait than can be downcast.
        trait Base<T>: Downcast {}
        impl_downcast!(concrete Base<u32>);

        // Concrete type implementing Base.
        struct Foo(u32);

        impl Base<u32> for Foo {}

        // Functions that can work on references to Base trait objects.
        fn get_val(base: &Box<Base<u32>>) -> u32 {
            match base.downcast_ref::<Foo>() {
                Some(val) => val.0,
                None => 0
            }
        }

        fn set_val(base: &mut Box<Base<u32>>, val: u32) {
            if let Some(foo) = base.downcast_mut::<Foo>() {
                foo.0 = val;
            }
        }

        #[test]
        fn test() {
            let mut base: Box<Base<u32>> = Box::new(Foo(42));
            assert_eq!(get_val(&base), 42);

            set_val(&mut base, 54);
            assert_eq!(get_val(&base), 54);

            assert!(base.is::<Foo>());
        }
    }
}
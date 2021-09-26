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
macro_rules! downcast_impl {
    ($trait_:ident) => {
        impl dyn $trait_ {
            /// Returns true if the boxed type is the same as 'T'.
            #[inline]
            pub fn is<T: $trait_>(&self) -> bool {
                crate::Downcast::as_any(self).is::<T>()
            }

            /// Returns a reference to the boxed value if it is of type'T', or
            /// 'None' if it isn't.
            #[inline]
            pub fn downcast_ref<T: $trait_>(&self) -> Option<&T> {
                crate::Downcast::as_any(self).downcast_ref::<T>()
            }

            /// Returns a mutable reference to the boxed value if it is of type
            /// 'T', or 'None' if it isn't.
            #[inline]
            pub fn downcast_mut<T: $trait_>(&mut self) -> Option<&mut T> {
                crate::Downcast::as_any_mut(self).downcast_mut::<T>()
            }
        }
    }
}


#[cfg(test)]
mod test {
    use Downcast;

    // A trait that can be downcast.
    trait Base: Downcast {}
    downcast_impl!(Base);

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
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
        impl_downcast! {
            @as_item
            impl dyn $trait_ {
                impl_downcast! {@impl_body @$trait_ [] }
            }
        }
    };
    
    (@$trait_:ident [$($args:ident,)*]) => {
        impl_downcast! {
            @as_item
            impl<$($args),*> dyn $trait_<$($args),*>
                where $($args: ::std::any::Any + 'static),*
            {
                impl_downcast! {@impl_body @$trait_ [$($args,)*]}
            }
        }
    };
    
    
    (@$trait_:ident [$($args:ident,)*] where [$($preds:tt)+]) => {
        impl_downcast! {
            @as_item
            impl<$($args),*> dyn $trait_<$($args),*>
                where $($args: ::std::any::Any + 'static,)*
                      $($preds)*
            {
                impl_downcast! {@impl_body @$trait_ [$($args,)*]}
            }
        }
    };

    (concrete @$trait_:ident [$($args:ident),* $(,)*]) => {
        impl_downcast! {
            @as_item
            impl dyn $trait_<$($args),*> {
                impl_downcast! {@impl_body @$trait_ [$($args,)*]}
            }
        }
    };
    
    (@impl_body @$trait_:ident [$($args:ident,)*]) => {
        #[inline]
        pub fn is<_T: $trait_<$($args),*>>(&self) -> bool {
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
    };

    (@as_item $i:item) => {$i};
    ($trait_:ident <>) => {impl_downcast! {@$trait_ }};
    ($trait_:ident < $($args:ident),* $(,)*>) => {impl_downcast! {@$trait_[$($args,)*]}};
    ($trait_:ident) => {impl_downcast! {@$trait_ }};
    (concrete $trait_:ident <$($args:ident),* $(,)*>) => {impl_downcast! {concrete @$trait_[$($args,)*]}};
    ($trait_:ident < $($args:ident),* $(,)* > where $($preds:tt)+) => {
        impl_downcast! {@$trait_ [$($args,)*] where [$($preds)*]}
    };
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
        fn get_val(base: &Box<dyn Base<u32>>) -> u32 {
            match base.downcast_ref::<Foo>() {
                Some(val) => val.0,
                None => 0
            }
        }
    
        fn set_val(base: &mut Box<dyn Base<u32>>, val: u32) {
            if let Some(foo) = base.downcast_mut::<Foo>() {
                foo.0 = val;
            }
        }

        #[test]
        fn test() {
            let mut base: Box<dyn Base<u32>> = Box::new(Foo(42));
            assert_eq!(get_val(&base), 42);
    
            set_val(&mut base, 6 * 9);
            assert_eq!(get_val(&base), 6 * 9);
    
            assert!(base.is::<Foo>());
        }
    }
    
    mod constrained_generic {
        use Downcast;
    
        trait Base<T: Copy>: Downcast {}
        impl_downcast!(Base<T> where T: Copy);
        
        struct Foo(u32);
        
        impl Base<u32> for Foo {}
        
        fn get_val(base: &Box<dyn Base<u32>>) -> u32 {
            match base.downcast_ref::<Foo>() {
                Some(val) => val.0,
                None => 0
            }
        }
        
        fn set_val(base: &mut Box<dyn Base<u32>>, val: u32) {
            if let Some(foo) = base.downcast_mut::<Foo>() {
                foo.0 = val;
            }
        }
        
        #[test]
        fn test() {
            let mut base: Box<dyn Base<u32>> = Box::new(Foo(42));
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
        fn get_val(base: &Box<dyn Base<u32>>) -> u32 {
            match base.downcast_ref::<Foo>() {
                Some(val) => val.0,
                None => 0
            }
        }
        
        fn set_val(base: &mut Box<dyn Base<u32>>, val: u32) {
            if let Some(foo) = base.downcast_mut::<Foo>() {
                foo.0 = val;
            }
        }

        #[test]
        fn test() {
            let mut base: Box<dyn Base<u32>> = Box::new(Foo(42));
            assert_eq!(get_val(&base), 42);

            set_val(&mut base, 54);
            assert_eq!(get_val(&base), 54);

            assert!(base.is::<Foo>());
        }
    }
}
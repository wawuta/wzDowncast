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
    (@impl_full
        $trait_:ident [$($param_types:tt)*]
        for [$($forall_types:ident),*]
        where [$($preds:tt)*]
    ) => {
        impl_downcast! {
            @inject_where
                [impl<$($forall_types),*> $trait_<$($param_types)*>]
                types [$($forall_types),*]
                where [$($preds)*]
                [{
                    impl_downcast! {@impl_body $trait_ [$($param_types)*]}
                }]
        }
    };
    
    (@impl_body $trait_:ident [$($types:tt)*]) => {
        #[inline]
        pub fn is<_T: $trait_<$($types),*>>(&self) -> bool {
            crate::Downcast::as_any(self).is::<_T>()
        }
        
        #[inline]
        pub fn downcast_ref<_T: $trait_<$($types),*>>(&self) -> Option<&_T> {
            crate::Downcast::as_any(self).downcast_ref::<_T>()
        }
        
        #[inline]
        pub fn downcast_mut<_T: $trait_<$($types),*>>(&mut self) -> Option<&mut _T> {
            crate::Downcast::as_any_mut(self).downcast_mut::<_T>()
        }
    };
    
    (@inject_where [$($before:tt)*] types [] where [] [$($after:tt)*]) => {
	    impl_downcast! {@as_item $($before)* $($after)*}
    };
    
    (@inject_where [$($before:tt)*] types [$($types:ident),*] where [] [$($after:tt)*]) =>{
        impl_downcast! {
            @as_item
                $($before)*
                where $($types: ::std::any::Any + 'static),*
                $($after)*
        }
    };
    
    (@inject_where [$($before:tt)*] types [$($types:ident),*] where [$($preds:tt)+] [$($after:tt)*]) => {
	    impl_downcast! {
            @as_item
                $($before)*
            where
                $($types: ::std::any::Any + 'static,)*
                $($preds)*
            $($after)*
        }
    };

    (@as_item $i:item) => {$i};
    
    // No type parameters.
    ($trait_:ident) => {impl_downcast! {@impl_full $trait_ [] for [] where []}};
    ($trait_:ident <>) => {impl_downcast! {@impl_full $trait_ [] for [] where []}};
    // Type parameters.
    ($trait_:ident < $($types:ident),*>) => {
        impl_downcast! {@impl_full $trait_ [$($types),*] for [$($types),*] where []}
    };
    // Type parameters and where clauses.
    ($trait_:ident <$($types:ident),*> where $($preds:tt)+) => {
        impl_downcast! {@impl_full $trait_ [$($types),*] for [$($types),*] where [$($preds)*]}
    };
    // Concretely-parametrized types.
    (concrete $trait_:ident <$($types:ident),*>) => {
        impl_downcast! {@impl_full $trait_ [$($types),*] for [] where[]}
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
        
        struct Bar(f64);
        
        impl Base for Bar {}
        
        // Functions that can work on references to Base trait objects.
        fn get_val(base: &Box<dyn Base>) -> f64 {
            match base.downcast_ref::<Bar>() {
                Some(val) => val.0,
                None => 0.0
            }
        }
        
        fn set_val(base: &mut Box<dyn Base>, val: f64) {
            if let Some(bar) = base.downcast_mut::<Bar>() {
                bar.0 = val;
            }
        }
        
        #[test]
        fn test() {
            let mut base: Box<dyn Base> = Box::new(Bar(42.0));
            assert_eq!(get_val(&base), 42.0);
            
            if let Some(foo) = base.downcast_ref::<Foo>() {
                assert_eq!(foo.0, 42);
            }
            if let Some(bar) = base.downcast_ref::<Bar>() {
                assert_eq!(bar.0, 42.0)
            }
            
            set_val(&mut base, 6.0 * 9.0);
            assert_eq!(get_val(&base), 6.0 * 9.0);
            
            assert!(base.is::<Bar>());
        }
    }
    
    mod generic {
        use Downcast;
        
        trait Base<T>: Downcast {}
        impl_downcast!(Base<T>);
        
        struct Foo(u32);
        
        impl Base<u32> for Foo {}
        
        struct Bar(f64);
        
        impl Base<u32> for Bar {}
        
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
    
            if let Some(foo) = base.downcast_ref::<Foo>() {
                assert_eq!(foo.0, 42);
            } else if let Some(bar) = base.downcast_ref::<Bar>() {
                assert_eq!(bar.0, 42.0);
            }
    
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
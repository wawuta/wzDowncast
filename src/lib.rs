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
                [impl<$($forall_types),*> dyn $trait_<$($param_types)*>]
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
    macro_rules! test_mod {
        (
            $test_name:ident,
            trait $base_trait:ty {$($base_impl:tt)*},
            type $base_type:ty,
            {$($def:tt)*}
        ) => {
            mod $test_name {
                use super::super::Downcast;
                
                // A trait that can be downcast.
                $($def)*
                
                // Concrete type implementing Base.
                struct Foo(u32);
                impl $base_trait for Foo {$($base_impl)*}
                struct Bar(f64);
                impl $base_trait for Bar {$($base_impl)*}
                
                // Functions that can work on references to Base trait objects.
                fn get_val(base: &Box<$base_type>) -> u32 {
                    match base.downcast_ref::<Foo>() {
                        Some(val) => val.0,
                        None => 0
                    }
                }
                
                fn set_val(base: &mut Box<$base_type>, val: u32) {
                    if let Some(foo) = base.downcast_mut::<Foo>() {
                        foo.0 = val;
                    }
                }
                
                #[test]
                fn test() {
                    let mut base: Box<$base_type> = Box::new(Foo(42));
                    assert_eq!(get_val(&base), 42);
                    
                    if let Some(foo) = base.downcast_ref::<Foo>() {
                        assert_eq!(foo.0, 42);
                    } else if let Some(bar) = base.downcast_ref::<Bar>() {
                        assert_eq!(bar.0, 42.0);
                    }
                    
                    set_val(&mut base, 6*9);
                    assert_eq!(get_val(&base), 6*9);
                    
                    assert!(base.is::<Foo>());
                }
            }
        };
        
        (
            $test_name:ident,
            trait $base_trait:ty {$($base_impl:tt)*},
            {$($def:tt)+}
        ) => {
            test_mod! {
                $test_name, trait $base_trait {$($base_impl)*}, type $base_trait, {$($def)*}
            }
        }
    }
    
    test_mod!(non_generic, trait Base {}, {
        trait Base: Downcast{}
        impl_downcast!(Base);
    });
    
    test_mod!(generic, trait Base<u32> {}, {
        trait Base<T>: Downcast{}
        impl_downcast!(Base<T>);
    });
    
    test_mod!(constrained_generic, trait Base<u32> {}, {
        trait Base<T: Copy>: Downcast {}
        impl_downcast!(Base<T> where T: Copy);
    });
    
    test_mod!(concrete_parametrized, trait Base<u32> {}, {
        trait Base<T>: Downcast {}
        impl_downcast!(concrete Base<u32>);
    });
}
macro_rules! deref {
    ($struct_name:ident<T>::$field_name:ident => T) => (
        impl<T> ::std::ops::Deref for $struct_name<T> {
            type Target = T;

            #[inline]
            fn deref(&self) -> &Self::Target {
                &self.$field_name
            }
        }

        impl<T> ::std::ops::DerefMut for $struct_name<T> {
            #[inline]
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.$field_name
            }
        }
    );
    ($struct_name:ident::$field_name:ident => $target:ty) => (
        impl ::std::ops::Deref for $struct_name {
            type Target = $target;

            #[inline]
            fn deref(&self) -> &Self::Target {
                &self.$field_name
            }
        }

        impl ::std::ops::DerefMut for $struct_name {
            #[inline]
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.$field_name
            }
        }
    );
}

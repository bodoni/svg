macro_rules! deref {
    ($kind:ident<T>::$field:ident => T) => (
        impl<T> ::std::ops::Deref for $kind<T> {
            type Target = T;

            #[inline]
            fn deref(&self) -> &Self::Target {
                &self.$field
            }
        }

        impl<T> ::std::ops::DerefMut for $kind<T> {
            #[inline]
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.$field
            }
        }
    );
    ($kind:ident::$field:ident => $target:ty) => (
        impl ::std::ops::Deref for $kind {
            type Target = $target;

            #[inline]
            fn deref(&self) -> &Self::Target {
                &self.$field
            }
        }

        impl ::std::ops::DerefMut for $kind {
            #[inline]
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.$field
            }
        }
    );
}

macro_rules! node {
    ($(#[$attribute:meta])* pub $name:ident) => (
        $(#[$attribute])*
        #[derive(Clone, Debug)]
        pub struct $name {
            node: ::node::Node,
        }

        deref! { $name::node => ::node::Node }
    );
}

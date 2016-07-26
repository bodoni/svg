macro_rules! deref {
    ($struct_name:ident::0 => $target:ty) => (
        impl ::std::ops::Deref for $struct_name {
            type Target = $target;

            #[inline]
            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl ::std::ops::DerefMut for $struct_name {
            #[inline]
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.0
            }
        }
    );
}

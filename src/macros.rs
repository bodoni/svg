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

macro_rules! node {
    (
        @implement(Attributes) $struct_name:ident
        $($attribute_setter:ident [$attribute_name:expr] [$($attribute_type:tt)*],)*
    ) => (
        impl $struct_name {
            /// Get an attribute.
            #[inline]
            pub fn get<T: AsRef<str>>(&self, name: T) -> Option<&str> {
                self.attributes.get(name)
            }

            /// Set an attribute.
            #[inline]
            pub fn set<T: Into<String>>(mut self, name: T, value: T) -> Self {
                self.attributes.set(name, value);
                self
            }

            $(
                #[inline]
                pub fn $attribute_setter<T>(self, _: T) -> Self {
                    self
                }
            )*
        }
    );
    (@implement(Base) $struct_name:ident) => (
        impl $struct_name {
            /// Create an instance.
            #[inline]
            pub fn new() -> Self {
                Default::default()
            }
        }

        impl ::node::Node for $struct_name {
        }
    );
    (@implement(Children) $struct_name:ident) => (
        impl $struct_name {
            /// Append a node.
            pub fn append<T: 'static + ::node::Node>(mut self, node: T) -> Self {
                self.children.append(node);
                self
            }
        }
    );
    (@implement(Display) @empty $struct_name:ident($tag_name:expr)) => (
        impl ::std::fmt::Display for $struct_name {
            fn fmt(&self, formatter: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                if self.attributes.is_empty() {
                    write!(formatter, "<{tag}/>",
                           tag=$tag_name)
                } else {
                    write!(formatter, "<{tag} {attributes}/>",
                           tag=$tag_name, attributes=self.attributes)
                }
            }
        }
    );
    (@implement(Display) $struct_name:ident($tag_name:expr)) => (
        impl ::std::fmt::Display for $struct_name {
            fn fmt(&self, formatter: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                if self.attributes.is_empty() {
                    write!(formatter, "<{tag}>\n{children}\n</{tag}>",
                           tag=$tag_name, children=self.children)
                } else {
                    write!(formatter, "<{tag} {attributes}>\n{children}\n</{tag}>",
                           tag=$tag_name, attributes=self.attributes, children=self.children)
                }
            }
        }
    );
    (
        @empty
        $(#[$attribute:meta])*
        pub struct $struct_name:ident($tag_name:expr) {
            $($attribute_setter:ident [$attribute_name:expr] [$($attribute_type:tt)*],)*
        }
    ) => (
        $(#[$attribute])*
        #[derive(Clone, Debug, Default)]
        pub struct $struct_name {
            attributes: ::node::Attributes,
        }

        node! { @implement(Base) $struct_name }
        node! { @implement(Display) @empty $struct_name($tag_name) }
        node! {
            @implement(Attributes) $struct_name
            $($attribute_setter [$attribute_name] [$($attribute_type)*],)*
        }
    );
    (
        $(#[$attribute:meta])*
        pub struct $struct_name:ident($tag_name:expr) {
            $($attribute_setter:ident [$attribute_name:expr] [$($attribute_type:tt)*],)*
        }
    ) => (
        $(#[$attribute])*
        #[derive(Debug, Default)]
        pub struct $struct_name {
            attributes: ::node::Attributes,
            children: ::node::Children,
        }

        node! { @implement(Base) $struct_name }
        node! { @implement(Display) $struct_name($tag_name) }
        node! { @implement(Children) $struct_name }
        node! {
            @implement(Attributes) $struct_name
            $($attribute_setter [$attribute_name] [$($attribute_type)*],)*
        }
    );
}

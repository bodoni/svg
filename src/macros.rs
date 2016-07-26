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
    ($(#[$attribute:meta])* pub struct $struct_name:ident($tag:expr, $leaf:expr)) => (
        $(#[$attribute])*
        #[derive(Debug)]
        pub struct $struct_name(::node::Node);

        impl $struct_name {
            /// Create a node.
            #[inline]
            pub fn new() -> Self {
                $struct_name(::node::Node::new($tag, $leaf))
            }

            /// Append a node.
            pub fn add<T: Into<::node::Node>>(mut self, node: T) -> Self {
                self.0.append(node);
                self
            }

            /// Assign an attribute.
            #[inline]
            pub fn set<T: Into<String>, U: ::node::Value>(mut self, name: T, value: U) -> Self {
                self.0.assign(name, value);
                self
            }
        }

        impl ::std::fmt::Display for $struct_name {
            #[inline]
            fn fmt(&self, formatter: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                self.0.fmt(formatter)
            }
        }

        impl From<$struct_name> for ::node::Node {
            #[inline]
            fn from(node: $struct_name) -> ::node::Node {
                node.0
            }
        }
    );
}

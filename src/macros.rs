macro_rules! node(
    ($struct_name:ident::$field_name:ident) => (
        impl $struct_name {
            /// Append a node.
            pub fn add<T>(mut self, node: T) -> Self where T: ::node::Node {
                ::node::Node::append(&mut self, node);
                self
            }

            /// Assign an attribute.
            #[inline]
            pub fn set<T, U>(mut self, name: T, value: U) -> Self
                where T: Into<String>, U: Into<::node::Value>
            {
                ::node::Node::assign(&mut self, name, value);
                self
            }
        }

        impl ::node::Node for $struct_name {
            #[inline]
            fn append<T>(&mut self, node: T) where T: ::node::Node {
                self.$field_name.append(node);
            }

            #[inline]
            fn assign<T, U>(&mut self, name: T, value: U)
                where T: Into<String>, U: Into<::node::Value>
            {
                self.$field_name.assign(name, value);
            }
        }

        impl ::std::fmt::Display for $struct_name {
            #[inline]
            fn fmt(&self, formatter: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                self.$field_name.fmt(formatter)
            }
        }
    );
);

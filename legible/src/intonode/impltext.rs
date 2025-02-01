use crate::{IntoNode, Node, Text};

macro_rules! text_into_node {
    ( $t:ty ) => {
        impl IntoNode for $t {
            fn into_node(self) -> Node {
                Text::from(self).into_node()
            }
        }
    };

    ( ref $t:ty ) => {
        impl<'a> IntoNode for &'a $t {
            fn into_node(self) -> Node {
                Text::from(self).into_node()
            }
        }
    };
}

text_into_node!(char);
text_into_node!(String);
text_into_node!(ref str);

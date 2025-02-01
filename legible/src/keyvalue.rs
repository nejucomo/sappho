use derive_more::{From, Into};
use derive_new::new;

use crate::{HeadAndTail, IntoNode, Node};

/// A `K: V`-style construct which can wrap-and-indent after the colon
#[derive(Copy, Clone, Debug, From, Into, new)]
pub struct KeyValue<K, V>(K, V);

impl<K, V> IntoNode for KeyValue<K, V>
where
    K: IntoNode,
    V: IntoNode,
{
    fn into_node(self) -> Node {
        let KeyValue(k, v) = self;
        HeadAndTail::new((k, ":"), " ", v).into_node()
    }
}

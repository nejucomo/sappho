use std::fmt;
use std::rc::Rc;

use crate::node::Node;

#[derive(Debug)]
pub struct List<T>(Option<Rc<Node<T>>>);

impl<T> List<T> {
    pub fn iter(&self) -> impl Iterator<Item = &T> + Into<Self> + Into<&Self> {
        ListIter { listptr: self }
    }

    pub fn prepend(&self, elem: T) -> Self {
        List(Some(Rc::new(Node::new(elem, self.clone()))))
    }

    pub fn length(&self) -> usize {
        self.map_ref(|node| node.length()).unwrap_or_default()
    }

    fn ref_node(&self) -> Option<&Node<T>> {
        self.map_ref(|rcnode| rcnode.as_ref())
    }

    fn map_ref<'a, F, U>(&'a self, f: F) -> Option<U>
    where
        F: FnOnce(&'a Rc<Node<T>>) -> U,
    {
        self.0.as_ref().map(f)
    }
}

impl<T> Default for List<T> {
    fn default() -> Self {
        List(None)
    }
}

impl<T> Clone for List<T> {
    fn clone(&self) -> Self {
        List(self.0.clone())
    }
}

impl<T> PartialEq for List<T>
where
    T: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.length() == other.length() && self.ref_node() == other.ref_node()
    }
}

impl<T> fmt::Display for List<T>
where
    T: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        struct DebugDisp<T>(T);

        impl<T> fmt::Debug for DebugDisp<T>
        where
            T: fmt::Display,
        {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                self.0.fmt(f)
            }
        }

        f.debug_list().entries(self.iter().map(DebugDisp)).finish()
    }
}

#[derive(Debug)]
pub struct ListIter<'a, T> {
    listptr: &'a List<T>,
}

impl<'a, T> Iterator for ListIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.listptr.map_ref(|node| {
            self.listptr = node.tail();
            node.elem()
        })
    }
}

impl<'a, T> From<ListIter<'a, T>> for &'a List<T> {
    fn from(li: ListIter<'a, T>) -> Self {
        li.listptr
    }
}

impl<T> From<ListIter<'_, T>> for List<T> {
    fn from(li: ListIter<'_, T>) -> Self {
        li.listptr.clone()
    }
}

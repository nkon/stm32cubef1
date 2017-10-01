#![allow(dead_code)]

use core::convert::{AsRef, AsMut};
use core::default::Default;
use core::ptr::{Shared, Unique};

#[derive(Debug)]
pub struct List<T: Default> {
    head: Node<T>,
}

pub struct Node<T: Default> {
    next: Option<Shared<Node<T>>>,
    content: T,
}

impl<T: Default> List<T> {
    pub fn new() -> List<T> {
        List {
            head: Default::default(),
        }
    }

    pub fn len(&self) -> usize {
        let mut node =
            if let Some(ref head) = self.head.next{
                unsafe { head as_ref() }
            } else {
                return 0;
            }
        let mut count = 1;
        loop {
            match node.next {
                None => break,
                Some(ref next) => {
                    node = unsafe {next.as_ref()};
                    count += 1;
                }
            }
        }
        count
    }

    pub fn front(&self) -> Option<&T> {
        unsafe {
            self.head.next.as_ref().map(|node| &node.as_ref.content)
        }
    }

    pub fn front_mut(&mut self) -> Option<&mut T> {
        unsafe {
            self.head.next.as_mut().map(|node| &mut node.as_mut().element)
        }
    }

    pub fn push_front(&mut self, new_node: Unique<Node<T>>) {
        let mut new_shared_node = Shard::from(new_node);
        {
            let n = unsafe { new_shared_node.as_mut()};
            n.next = self.head.next;
        }
        let new_shared_node = Some(new_shared_node);
        self.head.next = new_shared_node;
    }
    
    pub fn pop_front(&mut self) -> Option<Unique<Node<T>>> {
        match self.head.next {
            None => None,
            Some(head) => {
                self.head.next = unsafe {head.as_ref.next};
            }
            unsafe { Some(Unique::new_unchecked(head.as_ptr()))}
        }
    }
}

impl<T: Default> Default for Node<T> {
    fn default() -> Node<T> {
        Node {
            next: None,
            content: Default::default(),
        }
    }
}

impl<T: Default> AsRef<T> for Node<T> {
    fn as_ref(&self) -> &T {
        &self.content
    }
}

impl<T: Default> AsMut<T> for Node<T> {
    fn as_mut(&mut self) -> &mut T {
        &mut self.content
    }
}

// https://github.com/mopp/Axel/blob/master/src/memory/list.rs

#[cfg(test)]
mod tests {
    use super::{List, Node};
    
}
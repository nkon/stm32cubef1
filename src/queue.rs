#![allow(dead_code)]

use core;
//use lock::Lock;
use nostd_tool::lock;
use nostd_tool::lock::Lock;

#[derive(Debug)]
pub enum ErrorKind {
    NoSpace,
}

#[derive(Debug)]
pub struct Queue<'a, T: 'a + Copy> {
    memory: &'a mut[T],
    len: usize,
    lock: Lock,
}

use core::hash::{Hash, Hasher};
use core::ops;
pub use core::slice::Iter;
pub use core::slice::IterMut;

impl<'a, T> Queue<'a, T> where T: 'a + Copy {
    pub fn new(memory: &'a mut [T]) -> Self {
        Queue {
            memory:memory,
            len:0,
            lock: Lock::Unlocked,
        }
    }

    #[inline]
    pub fn capacity(&self) -> usize {
        self.memory.len()
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.len
    }

    #[inline]
    pub fn available(&self) -> usize {
        self.capacity() - self.len()
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    #[inline]
    pub fn get_lock(&mut self) {
        self.lock.get_lock()
    }

    #[inline]
    pub fn unlock(&mut self) {
        self.lock.unlock()
    }

    #[inline]
    pub fn as_slice(&self) -> &[T] {
        &self.memory[..self.len]
    }

    #[inline]
    pub fn as_mut_slice(&mut self) -> &mut [T] {
        &mut self.memory[..self.len]
    }

    pub fn peek(&self, index: usize) -> T {
        assert!(index <= self.len);
        self.memory[index]
    }

    /// index 番目に element を挿入する
    pub fn insert(&mut self, index: usize, element: T) {
        assert!(index <= self.len);
        if index == self.len || self.len == 0 {
            self.push(element);
        } else if self.available() >= 1 {
            self.lock.get_lock();
            self.len += 1;
            let mut i = self.len;
            loop {
                if i == index {
                    break;
                }
                self.memory[i] = self.memory[i - 1];
                i -= 1;
            }
            self.memory[index] = element;
            self.lock.unlock();
        }
    }

    /// index 番目の要素を入れ替える(古い要素を返す)
    pub fn replace(&mut self, index: usize, element: T) -> T {
        assert!(index < self.len);
        let ret = self.memory[index];
        self.memory[index] = element;
        ret
    }

    /// index 番目の要素を取り出して返す(元の要素は削除される)
    pub fn remove(&mut self, index: usize) -> T {
        assert!(index < self.len);
        let ret = self.memory[index];
        self.lock.get_lock();
        self.len -= 1;
        for i in index..self.len {
            self.memory[i] = self.memory[i + 1];
        }
        self.lock.unlock();
        ret
    }


    /// 末尾に要素を付加する
    #[inline]
    pub fn push(&mut self, value: T) {
        if self.available() >= 1 {
            self.lock.get_lock();
            self.memory[self.len] = value;
            self.len += 1;
            self.lock.unlock();
        }
    }

    /// 末尾の要素を取り出して返す(元の要素は削除される)
    #[inline]
    pub fn pop(&mut self) -> Option<T> {
        if self.len > 0 {
            self.lock.get_lock();
            self.len -= 1;
            self.lock.unlock();
            Some(self.memory[self.len])
        } else {
            None
        }
    }

    /// 末尾に複数の要素を付加する
    #[inline]
    pub fn push_all(&mut self, other: &[T]) {
        self.lock.get_lock();
        for item in other.iter() {
            self.memory[self.len] = *item;
            self.len += 1;
        }
        self.lock.unlock();
    }

    fn clear(&mut self) {
        self.len = 0
    }

    /// すべての要素に f を適用する
    pub fn map_in_place<F>(&mut self, f: F)
        where F: Fn(&mut T)
    {
        for i in 0..self.len {
            f(&mut self.memory[i]);
        }
    }

    #[inline]
    pub fn iter(&self) -> Iter<T> {
        let (slice, _) = self.memory.split_at(self.len);
        slice.iter()
    }

    #[inline]
    pub fn iter_mut(&mut self) -> IterMut<T> {
        let (mut slice, _) = self.memory.split_at_mut(self.len);
        slice.iter_mut()
    }

    /// 先頭の要素を取り除いて返す。
    fn shift(&mut self) -> T {
        assert!(self.len > 0);
        self.lock.get_lock();
        let ret = self.memory[0];
        for i in 1..self.len {
            self.memory[i-1] = self.memory[i];
        }
        self.len += 1;
        self.lock.unlock();
        ret
    }
}




//! Lock Library.
//!
//! # Example
//! ```
//! let l = lock::Lock;
//! l.get_lock();
//! l.unlock();
//! ```

// use core::sync::atomic::AtomicBool; が、なぜか使えないので、enum Lock を実装する。

#[derive(Debug)]
pub enum Lock {
    Locked,
    Unlocked,
}

impl Lock {

    /// get lock.
    /// 
    /// if can not get lock, wait forever until get lock.
    pub fn get_lock(&mut self) -> () {
        loop {
            match *self {
                Lock::Locked => continue,
                _ => {
                    *self = Lock::Locked;
                    break;
                }
            }
        }
    }

    /// unlock.
    pub fn unlock(&mut self) -> () {
        *self = Lock::Unlocked;
    }
}


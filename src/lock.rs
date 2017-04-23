
// use core::sync::atomic::AtomicBool; が、なぜか使えないので、enum Lock を実装する。

#[derive(Debug)]
pub enum Lock {
    Locked,
    Unlocked,
}

impl Lock {
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
    pub fn unlock(&mut self) -> () {
        *self = Lock::Unlocked;
    }
}


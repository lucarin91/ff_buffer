mod ff_ubuffer;

use crate::ff_ubuffer::FFUnbaundedBuffer;
use std::sync::Arc;

const BUFFER_SECTION_SIZE: u64 = 2048;

pub fn build<T>() -> (FFSender<T>, FFReceiver<T>) {
    let a = Arc::new(FFUnbaundedBuffer::<T>::new(BUFFER_SECTION_SIZE));
    (FFSender { queue: a.clone() }, FFReceiver { queue: a })
}

pub struct FFSender<T> {
    queue: Arc<FFUnbaundedBuffer<T>>,
}

pub struct FFReceiver<T> {
    queue: Arc<FFUnbaundedBuffer<T>>,
}

impl<T> FFSender<T> {
    pub fn push(&self, el: Box<T>) -> Option<&str> {
        self.queue.push(el)
    }
}

impl<T> FFReceiver<T> {
    pub fn pop(&self) -> Box<T> {
        loop {
            if let Some(el) = self.queue.pop() {
                return el;
            }
        }
    }
    pub fn try_pop(&self) -> Option<Box<T>> {
        return self.queue.pop();
    }
    pub fn iter(&self) -> FFReceiverIter<'_, T> {
        FFReceiverIter { rx: self }
    }
}

pub struct FFReceiverIter<'a, T: 'a> {
    rx: &'a FFReceiver<T>,
}

impl<'a, T> Iterator for FFReceiverIter<'a, T> {
    type Item = Box<T>;
    fn next(&mut self) -> Option<Box<T>> {
        Some(self.rx.pop())
    }
}

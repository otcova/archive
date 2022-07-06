pub struct Thread<'a, T> {
    handle: Option<std::thread::JoinHandle<T>>,
    self_lifetime: Option<&'a ()>,
}

impl<'a, T> Thread<'a, T> {
    pub fn spawn<F>(f: F) -> Self
    where
        F: FnOnce() -> T,
        F: Send + Sync + 'a,
        T: Send + Sync + 'a,
    {
        // Thread will be forced to join when 'a drops
        // so thread doesn't need a 'static lifetime
        Self {
            handle: Some(unsafe {
                std::thread::Builder::new()
                    .spawn_unchecked(f)
                    .expect("failed to spawn thread")
            }),
            self_lifetime: None,
        }
    }

    pub fn try_join(&mut self) -> Option<T> {
        if self.handle.as_ref()?.is_finished() {
            self.join()
        } else {
            None
        }
    }

    pub fn join(&mut self) -> Option<T> {
        match self.handle.take()?.join() {
            Ok(result) => Some(result),
            _ => Self::manage_thread_panick(),
        }
    }

    fn manage_thread_panick() -> Option<T> {
        if !std::thread::panicking() {
            panic!("Spawned Thread paniked");
        }
        println!("Spawned Thread paniked while paniking on this thread");
        None
    }
}

impl<'a, T> Drop for Thread<'a, T> {
    fn drop(&mut self) {
        self.join();
    }
}

#[cfg(test)]
mod test {
    use super::*;

    struct Pool<'a, T> {
        a: Thread<'a, T>,
        b: Thread<'a, T>,
    }

    #[test]
    #[should_panic]
    fn panic_two_threads() {
        Pool {
            a: Thread::spawn(|| {
                panic!("Ho no!");
            }),
            b: Thread::spawn(|| {
                panic!("Ha ha ha");
            }),
        };
    }
}

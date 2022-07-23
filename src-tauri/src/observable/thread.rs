pub struct Thread<'a> {
    handle: Option<std::thread::JoinHandle<()>>,
    _self_lifetime: Option<&'a ()>,
}

impl<'a> Thread<'a> {
    pub fn spawn<F>(f: F) -> Self
    where
        F: FnOnce() -> (),
        F: Send + Sync + 'a,
    {
        // Thread will be forced to join when 'a drops
        // so thread doesn't need a 'static lifetime
        Self {
            handle: Some(unsafe {
                std::thread::Builder::new()
                    .name("observable::Thread::spawn".into())
                    .spawn_unchecked(f)
                    .expect("failed to spawn thread")
            }),
            _self_lifetime: None,
        }
    }

    /// true -> thread is no longer runing
    /// false -> thread is still runing
    pub fn try_join(&mut self) -> bool {
        if let Some(ref mut thread) = self.handle {
            if thread.is_finished() {
                self.join();
                true
            } else {
                false
            }
        } else {
            true
        }
    }

    pub fn join(&mut self) {
        if let Some(thread) = self.handle.take() {
            if thread.join().is_err() {
                Self::manage_thread_panick();
            }
        }
    }

    fn manage_thread_panick() {
        if !std::thread::panicking() {
            panic!("Spawned Thread paniked");
        }
        println!("Spawned Thread paniked while paniking on this thread");
    }
}

impl<'a> Drop for Thread<'a> {
    fn drop(&mut self) {
        self.join();
    }
}

#[cfg(test)]
mod test {
    use super::*;

    struct Pool<'a> {
        a: Thread<'a>,
        b: Thread<'a>,
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

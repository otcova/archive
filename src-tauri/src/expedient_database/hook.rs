pub use crate::collections::Id;
use crate::collections::IdMap;
use serde::Serialize;

pub struct Hook<'a, Event: Send + Sync>(
    Box<dyn for<'b> FnMut(&Event, Box<dyn FnOnce() + 'b>) + Send + Sync + 'a>,
);

pub struct HookPool<'a, Event: Send + Sync> {
    hooks: IdMap<Hook<'a, Event>>,
}

impl<'a, Event: Send + Sync> Serialize for Hook<'a, Event> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        "[hook]".serialize(serializer)
    }
}

impl<'a, Event: Send + Sync> HookPool<'a, Event> {
    pub fn new() -> Self {
        Self {
            hooks: IdMap::new(),
        }
    }

    pub fn count_hooks(&self) -> usize {
        self.hooks.len()
    }

    pub fn hook(
        &mut self,
        callback: impl for<'b> FnMut(&Event, Box<dyn FnOnce() + 'b>) + Send + Sync + 'a,
    ) -> Id {
        self.hooks.push(Hook(Box::new(callback)))
    }

    pub fn release(&mut self, id: Id) {
        self.hooks.delete(id);
    }

    pub fn dispatch(&mut self, event: &Event) {
        let mut hooks_to_release = vec![];
        for (id, hook) in self.hooks.iter_mut() {
            hook.0(event, Box::new(|| hooks_to_release.push(id)));
        }
        for id in hooks_to_release {
            self.hooks.delete(id);
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::sync::{Arc, Mutex};

    #[test]
    fn dispatch_on_a_new_hook() {
        let mut hook = HookPool::new();
        hook.dispatch(&123);
    }

    #[test]
    fn hook_receives_events() {
        let value = Arc::new(Mutex::new(0));
        {
            let mut pool = HookPool::<i32>::new();
            let value = value.clone();
            pool.hook(move |event, _| *value.lock().unwrap() = *event);
            assert_eq!(1, pool.count_hooks());
            pool.dispatch(&123);
        }
        assert_eq!(123, *value.lock().unwrap());
    }

    #[test]
    fn hook_and_release_do_not_receive_events() {
        let value = Arc::new(Mutex::new(3));
        {
            let mut pool = HookPool::<i32>::new();
            let value = value.clone();
            let id = pool.hook(move |event, _| *value.lock().unwrap() = *event);
            pool.release(id);
            pool.dispatch(&123);
        }
        assert_eq!(3, *value.lock().unwrap());
    }

    #[test]
    fn multiple_hooks() {
        let value = Arc::new(Mutex::new(0));
        {
            let mut pool = HookPool::<i32>::new();

            let value_clone = value.clone();
            pool.hook(move |event, _| *value_clone.lock().unwrap() += *event);

            let value_clone = value.clone();
            let id = pool.hook(move |event, _| *value_clone.lock().unwrap() += *event);

            let value_clone = value.clone();
            pool.hook(move |event, _| *value_clone.lock().unwrap() += *event);

            let value_clone = value.clone();
            pool.hook(move |event, _| *value_clone.lock().unwrap() += *event);

            pool.release(id);
            pool.dispatch(&100);
        }
        assert_eq!(300, *value.lock().unwrap());
    }

    #[test]
    fn release_on_receive() {
        let value = Arc::new(Mutex::new(0));
        {
            let mut pool = HookPool::<i32>::new();

            let value = value.clone();
            pool.hook(move |event, release| {
                *value.lock().unwrap() += *event;
                release();
            });

            pool.dispatch(&123);
            pool.dispatch(&123);
        }
        assert_eq!(123, *value.lock().unwrap());
    }
}

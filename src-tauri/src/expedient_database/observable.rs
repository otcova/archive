use crate::collections::*;
use serde::Serialize;
use std::thread::{spawn, JoinHandle};

#[derive(Clone)]
pub struct Callback<Context: Clone + Send + Sync> {
    pub callback: fn(&Context),
    pub context: Context,
}

impl<Context: Clone + Send + Sync> Serialize for Callback<Context> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        "[callback]".serialize(serializer)
    }
}
impl<Context: Clone + Send + Sync> Callback<Context> {
    pub fn new(context: Context, callback: fn(&Context)) -> Self {
        Self { context, callback }
    }
    pub fn call(&self) {
        (self.callback)(&self.context)
    }
}

pub struct Observable<Context: Clone + Send + Sync> {
    hooks: IdMap<Callback<Context>>,
    async_trigger_joins: IdMap<JoinHandle<()>>,
}

impl<Context: Clone + Send + Sync> Observable<Context> {
    pub fn new() -> Self {
        Self {
            hooks: IdMap::new(),
            async_trigger_joins: IdMap::new(),
        }
    }

    pub fn subscrive(&mut self, callback: Callback<Context>, instant_trigger: bool) -> Id {
        if instant_trigger {
            callback.call()
        }
        self.hooks.push(callback)
    }

    pub fn unsubscrive(&mut self, id: Id) {
        self.hooks.delete(id);
    }

    pub fn trigger(&mut self) {
        for (_, callback) in self.hooks.iter_mut() {
            callback.call()
        }
    }
}

impl<Context: Clone + Send + Sync + 'static> Observable<Context> {
    pub fn async_trigger(&mut self) {
        self.try_join_handles();

        for (_, callback) in self.hooks.iter_mut() {
            let cloned_callback = callback.clone();

            let join_handle = spawn(move || cloned_callback.call());

            self.async_trigger_joins.push(join_handle);
        }
    }

    fn try_join_handles(&mut self) {
        let ids_to_delete: Vec<_> = self
            .async_trigger_joins
            .iter()
            .filter(|(_, handle)| handle.is_finished())
            .map(|(id, _)| id)
            .collect();

        for id in ids_to_delete {
            self.async_trigger_joins.pop(id).unwrap().join().unwrap();
        }
    }
}

impl<Context: Clone + Send + Sync> Drop for Observable<Context> {
    fn drop(&mut self) {
        for handle in self.async_trigger_joins.take_iter() {
            handle.join().unwrap();
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::sync::atomic::AtomicI8;
    use std::sync::atomic::Ordering;
    use std::sync::Arc;

    #[test]
    fn subscrive_and_triger() {
        let has_been_triggered = Arc::new(AtomicI8::new(0));

        let mut observable = Observable::<Arc<AtomicI8>>::new();

        observable.subscrive(
            Callback::new(has_been_triggered.clone(), |ctx| {
                ctx.fetch_add(2, Ordering::SeqCst);
            }),
            false,
        );

        observable.subscrive(
            Callback::new(has_been_triggered.clone(), |ctx| {
                ctx.fetch_add(3, Ordering::SeqCst);
            }),
            true,
        );

        observable.trigger();
        assert_eq!(8, has_been_triggered.load(Ordering::SeqCst));
    }

    #[test]
    fn subscrive_unsubscrive_and_triger() {
        let has_been_triggered = Arc::new(AtomicI8::new(0));

        let mut observable = Observable::<Arc<AtomicI8>>::new();

        let id = observable.subscrive(
            Callback::new(has_been_triggered.clone(), |ctx| {
                ctx.fetch_add(3, Ordering::SeqCst);
            }),
            true,
        );
        observable.unsubscrive(id);
        observable.trigger();

        assert_eq!(3, has_been_triggered.load(Ordering::SeqCst));
    }

    #[test]
    fn async_trigger() {
        let has_been_triggered = Arc::new(AtomicI8::new(0));
        {
            let mut observable = Observable::<Arc<AtomicI8>>::new();

            observable.subscrive(
                Callback::new(has_been_triggered.clone(), |ctx| {
                    // Delay to simulate computation
                    std::thread::sleep(std::time::Duration::from_millis(10));
                    ctx.fetch_add(3, Ordering::SeqCst);
                }),
                true,
            );
            observable.async_trigger();

            assert_eq!(3, has_been_triggered.load(Ordering::SeqCst));
        }
        assert_eq!(6, has_been_triggered.load(Ordering::SeqCst));
    }
}

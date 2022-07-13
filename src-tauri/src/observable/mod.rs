mod thread;
use crate::collections::*;
use serde::Serialize;
use thread::*;

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

pub struct Observable<'a, Context: Clone + Send + Sync + 'a> {
    hooks: IdMap<Callback<Context>>,
    async_trigger_joins: IdMap<Thread<'a, ()>>,
}

pub enum InstantTriggerType {
    Sync,
    Async,
    None,
}

impl<'a, Context: Clone + Send + Sync + 'a> Default for Observable<'a, Context> {
    fn default() -> Self {
        Self {
            hooks: Default::default(),
            async_trigger_joins: Default::default(),
        }
    }
}
impl<'a, Context: Clone + Send + Sync + 'a> Observable<'a, Context> {
    pub fn subscrive(
        &mut self,
        callback: Callback<Context>,
        instant_trigger: InstantTriggerType,
    ) -> Id {
        match instant_trigger {
            InstantTriggerType::Async => {
                Self::async_call(&mut self.async_trigger_joins, callback.clone())
            }
            InstantTriggerType::Sync => callback.call(),
            _ => (),
        };
        self.hooks.push(callback)
    }

    pub fn unsubscrive(&mut self, id: Id) {
        self.hooks.delete(id);
    }

    pub fn trigger(&mut self) {
        for callback_item in self.hooks.iter_mut() {
            callback_item.data.call()
        }
    }

    pub fn async_trigger(&mut self) {
        self.try_join_handles();

        for callback_item in self.hooks.iter_mut() {
            Self::async_call(&mut self.async_trigger_joins, callback_item.data.clone());
        }
    }

    fn async_call(async_trigger_joins: &mut IdMap<Thread<'a, ()>>, callback: Callback<Context>) {
        let join_handle = Thread::spawn(move || callback.call());
        async_trigger_joins.push(join_handle);
    }

    fn try_join_handles(&mut self) {
        let ids_to_delete: Vec<_> = self
            .async_trigger_joins
            .iter_mut()
            .filter_map(|thread_item| thread_item.data.try_join().map(|_| thread_item.id))
            .collect();

        for id in ids_to_delete {
            self.async_trigger_joins.delete(id);
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

        let mut observable = Observable::<Arc<AtomicI8>>::default();

        observable.subscrive(
            Callback::new(has_been_triggered.clone(), |ctx| {
                ctx.fetch_add(2, Ordering::SeqCst);
            }),
            InstantTriggerType::None,
        );

        observable.subscrive(
            Callback::new(has_been_triggered.clone(), |ctx| {
                ctx.fetch_add(3, Ordering::SeqCst);
            }),
            InstantTriggerType::Sync,
        );

        observable.trigger();
        assert_eq!(8, has_been_triggered.load(Ordering::SeqCst));
    }

    #[test]
    fn subscrive_unsubscrive_and_triger() {
        let has_been_triggered = Arc::new(AtomicI8::new(0));

        let mut observable = Observable::<Arc<AtomicI8>>::default();

        let id = observable.subscrive(
            Callback::new(has_been_triggered.clone(), |ctx| {
                ctx.fetch_add(3, Ordering::SeqCst);
            }),
            InstantTriggerType::Sync,
        );
        observable.unsubscrive(id);
        observable.trigger();

        assert_eq!(3, has_been_triggered.load(Ordering::SeqCst));
    }

    #[test]
    fn async_trigger() {
        let has_been_triggered = Arc::new(AtomicI8::new(0));
        {
            let mut observable = Observable::<Arc<AtomicI8>>::default();

            observable.subscrive(
                Callback::new(has_been_triggered.clone(), |ctx| {
                    // Delay to simulate computation
                    std::thread::sleep(std::time::Duration::from_millis(10));
                    ctx.fetch_add(1, Ordering::SeqCst);
                }),
                InstantTriggerType::Sync,
            );
            observable.async_trigger();

            assert_eq!(1, has_been_triggered.load(Ordering::SeqCst));
        }
        assert_eq!(2, has_been_triggered.load(Ordering::SeqCst));
    }

    #[test]
    fn async_instant_trigger() {
        let has_been_triggered = Arc::new(AtomicI8::new(0));
        {
            let mut observable = Observable::<Arc<AtomicI8>>::default();

            observable.subscrive(
                Callback::new(has_been_triggered.clone(), |ctx| {
                    // Delay to simulate computation
                    std::thread::sleep(std::time::Duration::from_millis(10));
                    ctx.fetch_add(1, Ordering::SeqCst);
                }),
                InstantTriggerType::Async,
            );

            assert_eq!(0, has_been_triggered.load(Ordering::SeqCst));
        }
        assert_eq!(1, has_been_triggered.load(Ordering::SeqCst));
    }
}

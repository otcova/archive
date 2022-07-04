use crate::collections::*;
use serde::Serialize;

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
    pub fn call(&self) {
        (self.callback)(&self.context)
    }
}

pub struct Observable<Context: Clone + Send + Sync> {
    hooks: IdMap<Callback<Context>>,
}

impl<Context: Clone + Send + Sync> Observable<Context> {
    pub fn new() -> Self {
        Self {
            hooks: IdMap::new(),
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
            Callback {
                callback: |ctx| {
                    ctx.fetch_add(2, Ordering::SeqCst);
                },
                context: has_been_triggered.clone(),
            },
            false,
        );

        observable.subscrive(
            Callback {
                callback: |ctx| {
                    ctx.fetch_add(3, Ordering::SeqCst);
                },
                context: has_been_triggered.clone(),
            },
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
            Callback {
                callback: |ctx| {
                    ctx.fetch_add(3, Ordering::SeqCst);
                },
                context: has_been_triggered.clone(),
            },
            true,
        );
        observable.unsubscrive(id);
        observable.trigger();

        assert_eq!(3, has_been_triggered.load(Ordering::SeqCst));
    }
}

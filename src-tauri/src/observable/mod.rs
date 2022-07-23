mod async_callback;
mod async_observable;
mod callback;
mod thread;
use crate::collections::*;
pub use async_callback::*;
pub use async_observable::*;
pub use callback::*;

pub struct Observable<Context> {
    hooks: IdMap<Callback<Context>>,
}

impl<Context> Default for Observable<Context> {
    fn default() -> Self {
        Self {
            hooks: Default::default(),
        }
    }
}

impl<Context> Observable<Context> {
    pub fn subscrive(&mut self, callback: Callback<Context>, trigger_now: bool) -> Id {
        if trigger_now {
            callback.call()
        }
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

        let mut observable = Observable::<Arc<AtomicI8>>::default();

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
            let mut observable = AsyncObservable::<Arc<AtomicI8>>::default();

            observable.subscrive(
                AsyncCallback::new(has_been_triggered.clone(), |ctx, _| {
                    // Delay to simulate computation
                    std::thread::sleep(std::time::Duration::from_millis(10));
                    ctx.fetch_add(1, Ordering::SeqCst);
                    Some(())
                }),
                true,
            );
            observable.trigger();

            assert_eq!(0, has_been_triggered.load(Ordering::SeqCst));
        }
        assert_eq!(2, has_been_triggered.load(Ordering::SeqCst));
    }

    #[test]
    fn async_instant_trigger() {
        let has_been_triggered = Arc::new(AtomicI8::new(0));
        {
            let mut observable = AsyncObservable::<Arc<AtomicI8>>::default();

            observable.subscrive(
                AsyncCallback::new(has_been_triggered.clone(), |ctx, _| {
                    // Delay to simulate computation
                    std::thread::sleep(std::time::Duration::from_millis(10));
                    ctx.fetch_add(1, Ordering::SeqCst);
                    Some(())
                }),
                true,
            );

            assert_eq!(0, has_been_triggered.load(Ordering::SeqCst));
        }
        assert_eq!(1, has_been_triggered.load(Ordering::SeqCst));
    }
}

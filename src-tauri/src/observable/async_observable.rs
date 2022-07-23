use super::*;

pub struct AsyncObservable<'a, Context: Send + Sync> {
    hooks: IdMap<AsyncCallback<'a, Context>>,
    callbacks_to_terminate: IdMap<AsyncCallback<'a, Context>>,
}

impl<'a, Context: Send + Sync> Default for AsyncObservable<'a, Context> {
    fn default() -> Self {
        Self {
            hooks: Default::default(),
            callbacks_to_terminate: Default::default(),
        }
    }
}

impl<'a, Context: Send + Sync> AsyncObservable<'a, Context> {
    pub fn subscrive(&mut self, mut callback: AsyncCallback<'a, Context>, trigger_now: bool) -> Id {
        if trigger_now {
            callback.call()
        }
        self.hooks.push(callback)
    }

    pub fn unsubscrive(&mut self, id: Id) {
        if let Some(mut callback) = self.hooks.take(id) {
            if !callback.try_terminate_calls() {
                self.clean_terminated_callbacks();
                self.callbacks_to_terminate.push(callback);
            }
        }
    }

    fn clean_terminated_callbacks(&mut self) {
        self.callbacks_to_terminate
            .filter(|callback| !callback.data.try_terminate_calls());
    }

    pub fn trigger(&mut self) {
        for callback_item in self.hooks.iter_mut() {
            callback_item.data.call()
        }
    }

    pub fn stop_trigger(&mut self) {
        for callback_item in self.hooks.iter_mut() {
            callback_item.data.try_terminate_calls();
        }
    }
}

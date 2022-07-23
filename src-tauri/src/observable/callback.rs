use super::thread::Thread;
use crate::collections::*;
use serde::Serialize;
use std::sync::{Arc, Mutex};

pub struct Callback<Context> {
    callback: fn(&Context),
    context: Context,
}

impl<Context> Serialize for Callback<Context> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        "[callback]".serialize(serializer)
    }
}

impl<Context> Callback<Context> {
    pub fn new(context: Context, callback: fn(&Context)) -> Self {
        Self { context, callback }
    }
    pub fn call(&self) {
        (self.callback)(&self.context)
    }
}

pub struct AsyncCallback<'a, Context: 'a + Send + Sync> {
    callback: fn(&Context, CallbackProcess) -> Option<()>,
    context: Arc<Context>,
    instances: IdMap<CallbackInstance<'a>>,
}

pub struct CallbackInstance<'a> {
    join_handle: Thread<'a, Option<()>>,
    request_join: Arc<Mutex<bool>>,
}

pub struct CallbackProcess {
    request_join: Arc<Mutex<bool>>,
}

impl CallbackProcess {
    pub fn terminate_if_requested(&self) -> Option<()> {
        if *self.request_join.lock().unwrap() {
            None
        } else {
            Some(())
        }
    }
}

impl<'a, Context: 'a + Send + Sync> AsyncCallback<'a, Context> {
    pub fn new(context: Context, callback: fn(&Context, CallbackProcess) -> Option<()>) -> Self {
        Self {
            context: Arc::new(context),
            callback,
            instances: Default::default(),
        }
    }
    pub fn call(&mut self) {
        let request_join = Arc::new(Mutex::new(false));

        let requested_join = CallbackProcess {
            request_join: request_join.clone(),
        };
        let context = self.context.clone();
        let cloned_callback = self.callback.clone();

        let join_handle =
            Thread::spawn(move || (cloned_callback)(context.as_ref(), requested_join));

        self.instances.push(CallbackInstance {
            join_handle,
            request_join,
        });
    }
    pub fn request_join_to_previous_instances(&mut self) {
        let ids_to_delete: Vec<_> = self
            .instances
            .iter_mut()
            .filter_map(|instance| instance.data.try_join().then_some(instance.id))
            .collect();

        for id in ids_to_delete {
            self.instances.delete(id);
        }
    }
}

impl<'a, Context: Send + Sync> Drop for AsyncCallback<'a, Context> {
    fn drop(&mut self) {
        for instance in self.instances.iter_mut() {
            instance.data.join()
        }
    }
}

impl<'a> CallbackInstance<'a> {
    fn request_join(&mut self) {
        *self.request_join.as_ref().lock().unwrap() = true;
    }
    pub fn try_join(&mut self) -> bool {
        self.request_join();
        self.join_handle.try_join().is_some()
    }
    pub fn join(&mut self) {
        self.request_join();
        self.join_handle.join();
    }
}

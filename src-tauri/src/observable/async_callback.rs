use super::thread::Thread;
use crate::collections::*;
use std::sync::{Arc, Mutex};

pub struct AsyncCallback<'a, Context: 'a + Send + Sync> {
    callback: fn(&Context, AsyncCallbackProcess) -> Option<()>,
    context: Arc<Context>,
    instances: IdMap<AsyncCallbackInstance<'a>>,
}

pub struct AsyncCallbackInstance<'a> {
    join_handle: Thread<'a>,
    request_join: Arc<Mutex<bool>>,
}

pub struct AsyncCallbackProcess {
    request_join: Arc<Mutex<bool>>,
}

impl AsyncCallbackProcess {
    pub fn terminate_if_requested(&self) -> Option<()> {
        if *self.request_join.lock().unwrap() {
            None
        } else {
            Some(())
        }
    }
}

impl<'a, Context: 'a + Send + Sync> AsyncCallback<'a, Context> {
    pub fn new(
        context: Context,
        callback: fn(&Context, AsyncCallbackProcess) -> Option<()>,
    ) -> Self {
        Self {
            context: Arc::new(context),
            callback,
            instances: Default::default(),
        }
    }
    pub fn call(&mut self) {
        let request_join = Arc::new(Mutex::new(false));

        let requested_join = AsyncCallbackProcess {
            request_join: request_join.clone(),
        };
        let context = self.context.clone();
        let cloned_callback = self.callback.clone();

        let join_handle = Thread::spawn(move || {
            (cloned_callback)(context.as_ref(), requested_join);
        });

        self.instances.push(AsyncCallbackInstance {
            join_handle,
            request_join,
        });
    }
    pub fn try_terminate_calls(&mut self) -> bool {
        self.instances.filter(|instance| !instance.data.try_join());
        self.instances.len() == 0
    }
}

impl<'a> AsyncCallbackInstance<'a> {
    fn request_join(&mut self) {
        *self.request_join.as_ref().lock().unwrap() = true;
    }
    pub fn try_join(&mut self) -> bool {
        self.request_join();
        self.join_handle.try_join()
    }

    pub fn join(&mut self) -> bool {
        self.request_join();
        self.join_handle.try_join()
    }
}

impl<'a> Drop for AsyncCallbackInstance<'a> {
    fn drop(&mut self) {
        if !self.try_join() {
            println!("WARN: Blocking thread (with join) because of drop");
            self.join();
        }
    }
}

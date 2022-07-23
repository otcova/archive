use serde::Serialize;

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

use std::future::Future;

use poll_promise::Promise;

pub enum State<T>
where
    T: Clone + Send + 'static,
{
    Ready(T),
    Pending(Promise<T>),
    Empty,
}

impl<T> Default for State<T>
where
    T: Clone + Send,
{
    fn default() -> Self {
        Self::Empty
    }
}

impl<T> State<T>
where
    T: Clone + Send,
{
    pub fn is_empty(&self) -> bool {
        matches!(self, State::Empty)
    }

    pub fn is_pending(&self) -> bool {
        matches!(self, State::Pending(_))
    }

    pub fn is_ready(&self) -> bool {
        matches!(self, State::Ready(_))
    }

    pub fn clear(&mut self) {
        *self = State::Empty
    }

    pub fn take_and_clear(&mut self) -> Option<T> {
        match &self {
            State::Ready(result) => {
                let result = result.clone();
                *self = State::Empty;
                Some(result)
            }
            _ => None,
        }
    }

    pub fn use_future(&mut self, future: impl Future<Output = T> + Send + 'static) -> bool {
        match self {
            State::Empty => {
                *self = State::Pending(Promise::spawn_async(future));
                true
            }
            _ => false,
        }
    }

    pub fn update(&mut self) {
        if let State::Pending(promise) = &self {
            if let Some(result) = promise.ready() {
                *self = State::Ready(result.clone());
            }
        }
    }
}

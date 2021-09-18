use std::ops::{Deref, DerefMut};

#[doc(hidden)]
pub auto trait Unanchored {}

pub struct Anchored<T> {
    data: T,
}

impl<T> !Unanchored for Anchored<T> {}

impl<T> Anchored<T> {
    pub fn new(data: T) -> Self {
        Self { data }
    }
}

impl<T> Deref for Anchored<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl<T> DerefMut for Anchored<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}

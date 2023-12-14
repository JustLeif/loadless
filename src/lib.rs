pub mod iter;
pub mod templates;

/// Iterator trait extension that makes `loadless` possible on iterators.
pub trait LoadlessIteratorExt<'a>: Sized {
    fn loadless(self) -> iter::LoadlessIterator<'a, Self>;
}

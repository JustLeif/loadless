pub mod iter;
pub mod templates;
pub trait LoadlessIteratorExt<'a>: Sized {
    fn loadless(self) -> iter::LoadlessIterator<'a, Self>;
}

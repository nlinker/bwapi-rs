struct BwRefIterator<T> {
    pub(crate) raw: *mut T,
}

impl<T> Iterator for BwRefIterator<T> {
    type Item = ();

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        todo!()
    }
}
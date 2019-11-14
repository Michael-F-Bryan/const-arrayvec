use crate::ArrayVec;
use core::{
    iter::{DoubleEndedIterator, FusedIterator},
    mem,
    ops::Range,
};

#[derive(Debug, PartialEq)]
pub struct Drain<'a, T, const N: usize> {
    inner: &'a mut ArrayVec<T, { N }>,
    /// The first item after the drained range.
    start_of_tail: usize,
    tail_length: usize,
    /// The front of the remaining drained range.
    head: *mut T,
    /// One after the last item in the range being drained.
    tail: *mut T,
}

impl<'a, T, const N: usize> Drain<'a, T, { N }> {
    pub(crate) fn with_range(
        vector: &'a mut ArrayVec<T, { N }>,
        range: Range<usize>,
    ) -> Self {
        debug_assert!(
            range.start <= range.end,
            "The range start must be before end"
        );
        debug_assert!(range.end <= vector.len(), "The range is out of bounds");
        debug_assert!(
            core::mem::size_of::<T>() != 0,
            "We can't deal with zero-sized types"
        );

        unsafe {
            let head = vector.as_mut_ptr().add(range.start);
            let tail = vector.as_mut_ptr().add(range.end);
            let tail_length = vector.len() - (range.end - range.start);

            Drain {
                inner: vector,
                start_of_tail: range.end,
                tail_length,
                head,
                tail,
            }
        }
    }

    pub fn as_slice(&self) -> &[T] { unimplemented!() }

    pub fn as_mut_slice(&mut self) -> &mut [T] { unimplemented!() }
}

impl<'a, T, const N: usize> Iterator for Drain<'a, T, { N }> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.head == self.tail {
            // No more items
            return None;
        }

        unsafe {
            // The tail points at tne end of our
            // copy the item onto the stack. The tail
            let item = self.head.read();
            // increment the head pointer
            self.head = self.head.add(1);
            Some(item)
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.len(), Some(self.len()))
    }
}

impl<'a, T, const N: usize> DoubleEndedIterator for Drain<'a, T, { N }> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.head == self.tail {
            // No more items
            return None;
        }

        unsafe {
            // the tail pointer is one PAST the end of our selection.
            // Pre-decrement so we're pointing at a valid item before reading
            self.tail = self.tail.sub(1);
            let item = self.tail.read();
            Some(item)
        }
    }
}

impl<'a, T, const N: usize> FusedIterator for Drain<'a, T, { N }> {}

impl<'a, T, const N: usize> ExactSizeIterator for Drain<'a, T, { N }> {
    fn len(&self) -> usize {
        let size = mem::size_of::<T>();
        assert!(0 < size && size <= isize::max_value() as usize);

        let difference = (self.tail as isize) - (self.head as isize);
        debug_assert!(difference >= 0, "Tail should always be after head");

        difference as usize / size
    }
}

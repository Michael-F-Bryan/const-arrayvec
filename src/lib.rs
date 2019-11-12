#![no_std]
#![feature(const_generics)]
#![allow(incomplete_features)]

use core::{
    fmt::{self, Display, Formatter},
    mem::MaybeUninit,
    ops::{Deref, DerefMut},
    ptr, slice,
};

macro_rules! out_of_bounds {
    ($method:expr, $index:expr, $len:expr) => {
        panic!(
            concat!(
                "ArrayVec::",
                $method,
                "(): index {} is out of bounds in vector of length {}"
            ),
            $index, $len
        );
    };
}

pub struct ArrayVec<T, const N: usize> {
    items: [MaybeUninit<T>; N],
    length: usize,
}

impl<T, const N: usize> ArrayVec<T, { N }> {
    pub fn new() -> ArrayVec<T, { N }> {
        unsafe {
            ArrayVec {
                // this is safe because we've asked for a big block of
                // uninitialized memory which will be treated as
                // an array of uninitialized items,
                // which perfectly valid for [MaybeUninit<_>; N]
                items: MaybeUninit::uninit().assume_init(),
                length: 0,
            }
        }
    }

    pub const fn len(&self) -> usize { self.length }

    pub const fn is_empty(&self) -> bool { self.len() == 0 }

    pub const fn capacity(&self) -> usize { N }

    pub const fn is_full(&self) -> bool { self.len() >= self.capacity() }

    pub fn as_ptr(&self) -> *const T { self.items.as_ptr() as *const T }

    pub fn as_mut_ptr(&mut self) -> *mut T { self.items.as_mut_ptr() as *mut T }

    /// Add an item to the end of the vector.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use const_arrayvec::ArrayVec;
    /// let mut vector: ArrayVec<u32, 5> = ArrayVec::new();
    ///
    /// assert!(vector.is_empty());
    ///
    /// vector.push(42);
    ///
    /// assert_eq!(vector.len(), 1);
    /// assert_eq!(vector[0], 42);
    /// ```
    pub fn push(&mut self, item: T) -> Result<(), CapacityError<T>> {
        if self.is_full() {
            Err(CapacityError(item))
        } else {
            unsafe {
                self.push_unchecked(item);
                Ok(())
            }
        }
    }

    /// Add an item to the end of the array without checking the capacity.
    ///
    /// # Safety
    ///
    /// It is up to the caller to ensure the vector's capacity is suitably
    /// large.
    ///
    /// This method uses *debug assertions* to detect overflows in debug builds.
    pub unsafe fn push_unchecked(&mut self, item: T) {
        debug_assert!(!self.is_full());
        let len = self.len();

        // index into the underlying array using pointer arithmetic and write
        // the item to the correct spot.
        self.as_mut_ptr().add(len).write(item);

        // only now can we update the length
        self.set_len(len + 1);
    }

    /// Set the vector's length without dropping or moving out elements.
    ///
    /// # Safety
    ///
    /// This method is `unsafe` because it changes the number of "valid"
    /// elements the vector thinks it contains, without adding or removing any
    /// elements. Use with care.
    pub unsafe fn set_len(&mut self, new_length: usize) {
        debug_assert!(new_length < self.capacity());
        self.length = new_length;
    }

    /// Remove an item from the end of the vector.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use const_arrayvec::ArrayVec;
    /// let mut vector: ArrayVec<u32, 5> = ArrayVec::new();
    ///
    /// vector.push(12);
    /// vector.push(34);
    ///
    /// assert_eq!(vector.len(), 2);
    ///
    /// let got = vector.pop();
    ///
    /// assert_eq!(got, Some(34));
    /// assert_eq!(vector.len(), 1);
    /// ```
    pub fn pop(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        }

        unsafe {
            let new_length = self.len() - 1;
            self.set_len(new_length);
            Some(ptr::read(self.as_ptr().add(new_length)))
        }
    }

    /// Shorten the vector, keeping the first `new_length` elements and dropping
    /// the rest.
    pub fn truncate(&mut self, new_length: usize) {
        unsafe {
            if new_length < self.len() {
                let start = self.as_mut_ptr().add(new_length);
                let num_elements_to_remove = self.len() - new_length;
                let tail: *mut [T] =
                    slice::from_raw_parts_mut(start, num_elements_to_remove);

                self.set_len(new_length);
                ptr::drop_in_place(tail);
            }
        }
    }

    /// Remove all items from the vector.
    pub fn clear(&mut self) { self.truncate(0); }

    pub fn try_insert(
        &mut self,
        index: usize,
        item: T,
    ) -> Result<(), CapacityError<T>> {
        let len = self.len();

        // bounds checks
        if index > self.len() {
            out_of_bounds!("try_insert", index, len);
        }
        if self.is_full() {
            return Err(CapacityError(item));
        }

        unsafe {
            // The spot to put the new value
            let p = self.as_mut_ptr().add(index);
            // Shift everything over to make space. (Duplicating the
            // `index`th element into two consecutive places.)
            ptr::copy(p, p.offset(1), len - index);
            // Write it in, overwriting the first copy of the `index`th
            // element.
            ptr::write(p, item);
            // update the length
            self.set_len(len + 1);
        }

        Ok(())
    }
}

impl<T, const N: usize> Deref for ArrayVec<T, { N }> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        unsafe { slice::from_raw_parts(self.as_ptr(), self.len()) }
    }
}

impl<T, const N: usize> DerefMut for ArrayVec<T, { N }> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { slice::from_raw_parts_mut(self.as_mut_ptr(), self.len()) }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct CapacityError<T>(pub T);

impl<T> Display for CapacityError<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Insufficient capacity")
    }
}

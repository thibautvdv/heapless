// NOTE this code has been based on slab (crates.io) v0.4.2

use core::{mem, slice};

/// Implementation detail
#[doc(hidden)]
pub enum Entry<T> {
    Vacant(usize),
    Occupied(T),
}

impl<A> crate::i::Slab<A> {
    /// `Vec` `const` constructor; wrap the returned value in [`Vec`](../struct.Vec.html)
    pub const fn new() -> Self {
        Self {
            entries: crate::i::Vec::new(),
            len: 0,
            next: 0,
        }
    }
}

/// TODO
pub struct Slab<T, const N: usize>(#[doc(hidden)] pub crate::i::Slab<[Entry<T>; N]>);

impl<T, const N: usize> Slab<T, N> {
    /// TODO
    pub fn new() -> Self {
        Slab(crate::i::Slab::new())
    }

    /// TODO
    pub fn insert(&mut self, val: T) -> Result<usize, T> {
        let key = self.0.next;
        self.insert_at(key, val)?;
        Ok(key)
    }

    fn insert_at(&mut self, key: usize, val: T) -> Result<(), T> {
        self.0.len += 1;

        if key == self.0.entries.len {
            self.0.entries.push(Entry::Occupied(val)).map_err(|entry| {
                if let Entry::Occupied(val) = entry {
                    val
                } else {
                    unreachable!()
                }
            })?;
            self.0.next = key + 1;
        } else {
            let prev = mem::replace(
                &mut self.0.entries.as_mut_slice()[key],
                Entry::Occupied(val),
            );

            match prev {
                Entry::Vacant(next) => {
                    self.0.next = next;
                }
                _ => unreachable!(),
            }
        }

        Ok(())
    }

    /// TODO
    pub fn remove(&mut self, key: usize) -> T {
        // Swap the entry at the provided value
        let prev = mem::replace(
            &mut self.0.entries.as_mut_slice()[key],
            Entry::Vacant(self.0.next),
        );

        match prev {
            Entry::Occupied(val) => {
                self.0.len -= 1;
                self.0.next = key;
                val
            }
            _ => {
                // Woops, the entry is actually vacant, restore the state
                self.0.entries.as_mut_slice()[key] = prev;
                panic!("invalid key");
            }
        }
    }

    /// TODO
    pub fn iter_mut(&mut self) -> IterMut<'_, T> {
        IterMut {
            entries: self.0.entries.as_mut_slice().iter_mut(),
            curr: 0,
        }
    }
}

impl<T, const N: usize> Default for Slab<T, N> {
    fn default() -> Self {
        Self::new()
    }
}

/// TODO
pub struct IterMut<'a, T> {
    entries: slice::IterMut<'a, Entry<T>>,
    curr: usize,
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = (usize, &'a mut T);

    fn next(&mut self) -> Option<(usize, &'a mut T)> {
        while let Some(entry) = self.entries.next() {
            let curr = self.curr;
            self.curr += 1;

            if let Entry::Occupied(ref mut v) = *entry {
                return Some((curr, v));
            }
        }

        None
    }
}

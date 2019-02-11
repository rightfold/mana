use super::*;

use std::cell::Cell;
use std::cell::RefCell;
use std::mem::transmute;
use std::ptr::NonNull;

use sigil::Sigil;

/// A collection of data.
///
/// A heap provides for the creation and garbage collection of data.
pub struct Heap {
    /// All data in the heap.
    ///
    /// The heap needs to keep track of all data, so that it knows what data to
    /// free when collecting garbage. The data are boxed so that they have a
    /// stable address; a reallocation of the vector will not cause pointers to
    /// the data to become invalid. There exist two invariants:
    ///
    ///  1. Data at a higher index in the vector were allocated later than data
    ///     at a lower index in the vector.
    ///  2. Data allocated later only point to other data allocated earlier.
    data: RefCell<Vec<Box<DatumInner>>>,
}

impl Heap {
    /// Create a new heap with no data.
    pub fn new() -> Self {
        Heap{data: RefCell::new(Vec::new())}
    }

    /// Create a datum.
    ///
    /// The datum is a root until the return value is dropped.
    ///
    /// This function is unsafe because the pointers must belong to this heap
    /// and this is currently not checked.
    pub unsafe fn allocate(&self,
                           enchantment: Sigil,
                           pointers:    &[Datum],
                           auxiliary:   &[u8],
                           ) -> Datum {
        let mut data = self.data.borrow_mut();

        let inner = Box::new(Self::construct(enchantment, pointers, auxiliary));
        let ptr = NonNull::from(inner.as_ref());
        data.push(inner);

        // This is safe because self.data owns the box, hence the pointer is
        // still valid here.
        Datum::enroot(ptr)
    }

    /// Perform garbage collection.
    ///
    /// This will free all data that are not accessible through any roots.
    pub fn collect_garbage(&self) -> CollectStatistics {
        // Keep in mind the invariants discussed earlier. With those invariants
        // guaranteed, garbage collection proceeds as follows:
        //
        //  1. Find the latest allocated datum, if any.
        //  2. If the datum is a root, mark it.
        //  3. If the datum is marked:
        //     1. Mark the direct pointees of the datum. They will be processed
        //        eventually, because of the invariants and the backwards
        //        traversal.
        //     2. Unmark the datum.
        //  4. Else:
        //     1. Free the datum.
        //     2. Remove the datum from the heap.
        //  5. Start over at last the datum allocated before the datum.
        let mut data = self.data.borrow_mut();
        let mut stat = CollectStatistics{data_freed: 0};

        /**********************************************************************/
        /* Step 1                                                             */
        for i in Iterator::rev(0 .. data.len()) {
            let mark = {
                let datum = data[i].as_ref();

        /**********************************************************************/
        /* Step 2                                                             */
                if datum.roots.get() > 0 {
                    datum.mark.set(true);
                }

        /**********************************************************************/
        /* Step 3                                                             */
                if datum.mark.get() {
                    for pointee in datum.pointers.iter() {
                        // This is safe because the pointee definitely has not
                        // yet been garbage collected, because of the
                        // invariants and the backwards traversal.
                        unsafe { pointee.as_ref() }.mark.set(true);
                    }
                    datum.mark.set(false);

        /**********************************************************************/
        /* Step 4                                                             */
                    true
                } else {
                    false
                }
            };

            if !mark {
                stat.data_freed += 1;
                data.pop();
            }

        /**********************************************************************/
        /* Step 5                                                             */
            continue;
        }

        stat
    }

    /// This function is unsafe because the pointers must belong to this heap
    /// and this is currently not checked.
    unsafe fn construct(enchantment: Sigil,
                        pointers:    &[Datum],
                        auxiliary:   &[u8],
                        ) -> DatumInner {
        // This is safe because the representation of Datum is equivalent to
        // that of DatumInner.
        let pointers_inner =
            transmute::<&[Datum], &[NonNull<DatumInner>]>(pointers);

        DatumInner{
            mark:        Cell::new(false),
            roots:       Cell::new(0),
            enchantment: enchantment,
            pointers:    Box::from(pointers_inner),
            auxiliary:   Box::from(auxiliary),
        }
    }
}

/// Statistics on a single garbage collection.
#[derive(Clone, Debug)]
pub struct CollectStatistics {
    /// The number of data that were freed by this garbage collection.
    pub data_freed: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_heap() {
        let heap = Heap::new();
        let stat = heap.collect_garbage();
        assert_eq!(stat.data_freed, 0);
    }

    #[test]
    fn test_singleton_heap() {
        let sigil = Sigil(0);

        let heap = Heap::new();
        let datum = unsafe { heap.allocate(sigil, &[], &[]) };

        { let stat = heap.collect_garbage()
        ; assert_eq!(stat.data_freed, 0) }

        drop(datum);

        { let stat = heap.collect_garbage()
        ; assert_eq!(stat.data_freed, 1) }

        { let stat = heap.collect_garbage()
        ; assert_eq!(stat.data_freed, 0) }
    }

    #[test]
    fn test_pointers_heap() {
        let sigil = Sigil(0);

        let heap = Heap::new();
        let datum_a = unsafe { heap.allocate(sigil, &[], &[]) };
        let datum_b = unsafe { heap.allocate(sigil, &[datum_a.clone()], &[]) };
        let datum_c = unsafe { heap.allocate(sigil, &[datum_b.clone()], &[]) };
        let datum_d = unsafe { heap.allocate(sigil, &[datum_b.clone(),
                                                      datum_c.clone()], &[]) };

        drop(datum_a);
        drop(datum_c);

        { let stat = heap.collect_garbage()
        ; assert_eq!(stat.data_freed, 0) }

        drop(datum_d);

        { let stat = heap.collect_garbage()
        ; assert_eq!(stat.data_freed, 2) }

        drop(datum_b);

        { let stat = heap.collect_garbage()
        ; assert_eq!(stat.data_freed, 2) }
    }
}

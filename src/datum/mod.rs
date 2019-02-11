//! See [Data] for an overview of data.
//!
//! [Data]: ../../../html/data.html

mod heap;

use std::cell::Cell;
use std::fmt;
use std::marker::PhantomData;
use std::mem::transmute;
use std::ptr::NonNull;

use sigil::Sigil;

pub use self::heap::*;

/// A reference to a datum.
///
/// When a value of this type exists, the datum will not be garbage collected;
/// the value acts as a root. To create a new datum, you need to use a heap.
/// See [Heap] for more information.
///
/// [Heap]: struct.Heap.html
#[repr(transparent)]
pub struct Datum<'a> {
    ptr:     NonNull<DatumInner>,
    phantom: PhantomData<&'a ()>,
}

struct DatumInner {
    mark:        Cell<bool>,
    roots:       Cell<usize>,
    enchantment: Sigil,
    pointers:    Box<[NonNull<DatumInner>]>,
    auxiliary:   Box<[u8]>,
}

impl Datum<'_> {
    pub fn enchantment(&self) -> Sigil {
        // This is safe because the enchantment is copied out of the datum.
        unsafe { self.ptr.as_ref() }.enchantment
    }

    pub fn pointers(&self) -> &[Datum] {
        // This is safe because the returned reference cannot outlive the root,
        // which in turn cannot outlive the heap.
        let pointers = &unsafe { self.ptr.as_ref() }.pointers;

        // This is safe because the representation of Datum is equivalent to
        // that of DatumInner.
        unsafe { transmute::<&[NonNull<DatumInner>], &[Datum]>(pointers) }
    }

    pub fn auxiliary(&self) -> &[u8] {
        // This is safe because the returned reference cannot outlive the root,
        // which in turn cannot outlive the heap.
        &unsafe { self.ptr.as_ref() }.auxiliary
    }

    unsafe fn enroot(ptr: NonNull<DatumInner>) -> Self {
        // TODO: Use Cell::update once stable.
        let roots = &ptr.as_ref().roots;
        roots.set(roots.get() + 1);
        Datum{ptr, phantom: PhantomData}
    }
}

impl Drop for Datum<'_> {
    fn drop(&mut self) {
        // TODO: Use Cell::update once stable.
        // This is safe because self.ptr is always a valid pointer.
        let roots = &unsafe { self.ptr.as_ref() }.roots;
        roots.set(roots.get() - 1);
    }
}

impl Clone for Datum<'_> {
    fn clone(&self) -> Self {
        // This is safe because self.ptr is always a valid pointer.
        unsafe { Datum::enroot(self.ptr) }
    }
}

impl fmt::Debug for Datum<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "heap.allocate({:?}, {:?}, {:?})",
               self.enchantment(),
               self.pointers(),
               self.auxiliary())
    }
}

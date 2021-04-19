use std::marker::PhantomData;
use std::ops::Deref;
use std::mem;

// we should use Unique<T> in place of *mut T when 
// we have a raw pointer to an allocation we own

struct Unique<T> {
    ptr: *const T,              // *const for variance
    _marker: PhantomData<T>,    // for the drop checker
}

// Deriving Send and Sync is safe because we are the Unique
// owners of this data. It's like Unique<T> is "just" T.


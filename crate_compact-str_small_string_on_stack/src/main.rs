extern crate compact_str;

use compact_str::CompactStr;

// Creates a new inline CompactStr at compile time.
const DEFAULT_NAME: CompactStr = CompactStr::new_inline("untitled");
// Trying to create a long string that can’t be inlined, will fail to build.
// const LONG: CompactStr = CompactStr::new_inline("this is a long string that can't be stored on
// the stack");

fn main() {
    // Stack
    // We can inline strings up to 12 characters long on 32-bit architectures...
    #[cfg(target_pointer_width = "32")]
    let s = "i'm 12 chars";
    // ...and up to 24 characters on 64-bit architectures!
    #[cfg(target_pointer_width = "64")]
    let s = "i am 24 characters long!";

    let compact = CompactStr::new(&s);

    assert_eq!(compact, s);
    // we are not allocated on the heap!
    assert!(!compact.is_heap_allocated());

    // A CompactStr will always have a capacity of at least std::mem::size_of::<String>()
    assert_eq!(compact.capacity(), std::mem::size_of::<String>());

    // Heap
    // For longer strings though, we get allocated on the heap
    let long = "I am a longer string that will be allocated on the heap";
    let compact = CompactStr::new(long);

    assert_eq!(compact, long);
    // we are allocated on the heap!
    assert!(compact.is_heap_allocated());
}

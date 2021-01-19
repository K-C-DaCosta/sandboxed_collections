# What is this? 
A set of custom vector-backed collections that i've written over time. This crate aims to have no dependencies to anything but the standard library.  

# Why upload this?
It's better to keep them all of these collections in one package, so when I find and fix a bug all my other projects can also 
benefit from the patch. 

# Why is it called sandboxed?
I call these collections "Sandboxed" because all memory of a collection lives on one contigious 
block of data. As a result, certain operations are impossible to do in `O(1)` like:
```rust
    pub fn merge(a:LinkedList,b:LinkedList)->LinkedList{
        ...
    }  
```

 Because memory address space is local to the underlying allocator(all collections own its own allocator).  `O(1)` merge can be achieved if I share memory with multiple collection instances, however,  Rust makes sharing any kind of resource hard to do(not *impossible* just hard) and so I won't bother to implement that unless I need to combine/split datastructres. 

 # TL;DR 
 Sandboxed collections can't merge/split efficently unless the underlying memory is shared (not the case in any of my collections so far)


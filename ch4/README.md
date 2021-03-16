# Chapter 4. Understanding Ownership

**Ownership Rules**
1. Each value in Rust has a variable that's called *owner.*
2. There can only be one owner at a time.
3. When the owner goes out of scope, the value will be dropped.


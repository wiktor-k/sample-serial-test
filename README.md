# Sample serial test

Illustrates a problem in `serial_test` crate.

  1. Install: `cargo install cargo-valgrind`
  2. Run the tests with `cargo valgrind test`.
     See that valgrind reports success.
  3. Now, uncomment the `#[serial]` line in `src/main.rs`
  4. re-run tests: `cargo valgrind test`.

Observe the following leak report:

```text
       Error leaked 212 B in 1 block
        Info at malloc
             at alloc::alloc::alloc (alloc.rs:87)
             at alloc::alloc::Global::alloc_impl (alloc.rs:169)
             at <alloc::alloc::Global as core::alloc::Allocator>::allocate (alloc.rs:229)
             at hashbrown::raw::alloc::inner::do_alloc (alloc.rs:11)
             at hashbrown::raw::RawTableInner<A>::new_uninitialized (mod.rs:1086)
             at hashbrown::raw::RawTableInner<A>::fallible_with_capacity (mod.rs:1115)
             at hashbrown::raw::RawTableInner<A>::prepare_resize (mod.rs:1359)
             at hashbrown::raw::RawTable<T,A>::reserve_rehash (mod.rs:1432)
             at hashbrown::raw::RawTable<T,A>::reserve (mod.rs:652)
             at hashbrown::raw::RawTable<T,A>::insert (mod.rs:731)
             at hashbrown::map::HashMap<K,V,S,A>::insert (map.rs:1508)
     Summary Leaked 212 B total
error: test failed, to rerun pass '--bin sample-serial-test'
```

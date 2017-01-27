## File-backed HashMap

This code is a **slightly** changed version of the `HashMap` from
`libcollections`. For the original version see
[libcolletions](https://github.com/rust-lang/rust/blob/master/src/libstd/collections/hash/).

The only difference is how memory is allocated. Specifically
this versions allocator uses file-backed memory (effectively swap).

This is probably a bad idea, but could be useful in situations where you plan
to allocate huge HashMaps, but do not want to depend on your systems swap.
There might be several reasons for this:

- You do not have enough swap to run your program , and uses this library as a
  way to temporarily increase your capacity.

- You do not have any swap at all, as you have enough memory to support your
  normal usage.

- You do not wish to minimize the performance impact of running the code, but
  not forcing other processes to swap and drop their caches as the `HashMap`
  claims all your memory. I have not tested how well this works in practice.

Right now this only works on Linux 3.11 and higher, since it uses the
`O_TMPFILE` flag.

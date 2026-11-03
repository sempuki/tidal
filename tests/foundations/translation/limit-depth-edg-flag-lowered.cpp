// tidal-rules: TR-13
// tidal-expect: reject
// tidal-flags: --pending_instantiations=10
// tidal-error-clang: unknown argument
// tidal-error-gcc: unrecognized command-line option
// tidal-error-edg: excessive recursion at instantiation
// tidal-note: The flag sets EDG's limit to 10 instantiations of one template, below the chain of 20. Clang and GCC have no such flag and reject the command line.
template <int N> struct Count { static constexpr int value = Count<N - 1>::value + 1; };
template <> struct Count<0> { static constexpr int value = 0; };
static_assert(Count<20>::value == 20);

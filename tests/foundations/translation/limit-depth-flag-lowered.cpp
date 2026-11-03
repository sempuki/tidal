// tidal-rules: TR-13
// tidal-expect: reject
// tidal-flags: -ftemplate-depth=40
// tidal-error-clang: in instantiation of static data member 'Count<10>::value'
// tidal-error-gcc: In instantiation of .constexpr const int Count<11>::value.
// tidal-error-edg: invalid option
// tidal-note: The flag sets the limit of Clang and GCC to 40, below the chain of 50. Clang stops at Count<10> and GCC at Count<11>, 40 levels down. EDG has no such flag and rejects the command line.
template <int N> struct Count { static constexpr int value = Count<N - 1>::value + 1; };
template <> struct Count<0> { static constexpr int value = 0; };
static_assert(Count<50>::value == 50);

// tidal-rules: TR-13
// tidal-expect: reject
// tidal-error-clang: in instantiation of static data member 'Count<976>::value'
// tidal-error-gcc: In instantiation of .constexpr const int Count<1101>::value.
// tidal-error-edg: excessive recursion at instantiation
// tidal-note: The chain ends at Count<0>, but its 2000 nested instantiations exceed the limit of every compiler in the matrix. Clang stops at Count<976>, 1024 levels down, and GCC at Count<1101>, 900 levels down.
template <int N> struct Count { static constexpr int value = Count<N - 1>::value + 1; };
template <> struct Count<0> { static constexpr int value = 0; };
static_assert(Count<2000>::value == 2000);

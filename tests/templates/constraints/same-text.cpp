// tidal-rules: CNS-4 CNS-24
// tidal-expect: reject
// tidal-error-clang: is ambiguous
// tidal-error-gcc: is ambiguous
// tidal-error-edg: more than one instance of overloaded function
// tidal-note: The two (sizeof(T) > 1) are two expressions in the source, so they are different atomic constraints.
template <class T> requires (sizeof(T) > 1) constexpr int f() { return 1; }
template <class T> requires (sizeof(T) > 1) && true constexpr int f() { return 2; }
int x = f<int>();

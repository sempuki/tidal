// tidal-rules: CTAD-26 CTAD-25
// tidal-expect: reject
// tidal-error-clang: call to constructor of 'Pair<int>' is ambiguous
// tidal-error-gcc: is ambiguous
// tidal-error-edg: more than one instance of constructor
// tidal-note: The user guide gives Pair<int>. The initialization of Pair<int> from two ints then finds two constructors that are equally good.
template <class T> struct Pair { Pair(T, long) {} Pair(long, T) {} };
template <class T> Pair(T, T) -> Pair<T>;
Pair p(1, 2);

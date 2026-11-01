// tidal-rules: CNS-3 CNS-24
// tidal-expect: reject
// tidal-error-clang: is ambiguous
// tidal-error-gcc: is ambiguous
// tidal-error-edg: more than one instance of overloaded function
// tidal-note: Each (!A<T>) is its own atomic constraint, so the second clause does not subsume the first.
template <class T> concept A = sizeof(T) > 1;
template <class T> requires (!A<T>) constexpr int f() { return 1; }
template <class T> requires (!A<T>) && true constexpr int f() { return 2; }
int x = f<char>();

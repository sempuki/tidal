// tidal-rules: CNS-3 CNS-24
// tidal-expect: reject
// tidal-error-clang: is ambiguous
// tidal-error-gcc: is ambiguous
// tidal-error-edg: more than one instance of overloaded function
// tidal-note: Each fold expression is one atomic constraint. The two folds are different expressions, so neither clause subsumes the other.
template <class T> concept A = sizeof(T) > 1;
template <class T> concept AB = A<T> && sizeof(T) < 100;
template <class... T> requires (A<T> && ...) constexpr int f() { return 1; }
template <class... T> requires (AB<T> && ...) constexpr int f() { return 2; }
int x = f<int, double>();

// tidal-rules: CNS-34 CNS-24
// tidal-expect: reject
// tidal-error-clang: call to 'f' is ambiguous
// tidal-error-gcc: is ambiguous
// tidal-error-edg: more than one instance of overloaded function
// tidal-note: The parameters correspond by position. The first function parameter has the type of the first template parameter in one template and of the second in the other, so the constraints are not compared.
template <class T> concept A = sizeof(T) > 0;
template <class T, class U> requires A<T> constexpr int f(T, U) { return 1; }
template <class U, class T> requires A<T> && true constexpr int f(T, U) { return 2; }
int chosen = f(1, 2);

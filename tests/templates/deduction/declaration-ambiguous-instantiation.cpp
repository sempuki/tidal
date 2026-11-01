// tidal-rules: DED-33
// tidal-expect: reject
// tidal-error-clang: partial ordering for explicit instantiation of .* is ambiguous
// tidal-error-gcc: ambiguous template specialization
// tidal-error-edg: more than one instance of overloaded function
// tidal-note: Both templates match the explicit instantiation of void(int, int), and neither is more specialized.
template <class T> void put(T, int) {}
template <class T> void put(int, T) {}
template void put(int, int);

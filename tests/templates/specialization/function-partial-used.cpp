// tidal-rules: TSP-7
// tidal-expect: reject
// tidal-error-clang: function template partial specialization is not allowed
// tidal-error-gcc: non-class, non-variable partial specialization
// tidal-error-edg: an explicit template argument list is not allowed on this
// tidal-note: The same declaration as function-partial.cpp, with a call that uses the template.
template <class T, class U> int pick(T, U) { return 1; }
template <class T> int pick<T, int>(T, int) { return 2; }
int chosen = pick(1.0, 1);

// tidal-rules: TPL-43
// tidal-expect: reject
// tidal-error-clang: does not refer into a class, class template or class template partial specialization
// tidal-error-gcc: invalid use of incomplete type
// tidal-error-edg: template argument list must match the parameter list
template <class T, class U> struct Pair { int first(); };
template <class T, class U> int Pair<U, T>::first() { return 0; }

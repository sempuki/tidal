// tidal-rules: TARG-7
// tidal-expect: reject
// tidal-error-clang: does not refer to a class or alias template|too few template arguments for concept
// tidal-error-gcc: expected .auto. or .decltype\(auto\). after
// tidal-error-edg: is not a class template
// tidal-note: A concept is not a class template or an alias template, so a template template parameter does not take it.
template <class T> concept Integral = T(1) / 2 == 0;
template <template <class> class P, class T> struct Test {};
Test<Integral, int> t;

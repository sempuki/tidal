// tidal-rules: TPL-4
// tidal-expect: reject
// tidal-error-clang: a template declaration can only declare a single entity
// tidal-error-gcc: expected .;. before .,. token
// tidal-error-edg: expected a .;.
template <class T> T first = T(), second = T();

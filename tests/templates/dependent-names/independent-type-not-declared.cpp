// tidal-rules: DEP-6
// tidal-expect: reject
// tidal-error-clang: unknown type name 'Later'
// tidal-error-gcc: Later. was not declared
// tidal-error-edg: identifier "Later" is undefined
// tidal-note: The type name Later does not depend on T. It is declared after the definition of the template, so the definition is not valid.
template <class T> T keep(T x) { Later l; (void)l; return x; }
struct Later {};
int y = keep(0);

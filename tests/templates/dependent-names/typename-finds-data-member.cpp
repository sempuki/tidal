// tidal-rules: DEP-22
// tidal-expect: reject
// tidal-error-clang: typename specifier refers to non-type member 'name'
// tidal-error-gcc: which is not a type
// tidal-error-edg: is not a type name
struct Mixed { struct name {}; int name; };
template <class T> int run() { typename T::name n; return 0; }
int x = run<Mixed>();

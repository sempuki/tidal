// tidal-rules: TPL-25
// tidal-expect: reject
// tidal-error-clang: destructor cannot be declared as a template
// tidal-error-gcc: declared as member template
// tidal-error-edg: expected a declaration
struct Holder { template <class T> ~Holder(); };

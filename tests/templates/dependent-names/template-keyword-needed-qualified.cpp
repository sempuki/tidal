// tidal-rules: DEP-24
// tidal-expect: reject
// tidal-error-clang: expected expression|use 'template' keyword
// tidal-error-gcc: expected primary-expression
// tidal-error-edg: type name is not allowed|expected an expression
struct Has { template <class U> static int make() { return sizeof(U); } };
template <class T> int run() { return T::make<int>(); }
int x = run<Has>();

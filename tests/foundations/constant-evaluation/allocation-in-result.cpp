// tidal-rules: CE-7
// tidal-expect: reject
// tidal-error-gcc: refers to a result of .operator new.
// tidal-error-clang: pointer to heap-allocated object is not a constant expression
// tidal-error-edg: invalid use of address of interpreter storage
constexpr int* make() { return new int(1); }
constexpr int* p = make();

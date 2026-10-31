// tidal-rules: NS-8
// tidal-expect: reject
// tidal-error-clang: expected namespace name
// tidal-error-gcc: S. is not a namespace-name
// tidal-error-edg: name must be a namespace name
struct S {};
using namespace S;

// tidal-rules: NS-14
// tidal-expect: reject
// tidal-error-clang: as an alias for a different namespace
// tidal-error-gcc: namespace L = B;. conflicts with a previous declaration
// tidal-error-edg: namespace .L. has already been defined
namespace A {}
namespace B {}
namespace L = A;
namespace L = B;

// tidal-rules: NS-15
// tidal-expect: reject
// tidal-error-clang: does not enclose namespace
// tidal-error-gcc: not in a namespace surrounding
// tidal-error-edg: cannot be defined in the current scope
// tidal-note: The definition is in v2, the namespace that declares area, so it is not outside that namespace.
namespace geo { inline namespace v2 { int area(); } }
namespace geo { inline namespace v2 { int geo::area() { return 4; } } }

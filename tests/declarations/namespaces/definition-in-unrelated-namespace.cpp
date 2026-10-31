// tidal-rules: NS-15
// tidal-expect: reject
// tidal-error-clang: does not enclose namespace
// tidal-error-gcc: not in a namespace surrounding
// tidal-error-edg: cannot be defined in the current scope
namespace N { int f(); }
namespace M { int N::f() { return 0; } }

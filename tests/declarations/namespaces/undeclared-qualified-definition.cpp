// tidal-rules: NS-15
// tidal-expect: reject
// tidal-error-clang: no member named .x. in namespace
// tidal-error-gcc: should have been declared inside
// tidal-error-edg: has no member .x
namespace N {}
int N::x = 7;

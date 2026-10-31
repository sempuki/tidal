// tidal-rules: SB-3
// tidal-expect: reject
// tidal-error-clang: private member .a. of .S
// tidal-error-gcc: inaccessible member .S::a
// tidal-error-edg: non-public member .S::a
class S { int a; public: int b; };
S s;
auto [x, y] = s;

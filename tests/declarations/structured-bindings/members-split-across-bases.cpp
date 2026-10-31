// tidal-rules: SB-3
// tidal-expect: reject
// tidal-error-clang: both it and its base class .B. have non-static data members
// tidal-error-gcc: both it and its base class .B. have non-static data members
// tidal-error-edg: has no components to bind to
struct B { int a; };
struct D : B { int b; };
D d;
auto [x, y] = d;

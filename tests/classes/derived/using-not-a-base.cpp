// tidal-rules: DER-9
// tidal-expect: reject
struct X { int f(); };
struct B {};
struct D : B { using X::f; };

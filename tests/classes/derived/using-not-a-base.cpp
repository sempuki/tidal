// tidal-rules: DER-9
// tidal-expect: reject
// tidal-error-clang: which is not a base class of .D
// tidal-error-gcc: is not a base type for type .D
// tidal-error-edg: invalid base class
struct X { int f(); };
struct B {};
struct D : B { using X::f; };

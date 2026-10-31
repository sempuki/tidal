// tidal-rules: DER-1
// tidal-expect: reject
// tidal-error-clang: unions cannot have base classes
// tidal-error-gcc: derived union .* invalid
// tidal-error-edg: union is not allowed to have a base class
struct B {};
union U : B { int i; };

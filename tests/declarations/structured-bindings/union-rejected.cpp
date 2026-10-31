// tidal-rules: SB-13
// tidal-expect: reject
// tidal-error-clang: union type .U
// tidal-error-gcc: cannot decompose union type .U
// tidal-error-edg: U. has no components to bind to
union U { int a; float b; };
U u;
auto [x] = u;

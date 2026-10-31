// tidal-rules: AUTO-11
// tidal-expect: reject
// tidal-error-clang: cannot initialize a variable of type .void \(\)
// tidal-error-gcc: has function type
// tidal-error-edg: cannot be used to initialize an entity of type .void \(\)
void fn();
decltype(auto) f = fn;

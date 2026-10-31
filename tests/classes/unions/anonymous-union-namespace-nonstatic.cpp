// tidal-rules: UNI-9
// tidal-expect: reject
// tidal-error-clang: must be declared .static
// tidal-error-gcc: namespace-scope anonymous aggregates must be static
// tidal-error-edg: must be declared static
union { int i; float f; };

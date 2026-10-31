// tidal-rules: STO-11
// tidal-expect: reject
// tidal-error-clang: anonymous unions at namespace or global scope must be declared .static
// tidal-error-gcc: namespace-scope anonymous aggregates must be static
// tidal-error-edg: must be declared static
// tidal-note: Gap review 2026-10-30.
union { int a; float b; };

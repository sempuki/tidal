// tidal-rules: CPT-14
// tidal-expect: reject
// tidal-error-clang: non-templated function cannot have a requires clause
// tidal-error-gcc: constraints on a non-templated function
// tidal-error-edg: a requires-clause is not allowed here
void use() requires true;

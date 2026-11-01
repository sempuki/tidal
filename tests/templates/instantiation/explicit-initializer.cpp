// tidal-rules: INST-23
// tidal-expect: reject
// tidal-error-clang: explicit template instantiation cannot have a definition
// tidal-error-gcc: expected .;. before .=. token
// tidal-error-edg: expected a ";"
template <class T> int zero = 0;
template int zero<int> = 1;

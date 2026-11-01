// tidal-rules: CPT-22
// tidal-expect: reject
// tidal-error-clang: deduced type 'void' does not satisfy 'Wide'
// tidal-error-gcc: deduced return type does not satisfy placeholder constraints
// tidal-error-edg: deduced placeholder type "void" failed
template <class T> concept Wide = sizeof(T) > 1;
Wide auto finish() {}

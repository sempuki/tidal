// tidal-rules: CPT-37
// tidal-expect: reject
// tidal-error-clang: use of concept 'Any' requires template arguments
// tidal-error-gcc: cannot be constrained
// tidal-error-edg: expected a "<"
template <class T> concept Any = true;
int copy = Any auto(1);

// tidal-rules: CPT-5
// tidal-expect: reject
// tidal-error-clang: concept cannot have associated constraints
// tidal-error-gcc: concept cannot be constrained
// tidal-error-edg: cannot have associated constraints
template <class T> concept Any = true;
template <Any T> concept Big = sizeof(T) > 1;

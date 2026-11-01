// tidal-rules: CPT-22
// tidal-expect: reject
// tidal-error-clang: deduced type 'char' does not satisfy 'Big'
// tidal-error-gcc: deduced return type does not satisfy placeholder constraints
// tidal-error-edg: deduced placeholder type "char" failed
template <class T> concept Big = sizeof(T) > 1;
Big auto get() { return 'a'; }

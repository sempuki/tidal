// tidal-rules: CPT-7
// tidal-expect: reject
// tidal-error-clang: must be an identifier
// tidal-error-gcc: expected .=. before .<.
// tidal-error-edg: invalid redeclaration of concept
template <class T> concept Big = sizeof(T) > 1;
template <class T> concept Big<T*> = true;

// tidal-rules: CPT-7
// tidal-expect: reject
// tidal-error-clang: redefinition of 'Big'
// tidal-error-gcc: redefinition of
// tidal-error-edg: invalid redeclaration of concept
template <class T> concept Big = sizeof(T) > 1;
template <class T> concept Big = sizeof(T) > 1;

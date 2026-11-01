// tidal-rules: CPT-7
// tidal-expect: reject
// tidal-error-clang: must be an identifier
// tidal-error-gcc: concept definition syntax
// tidal-error-edg: expected an identifier
template <class T> concept Big = sizeof(T) > 1;
template <> concept Big<char> = true;

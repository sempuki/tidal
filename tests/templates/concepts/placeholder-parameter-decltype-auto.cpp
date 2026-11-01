// tidal-rules: CPT-37
// tidal-expect: reject
// tidal-error-clang: 'decltype\(auto\)' not allowed in function prototype
// tidal-error-gcc: cannot declare a parameter with .decltype\(auto\).
// tidal-error-edg: expected "auto"
template <class T> concept Wide = sizeof(T) > 1;
int take(Wide decltype(auto) value);

// tidal-rules: CPT-18
// tidal-expect: reject
// tidal-error-clang: expected 'auto' or 'decltype\(auto\)'
// tidal-error-gcc: expected .auto. or .decltype\(auto\). after
// tidal-error-edg: is not a type name
template <class T> concept Big = sizeof(T) > 1;
Big value = 1;

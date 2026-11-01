// tidal-rules: CTAD-5 CTAD-23
// tidal-expect: reject
// tidal-error-clang: selected an explicit constructor for copy-list-initialization
// tidal-error-gcc: would use explicit constructor
// tidal-error-edg: copy-list-initialization cannot use a constructor marked "explicit"
// tidal-note: For the deduced T = int, sizeof(T) > 1 is true, so the chosen guide is explicit.
template <class T> struct Narrow { constexpr explicit(sizeof(T) > 1) Narrow(T) {} };
Narrow b = {1};

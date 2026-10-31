// tidal-rules: DCMP-3
// tidal-expect: reject
// tidal-error-clang: template cannot be defaulted
// tidal-error-gcc: template cannot be defaulted
// tidal-error-edg: templates cannot be defaulted
struct S { int x; template <class T> bool operator==(const T&) const = default; };

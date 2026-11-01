// tidal-rules: INST-25
// tidal-expect: reject
// tidal-error-clang: an attribute list cannot appear here
// tidal-error-gcc: expected unqualified-id before .template.
// tidal-error-edg: attributes are not allowed here
template <class T> int get(T) { return 1; }
[[nodiscard]] template int get(int);

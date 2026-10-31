// tidal-rules: DCMP-3
// tidal-expect: reject
struct S { int x; template <class T> bool operator==(const T&) const = default; };

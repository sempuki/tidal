// tidal-rules: DCMP-10
// tidal-expect: reject
// tidal-error-clang: strong_ordering. was not found
// tidal-error-gcc: strong_ordering. is not a member of .std
// tidal-error-edg: no valid std::strong_ordering type found
struct S { int x; auto operator<=>(const S&) const = default; };
bool f(S a, S b) { return a < b; }

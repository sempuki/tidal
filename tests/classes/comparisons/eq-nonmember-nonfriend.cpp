// tidal-rules: DCMP-3
// tidal-expect: reject
// tidal-error-clang: is not a friend of
// tidal-error-gcc: is not a friend of
// tidal-error-edg: must be a member or a friend
struct S { int x; };
bool operator==(const S&, const S&) = default;

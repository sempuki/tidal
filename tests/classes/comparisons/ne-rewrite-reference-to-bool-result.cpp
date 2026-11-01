// tidal-rules: DCMP-12
// tidal-expect: reject
// tidal-expect-gcc: accept
// tidal-error-clang: rewritten .!=. comparison is not .bool.
// tidal-error-edg: does not return type bool
// tidal-note: Q-0158. An == that rewrites != and returns const bool& does not return bool. GCC accepts the comparison.
struct Meter { const bool& operator==(int) const; };
bool check(Meter m) { return m != 1; }

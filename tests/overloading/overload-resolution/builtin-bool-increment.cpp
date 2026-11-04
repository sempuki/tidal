// tidal-rules: OPR-15 OVL-9
// tidal-expect: reject
// tidal-error-clang: cannot increment value of type .ToBoolRef.
// tidal-error-gcc: no match for .operator\+\+.
// tidal-error-edg: no operator .\+\+. matches
// tidal-note: The increment candidates exist for every arithmetic type except bool.
struct ToBoolRef { operator bool&(); };
auto r = ++ToBoolRef{};

// tidal-rules: CPY-3
// tidal-expect: reject
struct S { static S& operator=(const S&); };

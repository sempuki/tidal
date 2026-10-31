// tidal-rules: CPY-16
// tidal-expect: reject
struct S { static S& operator=(const S&); };

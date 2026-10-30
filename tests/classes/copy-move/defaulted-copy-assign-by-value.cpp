// tidal-rules: CPY-14
// tidal-expect: reject
struct S { S& operator=(S) = default; };

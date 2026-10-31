// tidal-rules: CPY-17
// tidal-expect: reject
struct S { S& operator=(S) = default; };

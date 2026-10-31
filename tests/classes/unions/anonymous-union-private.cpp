// tidal-rules: UNI-9
// tidal-expect: reject
struct S { union { private: int i; }; };

// tidal-rules: UNI-7
// tidal-expect: reject
struct S { union { private: int i; }; };

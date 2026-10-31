// tidal-rules: UNI-1
// tidal-expect: reject
union U { private: int i; };
int f(U u) { return u.i; }

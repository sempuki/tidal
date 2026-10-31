// tidal-rules: UNI-1
// tidal-expect: reject
// tidal-error-clang: is a private member
// tidal-error-gcc: private within this context
// tidal-error-edg: is inaccessible
union U { private: int i; };
int f(U u) { return u.i; }

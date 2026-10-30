// tidal-rules: ACC-1
// tidal-expect: accept
union U { int i; float f; };
int f(U u) { return u.i; }

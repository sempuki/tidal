// tidal-rules: PRIM-12
// tidal-expect: accept
void f();
void (&r)() = f;
void (*p)() = &f;

// tidal-rules: THR-3
// tidal-expect: accept
// tidal-note: A pointer to void is a pointer to an incomplete type that a throw allows. A pointer to a pointer to an incomplete class points to a complete type.
struct I;
void g(void* p) { throw p; }
void h(I** p) { throw p; }

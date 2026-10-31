// tidal-rules: DER-2
// tidal-expect: accept
struct B { int a; int b; };
struct D : B { int c; };
static_assert(sizeof(D) >= sizeof(B) + sizeof(int));

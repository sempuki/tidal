// tidal-rules: TYP-1
// tidal-expect: accept
// tidal-flags: -m32
int f();
static_assert(sizeof(void*) == 4 && sizeof(&f) == 4 && sizeof(nullptr) == 4);

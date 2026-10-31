// tidal-rules: TYP-1
// tidal-expect: accept
int f();
static_assert(sizeof(void*) == 8 && sizeof(&f) == 8 && sizeof(nullptr) == 8);

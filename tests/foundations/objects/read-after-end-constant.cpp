// tidal-rules: OBJ-2
// tidal-expect: reject
// tidal-error-gcc: accessing .x. outside its lifetime
constexpr int f() { const int* p; { int x = 1; p = &x; } return *p; }
static_assert(f() == 1);

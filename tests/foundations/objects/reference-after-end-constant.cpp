// tidal-rules: OBJ-2
// tidal-expect: reject
// tidal-error-gcc: accessing .x. outside its lifetime
constexpr int f() { const int* keep = nullptr; { int x = 4; const int& r = x; keep = &r; } return *keep; }
static_assert(f() == 4);

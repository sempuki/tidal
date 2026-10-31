// tidal-rules: OBJ-8
// tidal-expect: reject
constexpr int f() { const int* p = nullptr; { int x = 2; const int& r = x; p = &r; } return *p; }
static_assert(f() == 2);

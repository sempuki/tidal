// tidal-rules: OBJ-8 OBJ-2
// tidal-expect: reject
// tidal-error-gcc: accessing .x. outside its lifetime
// tidal-error-clang: read of object outside its lifetime
// tidal-error-edg: attempt to access expired storage
constexpr int f() { const int* p = nullptr; { int x = 2; const int& r = x; p = &r; } return *p; }
static_assert(f() == 2);

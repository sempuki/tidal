// tidal-rules: UN-7
// tidal-expect: reject
// tidal-error-gcc: outside its lifetime
// tidal-error-clang: read of object outside its lifetime
// tidal-error-edg: attempt to access expired storage
// tidal-note: Gap review 2026-10-31.
constexpr int f() { int* p = nullptr; { int x = 1; p = &x; } return *p; }
static_assert(f() == 1);

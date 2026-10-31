// tidal-rules: UN-7
// tidal-expect: reject
// tidal-error-gcc: outside its lifetime
// tidal-note: Gap review 2026-10-31.
constexpr int f() { int* p = nullptr; { int x = 1; p = &x; } return *p; }
static_assert(f() == 1);

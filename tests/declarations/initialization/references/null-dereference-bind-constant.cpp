// tidal-rules: REF-5
// tidal-expect: reject
// tidal-expect-edg: accept
// tidal-expect-clang-18: accept
// tidal-error-gcc: dereferencing a null pointer
// tidal-note: Q-0110. EDG and Clang 18 accept a reference bound through a null pointer during constant evaluation. GCC and Clang 22 reject it.
// tidal-note: Gap review 2026-10-31.
constexpr int f() { int* p = nullptr; int& r = *p; (void)r; return 0; }
static_assert(f() == 0);

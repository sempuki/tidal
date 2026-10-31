// tidal-rules: CE-14
// tidal-expect: reject
// tidal-note: Gap review 2026-10-31.
void f() { int local = 0; constexpr int* p = &local; }

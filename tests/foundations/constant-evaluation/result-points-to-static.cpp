// tidal-rules: CE-14
// tidal-expect: accept
// tidal-note: Gap review 2026-10-31.
int global = 0;
constexpr int* p = &global;
void f() { static int s = 0; constexpr int* q = &s; }

// tidal-rules: AUTO-14
// tidal-expect: run
// tidal-note: Gap review 2026-10-31.
struct S { static const auto n = 1; };
int main() { return S::n == 1 ? 0 : 1; }

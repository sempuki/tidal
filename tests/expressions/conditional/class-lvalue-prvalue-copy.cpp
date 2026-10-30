// tidal-rules: COND-8
// tidal-expect: reject
// tidal-note: Gap review 2026-10-30.
struct S { S() = default; S(const S&) = delete; };
S make();
S s;
void f(bool c) { auto&& r = c ? s : make(); }

// tidal-rules: COND-8
// tidal-expect: reject
// tidal-error-clang: call to deleted constructor
// tidal-error-gcc: use of deleted function
// tidal-error-edg: it is a deleted function
// tidal-note: Gap review 2026-10-30.
struct S { S() = default; S(const S&) = delete; };
S make();
S s;
void f(bool c) { auto&& r = c ? s : make(); }

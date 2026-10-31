// tidal-rules: COND-12
// tidal-expect: accept
// tidal-note: Gap review 2026-10-31 (third).
struct S { S() {} S(const S&) = delete; S(S&&) = delete; };
S make();
S pick(bool c) { return c ? make() : make(); }
void use(bool c) { S s = c ? make() : make(); }

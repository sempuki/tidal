// tidal-rules: CPY-13
// tidal-expect: accept
// tidal-flags: -Wall -Werror
struct S { S() {} S(const S&) {} S(S&&) {} };
S make() { S s; return s; }

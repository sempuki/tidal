// tidal-rules: UNW-8
// tidal-expect: accept
// tidal-flags: -Wall -Werror
void g();
void f() noexcept { g(); }

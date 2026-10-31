// tidal-rules: UNEV-2
// tidal-expect: accept
// tidal-flags: -Wall -Werror
[[nodiscard]] int f();
void g() { (void)f(); static_cast<void>(f()); }

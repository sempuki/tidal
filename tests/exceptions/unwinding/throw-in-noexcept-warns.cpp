// tidal-rules: UNW-8
// tidal-expect: reject
// tidal-flags: -Wall -Werror
void f() noexcept { throw 0; }

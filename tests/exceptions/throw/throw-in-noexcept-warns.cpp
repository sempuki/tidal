// tidal-rules: THR-11
// tidal-expect: reject
// tidal-flags: -Wall -Werror
void f() noexcept { throw 1; }

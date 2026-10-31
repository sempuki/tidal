// tidal-rules: LAM-21
// tidal-expect: reject
// tidal-error-clang: implicit capture of .this. with a capture default of .=. is deprecated
// tidal-error-gcc: implicit capture of .this. via .\[=\]. is deprecated
// tidal-note: Gap review 2026-10-31.
// tidal-flags: -Wall -Wextra -Werror
struct S { int v = 0; int f() { return [=] { return v; }(); } };

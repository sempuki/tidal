// tidal-rules: TRY-13
// tidal-expect: reject
// tidal-expect-gcc: accept
// tidal-note: Recommended practice only. Clang warns under -Wall. GCC gives no warning.
// tidal-flags: -Wall -Werror
struct S { int m; S() try : m(1) { } catch (int) { int v = m; (void)v; } };

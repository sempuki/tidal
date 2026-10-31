// tidal-rules: TRY-13
// tidal-expect: reject
// tidal-expect-gcc: accept
// tidal-error-clang: non-static member from the handler of a constructor function try block
// tidal-note: Recommended practice only. Clang warns under -Wall. GCC gives no warning.
// tidal-flags: -Wall -Werror
struct S { int m; S() try : m(1) { } catch (int) { int v = m; (void)v; } };

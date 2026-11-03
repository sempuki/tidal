// tidal-rules: TRY-13
// tidal-expect: reject
// tidal-expect-gcc: accept
// tidal-error-clang: non-static member from the handler of a destructor function try block
// tidal-flags: -Wall -Werror
// tidal-note: Recommended practice only. The handler of a destructor's function try block names a member. Clang warns under -Wall. GCC gives no warning.
struct S {
  int m = 1;
  ~S() try { } catch (...) { int v = m; (void)v; }
};

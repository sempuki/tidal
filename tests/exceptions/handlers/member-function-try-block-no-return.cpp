// tidal-rules: TRY-12
// tidal-expect: reject
// tidal-expect-gcc: accept
// tidal-error-clang: does not return a value in all control paths
// tidal-flags: -Wall -Werror
// tidal-note: A member function with a function try block. Clang warns under -Wall; GCC gives no warning.
struct S {
  int f() try { return 1; } catch (...) { }
};
int main() { return S{}.f(); }

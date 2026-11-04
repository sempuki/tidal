// tidal-rules: FN-7
// tidal-expect: accept
// tidal-expect-gcc: reject
// tidal-error-gcc: control reaches end of non-void function|no return statement
// tidal-flags: -Werror -Wreturn-type
// tidal-note: A goto back to its own label forms a loop with no exit, so control cannot reach the end. Clang gives no warning. GCC warns. FN-7 is recommended practice, so both results conform.
int f() {
top:
  goto top;
}

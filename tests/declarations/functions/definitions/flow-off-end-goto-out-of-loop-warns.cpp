// tidal-rules: FN-7
// tidal-expect: reject
// tidal-flags: -Werror -Wreturn-type
// tidal-error-clang: does not return a value
// tidal-error-gcc: control reaches end of non-void function|no return statement
// tidal-note: X24-4. The goto leaves the loop whose condition is a constant true, so control can reach the end of the function.
extern bool done;
int f() {
  while (true) {
    if (done) goto finished;
  }
finished:;
}

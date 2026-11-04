// tidal-rules: FN-7
// tidal-expect: reject
// tidal-flags: -Werror -Wreturn-type
// tidal-error-clang: does not return a value
// tidal-error-gcc: control reaches end of non-void function|no return statement
// tidal-note: The break leaves the loop, so control can reach the end even though the condition is a constant true.
extern bool done;
int f() { while (true) { if (done) break; } }

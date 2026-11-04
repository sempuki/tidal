// tidal-rules: FN-7
// tidal-expect: reject
// tidal-flags: -Werror -Wreturn-type
// tidal-error-clang: does not return a value
// tidal-error-gcc: control reaches end of non-void function|no return statement
// tidal-note: The call of a function without [[noreturn]] does not end the path, also if the function never returns.
void loop_forever();
int f() { loop_forever(); }

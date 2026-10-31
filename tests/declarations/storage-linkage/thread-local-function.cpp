// tidal-rules: STO-6
// tidal-expect: reject
// tidal-error-clang: thread_local. is only allowed on variable declarations
// tidal-error-gcc: thread_local. invalid for function
// tidal-error-edg: thread-local storage class is not valid here
thread_local void f();

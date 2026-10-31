// tidal-rules: CLS-9
// tidal-expect: reject
// tidal-error-clang: thread_local. is only allowed on variable declarations
// tidal-error-gcc: storage class specified for
// tidal-error-edg: thread-local storage class is not valid here
// tidal-note: Gap review 2026-10-31.
struct S { thread_local int v; };

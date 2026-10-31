// tidal-rules: LSP-2
// tidal-expect: reject
// tidal-error-clang: cannot combine with previous .extern
// tidal-error-gcc: invalid use of .static. in linkage specification
// tidal-error-edg: storage class may not be specified here
extern "C" static void f();

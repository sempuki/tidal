// tidal-rules: FN-14
// tidal-expect: reject
// tidal-error-clang: differ only in their return type
// tidal-error-gcc: ambiguating new declaration
// tidal-error-edg: distinguished by return type alone
int f();
long f();

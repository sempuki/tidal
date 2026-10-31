// tidal-rules: CPY-9
// tidal-expect: reject
// tidal-error-clang: implicitly-deleted|call to deleted
// tidal-error-gcc: use of deleted function
// tidal-error-edg: deleted function
struct M { M() {} M(M&&) {} };
void take(M);
void f() { M m; take(m); }

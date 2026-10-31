// tidal-rules: CPY-11
// tidal-expect: reject
// tidal-error-clang: implicitly-deleted|call to deleted
// tidal-error-gcc: use of deleted function
// tidal-error-edg: deleted function
struct M { M() {} M(M&&) {} };
M take(M& m) { return m; }

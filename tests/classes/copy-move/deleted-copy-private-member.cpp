// tidal-rules: CPY-7
// tidal-expect: reject
// tidal-error-clang: implicitly-deleted|call to deleted
// tidal-error-gcc: use of deleted function
// tidal-error-edg: deleted function
class M { M(const M&); public: M() {} };
struct S { M m; };
S a;
S b(a);

// tidal-rules: CPY-7
// tidal-expect: reject
class M { M(const M&); public: M() {} };
struct S { M m; };
S a;
S b(a);

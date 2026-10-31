// tidal-rules: THR-1
// tidal-expect: accept
struct N { N() {} N(const N&) = delete; N(N&&) = delete; };
void f() { throw N(); }

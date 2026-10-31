// tidal-rules: THR-3
// tidal-expect: reject
struct N { N() {} N(const N&) = delete; N(N&&) = delete; };
N global;
void f() { throw global; }

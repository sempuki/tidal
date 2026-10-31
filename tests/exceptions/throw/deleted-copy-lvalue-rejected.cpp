// tidal-rules: THR-3
// tidal-expect: reject
// tidal-error-clang: call to deleted constructor of .N
// tidal-error-gcc: use of deleted function .N::N\(const N&\)
// tidal-error-edg: N::N\(const N &\).*deleted function
struct N { N() {} N(const N&) = delete; N(N&&) = delete; };
N global;
void f() { throw global; }

// tidal-rules: TRY-5
// tidal-expect: reject
// tidal-error-clang: call to deleted constructor of .N
// tidal-error-gcc: use of deleted function .N::N\(const N&\)
// tidal-error-edg: N::N\(const N &\).*deleted function
struct N { N() {} N(const N&) = delete; };
void f() {
  try { throw N(); }    // the prvalue needs no copy
  catch (N n) {}        // the handler parameter does
}

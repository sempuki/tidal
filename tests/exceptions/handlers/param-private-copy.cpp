// tidal-rules: TRY-5
// tidal-expect: reject
// tidal-error-clang: calling a private constructor of class .P
// tidal-error-gcc: P::P\(const P&\). is private within this context
// tidal-error-edg: P::P\(const P &\).* is inaccessible
class P { P(const P&); public: P() {} };
void f() {
  try { throw P(); }
  catch (P p) {}
}

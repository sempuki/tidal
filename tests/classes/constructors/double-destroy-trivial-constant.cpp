// tidal-rules: DTOR-7
// tidal-expect: reject
// tidal-expect-edg: accept
// tidal-error-gcc: destroying .p. outside its lifetime
// tidal-error-clang: destroying object .* whose lifetime has already ended
// tidal-note: Q-0054. EDG 7.0 accepts a second destruction of an object during constant evaluation.
struct P { int v; };              // the destructor is trivial
constexpr int f() {
  P p{1};
  p.~P();
  return 0;                       // the end of the block destroys p again
}
static_assert(f() == 0);

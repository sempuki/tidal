// tidal-rules: FN-12
// tidal-expect: reject
// tidal-error-clang: incomplete result type .Later.
// tidal-error-gcc: return type .struct Later. is incomplete
// tidal-error-edg: returns incomplete type .Later.
// tidal-note: RV10-R-2. Inner is defined after Outer with a qualified name. Later is complete only after the definition of Inner ends, so the member function's return type is incomplete where the check falls.
struct Later;
struct Outer { struct Inner; };
struct Outer::Inner {
  Later make() const { throw 0; }
};
struct Later { int k; };

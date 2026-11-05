// tidal-rules: NS-12
// tidal-expect: reject
// tidal-error-clang: reference to .item. is ambiguous
// tidal-error-gcc: reference to .item. is ambiguous
// tidal-error-edg: ambiguous
// tidal-note: The class item is in the inline namespace v1 and the variable item in lib. Every matrix compiler calls the unqualified use ambiguous. The gap review of the text phase 8 wrote reads the C++23 text as setting the class aside (documentation/decisions/pending.md, Matrix against review).
namespace lib {
  inline namespace v1 { struct item { }; }
  int item = 3;
  int read() { return item; }
}

// tidal-rules: NS-12
// tidal-expect: reject
// tidal-error-clang: reference to .limit. is ambiguous
// tidal-error-gcc: reference to .limit. is ambiguous
// tidal-error-edg: ambiguous
// tidal-note: limit is declared in the inline namespace v1 and in its enclosing namespace lib. An unqualified use in lib finds both.
namespace lib {
  inline namespace v1 { int limit = 1; }
  int limit = 2;
  int read() { return limit; }
}

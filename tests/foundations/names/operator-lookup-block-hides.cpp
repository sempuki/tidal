// tidal-rules: NAM-38 OPR-15 OVL-7 OPR-6
// tidal-expect: reject
// tidal-error-gcc: no match for .operator\+.
// tidal-error-clang: invalid operands to binary expression
// tidal-error-edg: no operator .\+. matches these operands
// tidal-note: The block declaration of operator+ hides the global one. Argument-dependent lookup searches only m, which has no operator+.
namespace m { struct Value { int n; }; }
int operator+(m::Value a, m::Value b) { return a.n + b.n; }
int main() {
  int operator+(m::Value, int);
  return m::Value{1} + m::Value{2};
}

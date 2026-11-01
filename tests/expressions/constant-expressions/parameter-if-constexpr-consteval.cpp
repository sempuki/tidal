// tidal-rules: CE-30 CE-3
// tidal-expect: reject
// tidal-error-gcc: .n. is not a constant expression
// tidal-error-clang: constexpr if condition is not a constant expression
// tidal-error-edg: parameter .n.
// tidal-note: The condition of if constexpr starts its own constant evaluation, also inside a consteval function.
consteval int sign(int n) {
  if constexpr (n > 0) return 1;
  return 0;
}
int main() { return sign(1) - 1; }

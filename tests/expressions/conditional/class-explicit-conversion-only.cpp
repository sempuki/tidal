// tidal-rules: COND-5 COND-11 CVF-11
// tidal-expect: reject
// tidal-error-clang: incompatible operand types
// tidal-error-gcc: operands to .\?:. have different types
// tidal-error-edg: operand types are incompatible
// tidal-note: Roadmap round 9 (I4-5), I4 reduction r3-explicit-conversion.cpp. The class has only an explicit conversion function to int, and the other operand is an int. Neither operand converts, and COND-11 finds no candidate, so the expression is not valid.
struct Flag {
  explicit operator int() const { return 1; }
};
int main() {
  Flag f;
  return true ? f : 0;
}

// tidal-rules: COND-16 COND-5 CVF-3
// tidal-expect: reject
// tidal-error-clang: incompatible operand types
// tidal-error-gcc: operands to .\?:. have different types
// tidal-error-edg: operand types are incompatible
// tidal-note: Decision 0059, item F2 (I4-12). The class has only an explicit conversion function to int&, and the other operand is an int lvalue. Step 1 initializes its reference as copy initialization does, so the explicit function takes no part, and no operand converts.
int store;
struct Handle {
  explicit operator int&() { return store; }
};
int other;
extern bool c;
void f() {
  Handle h;
  (void)(c ? h : other);
}

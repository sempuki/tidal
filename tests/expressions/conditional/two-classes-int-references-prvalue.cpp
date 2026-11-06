// tidal-rules: COND-11 OVL-11
// tidal-expect: reject
// tidal-error-clang: cannot take the address of an rvalue
// tidal-error-gcc: lvalue required as unary .&. operand
// tidal-error-edg: expression must be an lvalue
// tidal-note: Decision 0059, item F2 (I4-10), I4 program p3-two-int-refs.cpp. Two classes convert to int&. The built-in candidate (int, int) wins, and the result is the prvalue that COND-11 gives, so its address cannot be taken.
int first, second;
struct A { operator int&() { return first; } };
struct B { operator int&() { return second; } };
extern bool c;
void f() {
  A a;
  B b;
  int* p = &(c ? a : b);
  (void)p;
}

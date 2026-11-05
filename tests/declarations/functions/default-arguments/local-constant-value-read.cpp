// tidal-rules: DEFARG-4
// tidal-expect: reject
// tidal-expect-clang: accept
// tidal-error-gcc: local variable .count. may not appear in this context
// tidal-error-edg: reference to local variable of enclosing function
// tidal-note: Q-0389. The default argument reads the value of a local constant, which needs no storage of the local. GCC and EDG reject any use of the local outside an operand that is not evaluated. Clang accepts, as the gap review of the text phase 8 wrote reads the C++23 text.
int main() {
  const int count = 3;
  int pick(int n = count);
  return 0;
}

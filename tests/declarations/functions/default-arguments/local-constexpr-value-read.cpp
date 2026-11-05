// tidal-rules: DEFARG-4
// tidal-expect: reject
// tidal-expect-clang: accept
// tidal-error-gcc: local variable .limit. may not appear in this context
// tidal-error-edg: reference to local variable of enclosing function
// tidal-note: Q-0389. The default argument computes a value from a constexpr local. GCC and EDG reject it. Clang accepts it.
int main() {
  constexpr int limit = 3;
  int pick(int n = limit + 1);
  return 0;
}

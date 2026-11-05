// tidal-rules: INST-12 CNS-12
// tidal-expect: reject
// tidal-error-clang: constraints not satisfied
// tidal-error-gcc: no matching function
// tidal-error-edg: requires-clause is not allowed here
// tidal-note: G149, Q-0392. The waiting requires clause names T::value, and int has no such member. The constraint is not satisfied, so the call of get is not valid.
template <class T> int f() {
  struct L {
    int get() requires (T::value > 0) { return 1; }
  };
  L l;
  return l.get();
}
int main() { return f<int>(); }

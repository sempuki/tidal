// tidal-rules: INST-12 CNS-12
// tidal-expect: run
// tidal-expect-edg: reject
// tidal-error-edg: requires-clause is not allowed here
// tidal-note: G149, Q-0392. The waiting requires clause names T::value, and int has no such member. At the use, the substitution failure makes the constraint not satisfied, so the requires expression is false. Clang and GCC give false. EDG rejects the requires clause itself.
template <class T> int f() {
  struct L {
    int get() requires (T::value > 0) { return 1; }
  };
  L l;
  if constexpr (requires { l.get(); })
    return 1;
  else
    return 0;
}
int main() { return f<int>(); }

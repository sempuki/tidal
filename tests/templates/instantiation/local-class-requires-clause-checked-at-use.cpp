// tidal-rules: INST-12 CNS-12
// tidal-expect: reject
// tidal-error-clang: constraints not satisfied
// tidal-error-gcc: no matching function
// tidal-error-edg: requires-clause is not allowed here
// tidal-note: Q-0392. The call of get needs its requires clause, which int does not satisfy, so the call is rejected. Clang and GCC reject the call. EDG rejects the requires clause itself.
template <class T> int make() {
  struct Local {
    int get() requires (sizeof(T) > 100) { return 1; }
  };
  return Local().get();
}
int use() { return make<int>(); }

// tidal-rules: INST-12
// tidal-expect: run
// tidal-expect-edg: reject
// tidal-error-edg: requires-clause is not allowed here
// tidal-note: Q-0392. The requires clause of get names T::value, which int does not have. The instantiation of make<int> instantiates the local class, and the requires clause waits for a use of get. Nothing uses get, so the program is valid. Clang and GCC accept and run it. EDG rejects a requires clause on a member of a local class.
template <class T> int make() {
  struct Local {
    int get() requires (T::value > 0) { return 1; }
    int other() { return 2; }
  };
  return Local().other();
}
int main() { return make<int>() == 2 ? 0 : 1; }

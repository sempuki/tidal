// tidal-rules: CPT-11
// tidal-expect: accept
// tidal-expect-edg: reject
// tidal-note: EDG rejects a trailing clause on a member of a local class in a function template (Q-0260).
template <class T> int outer() {
  struct Local { int get() requires (sizeof(T) > 1) { return 1; } };
  return Local{}.get();
}
int value = outer<int>();

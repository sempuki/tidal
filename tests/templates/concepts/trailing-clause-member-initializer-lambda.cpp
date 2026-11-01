// tidal-rules: CPT-11
// tidal-expect: accept
// tidal-expect-edg: reject
// tidal-error-edg: requires-clause is not allowed here
// tidal-note: The lambda is inside a class template, so it is templated. EDG rejects the trailing clause on a lambda that is not generic (Q-0260).
template <class T> struct Box {
  int value = []() requires (sizeof(T) > 1) { return 1; }();
};
Box<int> box;

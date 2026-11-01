// tidal-rules: CNS-21
// tidal-expect: accept
// tidal-expect-edg: reject
// tidal-error-edg: default constructor of "Slot" cannot be referenced
// tidal-note: The user-provided constructor is more constrained but not satisfied for int, so the defaulted one is eligible and trivial. EDG treats the default constructor as not trivial (Q-0266).
template <class T> concept Narrow = sizeof(T) == 1;
template <class T> struct Box {
  Box() = default;
  Box() requires Narrow<T> {}
  int value;
};
union Slot { Box<int> box; int raw; };
Slot slot;

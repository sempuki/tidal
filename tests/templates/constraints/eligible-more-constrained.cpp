// tidal-rules: CNS-21
// tidal-expect: accept
// tidal-expect-edg: reject
// tidal-note: Both constructors are satisfied for int. The defaulted one is more constrained, so it alone is eligible and the default constructor is trivial. EDG treats it as not trivial (Q-0266).
template <class T> concept A = sizeof(T) > 1;
template <class T> concept AB = A<T> && sizeof(T) < 100;
template <class T> struct Box {
  Box() requires A<T> {}
  Box() requires AB<T> = default;
  int value;
};
union Slot { Box<int> box; int raw; };
Slot slot;

// tidal-rules: TPL-23 CNS-26
// tidal-expect: reject
// tidal-expect-edg: accept
// tidal-error-clang: requires clause differs
// tidal-error-gcc: different constraints
// tidal-note: Q-0385. Clang and GCC reject the definition. EDG 7.0 accepts it, and rejects a definition that leaves out the type constraint.
// The definition after the class body leaves out the requires clause of
// the class's template head.
template <class T> concept Small = sizeof(T) <= 8;
template <Small T> requires (sizeof(T) > 1)
struct Cell {
  T value;
  T get() const;
};
template <Small T>
T Cell<T>::get() const { return value; }

// tidal-rules: ORD-13 ORD-15
// tidal-expect: reject
// tidal-expect-edg: accept
// tidal-error-clang: is ambiguous
// tidal-error-gcc: is ambiguous
// tidal-note: The results T* and T* const tie in ordering but are not the same type, so the constraint does not decide. EDG lets the constraint decide (Q-0210).
struct Source {
  template <class T> operator T*() const { return nullptr; }
  template <class T> requires (sizeof(T) > 1) operator T* const() const { return nullptr; }
};
int* p = Source{};

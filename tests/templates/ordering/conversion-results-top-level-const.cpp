// tidal-rules: ORD-2 ORD-8 ORD-15
// tidal-expect: reject
// tidal-error-clang: is ambiguous
// tidal-error-gcc: is ambiguous
// tidal-error-edg: more than one conversion function
// tidal-note: The results T* and T* const are the same after ORD-2 removes the top-level const, so each template is at least as specialized as the other. Nothing breaks the tie.
struct Source {
  template <class T> operator T*() const { return nullptr; }
  template <class T> operator T* const() const { return nullptr; }
};
int* target = Source{};

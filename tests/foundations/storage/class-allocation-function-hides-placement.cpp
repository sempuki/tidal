// tidal-rules: STG-5 NEW-13 OVL-35
// tidal-expect: reject
// tidal-error-clang: no matching function for call to .operator new.
// tidal-error-gcc: no matching function for call to
// tidal-error-edg: cannot be called with the given argument list
// tidal-note: Decision 0059, item RV5, point RV5-A-12. The class declares an allocation function, so the search for new (p) T stops in the class, which has no non-allocating form.
inline void* operator new(decltype(sizeof 0), void* place) noexcept { return place; }
struct Pooled {
  int v = 0;
  static void* operator new(decltype(sizeof 0));
};
alignas(Pooled) unsigned char storage[sizeof(Pooled)];
Pooled* make() { return new (storage) Pooled; }

// tidal-rules: STG-5 NEW-13
// tidal-expect: accept
// tidal-note: Decision 0059, item RV5, point RV5-A-12. The class declares an allocation function, and ::new (p) T searches only the global scope, so it finds the non-allocating form.
inline void* operator new(decltype(sizeof 0), void* place) noexcept { return place; }
struct Pooled {
  int v = 0;
  static void* operator new(decltype(sizeof 0));
};
alignas(Pooled) unsigned char storage[sizeof(Pooled)];
Pooled* make() { return ::new (storage) Pooled; }

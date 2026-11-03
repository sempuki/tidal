// tidal-rules: EXS-4
// tidal-expect: accept
// tidal-note: The implicit constructor of Whole calls the noexcept constructors of its members. The destructors that would run after a throw do not count.
struct Part {
  Part() noexcept {}
  ~Part() noexcept(false) {}
};
struct Whole {
  Part first;
  Part second;
  static void* operator new(decltype(sizeof 0), void* place) noexcept { return place; }
};
alignas(Whole) unsigned char storage[sizeof(Whole)];
static_assert(noexcept(new (storage) Whole()));
static_assert(!noexcept(Whole()));   // the temporary's destructor counts here

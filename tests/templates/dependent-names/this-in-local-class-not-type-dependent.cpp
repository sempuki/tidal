// tidal-rules: DEP-2
// tidal-expect: accept
// tidal-expect-clang-18: reject
// tidal-error-clang-18: static assertion failed
// tidal-note: I6-20. In a member of a local class of a function template, this is not type-dependent, because the local class is not a class template. The member n has a type that does not depend on T, so pick(this->n, 0) binds at the definition, and the later pick(Item, int) is not seen. Clang 18 treats the call as dependent (Q-0434).
namespace lib {
struct Item {};
constexpr int pick(Item, long) { return 1; }
}
template <class T> constexpr int run() {
  struct Local {
    lib::Item n;
    constexpr int get() const { return pick(this->n, 0); }
  };
  return Local{}.get();
}
namespace lib {
constexpr int pick(Item, int) { return 2; }
}
static_assert(run<int>() == 1);

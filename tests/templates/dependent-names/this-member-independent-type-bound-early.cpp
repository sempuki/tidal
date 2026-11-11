// tidal-rules: DEP-2 DEP-5
// tidal-expect: accept
// tidal-expect-clang-18: reject
// tidal-error-clang-18: static assertion failed
// tidal-note: I6-5. The member n has the type lib::Item, which does not depend on T. So this->n and n are not type-dependent, and each call to pick binds at the definition to pick(Item, long). The later pick(Item, int) is not seen. Clang 18 treats this->n as type-dependent and finds the later function at the instantiation (Q-0434).
namespace lib {
struct Item {};
constexpr int pick(Item, long) { return 1; }
}
template <class T> struct Holder {
  lib::Item n;
  constexpr int through_this() const { return pick(this->n, 0); }
  constexpr int by_name() const { return pick(n, 0); }
};
namespace lib {
constexpr int pick(Item, int) { return 2; }
}
static_assert(Holder<int>{}.through_this() == 1);
static_assert(Holder<int>{}.by_name() == 1);

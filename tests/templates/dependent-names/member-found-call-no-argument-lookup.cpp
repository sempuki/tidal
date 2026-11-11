// tidal-rules: DEP-40 DEP-14
// tidal-expect: accept
// tidal-note: I6-9. In Holder, the name pick finds the member function at the definition. The call pick(T{}, 0) has a dependent argument, and still takes no argument-dependent lookup, so the later lib::pick(Item, int) is not seen.
namespace lib { struct Item {}; }
template <class T> struct Holder {
  static constexpr int pick(T, long) { return 1; }
  static constexpr int run() { return pick(T{}, 0); }
};
namespace lib {
constexpr int pick(Item, int) { return 2; }
}
static_assert(Holder<lib::Item>::run() == 1);

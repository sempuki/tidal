// tidal-rules: DEP-40
// tidal-expect: accept
// tidal-note: I6-9. lib::pick(T{}, 0) is a qualified call with a dependent argument. Its name is looked up in lib, with no argument-dependent lookup, and the later pick(Item, int) in lib is not seen.
namespace lib {
struct Item {};
constexpr int pick(Item, long) { return 1; }
}
template <class T> constexpr int run() { return lib::pick(T{}, 0); }
namespace lib {
constexpr int pick(Item, int) { return 2; }
}
static_assert(run<lib::Item>() == 1);

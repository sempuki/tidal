// tidal-rules: OVL-1
// tidal-expect: accept
// tidal-note: Lookup finds each function through two declarations: two using-declarations, a using-declaration and argument-dependent lookup, or a block and a namespace. Each function is one candidate, so no call is ambiguous.
template <class T, class U> constexpr bool same = false;
template <class T> constexpr bool same<T, T> = true;
namespace store {
  struct Box {};
  int put(int);
  long put(Box);
}
namespace shelf { using store::put; }
using store::put;
using shelf::put;
static_assert(same<decltype(put(1)), int>);
static_assert(same<decltype(put(store::Box{})), long>);
void fill() {
  using store::put;
  put(2);
}

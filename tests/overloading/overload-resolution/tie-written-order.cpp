// tidal-rules: OVL-33
// tidal-expect: accept
// tidal-note: a < b has two rewritten candidates from one operator<=>, (a <=> b) < 0 and 0 < (b <=> a), with equal sequences. Both are rewritten, so OVL-32 does not decide. The one in the written order wins.
template <class T, class U> constexpr bool same = false;
template <class T> constexpr bool same<T, T> = true;
struct One {};
struct Two {};
struct Order {
  friend One operator<(Order, int);
  friend Two operator<(int, Order);
};
struct Item {};
Order operator<=>(const Item&, const Item&);
Item a, b;
static_assert(same<decltype(a < b), One>);

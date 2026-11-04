// tidal-rules: OVL-32
// tidal-expect: accept
// tidal-note: The written operator< and the rewritten (a <=> b) < 0 take the operands the same way, and neither is a template. The written candidate wins.
template <class T, class U> constexpr bool same = false;
template <class T> constexpr bool same<T, T> = true;
struct One {};
struct Two {};
struct Order { friend Two operator<(Order, int); };
struct Item {
  One operator<(const Item&) const;
  Order operator<=>(const Item&) const;
};
Item a, b;
static_assert(same<decltype(a < b), One>);

// tidal-rules: OVL-4
// tidal-expect: accept
// tidal-note: The object expression is the implied object argument, with its cv-qualifiers and value category. An arrow call acts as a dereference and a dot call.
template <class T, class U> constexpr bool same = false;
template <class T> constexpr bool same<T, T> = true;
struct One {};
struct Two {};
struct Item {
  One get() const&;
  Two get() &&;
};
Item v;
const Item c{};
Item* p = &v;
static_assert(same<decltype(v.get()), One>);
static_assert(same<decltype(c.get()), One>);
static_assert(same<decltype(Item{}.get()), Two>);
static_assert(same<decltype(p->get()), One>);

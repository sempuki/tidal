// tidal-rules: OVL-12
// tidal-expect: accept
// tidal-note: The non-member & and , take Item&, which a const Item does not bind. With no viable function the operators have their built-in meaning.
template <class T, class U> constexpr bool same = false;
template <class T> constexpr bool same<T, T> = true;
struct One {};
struct Two {};
struct Item {};
One operator&(Item&);
Two operator,(Item&, int);
Item v;
const Item c{};
static_assert(same<decltype(&v), One>);
static_assert(same<decltype(&c), const Item*>);
static_assert(same<decltype((v, 1)), Two>);
static_assert(same<decltype((c, 1)), int>);

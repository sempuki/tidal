// tidal-rules: ICS-45
// tidal-expect: accept
// tidal-expect-clang: reject
// tidal-error-clang: static assertion failed
// tidal-note: Q-0337. An lvalue reference takes only a conversion function that returns an lvalue (OVL-15), and a temporary for the reference is copy-initialized. So direct initialization of const Item& does not use an explicit conversion function that returns a value. Clang uses it.
template <class T, class S> concept makes = requires(S s) { static_cast<T>(s); };
struct Item { Item(); };
struct ToItem { explicit operator Item(); };
static_assert(!makes<const Item&, ToItem>);

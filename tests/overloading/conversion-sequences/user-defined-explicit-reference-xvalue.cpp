// tidal-rules: ICS-45
// tidal-expect: accept
// tidal-expect-clang: reject
// tidal-expect-edg: reject
// tidal-error-clang: static assertion failed
// tidal-error-edg: static assertion failed
// tidal-note: Q-0337. The explicit operator Item&&() returns an xvalue. Item&& binds directly to it on every compiler. An lvalue reference takes only a conversion function that returns an lvalue (OVL-15), so const Item& does not use the function. Clang and EDG use it.
template <class T, class S> concept makes = requires(S s) { static_cast<T>(s); };
struct Item { Item(); };
struct ToItemRvalue { explicit operator Item&&(); };
static_assert(makes<Item&&, ToItemRvalue>);
static_assert(!makes<const Item&, ToItemRvalue>);

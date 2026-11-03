// tidal-rules: ICS-45
// tidal-expect: accept
// tidal-expect-gcc: reject
// tidal-expect-edg: reject
// tidal-error-gcc: static assertion failed
// tidal-error-edg: static assertion failed
// tidal-note: Q-0156. An rvalue reference takes a conversion function that returns a value (OVL-15). In direct initialization ICS-45 adds the explicit one when only a qualification adjustment is left, and the reference binds directly to the result. A result that needs another conversion, such as int to long or Derived to Base, gives no binding. GCC and EDG reject every form.
template <class T, class S> concept makes = requires(S s) { static_cast<T>(s); };
struct Base { Base(); };
struct Derived : Base { Derived(); };
struct Item { Item(); };
struct ToItem { explicit operator Item(); };
struct ToDerived { explicit operator Derived(); };
struct ToInt { explicit operator int(); };
static_assert(!makes<long&&, ToInt>);
static_assert(!makes<Base&&, ToDerived>);
static_assert(makes<Item&&, ToItem>);
static_assert(makes<const Item&&, ToItem>);
static_assert(makes<int&&, ToInt>);

// tidal-rules: ICS-45
// tidal-expect: accept
// tidal-note: static_cast is direct initialization. An explicit conversion function serves a class target through the copy constructor's parameter, and a non-class target or a direct reference binding when only a qualification adjustment is left. A parameter is copy-initialized, so pass(Item) has no sequence.
template <class T, class U> constexpr bool same = false;
template <class T> constexpr bool same<T, T> = true;
struct One {};
struct Two {};
template <class T, class S> concept makes = requires(S s) { static_cast<T>(s); };
struct Base { Base(); };
struct Derived : Base { Derived(); };
struct Item { Item(); };
struct ToItem { explicit operator Item(); };
struct ToConstItem { explicit operator const Item(); };
struct ToItemRef { explicit operator Item&(); };
struct ToDerived { explicit operator Derived(); };
struct ToDerivedRef { explicit operator Derived&(); };
struct ToPointer { explicit operator int*(); };
struct ToIntRef { explicit operator int&(); };
struct ToInt { explicit operator int(); };
static_assert(makes<Item, ToItem>);
static_assert(makes<Item, ToConstItem>);
static_assert(makes<Item, ToItemRef>);
static_assert(!makes<Base, ToDerived>);
static_assert(makes<const int*, ToPointer>);
static_assert(!makes<const void*, ToPointer>);
static_assert(!makes<long, ToInt>);
static_assert(makes<const Item&, ToItemRef>);
static_assert(makes<const int&, ToIntRef>);
static_assert(!makes<Base&, ToDerivedRef>);
One pass(Item);
Two pass(...);
static_assert(same<decltype(pass(ToItem{})), Two>);

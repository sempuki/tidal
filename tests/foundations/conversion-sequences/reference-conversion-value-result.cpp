// tidal-rules: ICS-11 ICS-4
// tidal-expect: accept
// tidal-note: An rvalue reference binds directly to the class prvalue that operator Item() returns. Both functions use that conversion function, so the third parts compare and ICS-28 prefers the rvalue reference. Item& has no sequence.
template <class T, class U> constexpr bool same = false;
template <class T> constexpr bool same<T, T> = true;
struct One {};
struct Two {};
struct Item {};
struct Base {};
struct Derived : Base {};
struct Source { operator Item(); operator Derived(); };
Source s;
One take(Item&&);
Two take(const Item&);
static_assert(same<decltype(take(s)), One>);
One edit(Item&);
Two edit(...);
static_assert(same<decltype(edit(s)), Two>);
One part(Base&&);
Two part(const Base&);
static_assert(same<decltype(part(s)), One>);

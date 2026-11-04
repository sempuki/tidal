// tidal-rules: ICS-12
// tidal-expect: accept
// tidal-note: An rvalue reference binds a temporary when the lvalue's type is not the referenced type or derived from it. An lvalue of the referenced class, also one that a conversion function returns, has no sequence.
template <class T, class U> constexpr bool same = false;
template <class T> constexpr bool same<T, T> = true;
struct One {};
struct Two {};
struct Base {};
struct Derived : Base {};
struct ToInt { operator int(); };
struct ToBase { operator Base&(); };
enum Color { red };
short s = 0;
int i = 0;
Color c = red;
ToInt ti;
ToBase tb;
Derived d;
One take(int&&);
Two take(...);
static_assert(same<decltype(take(s)), One>);
static_assert(same<decltype(take(c)), One>);
static_assert(same<decltype(take(ti)), One>);
One wide(const long&&);
Two wide(...);
static_assert(same<decltype(wide(i)), One>);
One part(Base&&);
Two part(...);
static_assert(same<decltype(part(d)), Two>);
static_assert(same<decltype(part(tb)), Two>);

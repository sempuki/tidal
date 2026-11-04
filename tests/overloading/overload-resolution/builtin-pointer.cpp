// tidal-rules: OVL-9
// tidal-expect: accept
// tidal-note: The classes convert to pointers. The pointer candidates give the types of dereference, unary plus, arithmetic, subscript, comparison, and ->*.
template <class T, class U> constexpr bool same = false;
template <class T> constexpr bool same<T, T> = true;
struct ToPtr { operator int*() const; };
struct Holder { int v; };
struct ToHolder { operator Holder*() const; };
using Diff = decltype((int*)0 - (int*)0);
static_assert(same<decltype(*ToPtr{}), int&>);
static_assert(same<decltype(+ToPtr{}), int*>);
static_assert(same<decltype(ToPtr{} + 1), int*>);
static_assert(same<decltype(2 + ToPtr{}), int*>);
static_assert(same<decltype(ToPtr{} - ToPtr{}), Diff>);
static_assert(same<decltype(ToPtr{}[1]), int&>);
static_assert(same<decltype(1[ToPtr{}]), int&>);
static_assert(same<decltype(ToPtr{} == ToPtr{}), bool>);
static_assert(same<decltype(ToPtr{} < ToPtr{}), bool>);
static_assert(same<decltype(ToHolder{}->*&Holder::v), int&>);

// tidal-rules: OVL-9
// tidal-expect: accept
// tidal-note: The left operand is an lvalue of a built-in type, or a class that converts to a reference. The assignment and increment candidates take the left operand by reference.
template <class T, class U> constexpr bool same = false;
template <class T> constexpr bool same<T, T> = true;
struct ToDouble { operator double() const; };
struct ToLong { operator long() const; };
struct ToPtr { operator int*() const; };
struct ToIntRef { operator int&(); };
int n = 0;
int* p = nullptr;
static_assert(same<decltype(n = ToDouble{}), int&>);
static_assert(same<decltype(n += ToDouble{}), int&>);
static_assert(same<decltype(p = ToPtr{}), int*&>);
static_assert(same<decltype(p += ToLong{}), int*&>);
static_assert(same<decltype(++ToIntRef{}), int&>);
static_assert(same<decltype(ToIntRef{}++), int>);

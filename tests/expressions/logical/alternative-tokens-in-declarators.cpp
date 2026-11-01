// tidal-rules: LOG-13
// tidal-expect: accept
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
int value = 3;
int and moved = static_cast<int&&>(value);
int bitand alias = value;
static_assert(same<decltype(moved), int&&>);
static_assert(same<decltype(alias), int&>);
struct Handle { compl Handle() {} };

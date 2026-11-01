// tidal-rules: DTY-2 CONV-19
// tidal-expect: accept
// tidal-note: decltype of a built-in arithmetic expression names the common type of the usual arithmetic conversions. A conditional expression on two lvalues of one type is an lvalue, so decltype gives a reference.
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
short h = 1;
char c = 'a';
int i = 0;
unsigned u = 2;
long l = 3;
float f = 1.0f;
static_assert(same<decltype(h + h), int>);
static_assert(same<decltype(c + h), int>);
static_assert(same<decltype(u + -1), unsigned>);
static_assert(same<decltype(i + l), long>);
static_assert(same<decltype(i + f), float>);
static_assert(same<decltype(f + 1.0), double>);
static_assert(same<decltype(true ? i : i), int&>);
static_assert(same<decltype(true ? i : h), int>);
static_assert(same<decltype(+h), int>);

// tidal-rules: ADD-4
// tidal-expect: accept
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
char c = 1; short s = 1; long l = 1; unsigned u = 1; float f = 1;
static_assert(same<decltype(c + c), int>);
static_assert(same<decltype(s - l), long>);
static_assert(same<decltype(u + 1), unsigned>);
static_assert(same<decltype(f + 1), float>);
static_assert(same<decltype(f - 1.0), double>);
static_assert(-1 + 0u == ~0u);
int x = 1;
int* p = &x;
static_assert(same<decltype(p + 1), int*>);
static_assert(same<decltype(1 + p), int*>);

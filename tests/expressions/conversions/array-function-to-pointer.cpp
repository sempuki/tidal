// tidal-rules: CONV-13
// tidal-expect: accept
// tidal-note: Gap review 2026-10-30.
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
int a[3];
void f();
int* p = a;
void (*q)() = f;
static_assert(same<decltype(+a), int*>);
static_assert(same<decltype(+f), void (*)()>);

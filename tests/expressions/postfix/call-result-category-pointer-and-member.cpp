// tidal-rules: PFX-9
// tidal-expect: accept
// tidal-note: Calls through pointers to functions and calls of member functions.
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
int& l(); int&& x(); long p();
int& (*pl)() = l;
int&& (*px)() = x;
struct S { const int& get() const; int&& take(); S make(); };
static_assert(same<decltype((pl())), int&>);
static_assert(same<decltype((px())), int&&>);
static_assert(same<decltype(((&p)())), long>);
static_assert(same<decltype((S{}.get())), const int&>);
static_assert(same<decltype((S{}.take())), int&&>);
static_assert(same<decltype((S{}.make())), S>);

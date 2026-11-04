// tidal-rules: ICS-28 ICS-32 ICS-33 ICS-36
// tidal-expect: accept
// tidal-expect-edg: reject
// tidal-error-edg: static assertion failed
// tidal-note: Q-0152. Each pair has conversion rank. ICS-28 prefers the rvalue reference, and a same-rank tie-breaker prefers the other function. The tie-breaker comes first. EDG applies ICS-28 first.
template <class T, class U> constexpr bool same = false;
template <class T> constexpr bool same<T, T> = true;
struct One {};
struct Two {};
struct Base { int x; };
struct Middle : Base {};
struct Derived : Middle {};
int n = 0;
Derived d;
One flag(void* const&);
Two flag(bool&&);
static_assert(same<decltype(flag(&n)), One>);
One near(const Middle&);
Two near(Base&&);
static_assert(same<decltype(near(Derived{})), One>);
One base(Base* const&);
Two base(void*&&);
static_assert(same<decltype(base(&d)), One>);
One nearer(Middle* const&);
Two nearer(Base*&&);
static_assert(same<decltype(nearer(&d)), One>);
One member(int Middle::* const&);
Two member(int Derived::*&&);
static_assert(same<decltype(member(&Base::x)), One>);

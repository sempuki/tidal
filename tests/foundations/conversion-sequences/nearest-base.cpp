// tidal-rules: ICS-36
// tidal-expect: accept
// tidal-note: Each pair has two conversions of conversion rank between related classes, so only ICS-36 decides.
template <class T, class U> constexpr bool same = false;
template <class T> constexpr bool same<T, T> = true;
struct One {};
struct Two {};
struct Base { int x; };
struct Middle : Base {};
struct Derived : Middle {};
Derived d;
Derived* dp = nullptr;
One ptr(Base*);
Two ptr(Middle*);
static_assert(same<decltype(ptr(dp)), Two>);
One ref(Base&);
Two ref(Middle&);
static_assert(same<decltype(ref(d)), Two>);
One val(Base);
Two val(Middle);
static_assert(same<decltype(val(d)), Two>);
One mem(int Middle::*);
Two mem(int Derived::*);
static_assert(same<decltype(mem(&Base::x)), One>);

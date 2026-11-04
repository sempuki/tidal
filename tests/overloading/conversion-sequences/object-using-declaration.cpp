// tidal-rules: ICS-13
// tidal-expect: accept
// tidal-note: The object parameter of Base::get uses Derived, so the object sequences tie, the first arguments tie, and the second argument decides.
template <class T, class U> constexpr bool same = false;
template <class T> constexpr bool same<T, T> = true;
struct One {};
struct Two {};
struct Base { One get(long, int); };
struct Derived : Base { using Base::get; Two get(unsigned, long); };
struct Base2 { One get(long, long); };
struct Derived2 : Base2 { using Base2::get; Two get(unsigned, int); };
Derived d;
Derived2 d2;
static_assert(same<decltype(d.get(1, 0)), One>);
static_assert(same<decltype(d2.get(1, 0)), Two>);

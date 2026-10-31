// tidal-rules: ICS-8 CONV-17
// tidal-expect: reject
// tidal-error-clang: private base class
// tidal-error-gcc: inaccessible base
// tidal-error-edg: inaccessible base class
// tidal-note: Access does not stop the sequence, so take(Base*) is chosen and the conversion fails (CONV-17).
template <class T, class U> constexpr bool same = false;
template <class T> constexpr bool same<T, T> = true;
struct One {};
struct Two {};
struct Base {};
struct Derived : private Base {};
Derived* d = nullptr;
One take(Base*);
Two take(...);
auto r = take(d);

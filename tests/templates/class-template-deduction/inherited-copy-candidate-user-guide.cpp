// tidal-rules: CTAD-16 CTAD-26 CTAD-24
// tidal-expect: reject
// tidal-error-clang: no viable constructor or deduction guide
// tidal-error-gcc: no matching function for call to .Derived<int>::Derived
// tidal-error-edg: no instance of constructor "Derived<T>::Derived \[with T=int\]"
// tidal-note: Derived is not an aggregate (DER-8), so it has no aggregate candidate. GCC and EDG deduce Derived<int> through the inherited copy candidate, and the initialization then fails, because the copy constructor of Base is not inherited (CTAD-26). Clang has no inherited guides (Q-0239) and finds no guide (CTAD-24).
template <class T> struct Base { constexpr Base(T) {} };
template <class T> struct Derived : Base<T> { using Base<T>::Base; };
Derived(int, int) -> Derived<char>;
constexpr Base<int> b(1);
Derived d(b);

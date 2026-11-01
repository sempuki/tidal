// tidal-rules: CTAD-16 CTAD-24
// tidal-expect: reject
// tidal-error-clang: no viable constructor or deduction guide
// tidal-error-gcc: no matching function for call to .Derived<int>::Derived
// tidal-error-edg: no instance of constructor "Derived<T>::Derived \[with T=int\]"
// tidal-note: The user guide removes the aggregate candidate of Derived. GCC and EDG deduce Derived<int> through the inherited copy candidate and then reject the initialization. Clang has no inherited guides (Q-0239) and finds no guide.
template <class T> struct Base { constexpr Base(T) {} };
template <class T> struct Derived : Base<T> { using Base<T>::Base; };
Derived(int, int) -> Derived<char>;
constexpr Base<int> b(1);
Derived d(b);

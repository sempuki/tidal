// tidal-rules: INST-1 INST-6 DED-15
// tidal-expect: reject
// tidal-error: static assertion failed
// tidal-note: Deduction for base needs the bases of Derived<int>, so the class is instantiated, and its static assertion fails.
template <class T> struct Base {};
template <class T> struct Derived : Base<T> { static_assert(sizeof(T) == 0, "instantiated"); };
template <class T> T base(Base<T>*);
Derived<int>* p = nullptr;
using Result = decltype(base(p));

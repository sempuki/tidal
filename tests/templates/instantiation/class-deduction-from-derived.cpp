// tidal-rules: INST-1 DED-15
// tidal-expect: accept
// tidal-note: The program uses Derived<int> only through a pointer. Deduction for base needs the bases of Derived<int>, so the class is instantiated there.
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
template <class T> struct Base {};
template <class T> struct Derived : Base<T> {};
template <class T> T base(Base<T>*);
Derived<int>* p = nullptr;
static_assert(same<decltype(base(p)), int>);

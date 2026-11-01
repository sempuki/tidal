// tidal-rules: ORD-10 ORD-11 ORD-8
// tidal-expect: accept
// tidal-note: Each pair of conversion templates has the same sequences for int. Only the chosen template is instantiated, and the other one would fail its static_assert.
template <class T> constexpr bool never = false;
int a = 1;
struct Lvalue {
  template <class T> operator T&() const { return a; }
  template <class T> operator T&&() const { static_assert(never<T>); return static_cast<T&&>(a); }
};
struct Qualified {
  template <class T> operator T&() const { static_assert(never<T>); return a; }
  template <class T> operator const T&() const { return a; }
};
struct ConstLvalue {
  template <class T> operator const T&() const { return a; }
  template <class T> operator T&&() const { static_assert(never<T>); return static_cast<T&&>(a); }
};
struct ConstRvalue {
  template <class T> operator T&&() const { static_assert(never<T>); return static_cast<T&&>(a); }
  template <class T> operator const T&&() const { return static_cast<const T&&>(a); }
};
int w = Lvalue{};
int x = Qualified{};
int y = ConstLvalue{};
int z = ConstRvalue{};

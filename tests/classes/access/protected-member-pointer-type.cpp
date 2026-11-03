// tidal-rules: ACC-4 MPTR-4
// tidal-expect: accept
// tidal-note: &D::x names a member that B declares, so its type is int B::*. The value converts implicitly to int D::*.
template <class T, class U> constexpr bool same = false;
template <class T> constexpr bool same<T, T> = true;
class B { protected: int x = 1; };
class D : public B {
public:
  static_assert(same<decltype(&D::x), int B::*>);
  static int D::* ptr() { return &D::x; }
};

// tidal-rules: INST-5 INST-1 INST-7
// tidal-expect: run
// tidal-note: P3-6. Base<Derived> is made while Derived is incomplete. The member take has a parameter of type Cursor<Derived>, and the virtual member first returns Cursor<Derived>. The class instantiation makes only their declarations, so Cursor<Derived> is not needed complete then. The definitions are made later, where Derived is complete: take at its call, and first because it is virtual and Base<Derived> is used (INST-7).
template <class S> struct Cursor { typename S::value_type current; };
template <class D> struct Base {
  int take(Cursor<D> c) { return c.current; }
  virtual Cursor<D> first() { return Cursor<D>{3}; }
  virtual ~Base() {}
};
struct Derived : Base<Derived> { using value_type = int; };
int main() { Derived d; return d.take(Cursor<Derived>{3}) - d.first().current; }

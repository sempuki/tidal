// tidal-rules: INST-5 INST-7
// tidal-expect: run
// tidal-note: P3-6. Base<Derived> is made while Derived is incomplete. The member begin has a deduced return type, and its body makes Cursor<Derived>. The class instantiation makes the declaration of begin only. The call in main makes the definition and deduces the type, where Derived is complete.
template <class S> struct Cursor { typename S::value_type current; };
template <class D> struct Base {
  auto begin() { return Cursor<D>{5}; }
};
struct Derived : Base<Derived> { using value_type = int; };
int main() { Derived d; return d.begin().current - 5; }

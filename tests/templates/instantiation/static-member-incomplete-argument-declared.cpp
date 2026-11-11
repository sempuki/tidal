// tidal-rules: INST-5
// tidal-expect: accept
// tidal-expect-gcc: reject
// tidal-error-gcc: incomplete type
// tidal-note: P3-6. Base<Derived> is made while Derived is incomplete. The static data member first is declared with type Cursor<Derived> and has no definition here. The class instantiation makes the declaration only, so Cursor<Derived> is not made then. Clang and EDG accept. GCC makes Cursor<Derived> with the class (Q-0441).
template <class S> struct Cursor { typename S::value_type* current = nullptr; };
template <class D> struct Base { static Cursor<D> first; };
struct Derived : Base<Derived> { using value_type = int; };

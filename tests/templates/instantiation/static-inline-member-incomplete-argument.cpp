// tidal-rules: INST-5
// tidal-expect: accept
// tidal-expect-gcc: reject
// tidal-error-gcc: incomplete type
// tidal-expect-edg: reject
// tidal-error-edg: incomplete
// tidal-note: P3-6. Base<Derived> is made while Derived is incomplete. The inline static data member first has type Cursor<Derived> and an initializer. The class instantiation makes the declaration only, and the definition with its initializer waits for a use (Q-0231), so Cursor<Derived> is not made then. Clang accepts. GCC and EDG make Cursor<Derived> with the class (Q-0441).
template <class S> struct Cursor { typename S::value_type* current = nullptr; };
template <class D> struct Base { static inline Cursor<D> first{}; };
struct Derived : Base<Derived> { using value_type = int; };

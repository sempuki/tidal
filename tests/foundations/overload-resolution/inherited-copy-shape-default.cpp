// tidal-rules: OVL-17 OVL-35
// tidal-expect: reject
// tidal-error-clang: no matching constructor for initialization of .Derived.
// tidal-error-gcc: no matching function for call to .Derived::Derived\(Base&\).
// tidal-error-edg: no instance of constructor "Derived::Derived" matches the argument list
// tidal-note: Base(const Base&, int = 0) has a first parameter of reference to Base. A call with one argument removes it, although its second parameter has a default argument.
struct Base {
  Base() {}
  Base(const Base&, int = 0) {}
};
struct Derived : Base { using Base::Base; };
Base b;
Derived d(b);

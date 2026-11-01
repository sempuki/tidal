// tidal-rules: OVL-17 OVL-35
// tidal-expect: reject
// tidal-error-clang: no matching constructor for initialization of .Derived.
// tidal-error-gcc: no matching function for call to .Derived::Derived\(Base&\).
// tidal-error-edg: no instance of constructor "Derived::Derived" matches the argument list
// tidal-note: The one argument is a Base. The inherited copy and move constructors of Base take a reference to Base, so they are removed for a call with one argument. Derived has no other constructor that takes a Base.
struct Base {};
struct Derived : Base { using Base::Base; };
Base b;
Derived d(b);

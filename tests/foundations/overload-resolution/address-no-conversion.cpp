// tidal-rules: OVL-20 OVL-18
// tidal-expect: reject
// tidal-error-clang: address of overloaded function .pick. does not match required type
// tidal-error-gcc: no matches converting function .pick. to type
// tidal-error-edg: no instance of overloaded function .pick. matches the required type
// tidal-note: A function with a base class parameter does not match a target with a derived class parameter.
struct Base {};
struct Derived : Base {};
void pick(Base*);
void pick(int);
void (*p)(Derived*) = pick;

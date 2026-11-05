// tidal-rules: COND-16 COND-5
// tidal-expect: reject
// tidal-error-clang: incompatible operand types
// tidal-error-gcc: operands to .\?:. have different types
// tidal-error-edg: operand types are incompatible
// tidal-note: The const Derived operand converts to a const Base prvalue. The Base operand does not convert to its derived class, even through the constructor of Derived. The two operands then differ in cv-qualifiers and have no common type.
struct Base { Base(); };
struct Derived : Base { Derived(); Derived(const Base&); };
const Derived make_fixed();
Base make_base();
extern bool c;
auto r = c ? make_base() : make_fixed();

// tidal-rules: COND-16 COND-5
// tidal-expect: reject
// tidal-error-clang: incompatible operand types
// tidal-error-gcc: operands to .\?:. have different types
// tidal-error-edg: operand types are incompatible
// tidal-note: Base is a base of Derived, so the target for the const Derived lvalue is const Base, and that operand converts to a const Base prvalue. The Base operand does not convert to its derived class. The two operands then differ in cv-qualifiers, and no rule gives them a common type.
struct Derived;
struct Base { Base(); Base(const Derived&); };
struct Derived : Base { };
const Derived fixed{};
Base make();
extern bool c;
auto r = c ? make() : fixed;

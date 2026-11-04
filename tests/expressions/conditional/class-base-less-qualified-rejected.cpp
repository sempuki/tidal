// tidal-rules: COND-16 COND-5
// tidal-expect: reject
// tidal-error-clang: incompatible operand types
// tidal-error-gcc: operands to .\?:. have different types
// tidal-error-edg: operand types are incompatible
// tidal-note: Base is a base of Derived, so the target for the const Derived lvalue is const Base, and the Base prvalue does not have that cv-qualifier. The converting constructor of Base takes no part, because the base case fixes the target. Base does not convert to Derived. No conversion succeeds.
struct Derived;
struct Base { Base(); Base(const Derived&); };
struct Derived : Base { };
const Derived fixed{};
Base make();
extern bool c;
auto r = c ? make() : fixed;

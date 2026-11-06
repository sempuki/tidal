// tidal-rules: ICS-45 OVL-35
// tidal-expect: reject
// tidal-error-clang: no matching constructor
// tidal-error-gcc: invalid user-defined conversion
// tidal-error-edg: no instance of constructor
// tidal-note: Decision 0059, item RV5, point RV5-B-9. Direct initialization of Base from a class whose only conversion is an explicit conversion function to Derived. ICS-45 lets an explicit function serve the copy or move constructor only for a result of the class itself, and every baseline compiler rejects. The review reads the C++23 text as also allowing a derived result (documentation/decisions/pending.md, row 4).
struct Base { Base() = default; Base(const Base&) = default; };
struct Derived : Base {};
struct Source { explicit operator Derived() const { return Derived{}; } };
Base b(Source{});

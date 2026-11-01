// tidal-rules: OVL-6 OVL-35
// tidal-expect: reject
// tidal-error-clang: does not provide a call operator
// tidal-error-gcc: no match for call to .\(Maker\) \(int\).
// tidal-error-edg: call of an object of a class type without appropriate operator\(\) or conversion functions
// tidal-note: The only conversion is a template. Nothing fixes its result type for the call, so it gives no surrogate candidate.
struct Maker { template <class T> operator T*() const; };
void use(Maker m) { m(1); }

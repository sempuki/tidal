// tidal-rules: TYN-10
// tidal-expect: reject
// tidal-error-clang: an attribute list cannot appear here
// tidal-expect-gcc: accept
// tidal-error-edg: attributes are not allowed on explicit
// tidal-note: Ledger row RV9-R-4. An explicit instantiation is not the declaration of a specialization with no body, so the attribute after the class key is outside forward_class and specialization_class. Clang and EDG reject the attribute. GCC accepts it (Q-0445).
template <class T> struct Slot { T value; };
template struct [[maybe_unused]] Slot<int>;

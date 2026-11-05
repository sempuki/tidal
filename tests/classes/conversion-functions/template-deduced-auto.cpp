// tidal-rules: CVF-7
// tidal-expect: reject
// tidal-expect-edg: accept
// tidal-expect-gcc-14: accept
// tidal-error-clang: .auto. not allowed in declaration of conversion function template
// tidal-error-gcc: invalid use of .auto. in conversion operator
// tidal-note: Q-0388. A conversion function template cannot have auto as its name. Clang and GCC reject the declaration. EDG accepts it and warns that T is not used. GCC 14 accepts it.
struct S { template <class T> operator auto() const { return 1; } };

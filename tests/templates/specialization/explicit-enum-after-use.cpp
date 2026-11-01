// tidal-rules: TSP-6 TSP-30
// tidal-expect: reject
// tidal-error-clang: explicit specialization of 'Kind' after instantiation
// tidal-error-gcc: template specialization of .enum Holder<int>::Kind. not allowed
// tidal-error-edg: an unscoped enumeration must be opaque
// tidal-note: The class template defines Kind, so Holder<int>::Kind exists once Holder<int> is used. GCC rejects every explicit specialization of a member enumeration (Q-0205).
template <class T> struct Holder { enum Kind : int { a, b }; };
Holder<int> used;
template <> enum Holder<int>::Kind : int { c, d };

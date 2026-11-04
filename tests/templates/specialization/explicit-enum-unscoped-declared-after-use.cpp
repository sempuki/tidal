// tidal-rules: TSP-30 TSP-3
// tidal-expect: accept
// tidal-expect-gcc: reject
// tidal-error-gcc: template specialization of .enum Holder<int>::Kind. not allowed by ISO C\+\+
// tidal-note: The class template only declares the unscoped enumeration Kind, so its explicit specialization can come after a use of Holder<int>. The gap review of 2026-11-03 reads this form as not valid, and Clang and EDG accept it. GCC rejects every explicit specialization of a member enumeration under -pedantic-errors (Q-0205).
template <class T> struct Holder { enum Kind : int; };
Holder<int> used;
template <> enum Holder<int>::Kind : int { c, d };
static_assert(Holder<int>::d == 1);

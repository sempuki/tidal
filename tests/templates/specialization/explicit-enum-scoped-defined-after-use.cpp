// tidal-rules: TSP-30 TSP-3
// tidal-expect: accept
// tidal-expect-gcc: reject
// tidal-error-gcc: template specialization of .enum class Holder<int>::Kind. not allowed by ISO C\+\+
// tidal-note: The class template defines the scoped enumeration Kind. A use of Holder<int> makes only the declaration of Kind (INST-5), so the explicit specialization can come after the use. GCC rejects every explicit specialization of a member enumeration under -pedantic-errors (Q-0205).
template <class T> struct Holder { enum class Kind { a, b }; };
Holder<int> used;
template <> enum class Holder<int>::Kind { c, d, e };
static_assert(static_cast<int>(Holder<int>::Kind::e) == 2);

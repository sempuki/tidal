// tidal-rules: TSP-30 TSP-3
// tidal-expect: accept
// tidal-expect-gcc: reject
// tidal-error-gcc: template specialization of .enum class Holder<int>::Kind. not allowed by ISO C\+\+
// tidal-note: The class template only declares Kind, so its explicit specialization can come after a use of Holder<int>. GCC rejects it under -pedantic-errors (Q-0205).
template <class T> struct Holder { enum class Kind; };
Holder<int> used;
template <> enum class Holder<int>::Kind { c, d };
static_assert(static_cast<int>(Holder<int>::Kind::d) == 1);

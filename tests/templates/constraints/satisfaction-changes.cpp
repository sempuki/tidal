// tidal-rules: CNS-10
// tidal-expect: accept
// tidal-expect-gcc-16: reject
// tidal-expect-gcc-host: reject
// tidal-error-gcc: satisfaction value of atomic constraint
// tidal-note: Complete<Late> is checked before and after Late is defined. CNS-10 needs no diagnostic. GCC 16 diagnoses the change, the other compilers keep the first result (Q-0264).
struct Late;
template <class T> concept Complete = requires { sizeof(T); };
constexpr bool before = Complete<Late>;
struct Late {};
constexpr bool after = Complete<Late>;
static_assert(!before);

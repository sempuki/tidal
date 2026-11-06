// tidal-rules: OPR-8
// tidal-expect: accept
// tidal-note: Gap review 2026-11-05 of the text phase 10 wrote. The unsigned letter and the long letter of an integer suffix take their case separately, so 1Ul and 1uL are built-in literals. Only the two letters of ll keep one case.
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
static_assert(same<decltype(1Ul), unsigned long> && same<decltype(1uL), unsigned long> && same<decltype(1Ull), unsigned long long>);

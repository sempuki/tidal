// tidal-rules: CTAD-6 CTAD-11
// tidal-expect: accept
// tidal-note: The primary template is declared and not defined. The user guide from int gives Only<int>, which an explicit specialization defines.
template <class A, class B> constexpr bool same = false;
template <class A> constexpr bool same<A, A> = true;
template <class T> struct Only;
template <> struct Only<int> { constexpr Only(int) {} };
Only(int) -> Only<int>;
Only d(1);
static_assert(same<decltype(d), Only<int>>);

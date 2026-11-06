// tidal-rules: TARG-27
// tidal-expect: reject
// tidal-error-clang: found in multiple base classes
// tidal-note: Gap review 2026-11-05 (reports/review/2026-11-05-gap-review-phase-9-expressions-templates.md). Q-0409. D has two bases that are specializations of B, and B in D names both injected class names. As a template template argument the name is ambiguous for Clang and GCC. The review reads it as the template B.
template <class T> struct B {};
template <template <class> class TT> struct Use { static constexpr int v = 1; };
struct D : B<int>, B<long> {
  static constexpr int w = Use<B>::v;
};

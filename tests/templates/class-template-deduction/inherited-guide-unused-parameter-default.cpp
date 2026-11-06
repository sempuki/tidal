// tidal-rules: CTAD-16
// tidal-expect: accept
// tidal-expect-clang: reject
// tidal-error-clang: no viable constructor or deduction guide
// tidal-note: Gap review 2026-11-05 (reports/review/2026-11-05-gap-review-phase-9-expressions-templates.md). D has a parameter U that its base B<T> does not use, and U has a default. The inherited guide gives T = double and U its default, as GCC and EDG say (Q-0408). Clang rejects, as the review reads.
template <class T> struct B { B(T) {} };
template <class T, class U = int> struct D : B<T> { using B<T>::B; };
D d(1.5);

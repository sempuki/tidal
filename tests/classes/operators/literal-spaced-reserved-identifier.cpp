// tidal-rules: OPR-9
// tidal-expect: accept
// tidal-note: Gap review 2026-11-05 (reports/review/2026-11-05-gap-review-phase-9-classes-overloading-exceptions.md). In the form with a space after the empty string, the suffix _Xa is an identifier that starts with an underscore and an uppercase letter, which is reserved. OPR-9 needs no diagnostic, so the test checks only translation. Every compiler accepts it.
constexpr unsigned long long operator"" _Xa(unsigned long long v) { return v; }
static_assert(3_Xa == 3);

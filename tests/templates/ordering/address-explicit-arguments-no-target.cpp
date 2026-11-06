// tidal-rules: ORD-14 OVL-20
// tidal-expect: reject
// tidal-error-clang: incompatible initializer
// tidal-error-gcc: unable to deduce
// tidal-note: Gap review 2026-11-05 (reports/review/2026-11-05-gap-review-phase-9-expressions-templates.md). Q-0405. &f<int> with no target type names f<int>(int) and f<int>(int*). Clang and GCC find the address ambiguous. The review reads that partial ordering keeps the more specialized f(T*).
template <class T> int f(T) { return 1; }
template <class T> int f(T*) { return 2; }
int main() { auto p = &f<int>; return p(nullptr) == 2 ? 0 : 1; }

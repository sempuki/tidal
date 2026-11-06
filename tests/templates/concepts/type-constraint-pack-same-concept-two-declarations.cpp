// tidal-rules: CPT-19 CNS-4 OVL-36
// tidal-expect: reject
// tidal-expect-edg: accept
// tidal-error-clang: ambiguous
// tidal-error-gcc: ambiguous
// tidal-note: Propose X22 (reports/clean-room/X22/report.md), finding X22-3. The fold that A... makes in each declaration is an atom of its own, so neither constraint subsumes the other, and the call is ambiguous, as Clang and GCC say. EDG treats the two folds as one atom and calls the second f (Q-0411).
template <class T> concept A = sizeof(T) > 0;
template <class T> concept B = sizeof(T) > 1;
template <A... T> constexpr int f(T...) { return 1; }
template <A... T> requires B<int> constexpr int f(T...) { return 2; }
static_assert(f(1, 2) == 2);
int main() {}

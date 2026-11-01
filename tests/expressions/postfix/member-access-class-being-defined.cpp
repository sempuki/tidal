// tidal-rules: PFX-11 NAM-6
// tidal-expect: accept
// tidal-expect-clang: reject
// tidal-error-clang: member access into incomplete type
// tidal-note: Q-0316. The member access is in the definition of Pair, after the declaration of first. GCC accepts it and finds first. Clang rejects it because Pair is incomplete there.
struct Pair {
  int first;
  decltype(static_cast<Pair*>(nullptr)->first) second;
};
static_assert(sizeof(Pair) == 2 * sizeof(int));

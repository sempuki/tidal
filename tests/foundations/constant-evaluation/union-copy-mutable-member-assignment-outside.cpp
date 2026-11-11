// tidal-rules: CE-22
// tidal-expect: reject
// tidal-expect-gcc: accept
// tidal-expect-edg: accept
// tidal-error-clang: read of mutable member .count.
// tidal-note: RV9-10. The active member count of shared is mutable, and shared began outside the evaluation, so the implicit copy assignment of Cell is not valid there. Clang rejects. GCC and EDG accept (Q-0449).
union Cell { mutable int count; int fixed; };
constexpr Cell shared{1};
constexpr int copy_in() {
  Cell local{.fixed = 2};
  local = shared;
  return 1;
}
static_assert(copy_in() == 1);

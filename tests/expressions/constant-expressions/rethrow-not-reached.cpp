// tidal-rules: CE-8
// tidal-expect: accept
// tidal-note: The rethrow is on a path that the evaluation does not take, so the evaluation succeeds. termination-rethrow-rejected.cpp reaches the same rethrow.
constexpr int pass_on(bool rethrow) {
  if (rethrow) throw;
  return 1;
}
static_assert(pass_on(false) == 1);

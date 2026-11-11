// tidal-rules: CE-22
// tidal-expect: accept
// tidal-note: RV9-10. The evaluation creates the source union, so the copy of its mutable active member is valid, and the copy has the value. Every compiler accepts.
union Cell { mutable int count; int fixed; };
constexpr int copy_count() {
  Cell source{1};
  Cell copy = source;
  Cell other{.fixed = 2};
  other = source;
  return copy.count + other.count;
}
static_assert(copy_count() == 2);

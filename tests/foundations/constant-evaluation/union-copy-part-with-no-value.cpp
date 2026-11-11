// tidal-rules: CE-22
// tidal-expect: accept
// tidal-note: Ledger row X25-5. A trivial copy or move of a union, and a trivial copy assignment, copy the active member and read none of its parts. The active member of each source has a part with no value: an element of an int array, and a member of a class. Every compiler accepts the copies.
union Cells {
  int items[2];
  long other;
  constexpr Cells() {}
};
struct Point { int x; int y; };
union Spot {
  Point point;
  long other;
  constexpr Spot() {}
};
constexpr int copy_cells() {
  Cells source;
  source.items[0] = 1;
  Cells copied = source;
  Cells moved = static_cast<Cells&&>(source);
  Cells assigned;
  assigned = copied;
  assigned.items[1] = 4;
  return copied.items[0] + moved.items[0] + assigned.items[0] + assigned.items[1];
}
constexpr int copy_spot() {
  Spot source;
  source.point.x = 5;
  Spot copied = source;
  return copied.point.x;
}
static_assert(copy_cells() == 7);
static_assert(copy_spot() == 5);

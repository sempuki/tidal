// tidal-rules: UNI-14
// tidal-expect: accept
// tidal-note: Gap review RV6-A-15. A trivial copy of a whole union, by its copy constructor or its copy assignment, makes active in the target the member that is active in the source. Constant evaluation reads that member in both forms. Every compiler accepts it, and every compiler rejects a read of the member that was active in the target before the assignment (writer probe rv6-points.json).
union Value { int whole; float real; };
constexpr float assigned() {
  Value from; from.real = 1.5f;
  Value to; to.whole = 2;
  to = from;
  return to.real;
}
constexpr float constructed() {
  Value from; from.real = 2.5f;
  Value to = from;
  return to.real;
}
static_assert(assigned() == 1.5f);
static_assert(constructed() == 2.5f);

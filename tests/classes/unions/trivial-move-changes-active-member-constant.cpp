// tidal-rules: UNI-14
// tidal-expect: accept
// tidal-note: Gap review RV7-A-4. A trivial move of a whole union, by its move constructor or its move assignment, makes active in the target the member that is active in the source, as a trivial copy does. Constant evaluation reads that member in both forms. Every compiler accepts it, and every compiler rejects a read of the member that was active in the target before the move assignment (writer probe roadmap-11/writer-probes/rv7-points.json).
union Value { int whole; float real; };
constexpr float assigned() {
  Value from; from.real = 1.5f;
  Value to; to.whole = 2;
  to = static_cast<Value&&>(from);
  return to.real;
}
constexpr float constructed() {
  Value from; from.real = 2.5f;
  Value to = static_cast<Value&&>(from);
  return to.real;
}
static_assert(assigned() == 1.5f);
static_assert(constructed() == 2.5f);

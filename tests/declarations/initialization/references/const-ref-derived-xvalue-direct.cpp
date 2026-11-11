// tidal-rules: REF-2
// tidal-expect: accept
// tidal-note: Gap review RV7-A-5. An xvalue of a class derived from the referenced class is a type that REF-1 lets the reference bind, so a const lvalue reference binds directly to its base subobject, and no temporary is made. Every compiler accepts the static_assert (writer probe roadmap-11/writer-probes/rv7-points.json).
struct Base { int value = 1; };
struct Derived : Base { int extra = 2; };
constexpr bool binds_to_base() {
  Derived item;
  const Base& ref = static_cast<Derived&&>(item);
  return &ref == static_cast<const Base*>(&item);
}
static_assert(binds_to_base());

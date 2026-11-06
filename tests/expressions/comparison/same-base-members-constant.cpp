// tidal-rules: CMP-8 CMP-7
// tidal-expect: accept
// tidal-note: Gap review 2026-11-05 (reports/review/2026-11-05-gap-review-phase-9-expressions-templates.md). Two members of one base class subobject are members of one object, so their order is fixed, also in constant evaluation.
struct B { int b1; int b2; };
struct D : B { int d; };
constexpr bool f() {
  D x{};
  return &x.b1 < &x.b2;
}
static_assert(f());

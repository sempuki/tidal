// tidal-rules: CE-3
// tidal-expect: reject
// tidal-error-clang: read of mutable member
// tidal-error-gcc: mutable .Outer::in. is not usable in a constant expression
// tidal-error-edg: expression must have a constant value
// tidal-note: RV10-17. The member v of a mutable member of a constexpr variable is not usable in constant expressions, although v itself is not mutable, so the read is not valid in the constant. Every compiler rejects it. EDG names an uninitialized object, and EDG accepts a read of the mutable member itself (Q-0449).
struct Inner { int v; };
struct Outer { mutable Inner in; };
constexpr Outer shared{{3}};
constexpr int seen = shared.in.v;

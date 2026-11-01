// tidal-rules: UNI-6 UNI-13
// tidal-expect: reject
// tidal-expect-edg: accept
// tidal-error-clang: assignment to member .n. of union with active member
// tidal-error-gcc: member instead of initialized
// tidal-note: Q-0213. The member n has a default constructor that is not trivial, so the assignment does not make n active. EDG 7.0 accepts the evaluation.
struct N { int x = 5; };
union W { N n; int i; constexpr W() : i(0) {} };
constexpr int assign_inside() { W w; w.n.x = 4; return w.n.x; }
static_assert(assign_inside() == 4);

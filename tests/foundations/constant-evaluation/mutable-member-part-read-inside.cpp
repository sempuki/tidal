// tidal-rules: CE-3
// tidal-expect: accept
// tidal-note: RV10-17. The evaluation creates the variable, so the read of a part of its mutable member is valid. Every compiler accepts.
struct Inner { int v; };
struct Outer { mutable Inner in; };
constexpr int read() { Outer local{{3}}; return local.in.v; }
static_assert(read() == 3);

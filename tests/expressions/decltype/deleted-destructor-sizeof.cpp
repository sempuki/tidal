// tidal-rules: UNEV-5
// tidal-expect: reject
// tidal-error-clang: attempt to use a deleted function
// tidal-error-gcc: use of deleted function .D::~D
// tidal-error-edg: D::~D.* it is a deleted function
// tidal-note: Gap review 2026-10-31.
struct D { ~D() = delete; };
D make();
auto n = sizeof(make());

// tidal-rules: NS-9
// tidal-expect: reject
// tidal-note: Gap review 2026-10-31.
namespace lib { int v; }
struct S { using namespace lib; };

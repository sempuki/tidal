// tidal-rules: NS-9
// tidal-expect: reject
// tidal-error-clang: using namespace. is not allowed in classes
// tidal-error-gcc: expected nested-name-specifier before .namespace
// tidal-note: Gap review 2026-10-31.
namespace lib { int v; }
struct S { using namespace lib; };

// tidal-rules: TYN-4
// tidal-expect: reject
// tidal-error-clang: no struct named .Missing
// tidal-error-gcc: .Missing. in namespace .n. does not name a type
// tidal-error-edg: has no tag member named .Missing
namespace n {}
struct n::Missing* p;

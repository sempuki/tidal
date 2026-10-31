// tidal-rules: NS-7
// tidal-expect: reject
// tidal-error-clang: using declaration cannot refer to a namespace
// tidal-error-gcc: using-declaration may not name namespace
// tidal-error-edg: namespace name is not allowed
namespace N { namespace M {} }
using N::M;

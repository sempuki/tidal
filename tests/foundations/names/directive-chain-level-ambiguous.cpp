// tidal-rules: NAM-23 NAM-24
// tidal-expect: reject
// tidal-error-clang: reference to .limit. is ambiguous
// tidal-error-gcc: reference to .limit. is ambiguous
// tidal-error-edg: limit. is ambiguous
namespace app { namespace core { int limit = 1; } }
namespace bridge { using namespace app::core; }
namespace app {
  int limit = 2;
  namespace user { using namespace ::bridge; int read() { return limit; } }
}

// tidal-rules: NAM-23 NAM-25
// tidal-expect: accept
namespace lib { inline namespace v2 { int value = 2; } }
using namespace lib;
int read() { return value; }

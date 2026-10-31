// tidal-rules: NAM-13
// tidal-expect: accept
namespace m { int value = 1; }
using namespace m;
int read() { return ::value; }

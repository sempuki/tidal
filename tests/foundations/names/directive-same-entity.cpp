// tidal-rules: NAM-25
// tidal-expect: accept
namespace a { int value = 1; }
namespace b { using a::value; }
using namespace a;
using namespace b;
int read() { return value; }

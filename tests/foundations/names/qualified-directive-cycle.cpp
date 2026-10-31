// tidal-rules: NAM-13
// tidal-expect: accept
namespace b { }
namespace a { using namespace b; int value = 1; }
namespace b { using namespace a; }
int read() { return b::value; }

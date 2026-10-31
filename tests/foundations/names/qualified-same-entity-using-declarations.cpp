// tidal-rules: NAM-13
// tidal-expect: accept
namespace a { int value = 1; }
namespace b { using a::value; }
namespace top { using namespace a; using namespace b; }
int read() { return top::value; }

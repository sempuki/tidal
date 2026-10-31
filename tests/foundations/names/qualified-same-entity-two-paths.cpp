// tidal-rules: NAM-13
// tidal-expect: accept
namespace a { int value = 1; }
namespace b { using namespace a; }
namespace top { using namespace a; using namespace b; }
int read() { return top::value; }

// tidal-rules: NAM-13
// tidal-expect: accept
namespace deep { int value = 1; }
namespace mid { using namespace deep; }
namespace top { using namespace mid; }
int read() { return top::value; }

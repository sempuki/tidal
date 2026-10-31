// tidal-rules: NAM-23
// tidal-expect: accept
int value = 1;
namespace lib { int value = 2; }
namespace user { int value = 3; using namespace lib; int read() { return value; } }

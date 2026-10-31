// tidal-rules: NAM-23
// tidal-expect: accept
namespace m { int value = 1; }
namespace n { using namespace m; }
namespace n { int read() { return value; } }

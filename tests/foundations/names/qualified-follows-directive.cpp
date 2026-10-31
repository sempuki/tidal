// tidal-rules: NAM-13
// tidal-expect: accept
namespace inner { int value = 4; }
namespace outer { using namespace inner; }
int read() { return outer::value; }

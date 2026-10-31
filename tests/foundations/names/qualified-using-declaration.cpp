// tidal-rules: NAM-13
// tidal-expect: accept
namespace inner { int value = 4; }
namespace outer { using inner::value; }
int read() { return outer::value; }

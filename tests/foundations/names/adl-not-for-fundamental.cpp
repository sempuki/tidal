// tidal-rules: NAM-5
// tidal-expect: reject
namespace n { int twice(int v) { return v * 2; } }
int main() { return twice(3); }

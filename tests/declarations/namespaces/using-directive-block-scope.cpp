// tidal-rules: NS-8
// tidal-expect: reject
namespace N { int x; }
void f() { using namespace N; }
int y = x;

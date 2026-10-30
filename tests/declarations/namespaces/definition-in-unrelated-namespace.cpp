// tidal-rules: NS-6
// tidal-expect: reject
namespace N { int f(); }
namespace M { int N::f() { return 0; } }

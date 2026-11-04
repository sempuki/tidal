// tidal-rules: NAM-14 NAM-13
// tidal-expect: accept
// tidal-note: Two paths of using directives find two functions named f. Functions do not make the name ambiguous. They form one overload set, and overload resolution chooses a::f.
namespace a { int f(int); }
namespace b { int f(double); }
namespace both { using namespace a; using namespace b; }
int r = both::f(1);

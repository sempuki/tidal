// tidal-rules: NAM-21
// tidal-expect: accept
namespace n { struct C { int f(); }; int value = 1; }
int n::C::f() { return value; }

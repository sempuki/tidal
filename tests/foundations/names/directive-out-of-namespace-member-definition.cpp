// tidal-rules: NAM-23
// tidal-expect: accept
// tidal-note: The using directive is in namespace n. The definition of n::f outside n sees the members of lib after its qualified name.
namespace lib { int v = 1; }
namespace n { using namespace lib; int f(); }
int n::f() { return v; }

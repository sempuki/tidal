// tidal-rules: NAM-35 DER-3
// tidal-expect: accept
namespace lib { struct base { }; }
struct derived : lib::base { int base = 0; int get() { return base; } };

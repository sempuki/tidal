// tidal-rules: NAM-36 DER-4
// tidal-expect: reject
// tidal-error-clang: member .base. found in multiple base classes of different types
// tidal-error-gcc: reference to .base. is ambiguous
// tidal-error-edg: join::base. is ambiguous
namespace a { struct base { }; }
namespace b { struct base { }; }
struct join : a::base, b::base { void f() { base* p = nullptr; (void)p; } };

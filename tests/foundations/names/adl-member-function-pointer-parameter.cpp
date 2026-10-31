// tidal-rules: NAM-27
// tidal-expect: accept
namespace m { struct T { }; }
namespace n { struct S { void take(m::T) { } }; }
namespace m { int probe(void (n::S::*)(T)) { return 1; } }
int read() { return probe(&n::S::take); }

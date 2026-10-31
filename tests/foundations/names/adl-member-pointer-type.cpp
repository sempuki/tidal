// tidal-rules: NAM-27
// tidal-expect: accept
namespace m { struct T { }; }
namespace n { struct S { m::T t; }; }
namespace m { int probe(T n::S::*) { return 1; } }
int read() { return probe(&n::S::t); }

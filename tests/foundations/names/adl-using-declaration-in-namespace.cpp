// tidal-rules: NAM-29
// tidal-expect: accept
namespace other { struct S; }
namespace lib { int probe(const other::S*) { return 1; } }
namespace other { struct S { }; using lib::probe; }
int read() { other::S s; return probe(&s); }

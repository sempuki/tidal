// tidal-rules: NAM-27
// tidal-expect: accept
namespace n { struct S { }; int probe(S (*)()) { return 1; } }
n::S make() { return {}; }
int read() { return probe(&make); }

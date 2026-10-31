// tidal-rules: NAM-27
// tidal-expect: accept
namespace n { union U { int i; }; int probe(U) { return 1; } }
int read() { return probe(n::U{}); }

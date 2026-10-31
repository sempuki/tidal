// tidal-rules: NAM-27
// tidal-expect: accept
namespace n { struct outer { struct inner { }; }; int probe(outer::inner) { return 1; } }
int read() { return probe(n::outer::inner{}); }

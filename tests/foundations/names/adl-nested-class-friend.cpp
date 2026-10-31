// tidal-rules: NAM-27
// tidal-expect: accept
namespace n { struct outer { struct inner { }; friend int probe(inner) { return 1; } }; }
int read() { return probe(n::outer::inner{}); }

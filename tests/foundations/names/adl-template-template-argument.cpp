// tidal-rules: NAM-27
// tidal-expect: accept
namespace t { template <template <class> class X> struct holder { }; }
namespace n { template <class> struct tool { }; int probe(t::holder<tool>) { return 1; } }
int read() { return probe(t::holder<n::tool>{}); }

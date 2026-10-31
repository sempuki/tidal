// tidal-rules: NAM-27
// tidal-expect: accept
namespace n { inline auto action = [] { }; template <class F> int probe(F) { return 1; } }
int read() { return probe(n::action); }

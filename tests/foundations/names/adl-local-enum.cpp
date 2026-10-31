// tidal-rules: NAM-27
// tidal-expect: accept
namespace n { template <class T> int probe(T) { return 1; } inline auto make() { enum local { a }; return a; } }
int read() { return probe(n::make()); }

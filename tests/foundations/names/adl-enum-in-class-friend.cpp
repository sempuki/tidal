// tidal-rules: NAM-27
// tidal-expect: accept
namespace n { struct holder { enum mode { on }; friend int probe(mode) { return 1; } }; }
int read() { return probe(n::holder::on); }

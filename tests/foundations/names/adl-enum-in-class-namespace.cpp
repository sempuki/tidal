// tidal-rules: NAM-27
// tidal-expect: accept
namespace n { struct holder { enum mode { on }; }; int probe(holder::mode) { return 1; } }
int read() { return probe(n::holder::on); }

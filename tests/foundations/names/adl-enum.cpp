// tidal-rules: NAM-27
// tidal-expect: accept
namespace n { enum class mode { on }; int probe(mode) { return 1; } }
int read() { return probe(n::mode::on); }

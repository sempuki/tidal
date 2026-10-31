// tidal-rules: ENUM-11
// tidal-expect: accept
// tidal-note: Gap review 2026-10-31.
enum class S { off, on };
bool f() { return static_cast<bool>(S::on); }

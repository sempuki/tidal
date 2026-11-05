// tidal-rules: ATTR-2
// tidal-expect: accept
// tidal-note: nodiscard can go on an enumeration as on a class.
enum class [[nodiscard]] Status { ok, failed };
Status check() { return Status::ok; }
int main() { return check() == Status::ok ? 0 : 1; }

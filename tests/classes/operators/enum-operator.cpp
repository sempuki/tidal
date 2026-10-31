// tidal-rules: OPR-1
// tidal-expect: run
enum class Flags : unsigned { none = 0, read = 1, write = 2 };
Flags operator|(Flags a, Flags b) { return Flags(unsigned(a) | unsigned(b)); }
int main() { Flags f = Flags::read | Flags::write; return unsigned(f) == 3 ? 0 : 1; }

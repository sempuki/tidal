// tidal-rules: OVL-15
// tidal-expect: run
// tidal-note: operator long&() returns a long, which a reference to const int cannot bind directly. The reference binds to a temporary int, which holds the converted value.
long total = 5;
struct Source { operator long&() const { return total; } };
const int& r = Source{};
int main() { return (const void*)&r != (const void*)&total && r == 5 ? 0 : 1; }

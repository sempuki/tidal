// tidal-rules: OVL-15
// tidal-expect: run
// tidal-note: Direct initialization of the reference takes the explicit operator int&(), whose result differs from const int only in const. The reference binds to value.
int value = 5;
struct Source { explicit operator int&() const { return value; } };
Source s;
const int& r(s);
int main() { return &r == &value ? 0 : 1; }

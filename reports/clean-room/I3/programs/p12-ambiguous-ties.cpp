// ICS-7 and ICS-25: the ambiguous sequence ties with a user-defined one; the second argument decides.
struct Source { operator int() { return 1; } operator long() { return 2; } };
struct Wrap { Wrap(Source) {} };
int take(double, long) { return 1; }
int take(Wrap, int) { return 2; }
int main() { return take(Source{}, 0) == 2 ? 0 : 1; }

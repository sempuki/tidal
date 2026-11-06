// ICS-14: the static slot ties with a derived-to-base object sequence.
struct Tool { static int use(long, int) { return 1; } int use(long, long) const { return 2; } };
struct Derived : Tool {};
Derived d;
int main() { return d.use(1, 0) == 1 ? 0 : 1; }

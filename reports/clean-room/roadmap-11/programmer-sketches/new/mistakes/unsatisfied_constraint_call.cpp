// A call whose only candidate has a constraint that the argument fails.
template <class T> concept Number = requires(T a) { a + a; a * a; };
template <Number T> T square(T v) { return v * v; }
struct Name {};
int main() { square(Name{}); return 0; }

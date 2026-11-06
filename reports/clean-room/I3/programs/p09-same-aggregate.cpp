// ICS-25: the same aggregate, so the third parts compare; ICS-28 picks Point&&.
struct Point { int x, y; };
int take(const Point&) { return 1; }
int take(Point&&) { return 2; }
int main() { return take({1, 2}) == 2 ? 0 : 1; }

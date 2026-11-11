// Aggregate initialization of a class with a base.
struct Point { int x, y; };
struct Pixel : Point { unsigned char shade; };
int main() { Pixel p{{1, 2}, 200}; Pixel q{3, 4, 5}; return p.x + p.y + p.shade + q.x + q.shade == 211 ? 0 : 1; }

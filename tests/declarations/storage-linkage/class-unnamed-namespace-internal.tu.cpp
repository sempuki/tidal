namespace {
struct Point { int x; };
}
int read(Point p) { return p.x * 10; }
int read_elsewhere() { return read(Point{2}); }

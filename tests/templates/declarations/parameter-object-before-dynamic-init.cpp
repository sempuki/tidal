// tidal-rules: TPL-49
// tidal-expect: run
// tidal-note: The dynamic initialization of reader reads the template parameter object before main. The object already has its value.
struct Point { int v; };
const Point* early();
struct Reader {
  int seen;
  Reader() : seen(early()->v) {}
};
Reader reader;
template <Point p> const Point* where() { return &p; }
const Point* early() { return where<Point{7}>(); }
int main() { return reader.seen == 7 ? 0 : 1; }

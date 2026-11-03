// tidal-rules: UNI-16 UNI-15 UNI-10 STG-23
// tidal-expect: run
// tidal-note: Two standard-layout structs whose members match in full are layout-compatible. So members of such struct types match in the common initial sequence of the outer structs. The read of a Point object through a Pair glvalue is one that UNI-10 allows, so STG-23 does not make it undefined.
struct Point { int x; int y; };
struct Pair { int first; int second; };
struct Line { Point from; int width; };
struct Span { Pair range; int step; };
union Shape { Line line; Span span; };
int main() {
  Shape s{.line = {{1, 2}, 3}};
  return s.span.range.second == 2 && s.span.step == 3 ? 0 : 1;
}

// tidal-rules: REF-2
// tidal-expect: run
// tidal-note: Roadmap round 9 (P1-2), P1 reduction parse-class-converts-to-const-ref-declaration.cpp. A const lvalue reference to a class binds to a temporary that a converting constructor makes from an object of another class.
struct Code { int n; };
struct Status {
  int n;
  Status(Code c) : n(c.n * 3) {}
};
int main() {
  Code c{4};
  const Status& s = c;
  return s.n == 12 ? 0 : 1;
}

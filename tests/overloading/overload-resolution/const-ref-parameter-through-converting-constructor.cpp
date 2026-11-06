// tidal-rules: OVL-23 REF-2 ICS-3
// tidal-expect: run
// tidal-note: Roadmap round 9 (P1-2), P1 reduction parse-class-converts-to-const-ref.cpp and parse.cpp. A function with a const reference parameter is viable for an argument of another class that converts through a converting constructor. A hidden friend comparison takes such an argument for its second operand.
struct Error { int code; };
struct Result {
  int code;
  Result(Error e) : code(e.code) {}
  explicit Result(int c) : code(c) {}
  friend bool operator==(const Result& a, const Result& b) { return a.code == b.code; }
};
int code_of(const Result& r) { return r.code; }
int main() {
  Result r(5);
  return code_of(Error{2}) == 2 && r == Error{5} ? 0 : 1;
}

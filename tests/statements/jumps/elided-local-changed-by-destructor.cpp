// tidal-rules: JMP-11
// tidal-expect: run
struct Result { int v; Result() : v(1) {} Result(const Result& o) : v(o.v) {} };
struct Spoiler { Result* r; ~Spoiler() { r->v = 99; } };
Result make() {
  Result r;
  Spoiler s{&r};
  return r;                  // s ends after the return takes its value
}
int main() {
  int v = make().v;
  return v == 1 || v == 99 ? 0 : 1;  // 99 if r is the result object, 1 if r is copied
}

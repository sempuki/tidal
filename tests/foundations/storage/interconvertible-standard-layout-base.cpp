// tidal-rules: STG-16
// tidal-expect: run
struct Inner { int a; int b; };
struct Outer : Inner {};   // standard-layout; the members are in the base
int main() {
  Outer outer{};
  Inner* inner = reinterpret_cast<Inner*>(&outer);
  inner->b = 4;
  Outer* back = reinterpret_cast<Outer*>(inner);
  return outer.b == 4 && back == &outer ? 0 : 1;
}

// tidal-rules: CLS-3
// tidal-expect: run
// tidal-sources: static-data-member.tu.cpp
struct Counter { static int count; };
int Counter::count = 0;
void bump();
int main() {
  Counter::count += 1;
  bump();
  return Counter::count == 2 ? 0 : 1;
}

// tidal-rules: STG-7 STG-8
// tidal-expect: run
// tidal-flags: -O0 -fsanitize=address,undefined -fno-sanitize-recover=all
struct Pair { int first; double second; };
int main() {
  void* raw = ::operator new(sizeof(Pair));
  Pair* pair = static_cast<Pair*>(raw);   // a Pair exists from the call on
  pair->first = 1;
  pair->second = 2.5;
  bool ok = pair->first == 1 && pair->second == 2.5;
  ::operator delete(raw);
  return ok ? 0 : 1;
}

// tidal-rules: CMP-15
// tidal-expect: run
struct Pair { int first; int second; };
Pair pair;
Pair* volatile source = &pair;
int main() {
  Pair* p = source;
  bool same = static_cast<void*>(p) == static_cast<void*>(&p->first);
  bool differ = static_cast<void*>(p) != static_cast<void*>(&p->second);
  return same && differ ? 0 : 1;
}

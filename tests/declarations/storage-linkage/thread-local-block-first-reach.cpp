// tidal-rules: STO-8
// tidal-expect: run
// tidal-note: A block thread_local variable of class type. Its constructor runs only when control first reaches the declaration, and only once.
int constructed = 0;
struct Cache {
  int hits = 0;
  Cache() { ++constructed; }
};
int lookup() {
  thread_local Cache cache;
  return ++cache.hits;
}
int main() {
  if (constructed != 0) return 1;
  lookup();
  if (constructed != 1) return 2;
  return lookup() == 2 && constructed == 1 ? 0 : 3;
}

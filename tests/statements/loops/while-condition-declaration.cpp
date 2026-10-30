// tidal-rules: LOOP-1
// tidal-expect: run
int created = 0, destroyed = 0;
struct Counter {
  int v;
  Counter(int x) : v(x) { ++created; }
  ~Counter() { ++destroyed; }
  explicit operator bool() const { return v > 0; }
};
int main() {
  int left = 2;
  while (Counter c{left}) { --left; }
  return created == 3 && destroyed == 3 ? 0 : 1;
}

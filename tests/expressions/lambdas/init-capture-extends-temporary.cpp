// tidal-rules: LAM-7
// tidal-expect: run
int ended = 0;
struct Note { int v; ~Note() { ++ended; } };
int main() {
  auto f = [&r = static_cast<const Note&>(Note{3})] { return r.v; };
  int during = ended;        // 0 if the init capture keeps the Note alive
  int v = f();
  return during == 0 && v == 3 ? 0 : 1;
}

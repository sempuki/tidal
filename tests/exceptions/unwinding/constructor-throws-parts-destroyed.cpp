// tidal-rules: UNW-2
// tidal-expect: run
int trace = 0;
void mark(int d) { trace = trace * 10 + d; }
struct T { int id; T(int i) : id(i) { mark(i); } ~T() { mark(id); } };

struct W { T a; T b; W() : a(1), b(2) { mark(3); throw 0; } ~W() { mark(9); } };
int main() {
  try { W w; }
  catch (int) { mark(4); }
  return trace == 123214 ? 0 : 1;
}

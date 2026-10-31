// tidal-rules: UNW-1
// tidal-expect: run
int trace = 0;
void mark(int d) { trace = trace * 10 + d; }
struct T { int id; T(int i) : id(i) { mark(i); } ~T() { mark(id); } };

int main() {
  try { T a(1); T b(2); throw 7; }
  catch (int v) { mark(v); }
  return trace == 12217 ? 0 : 1;
}

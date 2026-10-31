// tidal-rules: UNW-1
// tidal-expect: run
int trace = 0;
void mark(int d) { trace = trace * 10 + d; }
struct T { int id; T(int i) : id(i) { mark(i); } ~T() { mark(id); } };

int main() {
  try { T a(1); throw 0; T b(2); }
  catch (int) { mark(3); }
  return trace == 113 ? 0 : 1;
}

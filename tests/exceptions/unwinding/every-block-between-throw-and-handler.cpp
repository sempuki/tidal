// tidal-rules: UNW-1
// tidal-expect: run
int trace = 0;
void mark(int d) { trace = trace * 10 + d; }
struct T { int id; T(int i) : id(i) { mark(i); } ~T() { mark(id); } };

void deep() { T c(3); throw 1; }
void middle() { T b(2); deep(); mark(9); }
int main() {
  try { T a(1); middle(); }
  catch (int) { mark(8); }
  return trace == 1233218 ? 0 : 1;
}

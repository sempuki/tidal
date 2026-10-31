// tidal-rules: UNW-4
// tidal-expect: run
int trace = 0;
void mark(int d) { trace = trace * 10 + d; }
struct T { int id; T(int i) : id(i) { mark(i); } ~T() { mark(id); } };

T global(1);
int f() { static T local(2); throw 0; }
int main() {
  try { f(); }
  catch (int) { mark(3); }
  return trace == 123 && global.id == 1 ? 0 : 1;
}

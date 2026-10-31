// tidal-rules: UNW-2
// tidal-expect: run
int trace = 0;
void mark(int d) { trace = trace * 10 + d; }
struct T { int id; T(int i) : id(i) { mark(i); } ~T() { mark(id); } };

struct Bad { Bad() { throw 0; } };
struct B { T t; B() : t(1) {} };
struct D : B { Bad bad; };
int main() {
  try { D d; }
  catch (int) { mark(4); }
  return trace == 114 ? 0 : 1;
}

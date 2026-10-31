// tidal-rules: OBJ-1
// tidal-expect: run
int trace = 0;
void mark(int d) { trace = trace * 10 + d; }
struct Part { int id; Part(int i) : id(i) { mark(i); } ~Part() { mark(id); } };
struct First { Part p{1}; };
struct Second { Part p{2}; };
struct Whole : First, Second {
  Part m{3};
  Part n{4};
  Whole() { mark(5); }
  ~Whole() { mark(6); }
};
int main() {
  { Whole w; }
  return trace == 1234564321 ? 0 : 1;
}

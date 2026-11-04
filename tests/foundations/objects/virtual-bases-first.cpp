// tidal-rules: OBJ-6
// tidal-expect: run
// tidal-note: The virtual bases V and W come first, in a depth-first walk of the bases of Whole. Then come the other bases (A inside B, then B's member), the member m, and the body.
long long trace = 0;
void mark(int d) { trace = trace * 10 + d; }
struct Part { int id; Part(int i) : id(i) { mark(i); } ~Part() { mark(id); } };
struct V { Part p{1}; };
struct W { Part p{2}; };
struct A { Part p{3}; };
struct B : A, virtual V { Part p{4}; };
struct Whole : B, virtual W { Part m{5}; Whole() { mark(6); } ~Whole() { mark(7); } };
int main() {
  { Whole w; }
  return trace == 123456754321LL ? 0 : 1;
}

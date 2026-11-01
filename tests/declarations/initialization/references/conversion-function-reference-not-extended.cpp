// tidal-rules: REF-5 CVF-2
// tidal-expect: run
int trace = 0;
void mark(int d) { trace = trace * 10 + d; }
struct Cell {
  int v;
  ~Cell() { mark(1); }
  operator const int&() const { return v; }
};
int main() {
  {
    const int& r = Cell{5};
    mark(2);
    (void)r;
  }
  return trace == 12 ? 0 : 1;
}

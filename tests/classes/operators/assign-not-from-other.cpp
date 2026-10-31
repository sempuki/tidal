// tidal-rules: OPR-6
// tidal-expect: run
struct V { int x;  V& operator+=(V o) { x += o.x; return *this; } };
int main() { V a{1}; a += V{2}; return a.x == 3 ? 0 : 1; }

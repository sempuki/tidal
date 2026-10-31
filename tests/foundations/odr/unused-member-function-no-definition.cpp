// tidal-rules: ODR-8
// tidal-expect: run
struct Plain { int v; int get() const; void set(int); };
int main() { Plain plain{2}; return plain.v - 2; }

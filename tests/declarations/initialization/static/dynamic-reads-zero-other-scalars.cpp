// tidal-rules: SINIT-2
// tidal-expect: run
volatile int source = 1;
int* read_p();
bool read_b();
double read_d();
int* early_p = read_p();
bool early_b = read_b();
double early_d = read_d();
int cell;
int* p = source ? &cell : nullptr;
bool b = source;
double d = source;
int* read_p() { return p; }
bool read_b() { return b; }
double read_d() { return d; }
int main() {
  return early_p == nullptr && !early_b && early_d == 0.0 && p == &cell && b ? 0 : 1;
}

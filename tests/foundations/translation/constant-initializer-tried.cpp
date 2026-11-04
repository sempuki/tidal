// tidal-rules: TR-5
// tidal-expect: run
// tidal-note: Nothing needs a constant in the initializer of later, but the initializer of a static variable is evaluated as a constant first. So later has its value before any code runs, and early reads 2.
constexpr int half(int v) { return v / 2; }
int read_later();
int early = read_later();
int later = half(4);
int read_later() { return later; }
int main() { return early == 2 ? 0 : 1; }

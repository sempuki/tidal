// tidal-rules: VC-9
// tidal-expect: run
// tidal-note: Gap review 2026-10-31.
int trace = 0;
void mark(int d) { trace = trace * 10 + d; }

struct Arg { ~Arg() { mark(9); } };
struct E { E(const Arg& = Arg{}) { mark(1); } };
int main() { E arr[2]; return trace == 1919 ? 0 : 1; }

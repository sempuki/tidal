// tidal-rules: RFOR-2
// tidal-expect: run
// tidal-expect-edg: reject
// tidal-note: Q-PENDING: EDG uses the member form only for member functions. Clang and GCC count data members.
// tidal-note: Gap review 2026-10-31 (third).
int data[3] = {1, 2, 3};
struct First { int* operator()() const { return data; } };
struct Last { int* operator()() const { return data + 3; } };
struct Range { First begin; Last end; };
int main() { Range rg; int s = 0; for (int v : rg) s += v; return s == 6 ? 0 : 1; }

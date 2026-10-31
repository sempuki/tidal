// tidal-rules: REF-4
// tidal-expect: run
// tidal-note: Gap review 2026-10-31.
int trace = 0;
struct T { int arr[2] = {1, 2}; ~T() { trace = trace * 10 + 1; } };
int main() { { int&& r = T{}.arr[1]; trace = trace * 10 + 2; if (r != 2) return 1; } return trace == 21 ? 0 : 1; }

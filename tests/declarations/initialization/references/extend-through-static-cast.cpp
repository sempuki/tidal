// tidal-rules: REF-4
// tidal-expect: run
// tidal-note: Gap review 2026-10-31.
int trace = 0;
struct T { ~T() { trace = trace * 10 + 1; } };
int main() { { const T& r = static_cast<const T&>(T{}); trace = trace * 10 + 2; (void)r; } return trace == 21 ? 0 : 1; }

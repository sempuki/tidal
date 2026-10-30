// tidal-rules: DTOR-5
// tidal-expect: run
int trace = 0;
struct T { int id; ~T() { trace = trace * 10 + id; } };
int main() { { T arr[3] = {{1}, {2}, {3}}; } return trace == 321 ? 0 : 1; }

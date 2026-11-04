// tidal-rules: RFOR-2 RFOR-7
// tidal-expect: run
// tidal-expect-edg: reject
// tidal-error-edg: requires a suitable .begin. function
// tidal-note: Q-0098. The data members begin and end are pointers to functions, so rg.begin() and rg.end() are valid calls and the member form works. Clang and GCC run the loop. EDG uses the member form only for member functions.
int data[3] = {1, 2, 3};
int* first() { return data; }
int* last() { return data + 3; }
struct Range { int* (*begin)(); int* (*end)(); };
int main() { Range rg{first, last}; int s = 0; for (int v : rg) s += v; return s == 6 ? 0 : 1; }

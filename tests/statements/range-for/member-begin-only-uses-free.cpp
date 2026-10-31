// tidal-rules: RFOR-2
// tidal-expect: run
// tidal-note: Gap review 2026-10-31 (third).
namespace r {
struct Only { int arr[3] = {1, 2, 3}; int* begin() { return arr; } };
int* begin(Only& o) { return o.arr; }
int* end(Only& o) { return o.arr + 3; }
}
int main() { r::Only o; int s = 0; for (int v : o) s += v; return s == 6 ? 0 : 1; }

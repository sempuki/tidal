// tidal-rules: ODR-1
// tidal-expect: run
extern int total;
int total = 1;
extern int total;
struct Box;
struct Box { int v; };
struct Box;
enum class Mode : int;
enum class Mode : int { on, off };
int twice(int);
int twice(int v) { return 2 * v; }
int twice(int);
int main() { Box box{1}; return twice(total) + box.v + static_cast<int>(Mode::off) == 4 ? 0 : 1; }

// tidal-rules: SINIT-7
// tidal-expect: run
// tidal-note: Second is defined before First, but the member of First is defined first. The members follow the order of their definitions.
int counter = 0;
int next() { return ++counter; }
struct Second { static int id; };
struct First { static int id; };
int First::id = next();
int Second::id = next();
int main() { return First::id == 1 && Second::id == 2 ? 0 : 1; }

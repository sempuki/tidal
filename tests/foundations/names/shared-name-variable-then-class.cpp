// tidal-rules: NAM-39
// tidal-expect: accept
// tidal-note: The global namespace declares the variable first and the class second, and defines the class after a forward declaration.
int record = 1;
struct record;
struct record { int id; };
struct record;
extern int record;

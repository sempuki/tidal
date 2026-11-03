// tidal-rules: NAM-40
// tidal-expect: accept
// tidal-note: The name alone finds the variable whether the variable comes before or after the class. struct finds the class.
struct block { char bytes[10]; };
int block = 1;
static_assert(sizeof(block) == sizeof(int));
static_assert(sizeof(struct block) == 10);

int frame = 1;
struct frame { char bytes[12]; };
static_assert(sizeof(frame) == sizeof(int));
static_assert(sizeof(struct frame) == 12);

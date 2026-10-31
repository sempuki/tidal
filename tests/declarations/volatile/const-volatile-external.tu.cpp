extern const volatile int shared;
const volatile int* other() { return &shared; }

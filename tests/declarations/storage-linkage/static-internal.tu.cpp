static int value = 2;
static int helper() { return 20; }
int other_value() { return value; }
int other_helper() { return helper(); }

namespace {
int value = 2;
int helper() { return 20; }
}
int other_value() { return value; }
int other_helper() { return helper(); }

#ifndef TIDAL_ODR_MODE_H
#define TIDAL_ODR_MODE_H
enum class Mode { idle, busy, done };
int next_code(Mode m);
#endif

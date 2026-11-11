using System;
using System.Collections.Generic;

// A sensor raises an event when a reading crosses a threshold. Handlers
// subscribe and unsubscribe with += and -=. One handler unsubscribes
// itself from inside the event, which must not disturb the others.
sealed class Sensor
{
    public event Action<int> Crossed;
    readonly int threshold;
    bool above = false;
    public Sensor(int threshold) { this.threshold = threshold; }

    public void Read(int value)
    {
        bool now = value > threshold;
        if (now != above) { above = now; Crossed?.Invoke(value); }
    }
}

static class Events
{
    static int Main()
    {
        int failures = 0;
        var sensor = new Sensor(50);
        var log = new List<int>();
        int once = 0;
        Action<int> logIt = v => log.Add(v);
        Action<int> onceOnly = null;
        onceOnly = v => { once++; sensor.Crossed -= onceOnly; };
        sensor.Crossed += logIt;
        sensor.Crossed += onceOnly;
        foreach (var v in new[] { 10, 60, 70, 40, 55, 20 }) sensor.Read(v);
        if (log.Count != 4) failures++;
        if (log.Count == 4 && (log[0] != 60 || log[1] != 40 || log[2] != 55 || log[3] != 20)) failures++;
        if (once != 1) failures++;
        sensor.Crossed -= logIt;
        sensor.Read(99);
        if (log.Count != 4) failures++;
        return failures;
    }
}

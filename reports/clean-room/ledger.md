# Ledger of exercise findings

Each finding (R10-1, W18-2, X22-6, and so on) that a report under
`reports/clean-room/` names has one row here (decision 0058). The
front-end findings (G1, G2, ...) have their own ledger in
`rust-frontend/ledger.md`. The State column says how the clean-room
writer answered the finding:

- `commit`: a commit acted on it, or named it as done.
- `question`: a question holds it.
- `text`: the specification already decides it, or it needs no change.
- `out of scope`: the topic is outside Tidal's scope.
- `planned`: an item of a phase decision takes it; the Record names the item.
- `open`: no answer yet.

For a `commit` row from before this ledger, the Record column names
the first commits after the report that cite the finding, alone or in
a range such as "RC-1 to RC-25". `tools/check_specification.py` fails
when a report names a finding that has no row here, when a `commit`
row names no commit, and when a `planned` row names an item that the
roadmap marks done or moved (decision 0059).

A gap review under `reports/review/` gives each point that its repair
commit leaves an ID such as `RV5-A3`, and the point gets a row here.
The rows RV4-1 to RV4-7 give IDs to the doubts that the RV4 repair
commit (7cf4574b) left for the next review. The RV5 reviews gave their
own IDs (RV5-A-n and RV5-B-n).

| Finding | Report | State | Record |
|---|---|---|---|
| DC-1 | `D-classes/report.md` | commit | 008bc71c |
| DC-2 | `D-classes/report.md` | commit | 008bc71c |
| DC-3 | `D-classes/report.md` | commit | 008bc71c |
| DC-4 | `D-classes/report.md` | commit | 008bc71c |
| DC-5 | `D-classes/report.md` | commit | 008bc71c |
| DC-6 | `D-classes/report.md` | commit | 008bc71c |
| DC-7 | `D-classes/report.md` | commit | 008bc71c |
| DC-8 | `D-classes/report.md` | commit | 008bc71c |
| DC-9 | `D-classes/report.md` | commit | 008bc71c |
| DC-10 | `D-classes/report.md` | commit | 008bc71c |
| DC-11 | `D-classes/report.md` | commit | 008bc71c |
| DC-12 | `D-classes/report.md` | commit | 008bc71c |
| DC-13 | `D-classes/report.md` | commit | 008bc71c |
| DC-14 | `D-classes/report.md` | commit | 008bc71c |
| DC-15 | `D-classes/report.md` | commit | 008bc71c |
| DC-16 | `D-classes/report.md` | commit | 008bc71c |
| DC-17 | `D-classes/report.md` | commit | 008bc71c |
| I1-1 | `I1/report.md` | commit | 076aa546 |
| I1-2 | `I1/report.md` | commit | 076aa546 |
| I1-3 | `I1/report.md` | commit | 076aa546 |
| I1-4 | `I1/report.md` | commit | 076aa546 |
| I1-5 | `I1/report.md` | commit | 076aa546 |
| I1-6 | `I1/report.md` | commit | 076aa546 |
| I1-7 | `I1/report.md` | commit | 076aa546 |
| I1-8 | `I1/report.md` | commit | 076aa546 |
| I1-9 | `I1/report.md` | commit | 076aa546 |
| I1-10 | `I1/report.md` | commit | 076aa546 |
| I1-11 | `I1/report.md` | commit | 076aa546 |
| I1-12 | `I1/report.md` | commit | 076aa546 |
| I1-13 | `I1/report.md` | commit | 076aa546 |
| I2-1 | `I2/report.md` | commit | d9c18bcc |
| I2-2 | `I2/report.md` | commit | d9c18bcc |
| I2-3 | `I2/report.md` | commit | d9c18bcc |
| I2-4 | `I2/report.md` | commit | d9c18bcc |
| I2-5 | `I2/report.md` | commit | d9c18bcc |
| I2-6 | `I2/report.md` | commit | d9c18bcc |
| I2-7 | `I2/report.md` | commit | d9c18bcc |
| I2-8 | `I2/report.md` | commit | d9c18bcc |
| I2-9 | `I2/report.md` | commit | d9c18bcc |
| I2-10 | `I2/report.md` | commit | d9c18bcc |
| I2-11 | `I2/report.md` | commit | d9c18bcc |
| I2-12 | `I2/report.md` | commit | d9c18bcc |
| I2-13 | `I2/report.md` | commit | d9c18bcc, 593f9e5a |
| I2-14 | `I2/report.md` | commit | d9c18bcc, 593f9e5a |
| I2-15 | `I2/report.md` | commit | d9c18bcc, 593f9e5a |
| I2-16 | `I2/report.md` | commit | d9c18bcc, 593f9e5a |
| I2-17 | `I2/report.md` | commit | d9c18bcc |
| I3-1 | `I3/report.md` | commit | 3b83788d |
| I3-2 | `I3/report.md` | commit | 4a193ea4 |
| I3-3 | `I3/report.md` | commit | 4a193ea4 |
| I3-4 | `I3/report.md` | text | a component error; the text decides it |
| I3-5 | `I3/report.md` | commit | 3b83788d |
| I3-6 | `I3/report.md` | text | a component error; the text decides it |
| I3-7 | `I3/report.md` | commit | 4a193ea4 |
| I3-8 | `I3/report.md` | commit | 4a193ea4 |
| I3-9 | `I3/report.md` | out of scope | extended floating-point types stay out of scope |
| I3-10 | `I3/report.md` | commit | 4a193ea4 |
| I3-11 | `I3/report.md` | commit | 4a193ea4 |
| I3-12 | `I3/report.md` | text | CONV-1 gives the promotion of every character type |
| I3-13 | `I3/report.md` | commit | 3b83788d |
| I3-14 | `I3/report.md` | commit | 4a193ea4, eff1a372 |
| I3-15 | `I3/report.md` | commit | 4a193ea4, eff1a372 |
| I3-16 | `I3/report.md` | commit | 4a193ea4, eff1a372 |
| I3-17 | `I3/report.md` | commit | 4a193ea4, eff1a372 |
| I3-18 | `I3/report.md` | commit | 4a193ea4, eff1a372 |
| PU1-1 | `PU1/report.md` | commit | b7e0adf5 |
| PU1-2 | `PU1/report.md` | commit | b7e0adf5 |
| PU1-3 | `PU1/report.md` | commit | b7e0adf5 |
| PU1-4 | `PU1/report.md` | commit | b7e0adf5 |
| PU1-5 | `PU1/report.md` | commit | b7e0adf5 |
| PU1-6 | `PU1/report.md` | commit | b7e0adf5 |
| PU1-7 | `PU1/report.md` | commit | b7e0adf5 |
| PU1-8 | `PU1/report.md` | commit | b7e0adf5 |
| PU2-1 | `PU2/report.md` | commit | 32c89f7b |
| PU2-2 | `PU2/report.md` | commit | 32c89f7b |
| PU2-3 | `PU2/report.md` | commit | 32c89f7b |
| PU2-4 | `PU2/report.md` | commit | 32c89f7b |
| PU2-5 | `PU2/report.md` | commit | 32c89f7b |
| PU2-6 | `PU2/report.md` | commit | 32c89f7b |
| PU3-1 | `PU3/report.md` | question | Q-0395 |
| PU3-2 | `PU3/report.md` | question | Q-0376 |
| PU3-3 | `PU3/report.md` | commit | 47f58739 |
| PU3-4 | `PU3/report.md` | commit | 47f58739 |
| R10-1 | `R10/report.md` | commit | 47f58739 |
| R10-2 | `R10/report.md` | commit | 47f58739 |
| R10-3 | `R10/report.md` | commit | 47f58739 |
| R10-4 | `R10/report.md` | commit | 3b83788d |
| R10-5 | `R10/report.md` | commit | 47f58739 |
| R10-6 | `R10/report.md` | commit | 3b83788d |
| R10-7 | `R10/report.md` | commit | 47f58739 |
| R10-8 | `R10/report.md` | text | no change needed; the finding says so |
| R10-9 | `R10/report.md` | commit | 47f58739 |
| R8A-1 | `R8/report-a.md` | commit | 60072525 |
| R8A-2 | `R8/report-a.md` | commit | 60072525 |
| R8A-3 | `R8/report-a.md` | commit | 60072525 |
| R8A-4 | `R8/report-a.md` | commit | 60072525 |
| R8A-5 | `R8/report-a.md` | commit | 60072525 |
| R8A-6 | `R8/report-a.md` | commit | 60072525 |
| R8A-7 | `R8/report-a.md` | commit | 60072525 |
| R8A-8 | `R8/report-a.md` | commit | 60072525 |
| R8A-9 | `R8/report-a.md` | commit | 60072525 |
| R8A-10 | `R8/report-a.md` | commit | 60072525 |
| R8B-1 | `R8/report-b.md` | commit | 60072525 |
| R8B-2 | `R8/report-b.md` | commit | 60072525 |
| R8B-3 | `R8/report-b.md` | commit | 60072525 |
| R8B-4 | `R8/report-b.md` | commit | 60072525 |
| R8B-5 | `R8/report-b.md` | commit | 60072525 |
| R8B-6 | `R8/report-b.md` | commit | 60072525 |
| R8B-7 | `R8/report-b.md` | commit | 60072525 |
| R8B-8 | `R8/report-b.md` | commit | 60072525 |
| R8B-9 | `R8/report-b.md` | commit | 60072525 |
| R8B-10 | `R8/report-b.md` | commit | 60072525 |
| R8C-1 | `R8/report-c.md` | commit | 60072525 |
| R8C-2 | `R8/report-c.md` | commit | 60072525 |
| R8C-3 | `R8/report-c.md` | commit | 60072525 |
| R8C-4 | `R8/report-c.md` | commit | 60072525 |
| R8C-5 | `R8/report-c.md` | commit | 60072525 |
| R8C-6 | `R8/report-c.md` | commit | 60072525 |
| R8C-7 | `R8/report-c.md` | commit | 60072525 |
| R8C-8 | `R8/report-c.md` | commit | 60072525 |
| R8C-9 | `R8/report-c.md` | commit | 60072525 |
| R8C-10 | `R8/report-c.md` | commit | 60072525 |
| R8C-11 | `R8/report-c.md` | commit | 60072525 |
| R8D-1 | `R8/report-d.md` | commit | 60072525 |
| R8D-2 | `R8/report-d.md` | commit | 60072525 |
| R8D-3 | `R8/report-d.md` | commit | 60072525 |
| R8D-4 | `R8/report-d.md` | commit | 60072525 |
| R8D-5 | `R8/report-d.md` | commit | 60072525 |
| R8D-6 | `R8/report-d.md` | commit | 60072525 |
| R8D-7 | `R8/report-d.md` | commit | 60072525 |
| R8D-8 | `R8/report-d.md` | commit | 60072525 |
| R9A-1 | `R9/report-a.md` | commit | 32c89f7b |
| R9A-2 | `R9/report-a.md` | commit | 32c89f7b |
| R9A-3 | `R9/report-a.md` | commit | 32c89f7b |
| R9A-4 | `R9/report-a.md` | commit | 32c89f7b |
| R9A-5 | `R9/report-a.md` | commit | 32c89f7b |
| R9A-6 | `R9/report-a.md` | commit | 32c89f7b |
| R9A-7 | `R9/report-a.md` | commit | 32c89f7b |
| R9A-8 | `R9/report-a.md` | commit | 32c89f7b |
| R9A-9 | `R9/report-a.md` | commit | 32c89f7b |
| R9B-1 | `R9/report-b.md` | commit | 32c89f7b |
| R9B-2 | `R9/report-b.md` | commit | 32c89f7b |
| R9B-3 | `R9/report-b.md` | commit | 32c89f7b |
| R9B-4 | `R9/report-b.md` | commit | 32c89f7b |
| R9B-5 | `R9/report-b.md` | commit | 32c89f7b |
| R9B-6 | `R9/report-b.md` | commit | 32c89f7b |
| R9B-7 | `R9/report-b.md` | commit | 32c89f7b |
| R9B-8 | `R9/report-b.md` | commit | 32c89f7b |
| R9B-9 | `R9/report-b.md` | commit | 32c89f7b |
| R9B-10 | `R9/report-b.md` | commit | 32c89f7b |
| R9C-1 | `R9/report-c.md` | commit | 32c89f7b |
| R9C-2 | `R9/report-c.md` | commit | 32c89f7b |
| R9C-3 | `R9/report-c.md` | commit | 32c89f7b |
| R9C-4 | `R9/report-c.md` | commit | 32c89f7b |
| R9C-5 | `R9/report-c.md` | commit | 32c89f7b |
| R9C-6 | `R9/report-c.md` | commit | 32c89f7b |
| R9C-7 | `R9/report-c.md` | commit | 32c89f7b |
| R9D-1 | `R9/report-d.md` | commit | 32c89f7b |
| R9D-2 | `R9/report-d.md` | commit | 32c89f7b |
| R9D-3 | `R9/report-d.md` | commit | 32c89f7b |
| R9D-4 | `R9/report-d.md` | commit | 32c89f7b |
| R9D-5 | `R9/report-d.md` | commit | 32c89f7b |
| R9D-6 | `R9/report-d.md` | commit | 32c89f7b |
| R9D-7 | `R9/report-d.md` | commit | 32c89f7b |
| RA-1 | `R-templates/report-a.md` | commit | b456a30d |
| RA-2 | `R-templates/report-a.md` | commit | b456a30d |
| RA-3 | `R-templates/report-a.md` | commit | b456a30d |
| RA-4 | `R-templates/report-a.md` | commit | b456a30d |
| RA-5 | `R-templates/report-a.md` | commit | b456a30d |
| RA-6 | `R-templates/report-a.md` | commit | b456a30d |
| RA-7 | `R-templates/report-a.md` | commit | b456a30d |
| RA-8 | `R-templates/report-a.md` | commit | b456a30d |
| RA-9 | `R-templates/report-a.md` | commit | b456a30d |
| RA-10 | `R-templates/report-a.md` | commit | b456a30d |
| RA-11 | `R-templates/report-a.md` | commit | b456a30d |
| RA-12 | `R-templates/report-a.md` | commit | b456a30d |
| RA-13 | `R-templates/report-a.md` | commit | b456a30d, 2c57d445 |
| RA-14 | `R-templates/report-a.md` | commit | b456a30d |
| RB-1 | `R-templates/report-b.md` | commit | b456a30d |
| RB-2 | `R-templates/report-b.md` | commit | b456a30d |
| RB-3 | `R-templates/report-b.md` | commit | b456a30d |
| RB-4 | `R-templates/report-b.md` | commit | b456a30d |
| RB-5 | `R-templates/report-b.md` | commit | b456a30d |
| RB-6 | `R-templates/report-b.md` | commit | b456a30d |
| RB-7 | `R-templates/report-b.md` | commit | b456a30d |
| RB-8 | `R-templates/report-b.md` | commit | b456a30d |
| RB-9 | `R-templates/report-b.md` | commit | b456a30d |
| RB-10 | `R-templates/report-b.md` | commit | b456a30d |
| RB-11 | `R-templates/report-b.md` | commit | b456a30d |
| RB-12 | `R-templates/report-b.md` | commit | b456a30d |
| RB-13 | `R-templates/report-b.md` | commit | b456a30d |
| RB-14 | `R-templates/report-b.md` | commit | b456a30d |
| RB-15 | `R-templates/report-b.md` | commit | b456a30d |
| RB-16 | `R-templates/report-b.md` | commit | b456a30d |
| RB-17 | `R-templates/report-b.md` | commit | b456a30d |
| RC-1 | `R-templates/report-c.md` | commit | 3c569c2b |
| RC-2 | `R-templates/report-c.md` | commit | 3c569c2b |
| RC-3 | `R-templates/report-c.md` | commit | 3c569c2b |
| RC-4 | `R-templates/report-c.md` | commit | 3c569c2b |
| RC-5 | `R-templates/report-c.md` | commit | 3c569c2b |
| RC-6 | `R-templates/report-c.md` | commit | 3c569c2b |
| RC-7 | `R-templates/report-c.md` | commit | 3c569c2b |
| RC-8 | `R-templates/report-c.md` | commit | 685bee7e, 3c569c2b |
| RC-9 | `R-templates/report-c.md` | commit | 3c569c2b |
| RC-10 | `R-templates/report-c.md` | commit | 3c569c2b |
| RC-11 | `R-templates/report-c.md` | commit | 3c569c2b |
| RC-12 | `R-templates/report-c.md` | commit | 3c569c2b |
| RC-13 | `R-templates/report-c.md` | commit | 3c569c2b |
| RC-14 | `R-templates/report-c.md` | commit | 3c569c2b |
| RC-15 | `R-templates/report-c.md` | commit | 3c569c2b |
| RC-16 | `R-templates/report-c.md` | commit | 3c569c2b |
| RC-17 | `R-templates/report-c.md` | commit | 3c569c2b |
| RC-18 | `R-templates/report-c.md` | commit | 3c569c2b |
| RC-19 | `R-templates/report-c.md` | commit | 685bee7e, 3c569c2b |
| RC-20 | `R-templates/report-c.md` | commit | 685bee7e, 3c569c2b |
| RC-21 | `R-templates/report-c.md` | commit | 685bee7e, 3c569c2b |
| RC-22 | `R-templates/report-c.md` | commit | 3c569c2b |
| RC-23 | `R-templates/report-c.md` | commit | 3c569c2b |
| RC-24 | `R-templates/report-c.md` | commit | 3c569c2b |
| RC-25 | `R-templates/report-c.md` | commit | 3c569c2b |
| RCA-1 | `R-classes/report-a.md` | commit | d4705c79 |
| RCA-2 | `R-classes/report-a.md` | commit | d4705c79 |
| RCA-3 | `R-classes/report-a.md` | commit | 2d92b5e3, d4705c79 |
| RCA-4 | `R-classes/report-a.md` | commit | d4705c79 |
| RCA-5 | `R-classes/report-a.md` | commit | d4705c79 |
| RCA-6 | `R-classes/report-a.md` | commit | d4705c79 |
| RCA-7 | `R-classes/report-a.md` | commit | d4705c79 |
| RCA-8 | `R-classes/report-a.md` | commit | d4705c79 |
| RCA-9 | `R-classes/report-a.md` | commit | d4705c79 |
| RCA-10 | `R-classes/report-a.md` | commit | d4705c79 |
| RCA-11 | `R-classes/report-a.md` | commit | d4705c79 |
| RCA-12 | `R-classes/report-a.md` | commit | d4705c79 |
| RCB-1 | `R-classes/report-b.md` | commit | d4705c79 |
| RCB-2 | `R-classes/report-b.md` | commit | d4705c79 |
| RCB-3 | `R-classes/report-b.md` | commit | d4705c79 |
| RCB-4 | `R-classes/report-b.md` | commit | d4705c79 |
| RCB-5 | `R-classes/report-b.md` | commit | d4705c79 |
| RCB-6 | `R-classes/report-b.md` | commit | d4705c79 |
| RCB-7 | `R-classes/report-b.md` | commit | d4705c79 |
| RCB-8 | `R-classes/report-b.md` | commit | d4705c79 |
| RCB-9 | `R-classes/report-b.md` | commit | d4705c79 |
| RCB-10 | `R-classes/report-b.md` | commit | d4705c79 |
| RCB-11 | `R-classes/report-b.md` | commit | d4705c79 |
| RCC-1 | `R-classes/report-c.md` | commit | d4705c79 |
| RCC-2 | `R-classes/report-c.md` | commit | d4705c79 |
| RCC-3 | `R-classes/report-c.md` | commit | d4705c79 |
| RCC-4 | `R-classes/report-c.md` | commit | d4705c79 |
| RCC-5 | `R-classes/report-c.md` | commit | d4705c79 |
| RCC-6 | `R-classes/report-c.md` | commit | d4705c79 |
| RCC-7 | `R-classes/report-c.md` | commit | d4705c79 |
| RCC-8 | `R-classes/report-c.md` | commit | d4705c79 |
| RCC-9 | `R-classes/report-c.md` | commit | d4705c79 |
| RCD-1 | `R-classes/report-d.md` | commit | d4705c79 |
| RCD-2 | `R-classes/report-d.md` | commit | a025c999, d4705c79 |
| RCD-3 | `R-classes/report-d.md` | commit | d4705c79 |
| RCD-4 | `R-classes/report-d.md` | commit | d4705c79 |
| RCD-5 | `R-classes/report-d.md` | commit | d4705c79 |
| RCD-6 | `R-classes/report-d.md` | commit | d4705c79 |
| RDA-1 | `R-declarations/report-a.md` | commit | 2d92b5e3 |
| RDA-2 | `R-declarations/report-a.md` | commit | 2d92b5e3 |
| RDA-3 | `R-declarations/report-a.md` | commit | 2d92b5e3 |
| RDA-4 | `R-declarations/report-a.md` | commit | 2d92b5e3 |
| RDA-5 | `R-declarations/report-a.md` | commit | 2d92b5e3 |
| RDA-6 | `R-declarations/report-a.md` | commit | 2d92b5e3 |
| RDA-7 | `R-declarations/report-a.md` | commit | 2d92b5e3 |
| RDA-8 | `R-declarations/report-a.md` | commit | 2d92b5e3 |
| RDA-9 | `R-declarations/report-a.md` | commit | 2d92b5e3 |
| RDA-10 | `R-declarations/report-a.md` | commit | 2d92b5e3 |
| RDA-11 | `R-declarations/report-a.md` | commit | 2d92b5e3 |
| RDB-1 | `R-declarations/report-b.md` | commit | 2d92b5e3 |
| RDB-2 | `R-declarations/report-b.md` | commit | 2d92b5e3 |
| RDB-3 | `R-declarations/report-b.md` | commit | 2d92b5e3 |
| RDB-4 | `R-declarations/report-b.md` | commit | 2d92b5e3 |
| RDB-5 | `R-declarations/report-b.md` | commit | 2d92b5e3 |
| RDB-6 | `R-declarations/report-b.md` | commit | 2d92b5e3 |
| RDB-7 | `R-declarations/report-b.md` | commit | 2d92b5e3 |
| RDB-8 | `R-declarations/report-b.md` | commit | 2d92b5e3 |
| RDB-9 | `R-declarations/report-b.md` | commit | 2d92b5e3 |
| RDB-10 | `R-declarations/report-b.md` | commit | 2d92b5e3 |
| RDC-1 | `R-declarations/report-c.md` | commit | 2d92b5e3 |
| RDC-2 | `R-declarations/report-c.md` | commit | 2d92b5e3 |
| RDC-3 | `R-declarations/report-c.md` | commit | 2d92b5e3 |
| RDC-4 | `R-declarations/report-c.md` | commit | 2d92b5e3 |
| RDC-5 | `R-declarations/report-c.md` | commit | 2d92b5e3 |
| RDC-6 | `R-declarations/report-c.md` | commit | 2d92b5e3 |
| RDC-7 | `R-declarations/report-c.md` | commit | 2d92b5e3 |
| RDC-8 | `R-declarations/report-c.md` | commit | 2d92b5e3 |
| RDC-9 | `R-declarations/report-c.md` | commit | 2d92b5e3 |
| RDC-10 | `R-declarations/report-c.md` | commit | 2d92b5e3 |
| RDD-1 | `R-declarations/report-d.md` | commit | 2d92b5e3 |
| RDD-2 | `R-declarations/report-d.md` | commit | 2d92b5e3 |
| RDD-3 | `R-declarations/report-d.md` | commit | 2d92b5e3 |
| RDD-4 | `R-declarations/report-d.md` | commit | 2d92b5e3 |
| RDD-5 | `R-declarations/report-d.md` | commit | 2d92b5e3 |
| RDD-6 | `R-declarations/report-d.md` | commit | 2d92b5e3 |
| RDD-7 | `R-declarations/report-d.md` | commit | 2d92b5e3 |
| RDD-8 | `R-declarations/report-d.md` | commit | 2d92b5e3 |
| RDD-9 | `R-declarations/report-d.md` | commit | 2d92b5e3 |
| RDE-1 | `R-declarations/report-e.md` | commit | 2d92b5e3 |
| RDE-2 | `R-declarations/report-e.md` | commit | 2d92b5e3 |
| RDE-3 | `R-declarations/report-e.md` | commit | 2d92b5e3 |
| RDE-4 | `R-declarations/report-e.md` | commit | 2d92b5e3 |
| RDE-5 | `R-declarations/report-e.md` | commit | 2d92b5e3 |
| RDE-6 | `R-declarations/report-e.md` | commit | 2d92b5e3 |
| RDE-7 | `R-declarations/report-e.md` | commit | 2d92b5e3 |
| RDE-8 | `R-declarations/report-e.md` | commit | 2d92b5e3 |
| REA-1 | `R-expressions/report-a.md` | commit | f8d7b652 |
| REA-2 | `R-expressions/report-a.md` | commit | f8d7b652 |
| REA-3 | `R-expressions/report-a.md` | commit | f8d7b652 |
| REA-4 | `R-expressions/report-a.md` | commit | f8d7b652 |
| REA-5 | `R-expressions/report-a.md` | commit | f8d7b652 |
| REA-6 | `R-expressions/report-a.md` | commit | f8d7b652 |
| REA-7 | `R-expressions/report-a.md` | commit | f8d7b652 |
| REA-8 | `R-expressions/report-a.md` | commit | f8d7b652 |
| REA-9 | `R-expressions/report-a.md` | commit | f8d7b652 |
| REA-10 | `R-expressions/report-a.md` | commit | f8d7b652 |
| REA-11 | `R-expressions/report-a.md` | commit | f8d7b652 |
| REA-12 | `R-expressions/report-a.md` | commit | f8d7b652 |
| REA-13 | `R-expressions/report-a.md` | commit | f8d7b652 |
| REB-1 | `R-expressions/report-b.md` | commit | f8d7b652 |
| REB-2 | `R-expressions/report-b.md` | commit | f8d7b652 |
| REB-3 | `R-expressions/report-b.md` | commit | f8d7b652 |
| REB-4 | `R-expressions/report-b.md` | commit | f8d7b652 |
| REB-5 | `R-expressions/report-b.md` | commit | f8d7b652 |
| REB-6 | `R-expressions/report-b.md` | commit | f8d7b652 |
| REB-7 | `R-expressions/report-b.md` | commit | f8d7b652 |
| REB-8 | `R-expressions/report-b.md` | commit | f8d7b652 |
| REB-9 | `R-expressions/report-b.md` | commit | f8d7b652 |
| REB-10 | `R-expressions/report-b.md` | commit | f8d7b652 |
| REB-11 | `R-expressions/report-b.md` | commit | f8d7b652 |
| REB-12 | `R-expressions/report-b.md` | commit | f8d7b652 |
| REB-13 | `R-expressions/report-b.md` | commit | f8d7b652 |
| REB-14 | `R-expressions/report-b.md` | commit | f8d7b652 |
| REC-1 | `R-expressions/report-c.md` | commit | 1b47525e |
| REC-2 | `R-expressions/report-c.md` | commit | 1b47525e |
| REC-3 | `R-expressions/report-c.md` | commit | 1b47525e |
| REC-4 | `R-expressions/report-c.md` | commit | 1b47525e |
| REC-5 | `R-expressions/report-c.md` | commit | 1b47525e |
| REC-6 | `R-expressions/report-c.md` | commit | 1b47525e |
| REC-7 | `R-expressions/report-c.md` | commit | 1b47525e |
| REC-8 | `R-expressions/report-c.md` | commit | 1b47525e |
| REC-9 | `R-expressions/report-c.md` | commit | 1b47525e |
| REC-10 | `R-expressions/report-c.md` | commit | 1b47525e |
| REC-11 | `R-expressions/report-c.md` | commit | 1b47525e |
| REC-12 | `R-expressions/report-c.md` | commit | 1b47525e |
| REC-13 | `R-expressions/report-c.md` | commit | 1b47525e |
| RED-1 | `R-expressions/report-d.md` | commit | 1b47525e |
| RED-2 | `R-expressions/report-d.md` | commit | 1b47525e |
| RED-3 | `R-expressions/report-d.md` | commit | 1b47525e |
| RED-4 | `R-expressions/report-d.md` | commit | 1b47525e |
| RED-5 | `R-expressions/report-d.md` | commit | 1b47525e |
| RED-6 | `R-expressions/report-d.md` | commit | 1b47525e |
| RED-7 | `R-expressions/report-d.md` | commit | 1b47525e |
| RED-8 | `R-expressions/report-d.md` | commit | 1b47525e |
| RED-9 | `R-expressions/report-d.md` | commit | 1b47525e |
| RED-10 | `R-expressions/report-d.md` | commit | 1b47525e |
| RED-11 | `R-expressions/report-d.md` | commit | 1b47525e |
| RED-12 | `R-expressions/report-d.md` | commit | 1b47525e |
| RED-13 | `R-expressions/report-d.md` | commit | 1b47525e |
| RED-14 | `R-expressions/report-d.md` | commit | 1b47525e |
| RED-15 | `R-expressions/report-d.md` | commit | 1b47525e |
| RED-16 | `R-expressions/report-d.md` | commit | 1b47525e |
| RPA-1 | `R-overloading/report-a.md` | commit | a9906a86 |
| RPA-2 | `R-overloading/report-a.md` | commit | a9906a86 |
| RPA-3 | `R-overloading/report-a.md` | commit | a9906a86 |
| RPA-4 | `R-overloading/report-a.md` | commit | a9906a86 |
| RPA-5 | `R-overloading/report-a.md` | commit | a9906a86 |
| RPA-6 | `R-overloading/report-a.md` | commit | a9906a86 |
| RPA-7 | `R-overloading/report-a.md` | commit | a9906a86 |
| RPA-8 | `R-overloading/report-a.md` | commit | a9906a86 |
| RPA-9 | `R-overloading/report-a.md` | commit | a9906a86 |
| RPA-10 | `R-overloading/report-a.md` | commit | a9906a86 |
| RPB-1 | `R-overloading/report-b.md` | commit | a9906a86 |
| RPB-2 | `R-overloading/report-b.md` | commit | a9906a86 |
| RPB-3 | `R-overloading/report-b.md` | commit | a9906a86 |
| RPB-4 | `R-overloading/report-b.md` | commit | a9906a86 |
| RPB-5 | `R-overloading/report-b.md` | commit | a9906a86 |
| RPB-6 | `R-overloading/report-b.md` | commit | a9906a86 |
| RPB-7 | `R-overloading/report-b.md` | commit | a9906a86 |
| RPB-8 | `R-overloading/report-b.md` | commit | a9906a86 |
| RPC-1 | `R-foundations/report-c.md` | commit | a9906a86 |
| RPC-2 | `R-foundations/report-c.md` | commit | a9906a86 |
| RPC-3 | `R-foundations/report-c.md` | commit | a9906a86 |
| RPC-4 | `R-foundations/report-c.md` | commit | a9906a86 (matched by diff hunk) |
| RPC-5 | `R-foundations/report-c.md` | commit | a9906a86 (matched by diff hunk) |
| RPC-6 | `R-foundations/report-c.md` | commit | a9906a86 (matched by diff hunk) |
| RPC-7 | `R-foundations/report-c.md` | commit | a9906a86 (matched by diff hunk) |
| RPC-8 | `R-foundations/report-c.md` | commit | a9906a86 (matched by diff hunk) |
| RPC-9 | `R-foundations/report-c.md` | commit | a9906a86 (matched by diff hunk) |
| RPC-10 | `R-foundations/report-c.md` | commit | a9906a86 (matched by diff hunk) |
| RPC-11 | `R-foundations/report-c.md` | commit | a9906a86 |
| RPD-1 | `R-foundations/report-d.md` | commit | a9906a86 |
| RPD-2 | `R-foundations/report-d.md` | commit | a9906a86 |
| RPD-3 | `R-foundations/report-d.md` | commit | a9906a86 |
| RPD-4 | `R-foundations/report-d.md` | commit | a9906a86 |
| RPD-5 | `R-foundations/report-d.md` | commit | a9906a86 |
| RPD-6 | `R-foundations/report-d.md` | commit | a9906a86 |
| RPD-7 | `R-foundations/report-d.md` | commit | a9906a86 |
| RPD-8 | `R-foundations/report-d.md` | commit | a9906a86 |
| RPD-9 | `R-foundations/report-d.md` | commit | a9906a86 |
| RPD-10 | `R-foundations/report-d.md` | commit | a9906a86 |
| RPD-11 | `R-foundations/report-d.md` | commit | a9906a86 |
| RPE-1 | `R-exceptions/report-e.md` | commit | a9906a86 |
| RPE-2 | `R-exceptions/report-e.md` | commit | a9906a86 |
| RPE-3 | `R-exceptions/report-e.md` | commit | a9906a86 |
| RPE-4 | `R-exceptions/report-e.md` | commit | a9906a86 |
| RPE-5 | `R-exceptions/report-e.md` | commit | a9906a86 |
| RPE-6 | `R-exceptions/report-e.md` | commit | a9906a86 |
| RS-1 | `R-statements/report.md` | commit | 1b47525e |
| RS-2 | `R-statements/report.md` | commit | 1b47525e |
| RS-3 | `R-statements/report.md` | commit | 1b47525e |
| RS-4 | `R-statements/report.md` | commit | 1b47525e |
| RS-5 | `R-statements/report.md` | commit | 1b47525e |
| RS-6 | `R-statements/report.md` | commit | 1b47525e |
| RS-7 | `R-statements/report.md` | commit | 1b47525e |
| RS-8 | `R-statements/report.md` | commit | 1b47525e |
| RS-9 | `R-statements/report.md` | commit | 1b47525e |
| RS-10 | `R-statements/report.md` | commit | 1b47525e |
| RS-11 | `R-statements/report.md` | commit | 1b47525e |
| RS-12 | `R-statements/report.md` | commit | 1b47525e |
| RS-13 | `R-statements/report.md` | commit | 1b47525e |
| RS-14 | `R-statements/report.md` | commit | 1b47525e |
| RS-15 | `R-statements/report.md` | commit | 1b47525e |
| RS-16 | `R-statements/report.md` | commit | 1b47525e |
| W1-1 | `W1/report.md` | commit | ea47eaf8, 0bf1a546 |
| W1-2 | `W1/report.md` | commit | 0bf1a546 (matched by diff hunk) |
| W1-3 | `W1/report.md` | commit | 0bf1a546 (matched by diff hunk) |
| W1-4 | `W1/report.md` | commit | 0bf1a546 (matched by diff hunk) |
| W1-5 | `W1/report.md` | commit | 0bf1a546 (matched by diff hunk) |
| W1-6 | `W1/report.md` | commit | 0bf1a546 (matched by diff hunk) |
| W1-7 | `W1/report.md` | commit | 0bf1a546 (matched by diff hunk) |
| W1-8 | `W1/report.md` | commit | 0bf1a546 (matched by diff hunk) |
| W1-9 | `W1/report.md` | commit | 0bf1a546 |
| W10-1 | `W10/report.md` | commit | d58799c6 |
| W10-2 | `W10/report.md` | commit | d58799c6 |
| W10-3 | `W10/report.md` | commit | d58799c6 |
| W10-4 | `W10/report.md` | commit | d58799c6 |
| W10-5 | `W10/report.md` | commit | d58799c6 |
| W10-6 | `W10/report.md` | commit | d58799c6 |
| W11-1 | `W11/report.md` | commit | d58799c6 |
| W11-2 | `W11/report.md` | commit | d58799c6 |
| W11-3 | `W11/report.md` | commit | d58799c6 |
| W11-4 | `W11/report.md` | commit | d58799c6 |
| W11-5 | `W11/report.md` | commit | d58799c6 |
| W11-6 | `W11/report.md` | commit | d58799c6 |
| W11-7 | `W11/report.md` | commit | d58799c6 |
| W12-1 | `W12/report.md` | commit | d58799c6 |
| W12-2 | `W12/report.md` | commit | d58799c6 |
| W12-3 | `W12/report.md` | commit | d58799c6 |
| W12-4 | `W12/report.md` | commit | d58799c6 |
| W12-5 | `W12/report.md` | commit | d58799c6 |
| W12-6 | `W12/report.md` | commit | d58799c6 |
| W12-7 | `W12/report.md` | commit | d58799c6 |
| W13-1 | `W13/report.md` | commit | 0bf1a546 (matched by diff hunk) |
| W13-2 | `W13/report.md` | commit | 0bf1a546 |
| W13-3 | `W13/report.md` | commit | 0bf1a546 (matched by diff hunk) |
| W13-4 | `W13/report.md` | commit | 0bf1a546 (matched by diff hunk) |
| W13-5 | `W13/report.md` | commit | 0bf1a546 (matched by diff hunk) |
| W13-6 | `W13/report.md` | commit | 0bf1a546 (matched by diff hunk) |
| W13-7 | `W13/report.md` | commit | 0bf1a546 (matched by diff hunk) |
| W13-8 | `W13/report.md` | commit | 0bf1a546 |
| W13-9 | `W13/report.md` | commit | 0bf1a546 (matched by diff hunk) |
| W14-1 | `W14/report.md` | commit | 0bf1a546 (matched by diff hunk) |
| W14-2 | `W14/report.md` | commit | 0bf1a546 (matched by diff hunk) |
| W14-3 | `W14/report.md` | commit | 0bf1a546 (matched by diff hunk) |
| W14-4 | `W14/report.md` | commit | 0bf1a546 (matched by diff hunk) |
| W14-5 | `W14/report.md` | commit | 0bf1a546 (matched by diff hunk) |
| W14-6 | `W14/report.md` | commit | 0bf1a546 (matched by diff hunk) |
| W14-7 | `W14/report.md` | commit | 0bf1a546 (matched by diff hunk) |
| W14-8 | `W14/report.md` | text | TR-P2 links the units into one program; SINIT-P2 gives the accessor idiom; the linker command is a tool matter |
| W15-1 | `W15/report.md` | commit | ef3408dd |
| W15-2 | `W15/report.md` | commit | ef3408dd |
| W15-3 | `W15/report.md` | commit | ef3408dd, b456a30d |
| W15-4 | `W15/report.md` | commit | ef3408dd, b456a30d |
| W15-5 | `W15/report.md` | commit | ef3408dd |
| W15-6 | `W15/report.md` | commit | ef3408dd |
| W15-7 | `W15/report.md` | commit | ef3408dd |
| W15-8 | `W15/report.md` | commit | ef3408dd |
| W15-9 | `W15/report.md` | commit | ef3408dd |
| W16-1 | `W16/report.md` | commit | 26f05fca |
| W16-2 | `W16/report.md` | commit | 26f05fca |
| W16-3 | `W16/report.md` | commit | 26f05fca |
| W16-4 | `W16/report.md` | commit | 26f05fca |
| W16-5 | `W16/report.md` | commit | 26f05fca |
| W16-6 | `W16/report.md` | commit | 26f05fca |
| W16-7 | `W16/report.md` | commit | 26f05fca |
| W17-1 | `W17/report.md` | commit | 26f05fca |
| W17-2 | `W17/report.md` | commit | 26f05fca |
| W17-3 | `W17/report.md` | commit | 26f05fca |
| W17-4 | `W17/report.md` | commit | 26f05fca |
| W17-5 | `W17/report.md` | commit | 26f05fca |
| W17-6 | `W17/report.md` | commit | 26f05fca |
| W17-7 | `W17/report.md` | commit | 26f05fca |
| W18-1 | `W18/report.md` | commit | eff1a372 |
| W18-2 | `W18/report.md` | commit | 4a193ea4 |
| W18-3 | `W18/report.md` | commit | eff1a372 |
| W18-4 | `W18/report.md` | commit | eff1a372 |
| W18-5 | `W18/report.md` | commit | eff1a372 |
| W18-6 | `W18/report.md` | commit | eff1a372 |
| W18-7 | `W18/report.md` | commit | eff1a372 |
| W18-8 | `W18/report.md` | commit | eff1a372 |
| W18-9 | `W18/report.md` | commit | 4a193ea4 |
| W18-10 | `W18/report.md` | commit | eff1a372 |
| W18-11 | `W18/report.md` | commit | eff1a372 |
| W18-12 | `W18/report.md` | commit | 4a193ea4 |
| W18-13 | `W18/report.md` | commit | 4a193ea4 |
| W18-14 | `W18/report.md` | text | UNI-7 gives the designator form; the -Wextra warning is outside Tidal |
| W18-15 | `W18/report.md` | commit | 4a193ea4 |
| W19-1 | `W19/report.md` | commit | b1d89e77 |
| W19-2 | `W19/report.md` | commit | b1d89e77 |
| W19-3 | `W19/report.md` | commit | eff1a372 |
| W19-4 | `W19/report.md` | commit | eff1a372 |
| W19-5 | `W19/report.md` | commit | b1d89e77 |
| W19-6 | `W19/report.md` | text | needs no change: domain knowledge and idioms that TSP-10 and the declarators page allow |
| W2-1 | `W2/report.md` | commit | d58799c6 |
| W2-2 | `W2/report.md` | commit | d58799c6 |
| W2-3 | `W2/report.md` | commit | d58799c6 |
| W2-4 | `W2/report.md` | commit | d58799c6 |
| W2-5 | `W2/report.md` | commit | d58799c6 |
| W2-6 | `W2/report.md` | commit | d58799c6 |
| W3-1 | `W3/report.md` | commit | 0bf1a546 (matched by diff hunk) |
| W3-2 | `W3/report.md` | commit | 0bf1a546 (matched by diff hunk) |
| W3-3 | `W3/report.md` | commit | 0bf1a546 (matched by diff hunk) |
| W3-4 | `W3/report.md` | commit | 0bf1a546 (matched by diff hunk) |
| W3-5 | `W3/report.md` | commit | 0bf1a546 |
| W3-6 | `W3/report.md` | commit | 0bf1a546 (matched by diff hunk) |
| W4-1 | `W4/report.md` | commit | 0bf1a546 |
| W4-2 | `W4/report.md` | commit | 0bf1a546 (matched by diff hunk) |
| W4-3 | `W4/report.md` | commit | 0bf1a546 (matched by diff hunk) |
| W4-4 | `W4/report.md` | commit | 0bf1a546 (matched by diff hunk) |
| W4-5 | `W4/report.md` | commit | 0bf1a546 (matched by diff hunk) |
| W4-6 | `W4/report.md` | commit | 0bf1a546 (matched by diff hunk) |
| W4-7 | `W4/report.md` | commit | 0bf1a546 (matched by diff hunk) |
| W4-8 | `W4/report.md` | commit | 0bf1a546 (matched by diff hunk) |
| W4-9 | `W4/report.md` | commit | 0bf1a546 (matched by diff hunk) |
| W4-10 | `W4/report.md` | commit | 0bf1a546 (matched by diff hunk) |
| W5-1 | `W5/report.md` | commit | e5cb0b27, d58799c6 |
| W5-2 | `W5/report.md` | commit | d58799c6 |
| W5-3 | `W5/report.md` | commit | d58799c6 |
| W5-4 | `W5/report.md` | commit | d58799c6 |
| W5-5 | `W5/report.md` | commit | d58799c6 |
| W5-6 | `W5/report.md` | commit | d58799c6 |
| W5-7 | `W5/report.md` | commit | d58799c6 |
| W6-1 | `W6/report.md` | commit | d58799c6 |
| W6-2 | `W6/report.md` | commit | d58799c6 |
| W6-3 | `W6/report.md` | commit | d58799c6 |
| W6-4 | `W6/report.md` | commit | d58799c6 |
| W6-5 | `W6/report.md` | commit | d58799c6 |
| W6-6 | `W6/report.md` | commit | d58799c6 |
| W6-7 | `W6/report.md` | commit | d58799c6 |
| W7-1 | `W7/report.md` | commit | d58799c6 |
| W7-2 | `W7/report.md` | commit | d58799c6 |
| W7-3 | `W7/report.md` | commit | d58799c6 |
| W7-4 | `W7/report.md` | commit | d58799c6 |
| W7-5 | `W7/report.md` | commit | d58799c6 |
| W7-6 | `W7/report.md` | commit | d58799c6 |
| W7-7 | `W7/report.md` | commit | d58799c6 |
| W8-1 | `W8/report.md` | commit | 0bf1a546 (matched by diff hunk) |
| W8-2 | `W8/report.md` | commit | 0bf1a546 (matched by diff hunk) |
| W8-3 | `W8/report.md` | commit | 0bf1a546 |
| W8-4 | `W8/report.md` | commit | 0bf1a546 |
| W8-5 | `W8/report.md` | commit | 0bf1a546 (matched by diff hunk) |
| W8-6 | `W8/report.md` | commit | 0bf1a546 (matched by diff hunk) |
| W8-7 | `W8/report.md` | commit | 0bf1a546 (matched by diff hunk) |
| W8-8 | `W8/report.md` | commit | 0bf1a546 (matched by diff hunk) |
| W8-9 | `W8/report.md` | text | needs no change: the preference for noexcept moves belongs to the library |
| W9-1 | `W9/report.md` | commit | d58799c6 |
| W9-2 | `W9/report.md` | commit | d58799c6 |
| W9-3 | `W9/report.md` | commit | d58799c6 |
| W9-4 | `W9/report.md` | commit | d58799c6 |
| W9-5 | `W9/report.md` | commit | d58799c6 |
| W9-6 | `W9/report.md` | commit | d58799c6 |
| W9-7 | `W9/report.md` | commit | d58799c6 |
| W9-8 | `W9/report.md` | commit | d58799c6 |
| W9-9 | `W9/report.md` | commit | d58799c6 |
| X1-1 | `X1/report.md` | commit | dd7372af |
| X1-2 | `X1/report.md` | commit | 1fd0bc54, dd7372af |
| X1-3 | `X1/report.md` | commit | 1fd0bc54, dd7372af |
| X1-4 | `X1/report.md` | commit | 1fd0bc54, dd7372af |
| X1-5 | `X1/report.md` | commit | 1fd0bc54, 5325b7ee |
| X1-6 | `X1/report.md` | commit | 1fd0bc54, dd7372af |
| X1-7 | `X1/report.md` | commit | 1fd0bc54, dd7372af, 2263667c |
| X1-8 | `X1/report.md` | commit | 1fd0bc54, dd7372af |
| X1-9 | `X1/report.md` | commit | dd7372af, 2263667c |
| X1-10 | `X1/report.md` | commit | dd7372af, 2263667c |
| X1-11 | `X1/report.md` | commit | 5325b7ee |
| X1-12 | `X1/report.md` | commit | dd7372af |
| X1-13 | `X1/report.md` | commit | dd7372af, 2263667c |
| X1-14 | `X1/report.md` | commit | dd7372af, b95c0b71 |
| X1-15 | `X1/report.md` | commit | 963caf5b, dd7372af |
| X1-16 | `X1/report.md` | commit | dd7372af |
| X1-17 | `X1/report.md` | commit | dd7372af |
| X10-1 | `X10/report.md` | commit | bddd2a08 |
| X10-2 | `X10/report.md` | commit | bddd2a08 |
| X10-3 | `X10/report.md` | commit | bddd2a08 |
| X10-4 | `X10/report.md` | commit | bddd2a08 |
| X10-5 | `X10/report.md` | commit | bddd2a08 |
| X10-6 | `X10/report.md` | commit | bddd2a08 |
| X10-7 | `X10/report.md` | commit | bddd2a08 |
| X10-8 | `X10/report.md` | commit | bddd2a08 |
| X10-9 | `X10/report.md` | commit | bddd2a08 |
| X10-10 | `X10/report.md` | commit | bddd2a08 |
| X10-11 | `X10/report.md` | commit | bddd2a08 |
| X10-12 | `X10/report.md` | commit | bddd2a08 |
| X11-1 | `X11/report.md` | commit | bddd2a08 |
| X11-2 | `X11/report.md` | commit | bddd2a08 |
| X11-3 | `X11/report.md` | commit | bddd2a08 |
| X11-4 | `X11/report.md` | commit | bddd2a08 |
| X11-5 | `X11/report.md` | commit | bddd2a08 |
| X11-6 | `X11/report.md` | commit | bddd2a08 |
| X11-7 | `X11/report.md` | commit | bddd2a08 |
| X11-8 | `X11/report.md` | commit | bddd2a08 |
| X11-9 | `X11/report.md` | commit | bddd2a08 |
| X12-1 | `X12/report.md` | commit | bddd2a08 |
| X12-2 | `X12/report.md` | commit | bddd2a08 |
| X12-3 | `X12/report.md` | commit | bddd2a08 |
| X12-4 | `X12/report.md` | commit | bddd2a08 |
| X12-5 | `X12/report.md` | commit | bddd2a08 |
| X12-6 | `X12/report.md` | commit | bddd2a08 |
| X12-7 | `X12/report.md` | commit | bddd2a08 |
| X12-8 | `X12/report.md` | commit | bddd2a08 |
| X12-9 | `X12/report.md` | commit | bddd2a08 |
| X12-10 | `X12/report.md` | commit | bddd2a08 |
| X12-11 | `X12/report.md` | commit | bddd2a08 |
| X12-12 | `X12/report.md` | commit | bddd2a08 |
| X12-13 | `X12/report.md` | commit | bddd2a08 |
| X13-1 | `X13/report.md` | commit | 966aa1b2 |
| X13-2 | `X13/report.md` | commit | 966aa1b2 |
| X13-3 | `X13/report.md` | commit | 966aa1b2 |
| X13-4 | `X13/report.md` | commit | 966aa1b2 |
| X13-5 | `X13/report.md` | commit | 966aa1b2 |
| X13-6 | `X13/report.md` | commit | 966aa1b2 |
| X13-7 | `X13/report.md` | commit | 966aa1b2 |
| X13-8 | `X13/report.md` | commit | 966aa1b2 |
| X13-9 | `X13/report.md` | commit | 966aa1b2 |
| X13-10 | `X13/report.md` | commit | 966aa1b2 |
| X13-11 | `X13/report.md` | commit | 966aa1b2 |
| X13-12 | `X13/report.md` | commit | 966aa1b2 |
| X13-13 | `X13/report.md` | commit | 966aa1b2 |
| X14-1 | `X14/report.md` | commit | 966aa1b2 |
| X14-2 | `X14/report.md` | commit | 966aa1b2 |
| X14-3 | `X14/report.md` | commit | 966aa1b2 |
| X14-4 | `X14/report.md` | commit | 966aa1b2 |
| X14-5 | `X14/report.md` | commit | 966aa1b2 |
| X14-6 | `X14/report.md` | commit | 966aa1b2 |
| X14-7 | `X14/report.md` | commit | 966aa1b2 |
| X14-8 | `X14/report.md` | commit | 966aa1b2 |
| X14-9 | `X14/report.md` | commit | 966aa1b2 |
| X14-10 | `X14/report.md` | commit | 966aa1b2 |
| X15-1 | `X15/report.md` | commit | 966aa1b2 |
| X15-2 | `X15/report.md` | commit | 966aa1b2 |
| X15-3 | `X15/report.md` | commit | 966aa1b2 |
| X15-4 | `X15/report.md` | commit | 966aa1b2 |
| X15-5 | `X15/report.md` | commit | 966aa1b2 |
| X15-6 | `X15/report.md` | commit | 966aa1b2 |
| X15-7 | `X15/report.md` | commit | 966aa1b2 |
| X15-8 | `X15/report.md` | commit | 966aa1b2 |
| X15-9 | `X15/report.md` | commit | 966aa1b2 |
| X16-1 | `X16/report.md` | commit | 966aa1b2 |
| X16-2 | `X16/report.md` | commit | 966aa1b2 |
| X16-3 | `X16/report.md` | commit | 966aa1b2 |
| X16-4 | `X16/report.md` | commit | 966aa1b2 |
| X16-5 | `X16/report.md` | commit | 966aa1b2 |
| X16-6 | `X16/report.md` | commit | 966aa1b2 |
| X16-7 | `X16/report.md` | commit | 966aa1b2 |
| X17-1 | `X17/report.md` | commit | 076aa546 |
| X17-2 | `X17/report.md` | commit | 076aa546 |
| X17-3 | `X17/report.md` | commit | 076aa546 |
| X17-4 | `X17/report.md` | commit | 076aa546 |
| X17-5 | `X17/report.md` | commit | 076aa546 |
| X17-6 | `X17/report.md` | commit | 076aa546 |
| X17-7 | `X17/report.md` | commit | 076aa546 |
| X17-8 | `X17/report.md` | commit | 076aa546 |
| X17-9 | `X17/report.md` | commit | 076aa546 |
| X17-10 | `X17/report.md` | commit | 076aa546 |
| X17-11 | `X17/report.md` | commit | 076aa546 |
| X18-1 | `X18/report.md` | commit | 076aa546 |
| X18-2 | `X18/report.md` | commit | 076aa546 |
| X18-3 | `X18/report.md` | commit | 076aa546 |
| X18-4 | `X18/report.md` | commit | 076aa546 |
| X18-5 | `X18/report.md` | commit | 076aa546 |
| X18-6 | `X18/report.md` | commit | 076aa546 |
| X18-7 | `X18/report.md` | commit | 076aa546 |
| X18-8 | `X18/report.md` | commit | 076aa546 |
| X18-9 | `X18/report.md` | commit | 076aa546 |
| X18-10 | `X18/report.md` | commit | 076aa546 |
| X18-11 | `X18/report.md` | commit | 076aa546 |
| X19-1 | `X19/report.md` | commit | 076aa546 |
| X19-2 | `X19/report.md` | commit | 076aa546 |
| X19-3 | `X19/report.md` | commit | 076aa546 |
| X19-4 | `X19/report.md` | commit | 076aa546 |
| X19-5 | `X19/report.md` | commit | 076aa546 |
| X19-6 | `X19/report.md` | commit | 076aa546 |
| X19-7 | `X19/report.md` | commit | 076aa546 |
| X19-8 | `X19/report.md` | commit | 076aa546 |
| X19-9 | `X19/report.md` | commit | 076aa546 |
| X19-10 | `X19/report.md` | commit | 076aa546 |
| X19-11 | `X19/report.md` | commit | 076aa546 |
| X20-1 | `X20/report.md` | commit | 076aa546 |
| X20-2 | `X20/report.md` | commit | 076aa546 |
| X20-3 | `X20/report.md` | commit | 076aa546 |
| X20-4 | `X20/report.md` | commit | 076aa546 |
| X20-5 | `X20/report.md` | commit | 076aa546 |
| X20-6 | `X20/report.md` | commit | 076aa546 |
| X20-7 | `X20/report.md` | commit | 076aa546, 3019417d |
| X20-8 | `X20/report.md` | commit | 076aa546 |
| X20-9 | `X20/report.md` | commit | 076aa546 |
| X20-10 | `X20/report.md` | commit | 076aa546 |
| X20-11 | `X20/report.md` | commit | 076aa546 |
| X21-1 | `X21/report.md` | commit | d9c18bcc |
| X21-2 | `X21/report.md` | commit | d9c18bcc |
| X21-3 | `X21/report.md` | commit | d9c18bcc |
| X21-4 | `X21/report.md` | commit | d9c18bcc |
| X21-5 | `X21/report.md` | commit | d9c18bcc |
| X21-6 | `X21/report.md` | commit | d9c18bcc |
| X21-7 | `X21/report.md` | commit | d9c18bcc |
| X21-8 | `X21/report.md` | commit | d9c18bcc |
| X21-9 | `X21/report.md` | commit | d9c18bcc |
| X21-10 | `X21/report.md` | commit | d9c18bcc |
| X21-11 | `X21/report.md` | commit | d9c18bcc |
| X22-1 | `X22/report.md` | commit | 1b05738b |
| X22-2 | `X22/report.md` | commit | 1b05738b |
| X22-3 | `X22/report.md` | commit | 1b05738b |
| X22-4 | `X22/report.md` | commit | 1b05738b |
| X22-5 | `X22/report.md` | commit | 1b05738b |
| X22-6 | `X22/report.md` | question | Q-0412 |
| X22-7 | `X22/report.md` | commit | 1b05738b |
| X22-8 | `X22/report.md` | out of scope | needs outside the specification (versions, cost) |
| X22-9 | `X22/report.md` | commit | 1b05738b |
| X3-1 | `X3/report.md` | commit | e9eacc80, dd7372af |
| X3-2 | `X3/report.md` | commit | 8d312937, dd7372af |
| X3-3 | `X3/report.md` | commit | dd7372af, 2263667c |
| X3-4 | `X3/report.md` | commit | dd7372af |
| X3-5 | `X3/report.md` | commit | e6961598, dd7372af |
| X3-6 | `X3/report.md` | commit | dd7372af |
| X3-7 | `X3/report.md` | commit | dd7372af |
| X3-8 | `X3/report.md` | commit | 963caf5b, dd7372af |
| X3-9 | `X3/report.md` | commit | dd7372af, b95c0b71 |
| X3-10 | `X3/report.md` | commit | dd7372af, b95c0b71 |
| X3-11 | `X3/report.md` | commit | dd7372af |
| X3-12 | `X3/report.md` | commit | dd7372af |
| X3-13 | `X3/report.md` | commit | dd7372af |
| X4-1 | `X4/report.md` | commit | 5325b7ee |
| X4-2 | `X4/report.md` | commit | e6961598 |
| X4-3 | `X4/report.md` | commit | 5325b7ee |
| X4-4 | `X4/report.md` | commit | 5325b7ee, 2263667c |
| X4-5 | `X4/report.md` | commit | 5325b7ee |
| X4-6 | `X4/report.md` | commit | e6961598 |
| X4-7 | `X4/report.md` | commit | e6961598 |
| X4-8 | `X4/report.md` | commit | e6961598 |
| X4-9 | `X4/report.md` | commit | 5325b7ee |
| X4-10 | `X4/report.md` | commit | 963caf5b, 5325b7ee |
| X4-11 | `X4/report.md` | commit | 5325b7ee |
| X4-12 | `X4/report.md` | commit | 5325b7ee, b95c0b71 |
| X4-13 | `X4/report.md` | commit | 5325b7ee, b95c0b71 |
| X4-14 | `X4/report.md` | commit | 5325b7ee, b95c0b71 |
| X4-15 | `X4/report.md` | commit | 5325b7ee |
| X4-16 | `X4/report.md` | commit | 5325b7ee |
| X5-1 | `X5/report.md` | commit | 3c569c2b |
| X5-2 | `X5/report.md` | commit | 3c569c2b |
| X5-3 | `X5/report.md` | commit | 3c569c2b |
| X5-4 | `X5/report.md` | commit | 3c569c2b |
| X5-5 | `X5/report.md` | commit | 3c569c2b |
| X5-6 | `X5/report.md` | commit | 3c569c2b |
| X5-7 | `X5/report.md` | commit | 3c569c2b |
| X5-8 | `X5/report.md` | commit | 3c569c2b |
| X5-9 | `X5/report.md` | commit | 3c569c2b |
| X5-10 | `X5/report.md` | commit | 3c569c2b |
| X5-11 | `X5/report.md` | commit | 3c569c2b |
| X5-12 | `X5/report.md` | commit | 3c569c2b |
| X5-13 | `X5/report.md` | commit | 3c569c2b |
| X5-14 | `X5/report.md` | commit | 3c569c2b |
| X5-15 | `X5/report.md` | commit | 3c569c2b |
| X5-16 | `X5/report.md` | commit | 3c569c2b |
| X5-17 | `X5/report.md` | commit | 3c569c2b |
| X5-18 | `X5/report.md` | text | needs no change: a judgment about usage, with no language rule involved |
| X5-19 | `X5/report.md` | question | Q-0007 |
| X5-20 | `X5/report.md` | commit | 3c569c2b |
| X6-1 | `X6/report.md` | commit | 3c569c2b |
| X6-2 | `X6/report.md` | commit | 3c569c2b (matched by diff hunk) |
| X6-3 | `X6/report.md` | commit | 9b60e6e4, 3c569c2b |
| X6-4 | `X6/report.md` | commit | 3c569c2b |
| X6-5 | `X6/report.md` | commit | 3c569c2b |
| X6-6 | `X6/report.md` | commit | 3c569c2b |
| X6-7 | `X6/report.md` | commit | 3c569c2b |
| X6-8 | `X6/report.md` | commit | 3c569c2b |
| X6-9 | `X6/report.md` | commit | 3c569c2b |
| X6-10 | `X6/report.md` | commit | 3c569c2b |
| X6-11 | `X6/report.md` | commit | 3c569c2b |
| X6-12 | `X6/report.md` | commit | 3c569c2b |
| X6-13 | `X6/report.md` | commit | 3c569c2b |
| X6-14 | `X6/report.md` | text | needs no change: a judgment about usage, with no language rule involved |
| X6-15 | `X6/report.md` | question | Q-0007 |
| X7-1 | `X7/report.md` | commit | 97a31fbb |
| X7-2 | `X7/report.md` | commit | 97a31fbb |
| X7-3 | `X7/report.md` | commit | 97a31fbb |
| X7-4 | `X7/report.md` | commit | 97a31fbb |
| X7-5 | `X7/report.md` | commit | 97a31fbb |
| X7-6 | `X7/report.md` | commit | 97a31fbb |
| X7-7 | `X7/report.md` | commit | 97a31fbb |
| X7-8 | `X7/report.md` | commit | 97a31fbb |
| X7-9 | `X7/report.md` | commit | 97a31fbb |
| X7-10 | `X7/report.md` | commit | 97a31fbb |
| X7-11 | `X7/report.md` | commit | 97a31fbb |
| X7-12 | `X7/report.md` | commit | 97a31fbb |
| X8-1 | `X8/report.md` | commit | 97a31fbb |
| X8-2 | `X8/report.md` | commit | 97a31fbb |
| X8-3 | `X8/report.md` | commit | 97a31fbb |
| X8-4 | `X8/report.md` | commit | 97a31fbb |
| X8-5 | `X8/report.md` | commit | 97a31fbb |
| X8-6 | `X8/report.md` | commit | 97a31fbb |
| X8-7 | `X8/report.md` | commit | 97a31fbb |
| X8-8 | `X8/report.md` | commit | 97a31fbb |
| X8-9 | `X8/report.md` | commit | 97a31fbb |
| X8-10 | `X8/report.md` | commit | 97a31fbb |
| X8-11 | `X8/report.md` | commit | 97a31fbb |
| X8-12 | `X8/report.md` | commit | 97a31fbb |
| X8-13 | `X8/report.md` | commit | 97a31fbb |
| X8-14 | `X8/report.md` | commit | 97a31fbb |
| X8-15 | `X8/report.md` | commit | 97a31fbb |
| X8-16 | `X8/report.md` | commit | 97a31fbb |
| X8-17 | `X8/report.md` | commit | 97a31fbb |
| X9-1 | `X9/report.md` | commit | bddd2a08 |
| X9-2 | `X9/report.md` | commit | bddd2a08 |
| X9-3 | `X9/report.md` | commit | bddd2a08 |
| X9-4 | `X9/report.md` | commit | bddd2a08 |
| X9-5 | `X9/report.md` | commit | bddd2a08 |
| X9-6 | `X9/report.md` | commit | bddd2a08 |
| X9-7 | `X9/report.md` | commit | bddd2a08 |
| X9-8 | `X9/report.md` | commit | bddd2a08 |
| X9-9 | `X9/report.md` | commit | bddd2a08 |
| X9-10 | `X9/report.md` | commit | bddd2a08 |
| X9-11 | `X9/report.md` | commit | bddd2a08 |
| X9-12 | `X9/report.md` | commit | bddd2a08 |
| X9-13 | `X9/report.md` | commit | bddd2a08 |
| X9-14 | `X9/report.md` | commit | bddd2a08 |
| X9-15 | `X9/report.md` | commit | bddd2a08 |
| P1-1 | `P1/report.md` | commit | 1017b19c (front-end round 23); test declarations/declarators/unknown-bound-converted-class-elements.cpp |
| P1-2 | `P1/report.md` | commit | 1017b19c (front-end round 23); test declarations/initialization/references/const-ref-through-converting-constructor.cpp and overloading/overload-resolution/const-ref-parameter-through-converting-constructor.cpp |
| P1-3 | `P1/report.md` | commit | 0f3f522f |
| P1-4 | `P1/report.md` | out of scope | the specification does not list what the language lacks |
| P1-5 | `P1/report.md` | commit | 0f3f522f |
| P1-6 | `P1/report.md` | text | DCL-19 and ADD-8 decide it; the finding asks for no change |
| P1-7 | `P1/report.md` | commit | 0f3f522f |
| P1-8 | `P1/report.md` | commit | 0f3f522f |
| P1-9 | `P1/report.md` | commit | 0f3f522f |
| I4-1 | `I4/report.md` | commit | ee6085aa: COND-16 base row |
| I4-2 | `I4/report.md` | commit | ee6085aa: COND-16 step 1, Q-0418 |
| I4-3 | `I4/report.md` | commit | ee6085aa: COND-P2 and COND-11 |
| I4-4 | `I4/report.md` | commit | 1017b19c (front-end round 23); test expressions/conditional/class-converts-to-int-reference-lvalue.cpp |
| I4-5 | `I4/report.md` | commit | 1017b19c (front-end round 23); test expressions/conditional/class-explicit-conversion-only.cpp |
| I4-6 | `I4/report.md` | commit | 1017b19c (front-end round 23); test expressions/conditional/class-ambiguous-base-operand.cpp |
| I4-7 | `I4/report.md` | commit | 1017b19c (front-end round 23); test expressions/conditional/class-private-base-operand.cpp |
| I4-8 | `I4/report.md` | commit | 03e578c4 |
| I4-9 | `I4/report.md` | commit | 03e578c4 |
| I4-10 | `I4/report.md` | commit | 03e578c4 |
| I4-11 | `I4/report.md` | commit | 03e578c4 |
| I4-12 | `I4/report.md` | commit | 03e578c4 |
| I4-13 | `I4/report.md` | commit | 03e578c4 |
| I4-14 | `I4/report.md` | commit | 03e578c4 |
| X23-1 | `X23/report.md` | commit | ee6085aa: ALN-4 and NEW-P5 |
| X23-2 | `X23/report.md` | commit | ee6085aa: NEW-29, Q-0417 |
| X23-3 | `X23/report.md` | commit | 0f3f522f |
| X23-4 | `X23/report.md` | commit | ee6085aa: NEW-21 |
| X23-5 | `X23/report.md` | commit | ee6085aa: STG-5 and NEW-29 |
| X23-6 | `X23/report.md` | commit | 0f3f522f |
| X23-7 | `X23/report.md` | commit | 0f3f522f |
| RV4-1 | `../review/2026-11-05-gap-review-phase-10-expressions-templates.md` | commit | 3bed9cb5: LAM-6 says simple capture (RV5-B-14) |
| RV4-2 | `../review/2026-11-05-gap-review-phase-10-expressions-templates.md` | text | LAM-29 states the right result (RV5-B-15) |
| RV4-3 | `../review/2026-11-05-gap-review-phase-10-classes-declarations-foundations.md` | commit | 3bed9cb5: SW-P1 (RV5-A-18) |
| RV4-4 | `../review/2026-11-05-gap-review-phase-10-classes-declarations-foundations.md` | commit | 6c6f69f8: the limits of JMP-10 in AUTO-P3 and RET-6 |
| RV4-5 | `../review/2026-11-05-gap-review-phase-10-classes-declarations-foundations.md` | commit | 3b83788d: SINIT-6 points at OBJ-11 |
| RV4-6 | `../review/2026-11-05-gap-review-phase-10-classes-declarations-foundations.md` | commit | 6c6f69f8: TR-17 for two definitions of main and the class of the main rule |
| RV4-7 | `../review/2026-11-05-gap-review-phase-10-expressions-templates.md` | commit | 6c6f69f8: DED-29 for an address with a target type, Q-0420 |
| RV5-A-1 | `../review/2026-11-05-gap-review-phase-12-classes-declarations-foundations.md` | commit | 3bed9cb5: CLS-26 and trailing requires clauses |
| RV5-A-2 | `../review/2026-11-05-gap-review-phase-12-classes-declarations-foundations.md` | commit | 3bed9cb5: ALN-4 cites STG-23 alone |
| RV5-A-3 | `../review/2026-11-05-gap-review-phase-12-classes-declarations-foundations.md` | commit | 3bed9cb5: the decltype(auto) row of AUTO-P3 |
| RV5-A-4 | `../review/2026-11-05-gap-review-phase-12-classes-declarations-foundations.md` | commit | 3bed9cb5: RET-6 |
| RV5-A-5 | `../review/2026-11-05-gap-review-phase-12-classes-declarations-foundations.md` | commit | 3bed9cb5: REF-1, every implicit pointer conversion |
| RV5-A-6 | `../review/2026-11-05-gap-review-phase-12-classes-declarations-foundations.md` | commit | 3bed9cb5: SINIT-6 |
| RV5-A-7 | `../review/2026-11-05-gap-review-phase-12-classes-declarations-foundations.md` | commit | 3bed9cb5: UNW-3 points at NEW-22 for a class |
| RV5-A-8 | `../review/2026-11-05-gap-review-phase-12-classes-declarations-foundations.md` | commit | 3bed9cb5: CONV-1 and BF-6, bit-fields of enumeration type |
| RV5-A-9 | `../review/2026-11-05-gap-review-phase-12-classes-declarations-foundations.md` | text | LSUP-11 keeps the matrix's answer; pending.md row 4 holds the split |
| RV5-A-10 | `../review/2026-11-05-gap-review-phase-12-classes-declarations-foundations.md` | commit | 3bed9cb5: NAM-21's friend sentence |
| RV5-A-11 | `../review/2026-11-05-gap-review-phase-12-classes-declarations-foundations.md` | question | Q-0416 |
| RV5-A-12 | `../review/2026-11-05-gap-review-phase-12-classes-declarations-foundations.md` | commit | 3bed9cb5: STG-5, a class's allocation function hides placement |
| RV5-A-13 | `../review/2026-11-05-gap-review-phase-12-classes-declarations-foundations.md` | commit | 3bed9cb5: STG-P3 |
| RV5-A-14 | `../review/2026-11-05-gap-review-phase-12-classes-declarations-foundations.md` | commit | 3bed9cb5: an Evidence note on TR-17 |
| RV5-A-15 | `../review/2026-11-05-gap-review-phase-12-classes-declarations-foundations.md` | commit | 3bed9cb5: TR-P1 |
| RV5-A-16 | `../review/2026-11-05-gap-review-phase-12-classes-declarations-foundations.md` | commit | 3bed9cb5: JMP-10, with Q-0422 for structured bindings |
| RV5-A-17 | `../review/2026-11-05-gap-review-phase-12-classes-declarations-foundations.md` | question | Q-0421 |
| RV5-A-18 | `../review/2026-11-05-gap-review-phase-12-classes-declarations-foundations.md` | commit | 3bed9cb5: SW-P1 |
| RV5-B-1 | `../review/2026-11-05-gap-review-phase-12-expressions-overloading-templates.md` | question | Q-0418 |
| RV5-B-2 | `../review/2026-11-05-gap-review-phase-12-expressions-overloading-templates.md` | commit | 3bed9cb5: an Evidence note on COND-16 and COND-3 |
| RV5-B-3 | `../review/2026-11-05-gap-review-phase-12-expressions-overloading-templates.md` | commit | 3bed9cb5: an Evidence note on COND-16 and COND-3 |
| RV5-B-4 | `../review/2026-11-05-gap-review-phase-12-expressions-overloading-templates.md` | commit | 3bed9cb5: an Evidence note on NEW-P5, pending.md row 4 |
| RV5-B-5 | `../review/2026-11-05-gap-review-phase-12-expressions-overloading-templates.md` | commit | 3bed9cb5: an Evidence note on NEW-22 |
| RV5-B-6 | `../review/2026-11-05-gap-review-phase-12-expressions-overloading-templates.md` | commit | 3bed9cb5: ICS-11 |
| RV5-B-7 | `../review/2026-11-05-gap-review-phase-12-expressions-overloading-templates.md` | commit | 3bed9cb5: an Evidence note on ICS-17 |
| RV5-B-8 | `../review/2026-11-05-gap-review-phase-12-expressions-overloading-templates.md` | question | Q-0419 |
| RV5-B-9 | `../review/2026-11-05-gap-review-phase-12-expressions-overloading-templates.md` | commit | 3bed9cb5: an Evidence note on ICS-45, pending.md row 4 |
| RV5-B-10 | `../review/2026-11-05-gap-review-phase-12-expressions-overloading-templates.md` | text | DED-29 is a fair reading; the doubt is in the reference's note |
| RV5-B-11 | `../review/2026-11-05-gap-review-phase-12-expressions-overloading-templates.md` | text | DEP-P6 follows the intent of the C++23 change; the doubt is in the reference |
| RV5-B-12 | `../review/2026-11-05-gap-review-phase-12-expressions-overloading-templates.md` | commit | 3bed9cb5: ORD-13 names the conditions it replaces |
| RV5-B-13 | `../review/2026-11-05-gap-review-phase-12-expressions-overloading-templates.md` | question | Q-0417 |
| RV5-B-14 | `../review/2026-11-05-gap-review-phase-12-expressions-overloading-templates.md` | commit | 3bed9cb5: LAM-5 to LAM-7, simple and init captures |
| RV5-B-15 | `../review/2026-11-05-gap-review-phase-12-expressions-overloading-templates.md` | text | LAM-29 states the right result; the reference is silent |
| W20-1 | `W20/report.md` | planned | decision 0061, item F3: no in-object storage form gives a pointer and a length; STG-P2 and the FixedVec example |
| W20-2 | `W20/report.md` | planned | decision 0061, items T2 and F3: no rule for the elements of an array member of a union (UNI-14) |
| W20-3 | `W20/report.md` | planned | decision 0061, items T2, F3, and FE: CPY-15 and CNS-11: when the by-value constructor template leaves the candidates; the front end gives CNS-35 |
| W20-4 | `W20/report.md` | planned | decision 0061, item F3: GCC with -fsanitize=undefined rejects a constant comparison of tag addresses (CMP-15 Evidence) |
| W20-5 | `W20/report.md` | planned | decision 0061, item F3: CAST-1 does not state the void* round trip that STG-16 gives |
| W20-6 | `W20/report.md` | planned | decision 0061, item F3: a table of the storage forms and what each allows |
| W20-7 | `W20/report.md` | text | outside knowledge on assignment idioms and sorting; no language rule involved |
| I5-1 | `I5/report.md` | planned | decision 0061, item F3: DED-30 does not point to DED-29's paragraph on an address with a target type |
| I5-2 | `I5/report.md` | planned | decision 0061, item F3: DED-29's first list against its third paragraph on a deduced result type |
| I5-3 | `I5/report.md` | planned | decision 0061, item F3: no rule drops const on a function type in deduction (DED-7, DED-14) |
| I5-4 | `I5/report.md` | planned | decision 0061, item F3: DED-15 does not point to OVL-37 for an ambiguous or private base |
| I5-5 | `I5/report.md` | planned | decision 0061, item F3: DED-24 and INST-1: T::type with an incomplete T |
| I5-6 | `I5/report.md` | planned | decision 0061, item F3: DED-8: the cv-qualifiers of a non-class prvalue |
| I5-7 | `I5/report.md` | planned | decision 0061, item F3: an example for DED-20 with explicit pack elements |
| I5-8 | `I5/report.md` | planned | decision 0061, item F3: DED-14 and DED-P4: a near match that fails |
| I5-9 | `I5/report.md` | planned | decision 0061, items T2, F3 (I5-5), and FE: front end: T::type with an incomplete T stops as unsupported |
| I5-10 | `I5/report.md` | planned | decision 0061, items T2 and FE: front end: a reference binds a private or ambiguous base |
| I5-11 | `I5/report.md` | planned | decision 0061, item F3: outside knowledge: arrays of functions or void in DED-24, the size type in DED-11 |

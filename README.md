2021-04-22 

#History of Sudoku Performance:

### First Attempt:

```
Instance:
0 5 0 | 0 0 0 | 0 0 0
3 0 0 | 4 2 0 | 0 0 0
0 0 6 | 0 0 0 | 9 8 4
---------------------
6 0 7 | 1 0 0 | 0 0 0
0 0 0 | 2 0 0 | 0 7 0
0 0 0 | 0 8 0 | 0 9 0
---------------------
4 0 0 | 6 0 0 | 0 3 0
7 0 0 | 0 0 0 | 0 0 5
0 3 0 | 0 0 8 | 4 1 0
Solved:
1 5 4 | 8 9 7 | 2 6 3
3 9 8 | 4 2 6 | 1 5 7
2 7 6 | 3 5 1 | 9 8 4
---------------------
6 2 7 | 1 3 9 | 5 4 8
8 4 9 | 2 6 5 | 3 7 1
5 1 3 | 7 8 4 | 6 9 2
---------------------
4 8 5 | 6 1 2 | 7 3 9
7 6 1 | 9 4 3 | 8 2 5
9 3 2 | 5 7 8 | 4 1 6
```

```
$ cargo bench

sudoku/solve/10         time:   [680.60 ms 782.41 ms 872.24 ms]
                        thrpt:  [11.465  elem/s 12.781  elem/s 14.693  elem/s]
                 change:
                        time:   [-14.740% +7.5833% +36.398%] (p = 0.56 > 0.05)
                        thrpt:  [-26.685% -7.0488% +17.288%]
                        No change in performance detected.
```


### Second Attempt with bitmap:

```
sudoku/solve/10         time:   [30.584 ms 30.858 ms 30.995 ms]
                        thrpt:  [322.64  elem/s 324.07  elem/s 326.97  elem/s]
                 change:
                        time:   [-96.345% -95.870% -95.191%] (p = 0.00 < 0.05)
                        thrpt:  [+1979.4% +2321.4% +2636.0%]
                        Performance has improved.
```

### Third Attempt with memorization:

```
sudoku/solve/10         time:   [15.173 ms 15.213 ms 15.265 ms]
                        thrpt:  [655.09  elem/s 657.35  elem/s 659.04  elem/s]
                 change:
                        time:   [-49.880% -49.763% -49.635%] (p = 0.00 < 0.05)
                        thrpt:  [+98.550% +99.057% +99.520%]
                        Performance has improved.
```

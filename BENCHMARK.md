# Benchmark Results

## Clone

```console
     Running benches/clone.rs (target/release/deps/clone-70fb9f18b1ff2436)
Gnuplot not found, using plotters backend
String clone small      time:   [37.215 ns 39.566 ns 42.005 ns]

String clone large      time:   [46.198 ns 47.326 ns 48.695 ns]
Found 13 outliers among 100 measurements (13.00%)
  5 (5.00%) high mild
  8 (8.00%) high severe

CompactString clone small
                        time:   [5.6553 ns 5.7248 ns 5.8085 ns]
Found 19 outliers among 100 measurements (19.00%)
  1 (1.00%) low mild
  12 (12.00%) high mild
  6 (6.00%) high severe

CompactString clone large
                        time:   [62.732 ns 63.701 ns 64.856 ns]
Found 13 outliers among 100 measurements (13.00%)
  9 (9.00%) high mild
  4 (4.00%) high severe

SmartString clone small time:   [6.4677 ns 6.5142 ns 6.5730 ns]
Found 12 outliers among 100 measurements (12.00%)
  7 (7.00%) high mild
  5 (5.00%) high severe

SmartString clone large time:   [48.296 ns 49.036 ns 49.842 ns]
Found 7 outliers among 100 measurements (7.00%)
  6 (6.00%) high mild
  1 (1.00%) high severe

EcoString clone small   time:   [27.577 ns 27.897 ns 28.285 ns]
Found 12 outliers among 100 measurements (12.00%)
  7 (7.00%) high mild
  5 (5.00%) high severe

EcoString clone large   time:   [27.320 ns 27.588 ns 27.955 ns]
Found 18 outliers among 100 measurements (18.00%)
  7 (7.00%) high mild
  11 (11.00%) high severe

ArcStr clone small      time:   [41.329 ns 41.876 ns 42.469 ns]
Found 14 outliers among 100 measurements (14.00%)
  9 (9.00%) high mild
  5 (5.00%) high severe

ArcStr clone large      time:   [40.842 ns 41.328 ns 41.912 ns]
Found 24 outliers among 100 measurements (24.00%)
  7 (7.00%) high mild
  17 (17.00%) high severe

ImString clone small    time:   [27.545 ns 27.913 ns 28.386 ns]
Found 21 outliers among 100 measurements (21.00%)
  3 (3.00%) high mild
  18 (18.00%) high severe

ImString clone large    time:   [27.608 ns 27.968 ns 28.381 ns]
Found 21 outliers among 100 measurements (21.00%)
  7 (7.00%) high mild
  14 (14.00%) high severe

Cow clone small         time:   [24.733 ns 24.985 ns 25.283 ns]
Found 17 outliers among 100 measurements (17.00%)
  9 (9.00%) high mild
  8 (8.00%) high severe

Cow clone large         time:   [34.350 ns 34.702 ns 35.148 ns]
Found 22 outliers among 100 measurements (22.00%)
  4 (4.00%) high mild
  18 (18.00%) high severe
```

## Push

```console
    Running benches/push.rs (target/release/deps/push-68e3cb170f330f69)
Gnuplot not found, using plotters backend
String push small       time:   [248.18 ns 250.63 ns 253.66 ns]
Found 17 outliers among 100 measurements (17.00%)
  5 (5.00%) high mild
  12 (12.00%) high severe

String push large       time:   [7.7993 µs 7.8998 µs 8.0224 µs]
Found 12 outliers among 100 measurements (12.00%)
  8 (8.00%) high mild
  4 (4.00%) high severe

CompactString push small
                        time:   [366.48 ns 371.83 ns 378.56 ns]
Found 9 outliers among 100 measurements (9.00%)
  7 (7.00%) high mild
  2 (2.00%) high severe

CompactString push large
                        time:   [15.567 µs 15.824 µs 16.115 µs]
Found 18 outliers among 100 measurements (18.00%)
  7 (7.00%) high mild
  11 (11.00%) high severe

SmartString push small  time:   [385.47 ns 389.45 ns 394.35 ns]
Found 19 outliers among 100 measurements (19.00%)
  9 (9.00%) high mild
  10 (10.00%) high severe

SmartString push large  time:   [24.796 µs 25.142 µs 25.558 µs]
Found 7 outliers among 100 measurements (7.00%)
  6 (6.00%) high mild
  1 (1.00%) high severe

EcoString push small    time:   [386.72 ns 390.75 ns 395.64 ns]
Found 14 outliers among 100 measurements (14.00%)
  2 (2.00%) high mild
  12 (12.00%) high severe

EcoString push large    time:   [24.714 µs 25.074 µs 25.342 µs]
Found 15 outliers among 100 measurements (15.00%)
  11 (11.00%) low severe
  4 (4.00%) low mild

ImString push small     time:   [1.6491 µs 1.7099 µs 1.7693 µs]
Found 28 outliers among 100 measurements (28.00%)
  17 (17.00%) low severe
  2 (2.00%) low mild
  5 (5.00%) high mild
  4 (4.00%) high severe

ImString push large     time:   [47.683 µs 51.173 µs 54.649 µs]
```

## Iterate

```console
     Running benches/iterate.rs (target/release/deps/iterate-d32ba16bc445ee17)
Gnuplot not found, using plotters backend
String iterate          time:   [3.8893 µs 4.1002 µs 4.3346 µs]
Found 2 outliers among 100 measurements (2.00%)
  2 (2.00%) high mild

CompactString iterate   time:   [5.0917 µs 5.2692 µs 5.4146 µs]

SmartString iterate     time:   [3.1570 µs 3.1893 µs 3.2270 µs]
Found 22 outliers among 100 measurements (22.00%)
  1 (1.00%) high mild
  21 (21.00%) high severe

EcoString iterate       time:   [3.1833 µs 3.2339 µs 3.2929 µs]
Found 21 outliers among 100 measurements (21.00%)
  3 (3.00%) high mild
  18 (18.00%) high severe

ArcStr iterate          time:   [3.2072 µs 3.2553 µs 3.3112 µs]
Found 2 outliers among 100 measurements (2.00%)
  1 (1.00%) high mild
  1 (1.00%) high severe

ImString iterate        time:   [111.85 µs 112.93 µs 114.27 µs]
Found 21 outliers among 100 measurements (21.00%)
  2 (2.00%) high mild
  19 (19.00%) high severe

Cow iterate             time:   [3.1849 µs 3.2214 µs 3.2661 µs]
Found 10 outliers among 100 measurements (10.00%)
  6 (6.00%) high mild
  4 (4.00%) high severe
```

## Replace

```console
String replace small    time:   [150.31 ns 150.64 ns 150.99 ns]
Found 1 outliers among 100 measurements (1.00%)
  1 (1.00%) high mild

String replace large    time:   [14.204 µs 14.219 µs 14.234 µs]
Found 4 outliers among 100 measurements (4.00%)
  1 (1.00%) high mild
  3 (3.00%) high severe

CompactString replace small
                        time:   [148.36 ns 148.58 ns 148.81 ns]
Found 1 outliers among 100 measurements (1.00%)
  1 (1.00%) high mild

CompactString replace large
                        time:   [14.206 µs 14.224 µs 14.244 µs]
Found 6 outliers among 100 measurements (6.00%)
  3 (3.00%) high mild
  3 (3.00%) high severe

SmartString replace small
                        time:   [154.03 ns 154.26 ns 154.48 ns]
Found 2 outliers among 100 measurements (2.00%)
  1 (1.00%) low mild
  1 (1.00%) high severe

SmartString replace large
                        time:   [14.150 µs 14.158 µs 14.166 µs]
Found 2 outliers among 100 measurements (2.00%)
  1 (1.00%) low mild
  1 (1.00%) high severe

EcoString replace small time:   [339.60 ns 341.15 ns 343.81 ns]
Found 8 outliers among 100 measurements (8.00%)
  5 (5.00%) high mild
  3 (3.00%) high severe

EcoString replace large time:   [33.191 µs 33.262 µs 33.357 µs]
Found 6 outliers among 100 measurements (6.00%)
  3 (3.00%) low mild
  3 (3.00%) high severe

ArcStr replace small    time:   [149.32 ns 149.68 ns 150.13 ns]
Found 5 outliers among 100 measurements (5.00%)
  1 (1.00%) high mild
  4 (4.00%) high severe

ArcStr replace large    time:   [14.103 µs 14.112 µs 14.122 µs]
Found 6 outliers among 100 measurements (6.00%)
  1 (1.00%) high mild
  5 (5.00%) high severe

ImString replace small  time:   [149.05 ns 149.38 ns 149.70 ns]
Found 5 outliers among 100 measurements (5.00%)
  1 (1.00%) low mild
  4 (4.00%) high mild

ImString replace large  time:   [14.138 µs 14.152 µs 14.171 µs]
Found 4 outliers among 100 measurements (4.00%)
  2 (2.00%) high mild
  2 (2.00%) high severe

Cow replace small       time:   [178.00 ns 178.22 ns 178.43 ns]
Found 6 outliers among 100 measurements (6.00%)
  5 (5.00%) high mild
  1 (1.00%) high severe

Cow replace large       time:   [14.530 µs 14.543 µs 14.557 µs]
Found 11 outliers among 100 measurements (11.00%)
  5 (5.00%) low mild
  2 (2.00%) high mild
  4 (4.00%) high severe
```

## `to_string`

```console
     Running benches/to_string.rs (target/release/deps/to_string-63196c013821e62b)
Gnuplot not found, using plotters backend
CompactString to_string small
                        time:   [48.823 ns 48.889 ns 48.962 ns]
Found 5 outliers among 100 measurements (5.00%)
  1 (1.00%) high mild
  4 (4.00%) high severe

CompactString to_string large
                        time:   [55.501 ns 55.565 ns 55.632 ns]
Found 2 outliers among 100 measurements (2.00%)
  1 (1.00%) low mild
  1 (1.00%) high mild

SmartString to_string small
                        time:   [51.152 ns 51.207 ns 51.263 ns]
Found 5 outliers among 100 measurements (5.00%)
  4 (4.00%) high mild
  1 (1.00%) high severe

SmartString to_string large
                        time:   [55.791 ns 55.843 ns 55.899 ns]
Found 5 outliers among 100 measurements (5.00%)
  1 (1.00%) low mild
  4 (4.00%) high mild

EcoString to_string small
                        time:   [48.007 ns 48.066 ns 48.138 ns]
Found 8 outliers among 100 measurements (8.00%)
  5 (5.00%) high mild
  3 (3.00%) high severe

EcoString to_string large
                        time:   [59.194 ns 59.229 ns 59.265 ns]
Found 3 outliers among 100 measurements (3.00%)
  1 (1.00%) low severe
  1 (1.00%) low mild
  1 (1.00%) high mild

ArcStr to_string small  time:   [25.261 ns 25.321 ns 25.383 ns]
Found 2 outliers among 100 measurements (2.00%)
  1 (1.00%) low mild
  1 (1.00%) high mild

ArcStr to_string large  time:   [42.685 ns 42.719 ns 42.760 ns]
Found 4 outliers among 100 measurements (4.00%)
  2 (2.00%) high mild
  2 (2.00%) high severe

ImString to_string small
                        time:   [46.870 ns 46.897 ns 46.925 ns]
Found 7 outliers among 100 measurements (7.00%)
  1 (1.00%) high mild
  6 (6.00%) high severe

ImString to_string large
                        time:   [59.869 ns 59.913 ns 59.962 ns]
Found 1 outliers among 100 measurements (1.00%)
  1 (1.00%) low mild

Cow to_string small     time:   [24.274 ns 24.306 ns 24.342 ns]
Found 6 outliers among 100 measurements (6.00%)
  1 (1.00%) low severe
  1 (1.00%) low mild
  2 (2.00%) high mild
  2 (2.00%) high severe

Cow to_string large     time:   [42.128 ns 42.157 ns 42.189 ns]
Found 5 outliers among 100 measurements (5.00%)
  4 (4.00%) high mild
  1 (1.00%) high severe
```

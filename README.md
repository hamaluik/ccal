# ccal

A continuous calendar for the command line

## Example

```
$ ccal -year 2025
Wk     Su Mo Tu We Th Fr Sa
---------------------------
 1 Jan 29 30 31  1  2  3  4
 2      5  6  7  8  9 10 11
 3     12 13 14 15 16 17 18
 4     19 20 21 22 23 24 25
 5 Feb 26 27 28 29 30 31  1
 6      2  3  4  5  6  7  8
 7      9 10 11 12 13 14 15
 8     16 17 18 19 20 21 22
 9 Mar 23 24 25 26 27 28  1
10      2  3  4  5  6  7  8
11      9 10 11 12 13 14 15
12     16 17 18 19 20 21 22
13     23 24 25 26 27 28 29
14 Apr 30 31  1  2  3  4  5
15      6  7  8  9 10 11 12
16     13 14 15 16 17 18 19
17     20 21 22 23 24 25 26
18 May 27 28 29 30  1  2  3
19      4  5  6  7  8  9 10
20     11 12 13 14 15 16 17
21     18 19 20 21 22 23 24
22 Jun 25 26 27 28 29 30 31
23      1  2  3  4  5  6  7
24      8  9 10 11 12 13 14
25     15 16 17 18 19 20 21
26     22 23 24 25 26 27 28
27 Jul 29 30  1  2  3  4  5
28      6  7  8  9 10 11 12
29     13 14 15 16 17 18 19
30     20 21 22 23 24 25 26
31 Aug 27 28 29 30 31  1  2
32      3  4  5  6  7  8  9
33     10 11 12 13 14 15 16
34     17 18 19 20 21 22 23
35     24 25 26 27 28 29 30
36 Sep 31  1  2  3  4  5  6
37      7  8  9 10 11 12 13
38     14 15 16 17 18 19 20
39     21 22 23 24 25 26 27
40 Oct 28 29 30  1  2  3  4
41      5  6  7  8  9 10 11
42     12 13 14 15 16 17 18
43     19 20 21 22 23 24 25
44 Nov 26 27 28 29 30 31  1
45      2  3  4  5  6  7  8
46      9 10 11 12 13 14 15
47     16 17 18 19 20 21 22
48     23 24 25 26 27 28 29
49 Dec 30  1  2  3  4  5  6
50      7  8  9 10 11 12 13
51     14 15 16 17 18 19 20
52     21 22 23 24 25 26 27
53 Jan 28 29 30 31  1  2  3
```

## Building

To build the project, you will need to have the following installed:

- [Rust](https://www.rust-lang.org/tools/install)

Once you have Rust installed, you can build the project by running:

```sh
cargo build --release
```

## Installing

To install `ccal` from the locally-checked out source code, you can use the following command:

```sh
cargo install --path .
```

Otherwise, install it from the git repo:

```sh
cargo install --git https://github.com/hamaluik/ccal
```

## Running

ccal is configured through several command line arguments. You can see the
full list of options by running:

```sh
$ ccal --help
```

## License

This project is licensed under the Apache-2.0 license, see the [LICENSE](LICENSE)
file for more information.

# pivot

Small program that creates very simplistic CSV-based 'pivot tables' on the command-line

# Example

Take for example the following CSV file:

    a,1,2
    a,42,11 
    b,2,3
    b,13,12
    c,3,4

It can be useful to summarize that data by grouping information based
on the first (0) column. For example, suppose that we want to group by
column 0 and output the sum of the other two columns. This can be achieved as follows:

    $ pivot 0 sum:1 sum:2 < test.csv
    a,43,13,
    b,15,15,
    c,3,4,

A single column of data can be referenced more than once, so to find the max and min in
column 1 do the following:

    $ pivot 0 max:1 min:2 < test.csv
    a,42,2,
    b,13,3,
    c,3,4,

# Rust

This is the first program I've ever written in Rust. I chose this particular task because
it was relatively small and something that I wanted to have. I would be grateful for
constructive criticism of my (lack of) Rust knowledge.

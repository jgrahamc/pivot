// pivot -- summarize and sort data from delimited text files
//
// Usage:
// pivot 0 sum:1 avg:3
//       ^ ^     ^
//       ^ ^     ^
//       ^ ^     Average values in column 3 of CSV
//       ^ ^
//       ^ Sum values in column one of CSV
//       ^
//       Column to pivot on (i.e. summarize)
//
// The operators are sum, avg, max, min
//
// Copyright (c) 2017 John Graham-Cumming

extern crate csv;

use csv::ReaderBuilder;
use std::env;
use std::io;
use std::collections::HashMap;
use std::process;
use std::str;

// Val is used to accumulate values from a single column of the CSV
struct Val {
    sum: i64, 
    count: i64, 
    max: i64,
    min: i64
}

// A Row is all the values accumulated as specified by the command-line
type Row = Vec<Val>;

// Pivot is the pivot table mapping some summarized value to a Row
type Pivot = HashMap<String, Row>;

// Op are the possible operations on a single column of the CSV
enum Op {
    Sum,
    Max,
    Min,
    Avg,
}

// ColOp keeps track of which operation to apply to which CSV column
struct ColOp {
    col: usize,
    op: Op
}

// Columns is the list of all CSV columns on which to perform operations and their
// corresponding operations
type Columns = Vec<ColOp>;

macro_rules! error {
    ($fmt:expr) => ({eprint!(concat!($fmt, "\n")); process::exit(1)});
    ($fmt:expr, $($arg:tt)*) => ({eprint!(concat!($fmt, "\n"), $($arg)*); process::exit(1)});
}

// run builds the pivot table by looking at rows in the CSV and summarizing based
// on the value indexed by row_index. The columns upon which operations are performed
// is in cols. The o parameter is used to kept track of the order in which items
// are added to the pivot table.
fn run(p: &mut Pivot, o: &mut Vec<String>, row_index: usize, cols: &Columns) {
    let mut r = ReaderBuilder::new()
        .has_headers(false)
        .from_reader(io::stdin());

    let mut j = -1;
    for res in r.records() {
        j += 1;
        if res.is_ok() {
            let rec = res.unwrap();
            if row_index > rec.len() {
                error!("Insufficient columns in CSV at row {}", j);
            }

            let row = p.entry(rec.get(row_index).unwrap().into()).or_insert(Vec::new());
            if row.len() == 0 {
                o.push(rec.get(row_index).unwrap().into());
                for _i in 0..cols.len() {
                    row.push(Val{sum: 0, count: 0, max: 0, min: 0});
                }
            }

            for i in 0..cols.len() {
                if cols[i].col > rec.len() {
                    error!("Insufficient columns in CSV row at row {}", j);
                }

                let num_string = rec.get(cols[i].col).unwrap();
                let num_res = num_string.parse::<i64>();
                if !num_res.is_ok() {
                    error!("Failed to parse number {} at CSV row {}", num_string, j);
                }
                let num = num_res.unwrap();
                if let Some(val) = row.get_mut(i) {
                    val.sum += num;
                    val.count += 1;
                    if val.count == 1 {
                        val.max = num;
                        val.min = num;
                    } else {
                        if num < val.min {
                            val.min = num;
                        } 
                        if num > val.max {
                            val.max = num;
                        }
                   }
               }
           }   
        }
    }
}

// parse the command line arguments and return the index of the CSV column
// which is the key for summarization. Also fills in the cols to specify
// which CSV columns are to be operated on and output
fn parse(cols: &mut Columns) -> usize {
    let args: Vec<String> = env::args().collect();

    if args.len() <= 2 {
        error!("Need at least two arguments; has {}", args.len());
    }

    let row_res = args[1].parse::<usize>();
    if !row_res.is_ok() {
        error!("First argument must be pivot row number, indexed from 1, not {}", args[1]);
    }
    let row_index = row_res.unwrap();

    for i in 2..args.len() {
        let parts: Vec<&str> = args[i].split(":").collect();
        if parts.len() != 2 {
            error!("Column parameters must be in the form op:index, don't understand {}", args[i]);
        }

        let index_res = parts[1].parse::<usize>();
        if !index_res.is_ok() {
            error!("Column parameters must be in the form op:index, don't understand {}", args[i]);
        }
        let index = index_res.unwrap();

        match parts[0] {
            "sum" => cols.push(ColOp{col: index, op: Op::Sum}),
            "max" => cols.push(ColOp{col: index, op: Op::Max}),
            "min" => cols.push(ColOp{col: index, op: Op::Min}),
            "avg" => cols.push(ColOp{col: index, op: Op::Avg}),

             _ => error!("The valid operators are: sum, max, min, avg; don't understand {}", parts[0]),
        }
    }   

    return row_index
}

fn main() {
    let mut cols: Columns = Vec::new();
    let row_index = parse(&mut cols);   

    let mut table: Pivot = HashMap::new();
    let mut order: Vec<String> = Vec::new();
    run(&mut table, &mut order, row_index, &cols);

    for row in order {
        let vals = &table[&row];
        print!("{},", row);
        for i in 0..cols.len() {
            match cols[i].op {
                Op::Sum => print!("{}", vals[i].sum),
                Op::Max => print!("{}", vals[i].max),
                Op::Min => print!("{}", vals[i].min),
                Op::Avg => print!("{}", vals[i].sum / vals[i].count),
            }
            print!(",");
        }
        println!();
    }
}

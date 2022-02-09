A CSV Parser/Reader (UTF-8 & ASCII only) library for Rust.

Three functions are exposed:

read(filename)
This reads a file and returns a Vec<u8> of the raw data

parse(buf)
This takes a Vec<u8> and parses the data as either UTF-8/ASCII into a row/col Vec<Vec<String>>

to_lines(row_cols)
This takes a row/col of a Vec<Vec<String>> and returns a Vec<String> with a dynamic column size for
all columns


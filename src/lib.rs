use std::{
    fs::File,
    io::{self, Error, Read},
    path::PathBuf,
};

pub fn read(filename: PathBuf) -> Result<Vec<u8>, Error> {
    let mut buf = Vec::new();

    let size_read: usize = if filename.exists() {
        // valid file - read it
        let file = File::open(&filename)?;

        let mut reader = io::BufReader::new(file);
        reader.read_to_end(&mut buf)?
    } else {
        // assume stdin pipe
        io::stdin().read_to_end(&mut buf)?
    };

    if size_read == 0 {
        // nothing read, nothing to do
        return Ok(buf);
    }

    // check for and remove UTF-8 BOM
    if buf[0..3] == [0xEF, 0xBB, 0xBF] {
        buf.remove(0);
        buf.remove(0);
        buf.remove(0);
    }

    Ok(buf)
}

pub fn parse(buf: Vec<u8>) -> Result<Vec<Vec<String>>, Error> {
    let mut row_cols: Vec<Vec<String>> = Vec::new();

    if buf.is_empty() {
        // nothing read, nothing to do
        return Ok(row_cols);
    }

    // split all
    let mut in_quotes: bool = false;
    let mut prev_pos: usize = 0;
    let mut add_col: bool = false;

    for ix in 0..buf.len() {
        if buf[ix] == 34 {
            // double quotes
            if in_quotes {
                // case: "text","","te""x,t"↩"tex↩ty",""""," "

                // already in double quotes, then check the next character
                // if it's another quote, then boil down to 1 double quote
                let next = if ix < buf.len() - 1 { buf[ix + 1] } else { 10 };

                if next == 10 || next == 13 || next == 44 {
                    in_quotes = false;
                    continue;
                }

            //continue;

            //in_quotes = false;
            } else {
                in_quotes = true;
            }
        }
        if (buf[ix] == 10 && !in_quotes) || ix == buf.len() - 1 {
            let subset = Vec::from(&buf[prev_pos..=ix]);
            let line = String::from_utf8(subset).unwrap();
            //println!("{:?}", line);
            let chr: Vec<char> = line.chars().collect();

            let mut cols: Vec<String> = Vec::new();

            let mut line_pos: usize = 0;
            for jx in 0..chr.len() {
                if chr[jx] == '"' {
                    // double quotes
                    if in_quotes {
                        // case: text,"","te""xt"↩"te,x↩ty"

                        let next = if jx < chr.len() - 1 {
                            chr[jx + 1]
                        } else {
                            '\n'
                        };

                        if next == '\n' || next == ',' {
                            in_quotes = false;

                            if jx == chr.len() - 1 {
                                add_col = true;
                            } else {
                                continue;
                            }
                        }
                    } else {
                        in_quotes = true;
                    }
                }
                if (chr[jx] == ',' && !in_quotes) || jx == chr.len() - 1 {
                    add_col = true;
                }

                if add_col {
                    //println!("col {line_pos} - {jx}    ::   chr.len() {}", chr.len());
                    let mut col = String::new();
                    let spacer = if jx - line_pos == 0 { 1 } else { 0 };
                    for i in line_pos..jx + spacer {
                        if chr[i] == '"' {
                            if i == line_pos {
                                // start of column, ignore a quote
                                //col.push('~');
                                continue;
                            }
                            if i == jx - 1 {
                                // end of column, ignore the quote
                                //col.push('¬');
                                continue;
                            }
                            if chr[i - 1] == '"' {
                                // previous char is another quote, ignore the quote
                                //col.push('¦');
                                continue;
                            }
                        }

                        //col.push('#');
                        col.push(chr[i]);
                    }
                    cols.push(col);
                    line_pos = jx + 1;
                    add_col = false;
                }
            }
            in_quotes = false;

            row_cols.push(cols);
            prev_pos = ix + 1;
        }
    }

    Ok(row_cols)
}

pub fn to_lines(row_cols: Vec<Vec<String>>) -> Vec<String> {
    let mut render: Vec<String> = Vec::new();
    let mut max_col_size: Vec<usize> = Vec::new();

    for row in &row_cols {
        // check per row for the max number of columns as some rows can be shorter or longer
        // than others and extend max_col_size accordingly to the length of the new
        if max_col_size.len() < row.len() {
            for col in row.iter().skip(max_col_size.len()) {
                max_col_size.push(col.len());
            }
        }

        for i in 0..max_col_size.len() {
            if i < row.len() && max_col_size[i] < row[i].len() {
                max_col_size[i] = row[i].len();
            }
        }
    }

    //    let mut i: usize = 0;
    for row in row_cols {
        //        i += 1;
        let mut line = String::new();
        line.push_str("| ");

        for x in 0..row.len() {
            line.push_str(format!("{:<w$} | ", row[x], w = max_col_size[x]).as_str());
        }
        //line.insert_str(0, format!("{i:>10} : {:>10} : ", line.len()).as_str());

        render.push(line);
    }

    render
}

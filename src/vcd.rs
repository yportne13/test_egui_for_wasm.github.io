use crate::signal::Signal;

pub fn dump_signal(input: &str) -> Vec<Signal> {
    let lines = input.lines();
    let dump_out = lines.fold((vec![],vec![],0),|(signal,identify_table,clock),line|
        parsing_line((signal,identify_table,clock), line.to_string())
    );
    dump_out.0
}

fn parsing_line((signal,identify_table,clock): (Vec<Signal>,Vec<Vec<i32>>,i32), line: String) -> (Vec<Signal>,Vec<Vec<i32>>,i32) {
    let mut line_item = line.split_whitespace();
    match line_item.next() {
        Some("$var") => {
            line_item.next();
            let size = line_item.next().unwrap().parse::<usize>().unwrap();
            let _identify = line_item.next().unwrap();
            let name = line_item.next().unwrap().to_string();
            let name_with_width = if size==1 {name} else {format!("{}[{}:0]",name,size-1)};
            let new_signal = Signal{name:name_with_width,size,value_change: vec![]};
            let signal_out = signal.into_iter().chain(vec![new_signal]).collect();//[signal,&vec![]].concat();
            (signal_out,identify_table,clock)
        },
        Some(_) => (signal,identify_table,clock),
        None => (signal,identify_table,clock),
    }
}
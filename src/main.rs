use std::io::{BufWriter, stdin, stdout, Write};
use anyhow::Result;
use token_read::TokenReader;

#[derive(Debug)]
struct TeaBox {
    index: usize,
    weight: usize,
    has_cover: bool,
}

#[derive(Debug, Default)]
struct Boxes {
    boxes: Vec<TeaBox>,
}

impl Boxes {
    pub fn get(&self, index: usize) -> Option<&TeaBox> {
        self.boxes.get(index - 1)
    }

    pub fn push(&mut self, mut tea: TeaBox) -> usize {
        let index = self.boxes.len() + 1;
        tea.index = index;
        self.boxes.push(tea);
        index
    }
}

struct Output {
    saved_tea: usize, // amount of saved tea
    changes: Vec<(usize, usize)>, // changes in covers' locations
}

fn calculate(boxes: &Boxes) -> Output {
    todo!()
}

#[derive(Debug)]
struct Input {
    boxes_count: usize, // N
    string_length: usize, // K
    boxes: Boxes,
}

fn read_input() -> Result<Input> {
    let mut input = TokenReader::new(stdin().lock());

    let (n, k): (usize, usize) = input.line()?;

    let mut boxes = Boxes::default();

    for line in input.take(n) {
        let (m, v): (usize, u8) = line?;
        let v = v == 1;

        boxes.push(TeaBox {
            index: 0, // index is automatically added
            weight: m,
            has_cover: v,
        });
    }

    Ok(Input {
        boxes_count: n,
        string_length: k,
        boxes,
    })
}

fn main() -> Result<()> {
    let input = read_input()?;

    let output = calculate(&input.boxes);
    let mut out = BufWriter::new(stdout().lock());

    writeln!(out, "{}", output.saved_tea)?;

    for (from, to) in output.changes {
        writeln!(out, "{} {}", from, to)?;
    }

    Ok(())
}

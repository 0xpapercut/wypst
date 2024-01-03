use crate::katex::*;

pub struct ArrayConstructor {
    pub body: NodeArray2D,
    pub h_lines_before_row: Vec<Vec<bool>>,
    pub cols: Option<Vec<AlignSpec>>,
}

impl ArrayConstructor {
    pub fn default() -> Self {
        Self {
            body: vec![],
            h_lines_before_row: vec![vec![]],
            cols: None
        }
    }

    pub fn next_row(&mut self) {
        self.body.push(Vec::new());
        self.h_lines_before_row.push(Vec::new());
    }

    pub fn push_node(&mut self, node: Node) {
        self.body.last_mut().unwrap().push(node);
    }

    pub fn map_body(&mut self, f: &dyn Fn(&mut Node) -> Node) {
        for row in self.body.iter_mut() {
            for node in row.iter_mut() {
                *node = f(node);
            }
        }
    }

    pub fn count_columns(&self) -> usize {
        self.body.iter().map(|row| row.len()).max().unwrap_or(0)
    }

    pub fn count_rows(&self) -> usize {
        self.body.iter().count()
    }

    pub fn cols_leftright_align(&mut self) -> &mut Self {
        let mut cols: Vec<AlignSpec> = Vec::new();
        for i in 0..self.count_columns() {
            let align = Align {
                align: if i % 2 == 0 { "r".to_string() } else { "l".to_string() },
                pregap: if i > 1 && i % 2 == 0 { Some(1f32) } else { Some(0f32) },
                postgap: Some(0f32),
            };
            cols.push(AlignSpec::Align(align));
        }
        self.cols = Some(cols);
        self
    }

    pub fn cols_center_align(&mut self) -> &mut Self {
        let mut cols: Vec<AlignSpec> = Vec::new();
        for i in 0..self.count_columns() {
            let align = Align {
                align: "c".to_string(),
                pregap: None,
                postgap: None
            };
            cols.push(AlignSpec::Align(align));
        }
        self.cols = Some(cols);
        self
    }

    pub fn builder(&self) -> ArrayBuilder {
        ArrayBuilder::default()
            .body(self.body.clone())
            .h_lines_before_row(self.h_lines_before_row.clone())
            .cols(self.cols.clone()).clone()
    }
}

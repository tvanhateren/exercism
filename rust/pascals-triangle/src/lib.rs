pub struct PascalsTriangle {
    contents: Vec<Vec<u32>>,
}

impl PascalsTriangle {
    pub fn new(row_count: usize) -> Self {
        let mut results: Vec<Vec<u32>> = Vec::with_capacity(row_count);

        if row_count > 0 {
            results.push(vec![1])
        };

        for _ in 1..row_count {
            let mut prev_row = vec![0];
            prev_row.extend_from_slice(results.last().unwrap());
            prev_row.push(0);

            let curr_row = prev_row
                .windows(2)
                .map(|window| window.iter().sum())
                .collect();

            results.push(curr_row);
        }

        PascalsTriangle { contents: results }
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        self.contents.clone()
    }
}

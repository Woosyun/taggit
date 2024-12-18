
#[derive(PartialEq, Debug)]
pub struct MiddleSnake {
    pub x: (usize, usize),
    pub y: (usize, usize),
}

impl MiddleSnake {
    fn from_edit_graph(diagonal: isize, x1: usize, x2: usize) -> MiddleSnake {
        let y1 = x1 as isize - diagonal;
        let y2 = x2 as isize - diagonal;
        
        MiddleSnake {
            x: (x1, x2),
            y: (y1 as usize, y2 as usize),
        }
    }

    fn is_bigger(&self, another: &Self) -> bool {
        let self_len = self.x.1 - self.x.0;
        let another_len = another.x.1 - another.x.0;

        self_len > another_len
    }
}

impl Default for MiddleSnake {
    fn default() -> MiddleSnake {
        MiddleSnake {
            x: (0, 0),
            y: (0, 0),
        }
    }
}

#[derive(Debug)]
pub struct EditGraph<'a>
{
    a: &'a [&'a str],
    b: &'a [&'a str],
    forward_diagonals: Vec<usize>,
    backward_diagonals: Vec<usize>,
}

impl<'a> EditGraph<'a>
{
    pub fn new(a: &'a [&'a str], b: &'a [&'a str]) -> Self {
        let max = std::cmp::max(a.len(), b.len());
        EditGraph {
            a,
            b,
            forward_diagonals: vec![0; 2*max + 1],
            backward_diagonals: vec![a.len(); 2*max + 1],
        }
    }

    fn max_axis(&self) -> usize {
        std::cmp::max(self.a.len(), self.b.len())
    }
    fn delta(&self) -> isize {
        self.a.len() as isize - self.b.len() as isize
    }
    fn get_forward_idx(&self, k: isize) -> usize {
        (k + self.max_axis() as isize) as usize
    }
    fn get_backward_idx(&self, k: isize) -> usize {
        (self.delta() + k + self.max_axis() as isize) as usize
    }

    fn update_forward_diagonal(&mut self, k: isize, d: isize, middle_snake: MiddleSnake) -> MiddleSnake {
        let idx = self.get_forward_idx(k);
        
        let l = || self.forward_diagonals[idx-1];
        let r = || self.forward_diagonals[idx+1];
        
        let mut x = if k==-d || (k!=d && r() > l()) {
            r()
        } else {
            l() + 1
        };
        
        let tmp_x = x;

        let mut y = (x as isize - k) as usize;
        while x < self.a.len() && y < self.b.len() && self.a[x] == self.b[y] {
            x += 1;
            y += 1;
        }
        self.forward_diagonals[idx] = x;

        let tmp_middle_snake = MiddleSnake::from_edit_graph(k, tmp_x, x);
        if !middle_snake.is_bigger(&tmp_middle_snake) {
            tmp_middle_snake
        } else {
            middle_snake
        }
    }

    // k and d are related to length of edit script, not direct diagonal index
    fn update_backward_diagonal(&mut self, k: isize, d: isize, middle_snake: MiddleSnake) -> MiddleSnake{
        let idx = self.get_backward_idx(k);
        
        let l = || self.backward_diagonals[idx-1];
        let r = || self.backward_diagonals[idx+1];
        let mut x = if k==d || (k != -d && l() < r()) {
            l()
        } else {
            r() - 1
        };

        let tmp_x = x;

        let mut y = x as isize - k - self.delta();
        while x > 0 && y > 0 && self.a[x-1] == self.b[y as usize - 1] {
            x -= 1;
            y -= 1;
        }
        self.backward_diagonals[idx] = x;

        let tmp_middle_snake = MiddleSnake::from_edit_graph(k, x, tmp_x);
        if !middle_snake.is_bigger(&tmp_middle_snake) {
            tmp_middle_snake
        } else {
            middle_snake
        }
    }

    pub fn find_middle_snake(&mut self) -> Option<MiddleSnake> {
        let boundary = self.a.len() + self.b.len();
        let mut middle_snake = MiddleSnake::default();

        for d in 0..= boundary.div_ceil(2) as isize {
            //phase 1
            for k in (-d..=d).step_by(2) {
                middle_snake = self.update_forward_diagonal(k, d, middle_snake);

                if self.delta().abs() % 2 == 1 && self.delta() - (d - 1) <= k && k <= self.delta() + (d - 1) {
                    let idx = self.get_forward_idx(k);
                    
                    let forward_x = self.forward_diagonals[idx];
                    let backward_x = self.backward_diagonals[idx];
                    if backward_x <= forward_x {
                        return Some(middle_snake);
                    }
                }
            }

            // phase 2
            for k in (-d..=d).step_by(2) {
                middle_snake = self.update_backward_diagonal(k, d, middle_snake);

                if self.delta().abs() % 2 == 0 && -d <= k + self.delta() && k + self.delta() <= d {
                    let idx = self.get_backward_idx(k);
                    
                    let forward_x = self.forward_diagonals[idx];
                    let backward_x = self.backward_diagonals[idx];
                    if backward_x <= forward_x {
                        return Some(middle_snake);
                    }
                }
            }
        }

        None
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test] 
    pub fn delete_1() {
        let a = vec!["0", "1", "2"];
        let b = vec!["1", "2"];

        let mut edit_graph = EditGraph::new(&a, &b);
        let middle_snake = edit_graph.find_middle_snake().expect("failed to find middle_snake");

        assert_eq!(middle_snake, MiddleSnake::from_edit_graph(1, 1, 3));
    } 

    #[test] 
    pub fn delete_2() {
        let a = vec!["0", "1", "2"];
        let b = vec!["0", "1"];
        let mut edit_graph = EditGraph::new(&a, &b);
        let middle_snake = edit_graph.find_middle_snake().expect("failed to find middle_snake");

        dbg!(&middle_snake);
        
        assert_eq!(middle_snake, MiddleSnake::from_edit_graph(0, 0, 2));
    }

    #[test] 
    pub fn delete_3() {
        let a = vec!["0", "1", "2"];
        let b = vec!["0", "2"];
        let mut edit_graph = EditGraph::new(&a, &b);
        let middle_snake = edit_graph.find_middle_snake().expect("failed to find middle_snake");

        assert_eq!(middle_snake, MiddleSnake::from_edit_graph(1, 2, 3));
    }

    #[test] 
    pub fn add_1() {
        let a = vec!["0", "1"];
        let b = vec!["0", "1", "2'"];

        let mut edit_graph = EditGraph::new(&a, &b);
        let middle_snake = edit_graph.find_middle_snake().expect("failed to find middle_snake");

        assert_eq!(middle_snake, MiddleSnake::from_edit_graph(0, 0, 2));
    }

    #[test] 
    pub fn add_2() {
        let a = vec!["0", "1"];
        let b = vec!["0", "2", "1"];

        let mut edit_graph = EditGraph::new(&a, &b);
        let middle_snake = edit_graph.find_middle_snake().expect("failed to find middle_snake");

        assert_eq!(middle_snake, MiddleSnake::from_edit_graph(-1, 1, 2))
    }

    #[test] 
    pub fn add_3() {
        let a = vec!["0", "1"];
        let b = vec!["2", "0", "1"];

        let mut edit_graph = EditGraph::new(&a, &b);
        let middle_snake = edit_graph.find_middle_snake().expect("failed to find middle_snake");

        assert_eq!(middle_snake, MiddleSnake::from_edit_graph(-1, 0, 2))
    }
}
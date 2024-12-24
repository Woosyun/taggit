#[derive(PartialEq, Debug)]
pub struct MiddleSnake {
    pub x: (usize, usize),
    pub y: (usize, usize),
}

impl MiddleSnake {
    fn from_edit_graph(diagonal: isize, x1: usize, x2: usize) -> MiddleSnake {
        let y1 = x1 as isize - diagonal;
        let y2 = x2 as isize - diagonal;

        assert!(y1 >= 0);
        assert!(y2 >= 0);
        
        MiddleSnake {
            x: (x1, x2),
            y: (y1 as usize, y2 as usize),
        }
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

    fn max_d(&self) -> usize {
        let m_n = self.a.len() + self.b.len();
        m_n.div_ceil(2)
    }
    fn delta(&self) -> isize {
        self.a.len() as isize - self.b.len() as isize
    }
    fn get_forward_idx(&self, k: isize) -> usize {
        let idx = if self.delta() < 0 {
            self.max_d() as isize - self.delta() + k
        } else {
            self.max_d() as isize + k
        };

        assert!(idx >= 0);
        idx as usize
    }
    fn get_backward_idx(&self, k: isize) -> usize {
        
        let idx = if self.delta() < 0 {
            self.max_d() as isize + k
        } else { // self.delta() >= 0
            self.max_d() as isize + self.delta() + k
        };

        assert!(idx >= 0);
        idx as usize
    }

    fn update_forward_diagonal(&mut self, k: isize, d: isize, middle_snake: MiddleSnake) -> MiddleSnake {
        let idx = self.get_forward_idx(k);
        
        let l = || self.forward_diagonals[idx-1];
        let r = || self.forward_diagonals[idx+1];
        
        //even if diagonal reach outside of the square, it can go futher because of this part.
        let lx = if k==-d || (k!=d && r() > l()) {
            r()
        } else {
            l() + 1
        };
        
        let mut rx = lx;
        let mut y = (lx as isize - k) as usize;


        //if diagonal is outside of the square, 
        if rx > self.a.len() || y > self.b.len() {
            return middle_snake;
        }
        
        while rx < self.a.len() && y < self.b.len() && self.a[rx] == self.b[y] {
            rx += 1;
            y += 1;
        }

        self.forward_diagonals[idx] = rx;

        if rx - lx >= middle_snake.x.1 - middle_snake.x.0 {
            MiddleSnake::from_edit_graph(k, lx, rx) // k가 아니라 diagoanl index를 보내야지!!
        } else {
            middle_snake
        }
    }

    // k and d are related to length of edit script, not direct diagonal index
    fn update_backward_diagonal(&mut self, k: isize, d: isize, middle_snake: MiddleSnake) -> MiddleSnake {
        let idx = self.get_backward_idx(k);
        
        let l = || self.backward_diagonals[idx-1];
        let r = || self.backward_diagonals[idx+1];
        
        let rx = if k==d || (k != -d && l() < r()) {
            l() as isize
        } else {
            r() as isize - 1 // backward diagonal could be 0. Then left of the diagonal would be -1 and overflow occured.
        };

        let mut y = rx - k - self.delta();

        if rx < 0 || y < 0 {
            return middle_snake;
        }
        let mut lx = rx as usize;

        while lx > 0 && y > 0 && self.a[lx-1] == self.b[y as usize - 1] {
            lx -= 1;
            y -= 1;
        }
        self.backward_diagonals[idx] = lx;

        if rx as usize - lx >= middle_snake.x.1 - middle_snake.x.0 {
            MiddleSnake::from_edit_graph(k + self.delta(), lx, rx as usize)
        } else {
            middle_snake
        }
    }

    pub fn find_middle_snake(&mut self) -> Option<MiddleSnake> {
        let mut middle_snake = MiddleSnake::default();

        for d in 0..=self.max_d() as isize {
            
            //phase 1
            for k in (-d..=d).step_by(2) {
                middle_snake = self.update_forward_diagonal(k, d, middle_snake);
                let idx = self.get_forward_idx(k);
                let forward_x = self.forward_diagonals[idx];
                let backward_x = self.backward_diagonals[idx];

                if self.delta().abs() % 2 == 1 && self.delta() - (d - 1) <= k && k <= self.delta() + (d - 1) {
                    if backward_x <= forward_x {
                        return Some(middle_snake);
                    }
                }
            }

            // println!("after updating forward diagonals, middle_snake is {:?}", &middle_snake);
            
            //무조건 끝이 나야하는데 안났다?
            
            // phase 2
            for k in (-d..=d).step_by(2) {
                middle_snake = self.update_backward_diagonal(k, d, middle_snake);
                let idx = self.get_backward_idx(k);
                let forward_x = self.forward_diagonals[idx];
                let backward_x = self.backward_diagonals[idx];

                if self.delta().abs() % 2 == 0 && -d <= self.delta() + k && self.delta() + k <= d {
                    if backward_x <= forward_x {
                        return Some(middle_snake);
                    }
                }
            }

            // println!("after updating backward diagonals, middle_snake is {:?}", &middle_snake);
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

    #[test] 
    pub fn modify_1() {
        let a = vec!["0", "1"];
        let b = vec!["01", "1", "2", "3"];

        let mut edit_graph = EditGraph::new(&a, &b);
        let middle_snake = edit_graph.find_middle_snake().expect("failed to find middle snake");

        assert_eq!(middle_snake, MiddleSnake::from_edit_graph(0, 1, 2));
    }

    #[test] 
    pub fn modify_single_line() {
        let a = ["0"];
        let b = ["1"];

        let mut edit_graph = EditGraph::new(&a, &b);
        let middle_snake = edit_graph.find_middle_snake().expect("failed to find middle_snake");

        assert_eq!(middle_snake, MiddleSnake::from_edit_graph(-1, 0, 0));
    }

    #[test] 
    pub fn modify_and_add_multiple_lines() {
        let before = "\
hello world
This line would be preserved";
        let after = "\
hello world!!
This line would be preserved
And I added 3rd line,
and 4th line";

        let before = before.lines().collect::<Vec<&str>>();
        let after = after.lines().collect::<Vec<&str>>();

        let mut edit_graph = EditGraph::new(&before, &after);
        let middle_snake = edit_graph.find_middle_snake().expect("failed to find middle snake");
        
        assert_eq!(middle_snake, MiddleSnake::from_edit_graph(0, 1, 2));
    }
}
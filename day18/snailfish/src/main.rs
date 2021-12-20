use std::fs;

struct Node<T> {
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
    value: Option<T>,
}

impl Node<usize> {
    pub fn new_blank() -> Self {
        Node::<usize> {
            left: None,
            right: None,
            value: None,
        }
    }

    pub fn add_expression(&mut self, char_iter: &mut std::iter::Peekable<std::str::Chars>) {
        if let Some(c) = char_iter.next() {

            match c {
                '[' => {
                    self.left = Some(Box::new(Node::new_blank()));
                    self.left.as_mut().unwrap().add_expression(char_iter);

                    if let Some(d) = char_iter.next() {
                        if d != ',' {
                            panic!("Invalid expression - expected comma");
                        }
                    } else {
                        panic!("End of data with unbalanced pair");
                    }

                    self.right = Some(Box::new(Node::new_blank()));
                    self.right.as_mut().unwrap().add_expression(char_iter);

                    if let Some(d) = char_iter.next() {
                        if d != ']' {
                            panic!("Pair doesn't end with ']'");
                        }
                    } else {
                        panic!("End of data without closing ']'");
                    }
                },

                ']' => {
                    println!("']' in main match - not sure I should get here");
                },

                ',' => {
                    println!("Comma in main case - not sure how we got here");
                    self.right = Some(Box::new(Node::new_blank()));
                    self.right.as_mut().unwrap().add_expression(char_iter);
                },

                '0'..='9' => {
                    let mut v = c.to_digit(10).unwrap();
                    loop {
                        if let Some(d) = char_iter.peek() {
                            if *d < '0' || *d > '9' { break; }
                        } else { break; }

                        let x = char_iter.next().unwrap();
                        v *= 10;
                        v += x.to_digit(10).unwrap();
                    }

                    self.value = Some(v as usize);
                },

                _ => panic!("Invalid character: '{}'", c),
            }
        }
    }

    pub fn _print(&self) {
        print!("{}", self._to_string());
    }

    pub fn _to_string(&self) -> String {
        let mut result;

        if let Some(l) = &self.left {
            result = format!("[{},", l._to_string());
            if let Some(r) = &self.right {
                result = format!("{}{}]", result, r._to_string());
            } else {
                panic!("Left value, but no right value");
            }
        } else {
            result = format!("{}", self.value.as_ref().unwrap());
        }

        result
    }

    fn send_right(&mut self, val: usize) {
        if let Some(_) = self.value {
            *self.value.as_mut().unwrap() += val;
            return;
        }

        self.left.as_mut().unwrap().send_right(val);
    }

    fn send_left(&mut self, val: usize) {
        if let Some(_) = self.value {
            *self.value.as_mut().unwrap() += val;
            return;
        }

        self.right.as_mut().unwrap().send_left(val);
    }

    // Start the explode process from a particular node - usually called from an Expression object
    // First traverse down the tree to find a node that meets the explode criteria
    // Then propagate the results back across the tree
    // Return values :-
    //      None - nothing has exploded
    //      Some(None, None) - Something did explode and values have been allocated (so nothing more to do)
    //      Some(Some, None), Some(None, Some), Some(Some, Some) - still have an explode value to allocate
    pub fn explode(&mut self, level: usize) -> Option<(Option<usize>, Option<usize>)> {
        // We shouldn't get down to an end node, but let's just check
        if let None = self.left { panic!("reached an end node in explode"); }

        let mut return_value = None;

        // Get the left and right pointers of the left and right nodes
        let left_node = self.left.as_mut().unwrap().value.clone();
        let right_node = self.right.as_ref().unwrap().value.clone();

        match (left_node, right_node) {
            (Some(l_val), Some(r_val)) => { // we have a numeric pair - [x,y], and we haven't already exploded
                if level > 4 {  // Need to explode
                    return_value = Some((Some(l_val), Some(r_val)));
                    self.left = None;
                    self.right = None;
                    self.value = Some(0);
                }
            },

            (Some(_value), None) => { //  Have a value to the left and an expression to the right
                let explode_result = self.right.as_mut().unwrap().explode(level + 1);
                if let Some(explode_vals) = explode_result {
                    if let (Some(l_val), _) = explode_vals {
                        *self.left.as_mut().unwrap().value.as_mut().unwrap() += l_val;
                        return_value = Some((None, explode_vals.1));
                    } else {
                        return_value = explode_result;
                    }
                } // return_value stays as default - None - nothing has exploded
            },

            (None, Some(_value)) => { // Have an expression to the left and a value to the right
                let explode_result = self.left.as_mut().unwrap().explode(level + 1);
                if let Some(explode_vals) = explode_result {
                    if let (_, Some(r_val)) = explode_vals {
                        *self.right.as_mut().unwrap().value.as_mut().unwrap() += r_val;
                        return_value = Some((explode_vals.0, None));
                    } else {
                        return_value = explode_result;
                    }
                } // return_value stays as default - None - nothing has exploded
            },

            (None, None) => {  // Expressions to left and right
                // Try and explode left first
                let left_explode_result = self.left.as_mut().unwrap().explode(level + 1);
                if let Some(explode_vals) = left_explode_result {
                    if let (_, Some(r_val)) = explode_vals {
                        self.right.as_mut().unwrap().send_right(r_val);
                        return Some((explode_vals.0, None));
                    } else {
                        return left_explode_result;
                    }
                }

                // Nothing exploded on left expression, so try right
                let right_explode_result = self.right.as_mut().unwrap().explode(level + 1);
                if let Some(explode_vals) = right_explode_result {
                    if let(Some(l_val), _) = explode_vals {
                        self.left.as_mut().unwrap().send_left(l_val);
                        return Some((None, explode_vals.1));
                    } else {
                        return right_explode_result;
                    }
                }

                // If we're here neither left nor right expression exploded, so return default - None
            }
        }

        return_value
    }

    pub fn split(&mut self) -> bool {
        // check for a split on the left leg
        if let Some(l) = &mut self.left {
            if l.split() {
                return true;  // if we got one, quit
            }
        }

        // No split of left leg - do we happen to be a value node?
        if let Some(val) = self.value {
            if val > 9 {
                let mut left_node = Box::new(Node::new_blank());
                left_node.value = Some(val / 2);
                self.left = Some(left_node);

                let mut right_node = Box::new(Node::new_blank());
                right_node.value = Some(val - val / 2);
                self.right = Some(right_node);

                self.value = None;

                return true;
            }
        }

        // No split to left or if this is a value it's <10, so try right leg
        if let Some(r) = &mut self.right {
            if r.split() {
                return true;
            }
        }

        false
    }

    pub fn magnitude(&self) -> usize {
        if let Some(val) = self.value {
            return val;
        } else {
            return self.left.as_ref().unwrap().magnitude() * 3
                   + self.right.as_ref().unwrap().magnitude() * 2;
        }
    }
}

struct Expression<T> {
    head: Node<T>
}

impl Expression<usize> {
    pub fn new() -> Self {
        Expression::<usize> { head: Node::new_blank() }
    }

    pub fn from_str(exp_str: &str) -> Self {
        let mut expr = Expression::new();
        expr.build_expression(exp_str);
        expr
    }

    pub fn build_expression(&mut self, data: &str) {
        self.head.add_expression(&mut data.chars().peekable());
    }

    pub fn _print(&self) {
        self.head._print();
        println!();
    }

    pub fn _to_string(&self) -> String {
        self.head._to_string()
    }

    pub fn explode(&mut self) -> bool {
        if let Some(_) = self.head.explode(1) {
            true
        } else {
            false
        }
    }

    pub fn split(&mut self) -> bool {
        self.head.split()
    }

    pub fn magnitude(&self) -> usize {
        self.head.magnitude()
    }

    pub fn add(lhs: Expression<usize>, rhs: Expression<usize>) -> Expression<usize>{
        let mut new_head = Node::new_blank();

        new_head.left = Some(Box::new(lhs.head));
        new_head.right = Some(Box::new(rhs.head));

        Expression { head: new_head }
    }

    pub fn reduce(&mut self) {
        let mut more = true;
        while more {
            more = self.explode();
            if !more {
                more = self.split();
            }
        }


    }

}


fn main() {
    let data_string = fs::read_to_string("input.txt").expect("Can't read file");

    let expression_list:Vec<&str> = data_string
                                        .lines()
                                        .collect();

    let mut max_lhs = String::new();
    let mut max_rhs = String::new();
    let mut max_mag = 0;

    for l in 0..expression_list.len() {
        for r in l..expression_list.len() {
            let expr1 = Expression::from_str(expression_list[l]);
            let expr2 = Expression::from_str(expression_list[r]);
            let mut test_expr = Expression::add(expr1, expr2);
            test_expr.reduce();
            let this_mag = test_expr.magnitude();

            if this_mag > max_mag {
                max_lhs = expression_list[l].to_string();
                max_rhs = expression_list[r].to_string();
                max_mag = this_mag;
            }

            let expr1 = Expression::from_str(expression_list[r]);
            let expr2 = Expression::from_str(expression_list[l]);
            let mut test_expr = Expression::add(expr1, expr2);
            test_expr.reduce();
            let this_mag = test_expr.magnitude();

            if this_mag > max_mag {
                max_lhs = expression_list[r].to_string();
                max_rhs = expression_list[l].to_string();
                max_mag = this_mag;
            }
        }
    }

    println!("Largest magnitude is:");
    println!("  {}", max_lhs);
    println!("+ {}", max_rhs);
    println!("With magnitude: {}", max_mag);

}

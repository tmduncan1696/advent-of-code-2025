use crate::day06::input::Input;

pub fn solve(input: &Input) -> Option<usize> {
    let homework = Homework::from(input);

    let out = homework.calculate();

    Some(out)
}

pub struct Homework {
    pub values: Vec<Vec<usize>>,
    pub ops: Vec<char>
}

impl Homework {
    pub fn calculate(&self) -> usize {
        self.values
            .iter()
            .zip(self.ops.clone())
            .map(|(x, op)| perform_op(x.to_vec(), op))
            .sum()
    }
}

impl From<&Input> for Homework {
    fn from(input: &Input) -> Homework {
        let mut input_lines = input.clone();

        let ops: Vec<char> = input_lines
            .pop()
            .unwrap()
            .chars()
            .filter(|c| !c.is_whitespace())
            .collect();

        let values_t: Vec<Vec<char>> = input_lines
            .iter()
            .map(|x| x.chars().collect::<Vec<_>>())
            .collect();

        let values_char = transpose(values_t);

        let values_str: Vec<String> = values_char
            .clone()
            .into_iter()
            .map(|v| v.into_iter().collect::<String>())
            .map(|s| s.trim().to_owned())
            .collect();

        let values: Vec<Vec<usize>> = split_vector(values_str, "".to_string())
            .into_iter()
            .map(|v| {
                v.into_iter()
                    .map(|x| {
                        x.parse::<usize>().unwrap()
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        Homework {
            values,
            ops
        }
    }
}

fn perform_op(v: Vec<usize>, op: char) -> usize {
    match op {
        '+' => v.into_iter().sum(),
        '*' => v.into_iter().product(),
        _ => panic!("`op` should be '+' or '*'")
    }
}

fn transpose<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>> {
    let len = v[0].len();

    let mut iters: Vec<_> = v
        .into_iter()
        .map(|x| x.into_iter())
        .collect();

    (0..len)
        .map(|_| {
            iters
                .iter_mut()
                .map(|n| n.next().unwrap())
                .collect::<Vec<T>>()
        })
        .collect()
}

fn split_vector<T: Clone + PartialEq>(v: Vec<T>, value: T) -> Vec<Vec<T>> {
    let mut v_clone = v.clone();

    let mut out = Vec::new();

    loop {
        let pos = v_clone.iter().position(|x| *x == value);
        if let None = pos {
            out.push(v_clone.to_vec());
            break
        }
        let new_vec = v_clone.split_off(pos.unwrap());
        out.push(v_clone.to_vec());
        v_clone = new_vec[1..].to_vec();
    }

    out
}

#[cfg(test)]
mod test {
    use super::*;

   use crate::day06::input::read;

    #[test]
    fn test_solve() {
        let input_str = "\
123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  ";

        let input = read(&input_str);

        let out = solve(&input);

        assert_eq!(
            out,
            Some(3263827)
        );
    }
}

#[warn(unused_variables)]
pub fn _2220(start: i32, goal: i32) -> i32 {
    // examplesi: start = 10, goal = 7
    // 10 => 5-0 => 2-1 => 1-0 => 0-1
    //  7 => 3-1 => 1-1 => 0-1
    // 可以发现通过比较二进制对应位数是否相同，完成计算。
    // 直到一个商为0时结束循环，剩余没有结束的继续，结束的为0完成比较。
}

pub fn _0821(s: String, c: char) -> Vec<i32> {
    // two for
    // left -> right , meeting 'c' record index as idx
    // and calc answer[i] = abs(i - idx)
    // right -> left , meeting 'c' record index as idx
    // and calc answer[i] = min(answer[i], abs(i - idx))
}

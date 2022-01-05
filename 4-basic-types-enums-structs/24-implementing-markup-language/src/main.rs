// Can be freely copied to/from memory, borrow references aren't needed and
// will be copied.
#[derive(Copy, Clone)]
enum State {
    Normal,
    Comment,
    Upper,
    Lower
}

fn process (state: &State, c: char) -> (Option<char>, State) {
    use self::State::*;
    match (state, c) {
        (Normal, '#') => (None, Comment),
        (Normal, '^') => (None, Upper),
        (Normal, '_') => (None, Lower),
        (Normal, other) => (Some(other), Normal),
        (Comment, '#') => (None, Normal),
        (Comment, _) => (None, Comment),
        (Upper, '^') => (None, Normal),
        (Upper, other) => (Some(other.to_ascii_uppercase()), Upper),
        (Lower, '_') => (None, Normal),
        (Lower, other) => (Some(other.to_ascii_uppercase()), Lower)
    }
}

fn main () {
    let mut state = State::Normal;
    let mut result = String::new();
    let mut buffer = "This _Is_ some ^input^. # Clear this comment";
    
    for c in buffer.chars() {
        let (output, state_new) = process(&state, c);
        if let Some(c) = output {
            result.push(c);
            // @todo Why doesn't this work.
            // state = state_new
        }
        state = state_new
    }

    println!("Result: {}", result);
}

// In search_replace.rs
// StateMachine I 💜 🦀

pub fn update(
    start: &str,
    end: &str,
    replace: &str,
    lines: Vec<String>,
    verbose: bool,
) -> (bool, Vec<String>) {
    let mut updated_lines = Vec::new();
    let mut found_start = 0;
    let mut found_end = 0;
    let mut line_cnt = 0;
    let mut exit_with_error = false;
    let mut sm = StateMachine::new();

    for line in lines {
        line_cnt += 1;
        let (err, action) = match &line {
            s if s.contains(start) => {
                found_start += 1;
                sm.transition(Event::StartMatch)
            }
            s if s.contains(end) => {
                found_end += 1;
                sm.transition(Event::EndMatch)
            }
            _ => sm.transition(Event::NormalLine),
        };

        if err {
            exit_with_error = true;
            if verbose {
                eprint!("ERR search and replace at line {}", line_cnt);
            }
        }

        match action {
            LineAction::Copy => {
                updated_lines.push(line.clone());
            }
            LineAction::Drop => {}
            LineAction::InsertReplaceAndCopy => {
                // Split the multiline replace string by "\n" and add its lines separately
                for replace_line in replace.lines() {
                    updated_lines.push(replace_line.to_string());
                }
                updated_lines.push(line.clone()); // Keep the end marker
            }
        }
    }
    if found_start == 0 {
        exit_with_error = true;
        if verbose {
            eprint!(
                "ERR start and end did not match anyting search and replace at line {}",
                line_cnt
            );
        }
    }
    if found_start != found_end {
        exit_with_error = true;
        if verbose {
            eprint!(
                "ERR number of matches for start and end is differs {} != {}",
                found_start, found_end
            );
        }
    }

    (exit_with_error, updated_lines)
}

// Define an enum for states
enum State {
    StateCopyLines,
    StateDropLines,
}

// Define an enum for events
enum Event {
    NormalLine,
    StartMatch,
    EndMatch,
}

// Define an enum actions
enum LineAction {
    Copy,
    Drop,
    InsertReplaceAndCopy,
}

// Define the state machine struct
struct StateMachine {
    state: State,
}

impl StateMachine {
    fn new() -> Self {
        StateMachine {
            state: State::StateCopyLines,
        }
    }

    fn transition(&mut self, event: Event) -> (bool, LineAction) {
        // Use a match statement with a tuple of enums to cover all state and event combinations
        match (&self.state, event) {
            (State::StateCopyLines, Event::NormalLine) => {
                self.state = State::StateCopyLines;
                (false, LineAction::Copy)
            }
            (State::StateCopyLines, Event::StartMatch) => {
                self.state = State::StateDropLines;
                (false, LineAction::Copy) //keep StartMatch
            }
            (State::StateCopyLines, Event::EndMatch) => {
                self.state = State::StateCopyLines;
                (true, LineAction::Copy) //ERR keep line
            }
            (State::StateDropLines, Event::NormalLine) => {
                self.state = State::StateDropLines;
                (false, LineAction::Drop)
            }
            (State::StateDropLines, Event::StartMatch) => {
                self.state = State::StateDropLines;
                (true, LineAction::Drop) //ERR invalid keep dropping
            }
            (State::StateDropLines, Event::EndMatch) => {
                self.state = State::StateCopyLines;
                (false, LineAction::InsertReplaceAndCopy)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_update_with_markers() {
        let lines = vec![
            "Line 1",
            "This is the mark-start line",
            "This is the line to replace",
            "Another line to replace",
            "This is the mark-end line",
            "Line 6",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect();

        let start_marker = "mark-start";
        let end_marker = "mark-end";
        let replace_text = "Replacement Text\nWith Multiple Lines";
        let debug = false;

        let (exit_err, updated_lines) =
            update(start_marker, end_marker, replace_text, lines, debug);

        assert_eq!(updated_lines.len(), 6);
        assert_eq!(updated_lines[0], "Line 1");
        assert_eq!(updated_lines[1], "This is the mark-start line");
        assert_eq!(updated_lines[2], "Replacement Text");
        assert_eq!(updated_lines[3], "With Multiple Lines");
        assert_eq!(updated_lines[4], "This is the mark-end line");
        assert_eq!(updated_lines[5], "Line 6");

        assert_eq!(exit_err, false);
    }

    #[test]
    fn test_update_with_no_markers() {
        let lines = vec![
            "Line 1",
            "This is a regular line",
            "Another regular line",
            "Line 4",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect();

        let start_marker = "mark-start";
        let end_marker = "mark-end";
        let replace_text = "Replacement Text\nWith Multiple Lines";
        let debug = false;

        let (exit_err, updated_lines) =
            update(start_marker, end_marker, replace_text, lines, debug);

        // Since there are no markers, the lines should remain unchanged
        assert_eq!(updated_lines.len(), 4);
        assert_eq!(updated_lines[0], "Line 1");
        assert_eq!(updated_lines[1], "This is a regular line");
        assert_eq!(updated_lines[2], "Another regular line");
        assert_eq!(updated_lines[3], "Line 4");

        assert_eq!(exit_err, true); //exit error, no match.
    }
}

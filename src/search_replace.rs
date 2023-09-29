// In search_replace.rs

pub fn update(
    start: &str,
    end: &str,
    replace: &str,
    lines: Vec<String>,
    verbose: bool,
) -> (bool, Vec<String>) {
    let mut updated_lines = Vec::new();
    let mut found_start = false;
    let mut found_end = false;
    let mut line_cnt = 0;
    let mut exit_with_error = false;

    for line in lines {
        line_cnt += 1;
        if !found_start && line.contains(start) {
            found_start = true;
            found_end = false;
            updated_lines.push(line.clone());
            if verbose {
                eprint!("found match for start marker at line {}", line_cnt);
            }
        } else if found_start && !found_end && line.contains(end) {
            found_end = true;
            found_start = false;
            // Split the multiline replace string by "\n" and add its lines separately
            for replace_line in replace.lines() {
                updated_lines.push(replace_line.to_string());
            }
            updated_lines.push(line.clone()); // Keep the end marker
            if verbose {
                eprint!("found match for end marker at line {}", line_cnt);
            }
        } else if !found_start {
            updated_lines.push(line.clone());
            if found_start {
                exit_with_error = true;
                if verbose {
                    eprint!("mismatch in start end markers found !");
                }
            }
        } else if line.contains(start) && verbose {
            eprint!("warn found unexpecred start again at line {}", line_cnt);
        } else if line.contains(end) && verbose {
            eprint!("warn found unexpected end again at line {}", line_cnt);
        }
    }
    if verbose && found_start {
        eprint!("missmatch in start end markers found !");
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
    EventNormalLine,
    EventStartMatch,
    EventEndMatch,
}

// Define an enum actions
enum Action {
    CopyLine,
    DropLine,
    InsertReplaceAndCopyLine,
    InsertReplace,
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

    fn transition(&mut self, event: Event) -> (bool, Action) {
        // Use a match statement with a tuple of enums to cover all state and event combinations
        match (&self.state, event) {
            (State::StateCopyLines, Event::EventNormalLine) => {
                self.state = State::StateCopyLines;
                (false, Action::CopyLine)
            }
            (State::StateCopyLines, Event::EventStartMatch) => {
                self.state = State::StateDropLines;
                (false, Action::CopyLine) //keep StartMatch
            }
            (State::StateCopyLines, Event::EventEndMatch) => {
                self.state = State::StateCopyLines;
                (true, Action::CopyLine) //ERR keep line
            }
            (State::StateDropLines, Event::EventNormalLine) => {
                self.state = State::StateDropLines;
                (false, Action::DropLine)
            }
            (State::StateDropLines, Event::EventStartMatch) => {
                self.state = State::StateDropLines;
                (true, Action::DropLine) //ERR invalid keep dropping
            }
            (State::StateDropLines, Event::EventEndMatch) => {
                self.state = State::StateCopyLines;
                (false, Action::InsertReplaceAndCopyLine)
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

        assert_eq!(exit_err, false);
    }
}

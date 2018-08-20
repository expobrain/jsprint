use jsprint::JSprint;

pub fn command(jsprint: &mut JSprint, line: &str) {
    // Parse arguments
    let args = line.split(' ').skip(1).collect::<Vec<&str>>();

    if args.len() > 1 {
        println!("Pass only one sprint ID")
    }

    // Switch to sprint
    let sprint = match args.len() {
        0 => jsprint.get_active_sprint(),
        _ => {
            let sprint_id = args[0]
                .parse::<u64>()
                .unwrap_or_else(|_| panic!("Sprint ID {} is not a number", &args[0]));

            jsprint
                .get_sprint(sprint_id)
                .or_else(|| panic!("No sprint found with ID {}", sprint_id))
        }
    };

    // Set current sprint
    jsprint.current_sprint = sprint;
}

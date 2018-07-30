use jsprint::JSprint;

pub fn command(jsprint: &mut JSprint, line: &str) {
    // Parse arguments
    let args = line.split(' ').skip(1).collect::<Vec<&str>>();

    if args.len() > 1 {
        println!("Pass only one sprint ID")
    }

    let sprint_id = args[0]
        .parse::<u64>()
        .unwrap_or_else(|_| panic!("Sprint ID {} is not a number", &args[0]));

    // Get sprint
    let sprint = jsprint.get_sprint(sprint_id);

    match sprint {
        Some(_) => jsprint.current_sprint = sprint,
        None => println!("No sprint found with ID {}", sprint_id),
    };
}

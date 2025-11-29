use clap::Parser;

#[derive(Parser)]
#[command(name = "nic-names")]
#[command(author, version, about, long_about = None)]
#[command(group(
    clap::ArgGroup::new("filter")
        .args(["only_virtual", "only_physical"]),
))]
struct Cli {
    #[arg(long, help = "Show only virtual interfaces")]
    only_virtual: bool,

    #[arg(long, help = "Show only physical interfaces")]
    only_physical: bool,
}

fn main() {
    let cli = Cli::parse();

    let interfaces = netdev::get_interfaces();

    let filtered_interfaces = interfaces.into_iter().filter(|iface| {
        let is_physical = iface.is_physical();
        if cli.only_virtual {
            !is_physical
        } else if cli.only_physical {
            is_physical
        } else {
            true // No filter, include all
        }
    });

    for iface in filtered_interfaces {
        println!("{}", iface.name);
    }
}

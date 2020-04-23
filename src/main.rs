use prettytable::{cell, format, row, Table};
use regex::Regex;
use structopt::StructOpt;
use uom::fmt::DisplayStyle;
use uom::si::f64::{Length, Time, Velocity};
use uom::si::length::{kilometer, meter};
use uom::si::ratio::{percent, ratio};
use uom::si::time::{hour, minute, second};
use uom::si::velocity::kilometer_per_hour;
use uom::si::Unit;

#[derive(StructOpt, Debug)]
#[structopt(
    name = "Today I Ran",
    about = "This tool provides you with basic information derived from the distance you ran and the time you needed. This currently contains your average velocity and estimated times for other distances."
)]
struct CommandLineOptions {
    #[structopt(help = "the distance you ran today")]
    distance: String,
    #[structopt(help = "the time you needed")]
    time: String,
    #[structopt(short = "v", long = "verbose", help = "show additional information")]
    verbose: bool,
}

#[derive(Debug)]
struct Run {
    distance: Length,
    time: Time,
}

impl Run {
    fn from_options(options: &CommandLineOptions) -> Option<Self> {
        let dist_reg = Regex::new(r"(?P<value>\d+(\.\d*)?)\s*(?P<unit>[[:alpha:]]*)")
            .expect("distance parsing regex is wrong!");
        let dist_caps = dist_reg.captures(&options.distance)?;
        let dist_value = dist_caps.name("value")?.as_str().parse().ok()?;
        let dist_unit = dist_caps.name("unit")?.as_str().to_lowercase();

        let distance = match &dist_unit[..] {
            "m" => Length::new::<meter>(dist_value),
            "km" => Length::new::<kilometer>(dist_value),
            _ => None?,
        };

        let time_reg = Regex::new(
            r"((?P<hours>\d+)\s*h)?\s*((?P<minutes>\d+)\s*min)?((?P<seconds>\d+)\s*(s|sec))?",
        )
        .expect("time parsing regex is wrong!");
        let time_caps = time_reg.captures(&options.time)?;
        let group_to_value = |group| {
            time_caps
                .name(group)
                .map_or(Ok(0.0), |m| m.as_str().parse())
                .ok()
        };
        let hours = group_to_value("hours")?;
        let minutes = group_to_value("minutes")?;
        let seconds = group_to_value("seconds")?;

        let time =
            Time::new::<hour>(hours) + Time::new::<minute>(minutes) + Time::new::<second>(seconds);

        return Some(Run { distance, time });
    }

    fn average_velocity(&self) -> Velocity {
        return self.distance / self.time;
    }

    fn time_for_distance(&self, other_distance: &Length) -> Time {
        *other_distance / self.distance * self.time
    }
}

struct NamedLength {
    name: String,
    distance: Length,
}

struct NamedVelocity {
    name: String,
    velocity: Velocity,
}

fn display_time(time: &Time) -> String {
    let mut t = time.clone();

    let hours = t.trunc::<hour>();
    t -= hours;
    let minutes = t.trunc::<minute>();
    t -= minutes;
    let seconds = t.trunc::<second>();

    let h = hours.get::<hour>() as i32;
    let m = minutes.get::<minute>() as i32;
    let s = seconds.get::<second>() as i32;

    if h > 0 {
        format!("{} h {} min {} s", h, m, s)
    } else {
        if m > 0 {
            format!("{} min {} s", m, s)
        } else {
            format!("{} s", s)
        }
    }
}

fn main() {
    let options = CommandLineOptions::from_args();
    if let Some(run) = Run::from_options(&options) {
        println!(
            "Today, you ran {} {} in {}.",
            run.distance.get::<kilometer>(),
            kilometer::abbreviation(),
            display_time(&run.time)
        );
        println!(
            "Your average velocity was {:.3} {}.",
            run.average_velocity().get::<kilometer_per_hour>(),
            kilometer_per_hour::abbreviation()
        );

        if options.verbose {
            let distances = &[
                NamedLength {
                    name: String::from("100 m"),
                    distance: Length::new::<meter>(100.0),
                },
                NamedLength {
                    name: String::from("1 km"),
                    distance: Length::new::<kilometer>(1.0),
                },
                NamedLength {
                    name: String::from("5 km"),
                    distance: Length::new::<kilometer>(5.0),
                },
                NamedLength {
                    name: String::from("10 km"),
                    distance: Length::new::<kilometer>(10.0),
                },
                NamedLength {
                    name: String::from("half marathon"),
                    distance: Length::new::<kilometer>(21.0975),
                },
                NamedLength {
                    name: String::from("marathon"),
                    distance: Length::new::<kilometer>(42.195),
                },
            ];

            let mut dist_table = Table::new();
            dist_table.set_format(*format::consts::FORMAT_CLEAN);
            for distance in distances {
                dist_table.add_row(row![
                    distance.name,
                    format!(
                        "{}",
                        display_time(&run.time_for_distance(&distance.distance))
                    )
                ]);
            }

            println!("\nThis is how long you would have needed for other distances:");
            dist_table.printstd();

            let velocities = &[
                NamedVelocity {
                    name: String::from("Usain Bolt\'s 100 m WR"),
                    velocity: Velocity::new::<kilometer_per_hour>(37.5783),
                },
                NamedVelocity {
                    name: String::from("Eliud Kipchoge\'s inofficial marathon WR"),
                    velocity: Velocity::new::<kilometer_per_hour>(21.1563),
                },
                NamedVelocity {
                    name: String::from("Yohann Diniz\' 50 km race walk WR"),
                    velocity: Velocity::new::<kilometer_per_hour>(14.1143),
                },
            ];

            let mut vel_table = Table::new();
            vel_table.set_format(*format::consts::FORMAT_CLEAN);
            for velocity in velocities {
                vel_table.add_row(row![
                    format!(
                        "{:.3} times",
                        (run.average_velocity() / velocity.velocity).get::<ratio>()
                    ),
                    velocity.name
                ]);
            }

            println!("\nYour average velocity divided by those of other performances:");
            vel_table.printstd();
        }
    } else {
        println!("Could not parse the given distance and time.");
    }
}

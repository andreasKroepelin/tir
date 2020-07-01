use anyhow::{anyhow, Context};
use colored::*;
use prettytable::{cell, format, row, Table};
use regex::Regex;
use structopt::StructOpt;
use uom::si::f64::{Length, Time, Velocity};
use uom::si::length::{foot, kilometer, meter, mile, yard};
use uom::si::ratio::{percent, ratio};
use uom::si::time::{hour, minute, second};
use uom::si::velocity::{kilometer_per_hour, mile_per_hour};
use uom::si::Unit;

#[derive(StructOpt, Debug)]
#[structopt(
    name = "Today I Ran",
    about = "This tool provides you with basic information derived from the distance you ran and the time you needed. This currently contains your average velocity, estimated times for other distances and comparisons with other performances."
)]
struct CommandLineOptions {
    #[structopt(help = "the distance you ran today")]
    distance: String,
    #[structopt(help = "the time you needed")]
    time: String,
    #[structopt(short = "v", long = "verbose", help = "show additional information")]
    verbose: bool,
    #[structopt(short = "m", long = "miles", help = "use miles as unit of length")]
    use_miles: bool,
}

#[derive(Debug)]
struct Run {
    distance: Length,
    time: Time,
}

impl Run {
    fn from_options(options: &CommandLineOptions) -> anyhow::Result<Self> {
        let dist_reg = Regex::new(r"(?P<value>\d+(\.\d*)?)\s*(?P<unit>[[:alpha:]]*)")
            .expect("distance parsing regex is wrong!");
        let dist_caps = dist_reg
            .captures(&options.distance)
            .with_context(|| "Could not parse distance.")?;
        let dist_value = dist_caps
            .name("value")
            .with_context(|| "Could not find a value for distance.")?
            .as_str()
            .parse()
            .with_context(|| "Could not parse distance value as number.")?;
        let dist_unit = dist_caps
            .name("unit")
            .with_context(|| "Could not find a unit for distance.")?
            .as_str()
            .to_lowercase();

        let distance = match &dist_unit[..] {
            "m" | "meter" | "meters" => Length::new::<meter>(dist_value),
            "km" | "kilometer" | "kilometers" => Length::new::<kilometer>(dist_value),
            "mi" | "mile" | "miles" => Length::new::<mile>(dist_value),
            "yd" | "yard" | "yards" => Length::new::<yard>(dist_value),
            "ft" | "foot" | "feet" => Length::new::<foot>(dist_value),
            _ => None.with_context(|| format!("Unknown unit \"{}\".", dist_unit))?,
        };

        let time_reg = Regex::new(
            r"((?P<hours>.+)\s*h)?\s*((?P<minutes>.+)\s*min)?((?P<seconds>.+)\s*(s|sec))?",
        )
        .expect("time parsing regex is wrong!");
        let time_caps = time_reg
            .captures(&options.time)
            .with_context(|| "Could not parse time.")?;

        if !["hours", "minutes", "seconds"]
            .iter()
            .map(|g| time_caps.name(g))
            .any(|m| m.is_some())
        {
            return Err(anyhow!("No hours, no minutes, and no seconds given."));
        }

        let group_to_value = |group| {
            time_caps.name(group).map_or(Ok(0.0), |m| {
                m.as_str()
                    .parse()
                    .with_context(|| format!("\"{}\" is not a number", m.as_str()))
            })
        };
        let hours =
            group_to_value("hours").with_context(|| "Could not parse hours value as number.")?;
        let minutes = group_to_value("minutes")
            .with_context(|| "Could not parse minutes value as number.")?;
        let seconds = group_to_value("seconds")
            .with_context(|| "Could not parse seconds value as number.")?;

        let time =
            Time::new::<hour>(hours) + Time::new::<minute>(minutes) + Time::new::<second>(seconds);

        return Ok(Run { distance, time });
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
    let seconds = t;

    let h = hours.get::<hour>() as i32;
    let m = minutes.get::<minute>() as i32;
    let s = seconds.get::<second>();

    if h > 0 {
        format!("{} h {} min {:.3} s", h, m, s)
    } else {
        if m > 0 {
            format!("{} min {:.3} s", m, s)
        } else {
            format!("{:.3} s", s)
        }
    }
}

fn main() -> anyhow::Result<()> {
    let options = CommandLineOptions::from_args();
    let run = Run::from_options(&options)
        .with_context(|| "Could not understand the passed arguments.")?;
    println!(
        "Today, you ran {} in {}.",
        if options.use_miles {
            format!("{:.3} {}", run.distance.get::<mile>(), mile::abbreviation()).bold()
        } else {
            format!(
                "{:.3} {}",
                run.distance.get::<kilometer>(),
                kilometer::abbreviation()
            )
            .bold()
        },
        display_time(&run.time).bold()
    );
    println!(
        "{}",
        if options.use_miles {
            format!(
                "Your average velocity was {:.3} {}.",
                run.average_velocity().get::<mile_per_hour>(),
                mile_per_hour::abbreviation()
            )
            .bold()
        } else {
            format!(
                "Your average velocity was {:.3} {}.",
                run.average_velocity().get::<kilometer_per_hour>(),
                kilometer_per_hour::abbreviation()
            )
            .bold()
        }
    );

    if options.verbose {
        let distances = if options.use_miles {
            [
                NamedLength {
                    name: String::from("100 yd"),
                    distance: Length::new::<yard>(100.0),
                },
                NamedLength {
                    name: String::from("1/8 mi"),
                    distance: Length::new::<mile>(0.125),
                },
                NamedLength {
                    name: String::from("1/4 mi"),
                    distance: Length::new::<mile>(0.25),
                },
                NamedLength {
                    name: String::from("1 mi"),
                    distance: Length::new::<mile>(1.0),
                },
                NamedLength {
                    name: String::from("half marathon"),
                    distance: Length::new::<kilometer>(21.0975),
                },
                NamedLength {
                    name: String::from("marathon"),
                    distance: Length::new::<kilometer>(42.195),
                },
            ]
        } else {
            [
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
            ]
        };

        let mut dist_table = Table::new();
        dist_table.set_format(*format::consts::FORMAT_CLEAN);
        for distance in &distances {
            dist_table.add_row(row![
                r -> distance.name,
                format!(
                    "{}",
                    display_time(&run.time_for_distance(&distance.distance))
                )
            ]);
        }

        println!(
            "{}",
            "\nThis is how long you would have needed for other distances:".bold()
        );
        dist_table.printstd();

        let velocities = &[
            NamedVelocity {
                name: String::from("Ashprihanal Aalto\'s 3100 mi (longest ultra marathon) WR"),
                velocity: Velocity::new::<kilometer_per_hour>(5.1480),
            },
            NamedVelocity {
                name: String::from("Yohann Diniz\' 50 km race walk WR"),
                velocity: Velocity::new::<kilometer_per_hour>(14.1143),
            },
            NamedVelocity {
                name: String::from("Eliud Kipchoge\'s inofficial marathon WR"),
                velocity: Velocity::new::<kilometer_per_hour>(21.1563),
            },
            NamedVelocity {
                name: String::from("Kenenisa Bekele\'s 10000 m WR"),
                velocity: Velocity::new::<kilometer_per_hour>(22.8205),
            },
            NamedVelocity {
                name: String::from("Usain Bolt\'s 100 m WR"),
                velocity: Velocity::new::<kilometer_per_hour>(37.5783),
            },
            NamedVelocity {
                name: String::from("Cheetah Sarah\'s 100 m animal WR"),
                velocity: Velocity::new::<kilometer_per_hour>(60.5042),
            },
        ];

        let mut vel_table = Table::new();
        vel_table.set_format(*format::consts::FORMAT_CLEAN);
        for velocity in velocities {
            vel_table.add_row(row![
                r -> format!(
                    "{:.3} times",
                    (run.average_velocity() / velocity.velocity).get::<ratio>()
                ),
                velocity.name
            ]);
        }

        println!(
            "{}",
            "\nYour average velocity compared to those of other performances:".bold()
        );
        vel_table.printstd();
    }
    Ok(())
}

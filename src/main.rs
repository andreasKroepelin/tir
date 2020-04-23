use regex::Regex;
use structopt::StructOpt;
use uom::si::f64::{Length, Time, Velocity};
use uom::si::length::{kilometer, meter};
use uom::si::time::{hour, minute, second};
use uom::si::velocity::{kilometer_per_hour};
use uom::fmt::DisplayStyle;

#[derive(StructOpt, Debug)]
struct Options {
    distance: String,
    time: String,
}

#[derive(Debug)]
struct Run {
    distance: Length,
    time: Time,
}

impl Run {
    fn from_options(options: &Options) -> Option<Self> {
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
        let hours = time_caps
            .name("hours")
            .map_or(Ok(0.0), |m| m.as_str().parse())
            .ok()?;
        let minutes = time_caps
            .name("minutes")
            .map_or(Ok(0.0), |m| m.as_str().parse())
            .ok()?;
        let seconds = time_caps
            .name("seconds")
            .map_or(Ok(0.0), |m| m.as_str().parse())
            .ok()?;

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

fn main() {
    let options = Options::from_args();
    if let Some(run) = dbg!(Run::from_options(&dbg!(options))) {
        let vel_format = Velocity::format_args(kilometer_per_hour, DisplayStyle::Abbreviation);

        println!("Your average velocity was {}", vel_format.with(run.average_velocity()));

    } else {
        println!("Could not parse the given distance and time.");
    }
}

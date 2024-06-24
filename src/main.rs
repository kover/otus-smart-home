trait Describable {
    fn describe(&self);
}

struct Outlet {
    is_turned_on: bool,
    power_consumption: u32,
}

impl Outlet {
    fn new() -> Outlet {
        Outlet {
            is_turned_on: false,
            power_consumption: 0,
        }
    }
    fn toggle(&mut self) {
        self.is_turned_on = !self.is_turned_on;
    }

    fn plug_consumer(&mut self, power_consumption: u32) {
        self.power_consumption = power_consumption;
    }

    fn unplug_consumer(&mut self) {
        self.power_consumption = 0;
    }

    fn report_consumption(&self) -> u32 {
        if !self.is_turned_on {
            0
        } else {
            self.power_consumption
        }
    }
}

impl Describable for Outlet {
    fn describe(&self) {
        println!("Power Outlet");
    }
}

struct Thermometer {
    temperature: i32,
}

impl Thermometer {
    fn new(initial_temperature: i32) -> Thermometer {
        Thermometer {
            temperature: initial_temperature,
        }
    }
    fn get_current_temperature(&self) -> i32 {
        self.temperature
    }
}

impl Describable for Thermometer {
    fn describe(&self) {
        println!("Thermometer");
    }
}

fn main() {
    let mut outlet = Outlet::new();
    outlet.describe();
    println!("Is Turned On: {}", outlet.is_turned_on);

    outlet.plug_consumer(42);
    println!("Power consumption: {}", outlet.report_consumption());

    outlet.toggle();
    println!("Is Turned On: {}", outlet.is_turned_on);
    println!("Power consumption: {}", outlet.report_consumption());

    outlet.unplug_consumer();
    println!("Power consumption: {}", outlet.report_consumption());

    let thermometer = Thermometer::new(42);
    thermometer.describe();
    println!(
        "The current temperature is: {}",
        thermometer.get_current_temperature()
    )
}

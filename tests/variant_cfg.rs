use enum_dispatch::enum_dispatch;

#[enum_dispatch(Application)]
enum App {
    Menu,
    #[cfg(test)]
    DebugClock,
    #[cfg(not(test))]
    Clock,
}

#[enum_dispatch]
trait Application {}

struct Menu;
#[cfg(test)]
struct DebugClock;
#[cfg(not(test))]
struct Clock;

impl Application for Menu {}

#[cfg(test)]
impl Application for DebugClock {}

#[cfg(not(test))]
impl Application for Clock {}

fn main() {
    use std::convert::TryInto;

    let menu = Menu;
    let app: App = menu.into();
    let _menu: Menu = app.try_into().unwrap();

    #[cfg(test)]
    {
        let clock = DebugClock;
        let app: App = clock.into();
        let _clock: DebugClock = app.try_into().unwrap();
    }

    #[cfg(not(test))]
    {
        let clock = Clock;
        let app: App = clock.into();
        let _clock: Clock = app.try_into().unwrap();
    }
}

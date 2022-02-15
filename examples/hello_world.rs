use bevy::{prelude::App, DefaultPlugins};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(hello_world::HelloPlugin)
        .run();
}

mod hello_world {
    use bevy::prelude::{App, Commands, Component, Plugin, Query, Res, ResMut, Time, Timer, With};

    #[derive(Component)]
    struct Person;

    #[derive(Component)]
    struct Name(String);

    impl Name {
        fn insert_as_person(self, commands: &mut Commands) {
            commands.spawn().insert(Person).insert(self);
        }
    }

    struct GreetTimer(Timer);

    pub struct HelloPlugin;

    impl HelloPlugin {
        fn add_people(mut commands: Commands) {
            Name("Elaina Proctor".into()).insert_as_person(&mut commands);
            Name("Renzo Hume".into()).insert_as_person(&mut commands);
            Name("Zayna Nieves".into()).insert_as_person(&mut commands);
        }

        fn greet_people(
            time: Res<Time>,
            mut timer: ResMut<GreetTimer>,
            query: Query<&Name, With<Person>>,
        ) {
            if timer.0.tick(time.delta()).just_finished() {
                for name in query.iter() {
                    println!("hello {}!", name.0)
                }
            }
        }
    }

    impl Plugin for HelloPlugin {
        fn build(&self, app: &mut App) {
            app.insert_resource(GreetTimer(Timer::from_seconds(2.0, true)))
                .add_startup_system(Self::add_people)
                .add_system(Self::greet_people);
        }
    }
}

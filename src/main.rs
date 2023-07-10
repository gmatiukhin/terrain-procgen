use bevy::tasks::{AsyncComputeTaskPool, TaskPool};
use bevy::{app::AppExit, prelude::*, winit::WinitSettings};
use easy_repl::{command, CommandStatus, Repl};
use std::sync::mpsc::{channel, Receiver};
use std::sync::Mutex;
use terrain_procgen::generation::*;

fn main() {
    let (gen_event_sender, gen_event_receiver) = channel::<GenerateTerrainEvent>();
    let (exit_event_sender, exit_event_receiver) = channel::<AppExit>();

    AsyncComputeTaskPool::init(TaskPool::new);
    let taskpool = AsyncComputeTaskPool::get();
    taskpool
        .spawn(async move {
            println!("hello from task");
            let mut repl = Repl::builder()
                .add(
                    "send",
                    command!("Send config to terrain generator",() => || {
                        gen_event_sender.send(GenerateTerrainEvent).unwrap();
                        Ok(CommandStatus::Done)
                    }),
                )
                .build()
                .expect("Failed to create REPL");
            repl.run().expect("Critical REPL error");
            println!("exited repl");
            exit_event_sender.send(AppExit).unwrap();
        })
        .detach();

    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(WinitSettings::desktop_app())
        .init_resource::<TerrainGeneratorConfig>()
        .add_event::<GenerateTerrainEvent>()
        .add_event::<AppExit>()
        .add_event_channel(gen_event_receiver)
        .add_event_channel(exit_event_receiver)
        .add_systems(Update, bevy::window::close_on_esc)
        .run();
}

// This introduces event channels, on one side of which is mpsc::Sender<T>, and on another
// side is bevy's EventReader<T>, and it automatically bridges between the two.
#[derive(Resource, Deref, DerefMut)]
struct ChannelReceiver<T>(Mutex<Receiver<T>>);

pub trait AppExtensions {
    // Allows you to create bevy events using mpsc Sender
    fn add_event_channel<T: Event>(&mut self, receiver: Receiver<T>) -> &mut Self;
}

impl AppExtensions for App {
    fn add_event_channel<T: Event>(&mut self, receiver: Receiver<T>) -> &mut Self {
        assert!(
            !self.world.contains_resource::<ChannelReceiver<T>>(),
            "this event channel is already initialized",
        );

        self.add_event::<T>();
        self.add_systems(
            Update,
            channel_to_event::<T>.after(Events::<T>::update_system),
        );
        self.insert_resource(ChannelReceiver(Mutex::new(receiver)));
        self
    }
}

fn channel_to_event<T: 'static + Send + Sync + Event>(
    receiver: Res<ChannelReceiver<T>>,
    mut writer: EventWriter<T>,
) {
    // this should be the only system working with the receiver,
    // thus we always expect to get this lock
    let events = receiver.lock().expect("unable to acquire mutex lock");

    writer.send_batch(events.try_iter());
}

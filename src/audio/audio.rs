use std::iter::Cycle;
use std::vec::IntoIter;

use amethyst::assets::{AssetStorage, Loader, Handle};
use amethyst::audio::output::Output;
use amethyst::audio::{AudioSink, Source, SourceHandle};
use amethyst::ecs::prelude::World;

pub struct Sounds {
    pub small_rock_sfx: SourceHandle,
    pub large_rock_sfx: SourceHandle,
    pub wrench_sfx: SourceHandle,
}

/// Loads an ogg audio track.
fn load_audio_track(loader: &Loader, world: &World, file: &str) -> SourceHandle {
    use amethyst::audio::OggFormat;

    loader.load(file, OggFormat, (), &world.read_resource())
}

/// Initialise audio in the world. This includes the background track and the
/// sound effects.
pub fn initialise_audio(world: &mut World) {

    let sound_effects = {
        let loader = world.read_resource::<Loader>();

        //let mut sink = world.write_resource::<AudioSink>();
        //sink.set_volume(0.25); // Music is a bit loud, reduce the volume.

        let sound = Sounds {
            small_rock_sfx: load_audio_track(&loader, &world, "audio/small_rock.ogg"),
            large_rock_sfx: load_audio_track(&loader, &world, "audio/large_rock.ogg"),
            wrench_sfx: load_audio_track(&loader, &world, "audio/wrench.ogg"),
        };

        sound
    };

    // Add sound effects to the world. We have to do this in another scope because
    // world won't let us insert new resources as long as `Loader` is borrowed.
    world.add_resource(sound_effects);
}

pub fn play_sfx(sound: &Handle<Source>, storage: &AssetStorage<Source>, output: Option<&Output>) {
    if let Some(ref output) = output.as_ref() {
        if let Some(sound) = storage.get(sound) {
            output.play_once(sound, 1.0);
        }
    }
}
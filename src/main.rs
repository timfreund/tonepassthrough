extern crate jack;

use std::{thread, time};

fn main() {
    let(jack_client, _status) = jack::Client::new("tone_passthrough", jack::ClientOptions::NO_START_SERVER).unwrap();
    let audio_in = jack_client.register_port("audio_in", jack::AudioIn::default()).unwrap();
    let mut audio_out = jack_client.register_port("audio_out", jack::AudioOut::default()).unwrap();

    let jack_callback = jack::ClosureProcessHandler::new(
        move |_: &jack::Client, ps: &jack::ProcessScope| -> jack::Control {
            let ao = audio_out.as_mut_slice(ps);
            let ai = audio_in.as_slice(ps);
            let amplitude_multiplier = 0.5 as f32;

            for (iv, ov) in ai.iter().zip(ao){
                *ov = iv * amplitude_multiplier;
            }
            
            jack::Control::Continue
        },
    );

    let _ajc = jack_client.activate_async((), jack_callback).unwrap();

    let sleep_period = time::Duration::from_millis(500);
    loop {
        thread::sleep(sleep_period);
    }
}

#![feature(duration)]
#![feature(duration_span)]
#![feature(collections_drain)]

extern crate image;
extern crate rand;
extern crate docopt;
extern crate rustc_serialize as rustc_serialize;
extern crate threadpool;
extern crate num_cpus;
extern crate tray_rust;

use std::vec::Vec;
use std::sync::{Arc};
use std::sync::mpsc::{self, Sender, Receiver};
use std::path::Path;
use std::time::Duration;

use threadpool::ThreadPool;
use rand::StdRng;
use docopt::Docopt;

use tray_rust::film;
use tray_rust::geometry::Geometry;
use tray_rust::sampler::{self, Sampler};
use tray_rust::scene;
use tray_rust::integrator::Integrator;

static WIDTH: usize = 800;
static HEIGHT: usize = 600;
static USAGE: &'static str = "
Usage: tray_rust [options]

Options:
  -o <file>     Specify the output file to save the image. Supported formats are
                PNG, JPG and PPM. Default is 'out.png'.
  -n <number>   Specify the number of threads to use for rendering. Defaults to the number of cores
                on the system.
  -h, --help    Show this message.
";

#[derive(RustcDecodable, Debug)]
struct Args {
    flag_o: Option<String>,
    flag_n: Option<usize>,
}

// TODO: AREA LIGHTS NOTES:
// - They should be intersectable so they're like wierd instances.
// - What if point lights were also instances that just always returned None
//   for intersection tests?
// - Then each thread would build a list of &Instance that refer to the emissive
//   instances in the scene and we'd pass this around for the various lighting calculations.
//   It's a bit ugly but maybe cleaner than what was done previously in tray
//   It breaks the awkward handling of mixing point and area lights with each other which
//   is a plus.

/// Threads are each sent a sender end of the channel that is
/// read from by the render target thread which then saves the
/// values recieved to the render target
fn thread_work(tx: Sender<Vec<(f32, f32, film::Colorf)>>, queue: Arc<sampler::BlockQueue>,
               scene: Arc<scene::Scene>) {
    let mut sampler = sampler::LowDiscrepancy::new(queue.block_dim(), 4);
    let mut sample_pos = Vec::with_capacity(sampler.max_spp());
    let mut rng = match StdRng::new() {
        Ok(r) => r,
        Err(e) => { println!("Failed to get StdRng, {}", e); return }
    };
    // Grab a block from the queue and start working on it, submitting samples
    // to the render target thread after each pixel
    for b in queue.iter() {
        sampler.select_block(b);
        while sampler.has_samples() {
            // Get samples for a pixel and render them
            sampler.get_samples(&mut sample_pos, &mut rng);
            let mut samples = Vec::with_capacity(sampler.max_spp());
            for s in sample_pos.iter() {
                let mut ray = scene.camera.generate_ray(s);
                if let Some(hit) = scene.intersect(&mut ray) {
                    let c = scene.integrator.illumination(&*scene, &ray, &hit, &mut sampler, &mut rng).clamp();
                    samples.push((s.0, s.1, c));
                }
            }
            if let Err(e) = tx.send(samples) {
                println!("Worker thread exiting with send error {:?}", e);
                return;
            }
        }
    }
}

/// Spawn `n` worker threads to render the scene in parallel. Returns the receive end
/// of the channel where the threads will write their samples so that the receiver
/// can write these samples to the render target
fn spawn_workers(pool: &ThreadPool, n: usize, scene: Arc<scene::Scene>)
                 -> Receiver<Vec<(f32, f32, film::Colorf)>> {
    let (tx, rx) = mpsc::channel();
    let block_queue = Arc::new(sampler::BlockQueue::new((WIDTH as u32, HEIGHT as u32), (8, 8)));
    for _ in 0..n {
        let q = block_queue.clone();
        let t = tx.clone();
        let s = scene.clone();
        pool.execute(move || {
            thread_work(t, q, s);
        });
    }
    rx
}

/// Render the scene in parallel to the render target
fn render_parallel(rt: &mut film::RenderTarget, n: usize){
    let scene = Arc::new(scene::Scene::new(WIDTH, HEIGHT));
    let pool = ThreadPool::new(n);
    let rx = spawn_workers(&pool, n, scene);
    for mut v in rx.iter() {
        for s in v.drain(..) {
            rt.write(s.0, s.1, &s.2);
        }
    }
}

fn main() {
    let args: Args = Docopt::new(USAGE).and_then(|d| d.decode()).unwrap_or_else(|e| e.exit());

    let mut rt = film::RenderTarget::new(WIDTH, HEIGHT);
    let n = match args.flag_n {
        Some(n) => n,
        None => num_cpus::get(),
    };
    println!("Rendering using {} threads", n);
    let d = Duration::span(|| render_parallel(&mut rt, n));
    println!("Rendering took {}", d);
    let img = rt.get_render();
    let out_file = match args.flag_o {
        Some(f) => f,
        None => "out.png".to_string(),
    };
    match image::save_buffer(&Path::new(&out_file), &img[..], WIDTH as u32, HEIGHT as u32, image::RGB(8)) {
        Ok(_) => {},
        Err(e) => println!("Error saving image, {}", e),
    };
}


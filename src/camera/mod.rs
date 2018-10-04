use std::thread;
use std::time::Duration;
use std::collections::VecDeque;
use std::sync::{Arc, Mutex};
use gphoto as gp;

pub enum CameraAction {
    Sleep { millis: u64 }
}

pub struct CameraHandler {
    pub action_queue: Arc<Mutex<VecDeque<CameraAction>>>
}

impl CameraHandler {
    pub fn new() -> CameraHandler {
        let aq = Arc::new(Mutex::new(VecDeque::<CameraAction>::new()));
        let aq_thread = aq.clone();
        thread::spawn(|| run_camera_queue(aq_thread));

        CameraHandler {
            action_queue: aq
        }
    }

    pub fn sleep_from_secs(&self, seconds: u64) {
        let action = CameraAction::Sleep {millis: seconds * 1000};
        self.action_queue.lock().unwrap().push_back(action);
    }
}

fn run_camera_queue(aq: Arc<Mutex<VecDeque<CameraAction>>>) {
    info!("Started CameraHandler worker thread");

    loop {
        // Get action from queue
        let action = {
            let mut queue = aq.lock().unwrap();
            if !queue.is_empty() {
                queue.pop_front().unwrap()
            } else {
                CameraAction::Sleep { millis: 1000 }
            }
        };

        match action {
            CameraAction::Sleep { millis } => {
                info!("CameraAction: Sleeping for {}ms", millis);
                thread::sleep(Duration::from_millis(millis));
            }
        }
    }
}

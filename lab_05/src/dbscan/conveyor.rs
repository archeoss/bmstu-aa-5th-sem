use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;

use super::task::DBSCANTaskResult;
pub use super::task::{DBSCANTask, ThreeTask};

pub struct Conveyor3
{
    output: Receiver<DBSCANTask>,
}

impl Conveyor3
{
    pub fn new(queue: Vec<DBSCANTask>) -> Self
    {
        let (task_input, part1_receiver): (Sender<DBSCANTask>, Receiver<DBSCANTask>) = channel();
        let (part1_sender, part2_receiver): (Sender<DBSCANTask>, Receiver<DBSCANTask>) = channel();
        let (part2_sender, part3_receiver): (Sender<DBSCANTask>, Receiver<DBSCANTask>) = channel();
        let (part3_sender, task_output): (Sender<DBSCANTask>, Receiver<DBSCANTask>) = channel();

        thread::spawn(move || {
            for mut task in part1_receiver.iter() {
                task.part_a();
                part1_sender.send(task).expect("Send task to part2");
            }
        });

        thread::spawn(move || {
            for mut task in part2_receiver.iter() {
                task.part_b();
                part2_sender.send(task).expect("Send task to part3");
            }
        });

        thread::spawn(move || {
            for mut task in part3_receiver.iter() {
                task.part_c();
                part3_sender.send(task).expect("Send task to output");
            }
        });

        for task in queue {
            task_input.send(task).expect("Send task to conveyor");
        }

        Self {
            output: task_output,
        }
    }

    pub fn recv(&self) -> Option<DBSCANTaskResult>
    {
        self.output.recv().map(|res| res.result()).ok()
    }
}

use std::sync::mpsc::{self, Receiver, Sender, SyncSender};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum NativeEventOperation {
    /// Do not pass the generated event to the next program.
    Block,

    /// Pass the generated event to the next program.
    Dispatch,
}

impl Default for &NativeEventOperation {
    fn default() -> Self {
        &NativeEventOperation::Dispatch
    }
}

impl Default for NativeEventOperation {
    fn default() -> Self {
        *<&NativeEventOperation>::default()
    }
}

/* 
/// Decide whether to notify other programs of generated events.
#[derive(Debug)]
pub struct NativeEventHandler {
    tx: Option<Sender<NativeEventOperation>>,
}

impl NativeEventHandler {
    fn new(tx: Sender<NativeEventOperation>) -> Self {
        Self { tx: Some(tx) }
    }

    /// Decides whether or not to notify by argument.
    pub fn handle(mut self, operation: NativeEventOperation) {
        self.tx.take().unwrap().send(operation).unwrap();
    }

    // Notifies an event.
    pub fn dispatch(self) {
        self.handle(NativeEventOperation::Dispatch);
    }

    // Does not notify an event.
    pub fn block(self) {
        self.handle(NativeEventOperation::Block);
    }
}

impl Drop for NativeEventHandler {
    fn drop(&mut self) {
        if let Some(tx) = self.tx.take() {
            tx.send(NativeEventOperation::default()).unwrap();
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct EventSender {
    tx: SyncSender<String>,
}

impl EventSender {
    pub(crate) fn new(tx: SyncSender<String>) -> Self {
        Self { tx }
    }

    pub(crate) fn send(&self, event: String) -> NativeEventOperation {
        let (tx, rx) = mpsc::channel();
        let sent_data = (event, NativeEventHandler::new(tx));

        self.tx.send(sent_data).unwrap();
        rx.recv().unwrap()
    }
}

pub type EventReceiver = Receiver<String>;

pub(crate) fn channel() -> (EventSender, EventReceiver) {
    const BOUND: usize = 1;
    let (tx, rx) = mpsc::sync_channel(BOUND);
    (EventSender::new(tx), rx)
}

*/
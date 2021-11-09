use netfilter_queue::queue::*;
use netfilter_queue::log_entry::*;
use netfilter_queue::nfq::*;

fn handle_verdict(qh: NfQueueData, nfad: NfLogData){
    println!("handle_verdict");
    let entry = LogEntry::new(qh, nfad);
    entry.set_verdict(Verdict::Accept);
}

fn main() {
    let mut q = Queue::new(());
    q.unbind(ProtocolFamily::IPv4);
    q.bind(ProtocolFamily::IPv4);
    q.create(0, handle_verdict);
    q.set_mode(CopyMode::Packet, 0xffff);
    q.run();
}

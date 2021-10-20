use netfilter_queue::{nfq_create_queue, nfq_q_handle, nfq_handle, nfq_callback, nfq_data, nfgenmsg, nfq_open, nfq_set_verdict, nfq_get_msg_packet_hdr, nfqnl_msg_packet_hdr};

// unsafe extern "C" fn queue_callback(qh: nfq_q_handle, nfmsg: nfgenmsg, nfa: nfq_data, data_len: u32) -> ::std::os::raw::c_int {

//     let id: u32;

//     let header = nfq_get_msg_packet_hdr(nfa);
// 	ph = nfq_get_msg_packet_hdr(nfa);	
// 	// id = ntohl(ph->packet_id);
//     nfq_set_verdict(qh, nfmsg, nfa)
// }

struct QueueCallback {}
impl QueueCallback {
    const callback: nfq_callback = Some(QueueCallback::callback_handler);

    unsafe extern "C" fn callback_handler(
        gh: *mut nfq_q_handle,
        nfmsg: *mut nfgenmsg,
        nfad: *mut nfq_data,
        data: *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int {
        1
    }
}

fn main() {
    unsafe {
        let deref: *mut nfq_handle = nfq_open();
        let data: *mut ::std::os::raw::c_void = std::ptr::null_mut();
        let queue_handle: *mut nfq_q_handle = nfq_create_queue(deref, 0, QueueCallback::callback, data);
    }
}

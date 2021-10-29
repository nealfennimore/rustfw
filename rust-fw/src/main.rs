use libc;
use netfilter_queue::{nfq_create_queue, nfq_open, nfq_close, nfq_set_mode, nfq_unbind_pf, nfq_bind_pf, nfq_handle_packet, nfq_fd, nfq_set_verdict2, nfq_get_msg_packet_hdr, NfMsgPacketHdr};

const NF_ACCEPT: u32 = 0x0001;
const NFQNL_COPY_PACKER: u8 = 0x02;

extern "C" fn callback(qh: *const std::ffi::c_void, msg: *const std::ffi::c_void, nfq_data: *const std::ffi::c_void, data: *const std::ffi::c_void) {
    let msg_hdr = unsafe { nfq_get_msg_packet_hdr(nfq_data) as *const NfMsgPacketHdr };
    assert!(!msg_hdr.is_null());
    let id = u32::from_be(unsafe { (*msg_hdr).packet_id });
    println!("Received {}", id);
    unsafe { 
        nfq_set_verdict2(qh, id, NF_ACCEPT, 0, 0, std::ptr::null_mut())
    };  
}

fn main() {
    let qh = unsafe { nfq_open() };
    assert!(!qh.is_null());

    unsafe { nfq_unbind_pf(qh, libc::AF_INET) };

    let rc = unsafe { nfq_bind_pf(qh, libc::AF_INET) };
    assert!(rc == 0, "{} = rc", rc);

    println!("RC {}", rc);

    let self_ptr = std::ptr::null_mut();
    let queue = unsafe { nfq_create_queue(qh, 0, callback, self_ptr) };

    assert!(!queue.is_null());


    let mode = unsafe { nfq_set_mode(queue, NFQNL_COPY_PACKER, 0xffff) };
    assert!(mode == 0, "{}", mode);

    let fd = unsafe { nfq_fd(qh) };
    let mut buf: [u8; 65536] = [0; 65536];
    let buf_ptr = buf.as_mut_ptr() as *mut libc::c_void;
    let buf_len = buf.len() as libc::size_t;

    println!("Starting loop");

    loop {
        println!("UH");
        let rc = unsafe { libc::recv(fd, buf_ptr, buf_len, 0) };
        println!("{}", rc);
        if rc < 0 {
            panic!("error in recv()");
        };

        let rv = unsafe { nfq_handle_packet(qh, buf_ptr, rc as libc::c_int) };
        if rv < 0 {
            println!("error in nfq_handle_packet()");
        }; // not critical
    };
    // unsafe { nf_close(qh) };
}

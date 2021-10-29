use libc;

type NfqueueData = *const libc::c_void;

/// Opaque struct `Message`: abstracts NFLOG data representing a packet data and metadata
pub struct Message {
    qqh: *const libc::c_void,
    nfad: NfqueueData,
    id: u32,
    l3_proto: u16,
}


type NfqueueHandle = *const libc::c_void;
type NfqueueQueueHandle = *const libc::c_void;

/// Prototype for the callback function, triggered when a packet is received
pub type NfqueueCallback = fn(&Message) -> ();

type NfqueueCCallback = extern "C" fn(
    *const libc::c_void,
    *const libc::c_void,
    *const libc::c_void,
    *const libc::c_void,
);

#[repr(C)]
pub struct NfMsgPacketHdr {
    /// unique ID of the packet
    pub packet_id: u32,
    /// hw protocol (network order)
    pub hw_protocol: u16,
    /// Netfilter hook
    pub hook: u8,
}

#[link(name = "netfilter_queue")]
extern "C" {
    // library setup
    pub fn nfq_open() -> NfqueueHandle;
    pub fn nfq_close(qh: NfqueueHandle);
    pub fn nfq_bind_pf(qh: NfqueueHandle, pf: libc::c_int) -> libc::c_int;
    pub fn nfq_unbind_pf(qh: NfqueueHandle, pf: libc::c_int) -> libc::c_int;

    // queue handling
    pub fn nfq_fd(h: NfqueueHandle) -> libc::c_int;
    pub fn nfq_create_queue(
        qh: NfqueueHandle,
        num: u16,
        cb: NfqueueCCallback,
        data: *mut libc::c_void,
    ) -> NfqueueQueueHandle;
    pub fn nfq_destroy_queue(qh: NfqueueHandle) -> libc::c_int;
    pub fn nfq_handle_packet(qh: NfqueueHandle, buf: *mut libc::c_void, rc: libc::c_int)
        -> libc::c_int;
    pub fn nfq_set_mode(gh: NfqueueQueueHandle, mode: u8, range: u32) -> libc::c_int;
    pub fn nfq_set_queuelen(gh: NfqueueQueueHandle, queuelen: u32) -> libc::c_int;
    pub fn nfq_set_verdict2(
        qqh: *const libc::c_void,
        id: u32,
        verdict: u32,
        mark: u32,
        data_len: u32,
        data: *const libc::c_uchar,
    );
    pub fn nfq_get_msg_packet_hdr(nfad: NfqueueData) -> *const libc::c_void;
}
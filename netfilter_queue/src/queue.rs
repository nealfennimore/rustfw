use libc;

use crate::nfq::*;

type QueueCallback = fn(
    qh: NfQueueHandle,
    nfad: NfLogData,
);

pub struct Queue<T> {
    h: NfHandle,
    qh: Option<NfQueueHandle>,
    data: T,
    cb: Option<QueueCallback>,
}

impl<T> Drop for Queue<T> {
    fn drop(&mut self) {
        self.close();
    }
}

impl<T> Queue<T> {
    pub fn new(data: T) -> Self {
        let h = unsafe { nfq_open() };
        assert!(! h.is_null(), "Could not open handler");

        Self {
            h,
            qh: None,
            data,
            cb: None
        }
    }

    pub fn fd(&self) -> FileDescriptor {
        unsafe { nfq_fd( self.h ) }
    }

    pub fn close(&self) {
        unsafe { nfq_close( self.h ) };
    }

    pub fn bind(&self, pf: ProtocolFamily) {
        let result = unsafe { nfq_bind_pf( self.h, pf ) };
        assert!(result >= 0, "Failed to bind to queue handler");
    }

    pub fn unbind(&self, pf: ProtocolFamily) {
        let result = unsafe { nfq_unbind_pf( self.h, pf ) };
        assert!(result >= 0, "Failed to unbind");
    }

    pub fn create(&mut self, num: QueueNum, cb: QueueCallback) {
        let queue_ptr = &*self as *const Queue<T> as *mut libc::c_void;
        let qh = unsafe { nfq_create_queue( self.h, num, nfq_callback::<T>, queue_ptr ) };
        assert!(! qh.is_null(), "Could not create queue");
        self.qh = Some(qh);
        self.cb = Some(cb);
    }

    pub fn destroy(&mut self) {
        let qh = self.qh.expect("No queue handler to destroy");
        unsafe { nfq_destroy_queue( qh ) };
        self.qh = None;
    }

    pub fn set_mode(&mut self, mode: CopyMode, range: u32) {
        let qh = self.qh.expect("No queue handler");
        let result = unsafe { nfq_set_mode(qh, mode, range) };
        assert!(result >= 0, "Could not set mode");
    }

    pub fn run(&self){
        assert!(self.qh.is_some());
        assert!(self.cb.is_some());

        const SIZE: usize = u32::MAX as usize;
        let fd = self.fd();
        let mut buf: Vec<u8> = vec![0; SIZE];
        let buf_ptr = buf.as_mut_ptr() as *mut libc::c_void;
        let buf_len = buf.len() as libc::size_t;

        loop {
            let rc = unsafe { libc::recv(fd, buf_ptr, buf_len, libc::MSG_DONTWAIT) };
            if rc < 0 {
                continue;
            }
            // assert!(rc >= 0, "Error while receiving: {}", rc);

            let rv = unsafe { nfq_handle_packet(self.h, buf_ptr, rc as libc::c_int) };
            if rv < 0 {
                println!("error in nfq_handle_packet()");
            }; // not critical
        }
    }
}

extern "C" fn nfq_callback<T>(
    qh: NfQueueHandle, 
    _nfmsg: NfCallbackGenMsg, 
    nfad: NfLogData, 
    data: NfData
) {
    let queue_ptr: *mut Queue<T> = data as *mut Queue<T>;
    let q = &mut unsafe { &mut *queue_ptr };

    match q.cb {
        None => panic!("No callback given"),
        Some(callback) => {
            callback(qh, nfad)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn dummy_callback(_qh: NfQueueHandle, _nfad: NfLogData){

    }

    #[test]
    #[should_panic]
    fn it_panics_when_cannot_bind() {
        let mut q = Queue::new(());
        q.bind(ProtocolFamily::IPv4);
    }
    #[test]
    #[should_panic]
    fn it_panics_when_cannot_unbind() {
        let mut q = Queue::new(());
        q.unbind(ProtocolFamily::IPv4);
    }
    #[test]
    #[should_panic]
    fn it_panics_when_no_queue_handler() {
        let mut q = Queue::new(());
        q.destroy();
    }
    #[test]
    fn it_should_get_a_file_descriptor() {
        let q = Queue::new(());
        println!("{}", q.fd());
        assert!(q.fd() > 0);
    }
}
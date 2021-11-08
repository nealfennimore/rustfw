use libc;

pub mod nfq;
use nfq::*;

struct Queue<T> {
    h: NfHandle,
    qh: Option<NfQueueHandle>,
    num: QueueNum,
    data: T
}

impl<T> Drop for Queue<T> {
    fn drop(&mut self) {
        self.close();
    }
}

impl<T> Queue<T> {
    pub fn new(data: T, num: QueueNum) -> Self {
        let h = unsafe { nfq_open() };
        assert!(! h.is_null(), "Could not open handler");

        Self {
            h,
            qh: None,
            num,
            data
        }
    }

    fn fd(&self) -> FileDescriptor {
        unsafe { nfq_fd( self.h ) }
    }

    fn close(&self) {
        unsafe { nfq_close( self.h ) };
    }

    fn bind(&self, pf: ProtocolFamily) {
        let result = unsafe { nfq_bind_pf( self.h, pf ) };
        assert!(result >= 0, "Failed to bind to queue handler");
    }

    fn unbind(&self, pf: ProtocolFamily) {
        let result = unsafe { nfq_unbind_pf( self.h, pf ) };
        assert!(result >= 0, "Failed to unbind");
    }

    fn create(&mut self, cb: NfQueueCallback) {
        let queue_ptr = &*self as *const Queue<T> as *mut libc::c_void;
        let qh = unsafe { nfq_create_queue( self.h, self.num, nfq_callback::<T>, queue_ptr ) };
        assert!(! qh.is_null(), "Could not create queue");
        self.qh = Some(qh);
    }

    fn destroy(&mut self) {
        let qh = self.qh.expect("No queue handler to destroy");
        unsafe { nfq_destroy_queue( qh ) };
        self.qh = None;
    }
}

extern "C" fn nfq_callback<T>(
    qh: NfQueueHandle, 
    nfmsg: NfCallbackGenMsg, 
    nfad: NfqCallbackData, 
    data: NfData
) {
    let queue_ptr: *mut Queue<T> = data as *mut Queue<T>;
    let q = &mut unsafe {  &mut *queue_ptr };

    match q.cb {
        None => panic!(""),
        Some(callback) => {
            callback()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn it_panies_when_no_queue_handler() {
        let q = Queue::new((), 0);
        q.create(do_it);
        q.destroy();
    }
}
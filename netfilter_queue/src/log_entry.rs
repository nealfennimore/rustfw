use crate::nfq::*;
pub struct LogEntry {
    qh: NfQueueHandle,
    nfad: NfLogData,
    pub id: u32,
    pub l3_proto: u16,
}

impl LogEntry {
    pub fn new(qh: NfQueueHandle, nfad: NfLogData) -> LogEntry {
        let msg_hdr = unsafe { nfq_get_msg_packet_hdr(nfad) as *const NfMsgPacketHdr };
        assert!(!msg_hdr.is_null());
        let id = u32::from_be(unsafe { (*msg_hdr).packet_id });
        let l3_proto = u16::from_be(unsafe { (*msg_hdr).hw_protocol });
        LogEntry {
            qh,
            nfad,
            id,
            l3_proto,
        }
    }

    pub fn set_verdict(&self, verdict: Verdict){
        assert!(!self.qh.is_null());
        unsafe { nfq_set_verdict2(self.qh, self.id, verdict, 0, 0, std::ptr::null_mut()) };
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn it_panics_when_no_valid_message() {
        let mut entry = LogEntry::new(std::ptr::null(),std::ptr::null());
    }
}
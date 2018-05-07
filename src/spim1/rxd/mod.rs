#[doc = "Data pointer."]
pub struct PTR {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Data pointer."]
pub mod ptr;
#[doc = "Maximum number of buffer bytes to receive."]
pub struct MAXCNT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Maximum number of buffer bytes to receive."]
pub mod maxcnt;
#[doc = "Number of bytes received in the last transaction."]
pub struct AMOUNT {
    register: ::vcell::VolatileCell<u32>,
}
#[doc = "Number of bytes received in the last transaction."]
pub mod amount;

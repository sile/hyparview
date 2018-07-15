/// TTL of a message.
///
/// It decreases by one each time the message is forwarded.
/// If the TTL of a message reaches zero,
/// the message will be handled by the node that keeps the message at the time.
/// So, a TTL can be regarded as the hop count of a message.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct TimeToLive(u8);
impl TimeToLive {
    /// Makes a new `TimeToLive` instance.
    pub fn new(ttl: u8) -> Self {
        TimeToLive(ttl)
    }

    /// Returns the value of the TTL.
    pub fn as_u8(self) -> u8 {
        self.0
    }

    /// Returns `true` if the TTL is expired, otherwise `false`.
    ///
    /// # Examples
    ///
    /// ```
    /// use hyparview::TimeToLive;
    ///
    /// let ttl = TimeToLive::new(10);
    /// assert!(!ttl.is_expired());
    ///
    /// let ttl = TimeToLive::new(0);
    /// assert!(ttl.is_expired());
    /// ```
    pub fn is_expired(self) -> bool {
        self.0 == 0
    }

    pub(crate) fn decrement(self) -> Self {
        TimeToLive(self.0.saturating_sub(1))
    }
}

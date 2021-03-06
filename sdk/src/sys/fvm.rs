#[link(wasm_import_module = "fvm")]
extern "C" {
    /* Message */

    /// Get's the message's "block ID". The message can then be manipulated using the standard IPLD
    /// APIs.
    ///
    /// TODO: Return the codec & size as well, or just rely on ipld_stat?
    /// TODO: Can we just say that ID 0 is always the params?
    pub fn message_params() -> u32;

    /// Returns the message's method number.
    pub fn message_method() -> u64;

    /// Returns ID address of the message sender.
    pub fn message_caller() -> u64;

    /// Returns ID address of the message receiver.
    pub fn message_receiver() -> u64;

    /* Control */

    /// Abort execution with the given code and message. The code is recorded in the receipt, the
    /// message is for debugging only.
    pub fn abort(code: u32, message: *const u8, message_len: u32) -> !;

    /// Revert any state-changes and return the IPLD block referenced by the passed block ID.
    pub fn revert(id: u32) -> !;

    /// Commit any state-changes and return the IPLD block referenced by the passed block ID.
    pub fn finish(id: u32) -> !;

    /* TODO Syscalls */

    // Ignored for now. These should all be pre-compiles, not syscalls.
    // Except verify consensus fault... Which needs to look back in history. Can we just kill that?
}

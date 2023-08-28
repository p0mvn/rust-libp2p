// Copyright 2022 Parity Technologies (UK) Ltd.
//
// Permission is hereby granted, free of charge, to any person obtaining a
// copy of this software and associated documentation files (the "Software"),
// to deal in the Software without restriction, including without limitation
// the rights to use, copy, modify, merge, publish, distribute, sublicense,
// and/or sell copies of the Software, and to permit persons to whom the
// Software is furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS
// OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
// FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
// DEALINGS IN THE SOFTWARE.

use super::poll_data_channel::{DataChannel, PollDataChannel};
use asynchronous_codec::Framed;
use libp2p_webrtc_utils::proto::Message;
use libp2p_webrtc_utils::stream::{MAX_DATA_LEN, MAX_MSG_LEN, VARINT_LEN};
use std::cell::RefCell;
use std::rc::Rc;

pub(crate) type FramedDc = Framed<PollDataChannel, quick_protobuf_codec::Codec<Message>>;
pub(crate) fn new(data_channel: Rc<RefCell<DataChannel>>) -> FramedDc {
    let mut inner = PollDataChannel::new(data_channel);
    inner.set_read_buf_capacity(MAX_MSG_LEN);

    let mut framed = Framed::new(
        inner,
        quick_protobuf_codec::Codec::new(MAX_MSG_LEN - VARINT_LEN),
    );
    // If not set, `Framed` buffers up to 16377 bytes of data before sending
    framed.set_send_high_water_mark(MAX_DATA_LEN);
    framed
}

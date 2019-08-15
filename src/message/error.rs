use std::{fmt, error};

#[derive(Debug)]
pub struct MessageError<I,M> {
    error_repr: ErrorRepr<I,M>
}

impl<I,M> error::Error for MessageError<I,M> 
where
    I: fmt::Debug,
    M: fmt::Debug
{
    // Rust 1.37-nightly has deprecated methods for the Error trait.
}

impl<I,M> fmt::Display for MessageError<I,M> 
where
    I: fmt::Debug,
    M: fmt::Debug
{  
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", &self.error_repr)
    }
}

impl<I,M> From<MessageErrorKind<I,M>> for MessageError<I,M> {
    fn from(error_kind: MessageErrorKind<I,M>) -> Self {
        Self {
            error_repr: ErrorRepr::from(error_kind)
        }
    }
}

#[derive(Debug)]
enum ErrorRepr<I,M> {
    Simple(MessageErrorKind<I,M>),
    // Custom(Box<CustomError<I,M>>)
}

impl<I,M> From<MessageErrorKind<I,M>> for ErrorRepr<I,M> {
    fn from(error_kind: MessageErrorKind<I,M>) -> Self {
        ErrorRepr::Simple(error_kind)
    }
}

/* #[derive(Debug)]
pub struct CustomError<I,M> {
    kind: MessageErrorKind<I,M>,
    error: Box<dyn error::Error+Send+Sync>
} */

#[derive(Debug)]
pub enum MessageErrorKind<I,M> {
    // LockTimedOut(AbortedMessage<I,M>),
    AbortedMessages(Vec<AbortedMessage<I,M>>)
}

/* impl<I,M> MessageErrorKind<I,M> {
    fn as_str(&self) -> &'static str {
        match self {
            MessageErrorKind::AbandonedMessages(_) => "Multiple Messages Abandoned",
           MessageErrorKind::LockTimedOut(_) => "Abandoned Message Transmission: Lock on Write Permissions Timed Out"
        }
    }
} */

#[derive(Debug)]
pub struct AbortedMessage<I,M> {
    sender_id: I,
    recipient_id: I,
    message_contents: M
}

impl<I,M> AbortedMessage<I,M> {
    pub fn new(sender_id: I, recipient_id: I, message_contents: M) -> Self {
        Self {sender_id, recipient_id, message_contents}
    }
}

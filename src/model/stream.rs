use super::user::ToStrUser;
use super::report::Report;

use std::task::{Context, Poll};
use std::collections::HashMap;

use pin_project_lite::pin_project;
use futures::ready;
use futures::stream::Stream;
use actix_web::{Responder, HttpResponse, web};
use actix_web::body::{BoxBody, MessageBody, BodySize};
use tokio_stream::wrappers;
use serde::Serialize;

#[derive(Serialize)]
#[serde(tag = "type", content = "payload")]
pub enum StreamPayload<'a> {
  Ready {
    user: ToStrUser<'a>,
    reports: Vec<&'a Report>,
    nicknames: HashMap<&'a String, &'a String>, 
  },
  ReadyAdmin {
    id: &'a String,
    users: Vec<ToStrUser<'a>>,
    reports: &'a Vec<Report>,
  }
}

pin_project! {
  pub struct InfallibleStream<S> {
    #[pin]
    stream: S,
  }
}

impl<S> InfallibleStream<S> {
  pub fn new(stream: S) -> Self {
    Self { stream }
  }
}

impl<S: Stream> Stream for InfallibleStream<S> {
  type Item = Result<S::Item, std::convert::Infallible>;

  fn poll_next(self: std::pin::Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
    Poll::Ready(ready!(self.project().stream.poll_next(cx)).map(Ok))
  }

  fn size_hint(&self) -> (usize, Option<usize>) {
    self.stream.size_hint()
  }
}

impl Responder for InfallibleStream<wrappers::ReceiverStream<web::Bytes>> {
  type Body = BoxBody;

  fn respond_to(self, _req: &actix_web::HttpRequest) -> HttpResponse<Self::Body> {
    HttpResponse::Ok()
      .content_type("text/plain")
      .body(self)
  }
}

impl MessageBody for InfallibleStream<wrappers::ReceiverStream<web::Bytes>> {
  type Error =  Box<dyn std::error::Error>;

  fn size(&self) -> BodySize {
    BodySize::Stream
  }

  fn poll_next(self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> std::task::Poll<Option<Result<web::Bytes, Self::Error>>> {
    let this = self.project();

    if let Poll::Ready(item) = this.stream.poll_next(cx) {
      return match item {
        Some(item) => Poll::Ready(Some(Ok(item))),
        None => Poll::Ready(None),
      }
    }

    Poll::Pending
  }
}


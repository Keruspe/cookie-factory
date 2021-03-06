use std::str;

use cookie_factory::*;

#[derive(Debug,Clone,PartialEq)]
pub struct Request<'a> {
  pub method: &'a str,
  pub uri: &'a str,
  pub headers: Vec<Header<'a>>,
  pub body: &'a [u8],
}

#[derive(Debug,Clone,PartialEq)]
pub struct Header<'a> {
  pub name: &'a str,
  pub value: &'a str,
}

pub fn cf_request<'a, 'b, 'c>(i:(&'a mut [u8],usize), r: &'c Request<'b>) -> Result<(&'a mut [u8],usize),GenError> {
  do_gen!((i.0, i.1),
    gen_call!(cf_request_line, r.method, r.uri) >>
    gen_many!(&r.headers, cf_header) >>
    gen_slice!(b"\r\n") >>
    gen_slice!(r.body)
  )
}

pub fn cf_request_line<'a, 'b>(i:(&'a mut [u8],usize), method: &'b str, uri: &'b str) -> Result<(&'a mut [u8],usize),GenError> {
  do_gen!((i.0, i.1),
    gen_slice!(method.as_bytes()) >>
    gen_slice!(b" ") >>
    gen_slice!(uri.as_bytes()) >>
    gen_slice!(b" HTTP/1.1\r\n")
  )
}

pub fn cf_header<'a, 'b, 'c>(i:(&'a mut [u8],usize), h: &'c Header<'b>) -> Result<(&'a mut [u8],usize),GenError> {
  do_gen!((i.0, i.1),
    gen_slice!(h.name.as_bytes()) >>
    gen_slice!(b": ") >>
    gen_slice!(h.value.as_bytes()) >>
    gen_slice!(b"\r\n")
  )
}

#[derive(Debug,Clone,PartialEq)]
pub struct RequestHeaders<'a> {
  pub method: &'a str,
  pub uri: &'a str,
  pub headers: Vec<Header<'a>>,
}

pub fn fn_request<'a:'c, 'b: 'a, 'c>(r: &'b Request<'a>) -> impl SerializeFn<&'c mut[u8]> + 'a {
  move |out: &'c mut [u8]| {
    let out = fn_request_line(&r.method, &r.uri)(out)?;
    let out = all(r.headers.iter().map(fn_header))(out)?;
    let out = string("\r\n")(out)?;
    slice(r.body)(out)
  }
}

pub fn fn_request_line<'a:'c, 'c, S: AsRef<str>>(method: &'a S, uri: &'a S) -> impl SerializeFn<&'c mut[u8]> + 'a {
  move |out: &'c mut [u8]| {
    let out = string(method)(out)?;
    let out = string(" ")(out)?;
    let out = string(uri)(out)?;
    string(" HTTP/1.1\r\n")(out)
  }
}

pub fn fn_header<'a:'c, 'c>(h: &'a Header) -> impl SerializeFn<&'c mut[u8]> + 'a {
  move |out: &'c mut [u8]| {
    let out = string(h.name)(out)?;
    let out = string(": ")(out)?;
    let out = string(h.value)(out)?;
    string("\r\n")(out)
  }
}

pub fn fn_request_headers<'a:'c, 'c, 'b: 'a>(r: &'b RequestHeaders<'a>) -> impl SerializeFn<&'c mut[u8]> + 'a {
  move |out: &'c mut [u8]| {
    let out = fn_request_line(&r.method, &r.uri)(out)?;
    let out = all(r.headers.iter().map(fn_header))(out)?;
    string("\r\n")(out)
  }
}

pub fn fn_chunk<'a:'c,'c>(sl: &'a[u8]) -> impl SerializeFn<&'c mut[u8]> + 'a {
  move |out: &'c mut [u8]| {
    let out = hex(sl.len())(out)?;
    let out = string("\r\n")(out)?;
    let out = slice(sl)(out)?;
    string("\r\n")(out)
  }
}

/*
pub fn chunked_request<'a, 'b: 'a, 'c, T: Serializer + 'a>(r: &'b RequestHeaders<'a>) -> Then<impl Serializer + 'a, Stream<T>> {
  let s: Stream<T> = Stream::new();
  rw_request_headers(r).then(s)
}*/

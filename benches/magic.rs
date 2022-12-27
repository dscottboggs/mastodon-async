use std::{
    fs::{read, write},
    io::{stdout, ErrorKind, Write},
};

use criterion::{criterion_group, criterion_main, Criterion};
use magic::CookieFlags;

async fn load_image() -> Vec<u8> {
    match read("/tmp/test.png") {
        Ok(img) => img,
        Err(err) if err.kind() == ErrorKind::NotFound => {
            let image = reqwest::Client::new()
                .get("https://httpbin.org/image/png")
                .header("Accept", "image/png")
                .send()
                .await
                .expect("connection")
                .error_for_status()
                .expect("OK response")
                .bytes()
                .await
                .expect("read response");
            write("/tmp/test.png", &image).expect("cache file");
            image.into()
        },
        Err(err) => panic!("error reading cached PNG file: {err:?}"),
    }
}

fn load_once(c: &mut Criterion) {
    let cookie = magic::Cookie::open(CookieFlags::MIME_TYPE).expect("Cookie::open");
    cookie.load::<&str>(&[]).expect("cookie.load");
    eprintln!("file: {}", file!());
    let buf = read(file!()).expect("read");

    c.bench_function("rust file, load once", |b| {
        b.iter(|| {
            // mis-detected as text/x-asm when chaining .bytes() at the beginning of a line
            assert_eq!("text/", &cookie.buffer(&buf).expect("detection")[0..5]);
        });
    });

    let image = tokio_test::block_on(async { load_image().await });

    c.bench_function("PNG file, load once", |b| {
        b.iter(|| {
            assert_eq!("image/png", cookie.buffer(&image).expect("detection"));
        });
    });
}

fn load_each_time(c: &mut Criterion) {
    let buf = read(file!()).expect("read");

    c.bench_function("rust file, load each time", |b| {
        b.iter(|| {
            let cookie = magic::Cookie::open(CookieFlags::MIME_TYPE).expect("Cookie::open");
            cookie.load::<&str>(&[]).expect("cookie.load");
            assert_eq!("text/", &cookie.buffer(&buf).expect("detection")[0..5]);
        });
    });

    let image = tokio_test::block_on(async { load_image().await });

    c.bench_function("PNG file, load each time", |b| {
        b.iter(|| {
            let cookie = magic::Cookie::open(CookieFlags::MIME_TYPE).expect("Cookie::open");
            cookie.load::<&str>(&[]).expect("cookie.load");
            assert_eq!("image/png", cookie.buffer(&image).expect("detection"));
        });
    });
}

fn load_from_buffer_each_time(c: &mut Criterion) {
    let cookie = magic::Cookie::open(CookieFlags::MIME_TYPE).expect("Cookie::open");
    let db = read("/usr/share/file/misc/magic.mgc").expect("read database");
    let buf = read(file!()).expect("read");

    c.bench_function("rust file, load from buffer each time", |b| {
        b.iter(|| {
            cookie.load_buffers(&[&db]).expect("cookie.load_buffers");
            assert_eq!("text/", &cookie.buffer(&buf).expect("detection")[0..5]);
        });
    });

    let image = tokio_test::block_on(async { load_image().await });

    c.bench_function("PNG file, load from buffer each time", |b| {
        b.iter(|| {
            cookie.load_buffers(&[&db]).expect("cookie.load_buffers");
            assert_eq!("image/png", cookie.buffer(&image).expect("detection"));
        });
    });
}

criterion_group!(
    magic_bench,
    load_once,
    load_each_time,
    load_from_buffer_each_time
);
criterion_main!(magic_bench);

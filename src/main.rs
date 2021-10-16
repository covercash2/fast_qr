#[cfg(test)]
mod tests;

mod bitstring;
mod datamasking;
mod default;
mod encode;
mod hardcode;
mod helpers;
mod placement;
mod polynomials;
mod qrcode;
mod score;
mod vecl;
mod version;

/// Still useless, only test purposes for now.
fn main() {
    const content: &str = "https://vahan.dev/";
    const qrcode: Option<qrcode::QRCode> =
        qrcode::QRCode::new(content.as_bytes(), vecl::ECL::H, None);

    if let Some(q) = qrcode {
        q.print();
        match q {
            qrcode::QRCode::V3(m) => println!("{:?}", m),
            _ => (),
        }
    }
}

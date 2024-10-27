use std::time::SystemTime;

// use openssl::{
//     hash::MessageDigest,
//     pkey::{PKey, Private},
// };

use crate::cryptography::{digest::sha256_hash, key::PrivateKey};

pub async fn post_to_inbox<T: PrivateKey>(
    // activity: &ActivityStream,
    activity: &str,
    from_id: &str,
    to_domain: &str,
    to_inbox: &str,
    keypair: &mut T,
) {
    // let keypair: PKey<Private> = PKey::from_rsa(private_key).unwrap();

    // let document = serde_json::to_string(activity).unwrap();

    let date = httpdate::fmt_http_date(SystemTime::now());

    let digest_base64 = &sha256_hash(activity.as_bytes());

    //string to be signed
    let signed_string = format!("(request-target): post /inbox\nhost: {to_domain}\ndate: {date}\ndigest: SHA-256={digest_base64}");
    let signature = keypair.sign(&signed_string);
    // let mut signer = openssl::sign::Signer::new(MessageDigest::sha256(), keypair).unwrap();
    // signer.update(signed_string.as_bytes()).unwrap();
    // let signature = openssl::base64::encode_block(&signer.sign_to_vec().unwrap());

    // dbg!(&from_id);

    // let header: String = r#"keyId=""#.to_string()
    //     + from_id
    //     + r#"#main-key",headers="(request-target) host date digest",signature=""#
    //     + &signature
    //     + r#"""#;
    let header = format!(
        r#"keyId="{}#main-key",headers="(request-target) host date digest",signature="{}""#,
        from_id, &signature,
    );

    println!("{}", &header);

    let client = reqwest::Client::new();
    let client = client
        .post(to_inbox)
        .header("Host", to_domain)
        .header("Date", date)
        .header("Signature", header)
        .header("Digest", "SHA-256=".to_owned() + digest_base64)
        .body(activity.to_string());

    dbg!(&client);

    let res = client.send().await;
    // dbg!(&res);

    let response = res.unwrap().text().await;

    dbg!(&response);

    if let Ok(x) = response {
        println!("{}", x);
    }
}

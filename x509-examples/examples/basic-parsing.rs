use openssl::x509::X509;

const WWW_GOOGLE_COM_CRT: &'static str = "-----BEGIN CERTIFICATE-----
MIIEvjCCA6agAwIBAgIQLi/6Fx6+6KICAAAAAECl1jANBgkqhkiG9w0BAQsFADBC
MQswCQYDVQQGEwJVUzEeMBwGA1UEChMVR29vZ2xlIFRydXN0IFNlcnZpY2VzMRMw
EQYDVQQDEwpHVFMgQ0EgMU8xMB4XDTE5MDgxMzE2MjAyNVoXDTE5MTExMTE2MjAy
NVowaDELMAkGA1UEBhMCVVMxEzARBgNVBAgTCkNhbGlmb3JuaWExFjAUBgNVBAcT
DU1vdW50YWluIFZpZXcxEzARBgNVBAoTCkdvb2dsZSBMTEMxFzAVBgNVBAMTDnd3
dy5nb29nbGUuY29tMFkwEwYHKoZIzj0CAQYIKoZIzj0DAQcDQgAErDSfErsJHyjZ
wkccquFwr0sCUwPX7TrMBRY5P4u1mXVm7QRabPXQebHrdom7ZM+hwHu7eHXjrKNO
HQ/zwmp1uqOCAlMwggJPMA4GA1UdDwEB/wQEAwIHgDATBgNVHSUEDDAKBggrBgEF
BQcDATAMBgNVHRMBAf8EAjAAMB0GA1UdDgQWBBTkwLyGwlGyeq7XqFjXFJIU5GVe
STAfBgNVHSMEGDAWgBSY0fhuEOvPm+xgnxiQG6DrfQn9KzBkBggrBgEFBQcBAQRY
MFYwJwYIKwYBBQUHMAGGG2h0dHA6Ly9vY3NwLnBraS5nb29nL2d0czFvMTArBggr
BgEFBQcwAoYfaHR0cDovL3BraS5nb29nL2dzcjIvR1RTMU8xLmNydDAZBgNVHREE
EjAQgg53d3cuZ29vZ2xlLmNvbTAhBgNVHSAEGjAYMAgGBmeBDAECAjAMBgorBgEE
AdZ5AgUDMC8GA1UdHwQoMCYwJKAioCCGHmh0dHA6Ly9jcmwucGtpLmdvb2cvR1RT
MU8xLmNybDCCAQMGCisGAQQB1nkCBAIEgfQEgfEA7wB2AGPy283oO8wszwtyhCdX
azOkjWF3j711pjixx2hUS9iNAAABbIv+x7oAAAQDAEcwRQIgdVGP/bUKdYykhLh9
9zUUYxteaPGfppx9r/yknsVTjI8CIQDTHVM8i98rp8gk9mX5kKEON/TDtc9i+XfO
9qckKSRvwQB1AHR+2oMxrTMQkSGcziVPQnDCv/1eQiAIxjc1eeYQe8xWAAABbIv+
x8MAAAQDAEYwRAIgG9hbzgW+Yp8spYgkBAWNNZ381bYjN6TfpcxhM0GsOhgCIDMn
NTVxPC0Umgh9gRtFmM5pvjX71Sd/59iNR4jFtLjOMA0GCSqGSIb3DQEBCwUAA4IB
AQCTzUJExrNYCuRGRTXXqaHefaRdc1n30HZBUgUFTH3WqOBdbd0L1r1Vc2uYq7xO
fA95tKjflgXW74fwSZ6UZeQa0X0jBbtJs2aykCeh9PqQtGqlmVfOVUVhxtD0qSUG
Gy9HoX2V/mPC+rOMZfaCzvSqd5yUz7TxwlMFH8GlAxd0s2Gqyq2OdWFMfpfZpvyD
AEJG3VFzPWAH5jCPLD8hX/J/xPCsA/sdt6vBlFPvSXqoAgYaUkrkQ7SZv/pkQ0Cn
3GZL+Ofa0btisJ4XLOP5YWW1f+oERZ3JqwIgXdpzFfCvVIpunGwCbb3mxd7/2DZ6
4PnmK3SOQ3dsM8QLcoBa3ASe
-----END CERTIFICATE-----";

fn main() {
    println!("Using OpenSSL Version: {}", openssl::version::version());

    let x509: X509 = X509::from_pem(WWW_GOOGLE_COM_CRT.as_bytes())
        .expect("could not parse x509 certificate from PEM encoded data");

    println!("Subject:");
    for name_entry in x509.subject_name().entries() {

        let subject_name = name_entry.data().as_utf8()
            .expect("it was not possible to convert subject name to utf8 string");
        
        let nid = name_entry.object().nid();
        let nid_short_name = nid.short_name()
            .expect("it was not possible to get NID shortname of the name entry");
        let _nid_long_name = nid.long_name()
            .expect("it was not possible to get NID longname of the name entry");

        println!("  {:02}: {}", nid_short_name, subject_name);
    }

    println!("Subject Alternative Names (SAN):");
    let san = x509.subject_alt_names();
    if san.is_none() {
        println!("  No SAN was defined");
    } else {
        let san = san.unwrap();
        for general_name in &san {
            let email: Option<&str> = general_name.email();
            let dnsname: Option<&str> = general_name.dnsname();
            let uri: Option<&str> = general_name.uri();
            let ipaddress: Option<&[u8]> = general_name.ipaddress();

            println!("  Email: {:?}", email);
            println!("  DNSName: {:?}", dnsname);
            println!("  URI: {:?}", uri);
            println!("  ipaddress: {:?}", ipaddress);
        }
    }

    println!("Issuer:");
    for name_entry in x509.issuer_name().entries() {
        let subject_name = name_entry.data().as_utf8()
            .expect("it was not possible to convert subject name to utf8 string");
        
        let nid = name_entry.object().nid();
        let nid_short_name = nid.short_name()
            .expect("it was not possible to get NID shortname of the name entry");
        let _nid_long_name = nid.long_name()
            .expect("it was not possible to get NID longname of the name entry");

        println!("  {:02}: {}", nid_short_name, subject_name);
    }
    println!("Issuer Alternative Names (IAN):");
    let ian = x509.issuer_alt_names();
    if ian.is_none() {
        println!("  No IAN was defined");
    } else {
        let ian = ian.unwrap();
        for general_name in &ian {
            let email: Option<&str> = general_name.email();
            let dnsname: Option<&str> = general_name.dnsname();
            let uri: Option<&str> = general_name.uri();
            let ipaddress: Option<&[u8]> = general_name.ipaddress();

            println!("  Email: {:?}", email);
            println!("  DNSName: {:?}", dnsname);
            println!("  URI: {:?}", uri);
            println!("  ipaddress: {:?}", ipaddress);
        }
    }

    println!("Expiration:");
    println!("  Not Before: {}", x509.not_before());
    println!("  Not After: {}", x509.not_after());

    println!("Signature:");
    println!("  Signature: {:?}", x509.signature().as_slice());
    let algorithm_nid = x509.signature_algorithm().object().nid();
    let algorithm_short_name = algorithm_nid.short_name()
        .expect("could not get short name for signature algorithm");
    let algorithm_long_name = algorithm_nid.long_name()
        .expect("could not get long name for signature algorithm");
    println!("  Algorithm: {} ({})", algorithm_short_name, algorithm_long_name);

    print!("Serial Number: ");
    let serial_number = x509.serial_number().to_bn()
        .expect("could not get big number from serial number");
    println!("{}", serial_number);

    println!("Public Key:");
    let _public_key = x509.public_key()
        .expect("could not get public key");
    println!("  TODO");
}

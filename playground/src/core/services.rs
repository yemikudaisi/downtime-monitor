use lettre::transport::smtp::authentication::Credentials;
use lettre::{transport::smtp::SmtpTransportBuilder, SmtpTransport};
// TODO: use tauri_plugin_http::reqwest;
use super::types::{ServiceParameters, ServiceVerificationResult};
use reqwest::header::HeaderValue;
use reqwest::Client;
use reqwest::StatusCode;

///
/// Verifies if an SMTP service is up (Tecta)
/// ## Panics
///
/// Panics if .
///
/// ## Errors
///
/// This function will return an error.
#[allow(unused)]
pub fn verify_smtp(smtp_config: &ServiceParameters) -> ServiceVerificationResult {
    let mut builder: SmtpTransportBuilder;
    match smtp_config.secure != None && smtp_config.secure.unwrap() {
        true => {
            let result: Result<SmtpTransportBuilder, lettre::transport::smtp::Error> =
                SmtpTransport::starttls_relay(&smtp_config.host);
            match result {
                Ok(smtp_transport_builder) => builder = smtp_transport_builder,
                Err(e) => {
                    println!("Error: {:?}", e);
                    return ServiceVerificationResult {
                        success: false,
                        message: format!("{}", e),
                    };
                }
            }
        }
        false => {
            builder = SmtpTransport::builder_dangerous(&smtp_config.host);
            println!("[+] Testing SMTP without TLS.");
        }
    };
    builder = builder.port(smtp_config.port as u16);

    let transport: SmtpTransport;

    if !(smtp_config.user.as_ref() == None) {
        let creds = Credentials::new(
            smtp_config.user.as_ref().unwrap().to_string(),
            smtp_config.pass.as_ref().unwrap().clone(),
        );
        transport = builder.credentials(creds).build();
    } else {
        transport = builder.build();
    }

    match transport.test_connection() {
        Ok(_) => ServiceVerificationResult {
            success: true,
            message: "Connection successful".to_string(),
        },
        Err(e) => ServiceVerificationResult {
            success: false,
            message: format!("Test failed: {}", e),
        },
    }
}

/// .
/// Verify that a wesbite is online by making an HTTP request to the host using the specified port
/// At staus code of 200 assumed ut website is up.
///
/// Road Map: accept dynamic list of status codes
///
/// ## Example
///
/// ```
/// let website_config = types::ServiceConfig {
///   host: String::from("https://www.website.org"),
///   port: 80,
///   ..Default::default()
/// };
/// ```
/// let result = services::verify_website(website_config).await;
/// match result.success {
///   true => {
///     println!("{}", result.message)
///   }
///   false => {
///     println!("{}", result.message)
///   }
/// }
///
/// ## Panics
///
/// The function will panic if the host is malformed or does not exist.
///
/// ## Errors
///
/// This function will return an error if .
#[allow(unused)]
pub async fn verify_website(website_config: &ServiceParameters) -> ServiceVerificationResult {
    let client = Client::new();
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert("User-Agent", HeaderValue::from_static("hello"));
    let full_url;
    if website_config.port != 80 {
        full_url = format!("{}:{}", website_config.host, website_config.port);
    } else {
        full_url = website_config.host.clone();
    }
    // Send a GET request to the specified URL
    match client.get(&full_url).headers(headers).send().await {
        Ok(response) => {
            if response.status() == StatusCode::OK {
                ServiceVerificationResult {
                    success: true,
                    message: format!("Website {} is online", full_url),
                }
            } else {
                ServiceVerificationResult {
                    success: false,
                    message: format!(
                        "Website {} returned status code: {}",
                        full_url,
                        response.status()
                    ),
                }
            }
        }
        Err(e) => ServiceVerificationResult {
            success: false,
            message: format!("Error ({}): {}", full_url, e),
        },
    }
}

mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test_verify_smtp_return_true() {
        let smtp_config = ServiceParameters {
            host: String::from("smtp.freesmtpservers.com"),
            port: 25,
            ..Default::default()
        };
        let result: ServiceVerificationResult = verify_smtp(&smtp_config);
        assert_eq!(result.success, true);
    }

    #[tokio::test]
    async fn test_verify_website_return_false() {
        let website_config = ServiceParameters {
            host: String::from("wronghost"),
            port: 80,
            ..Default::default()
        };
        let result: ServiceVerificationResult = verify_website(&website_config).await;
        assert_eq!(result.success, false);
    }

    #[tokio::test]
    async fn test_verify_website_return_true() {
        let website_config = ServiceParameters {
            host: String::from("https://www.google.com"),
            port: 80,
            ..Default::default()
        };
        let result: ServiceVerificationResult = verify_website(&website_config).await;
        println!("Website > Failed: {}", result.message);
        assert_eq!(result.success, true);
    }
}

#[cfg(test)]
mod tests {
    use crate::{Domain, Method, Request, Response, SipMessage, Version};
    use aglaea::{ToGrammar, ToTree};
    use std::convert::TryFrom;

    #[test]
    fn test_request_aglaea() {
        // Create a simple SIP request
        let request = Request {
            method: Method::Register,
            uri: crate::Uri {
                scheme: Some(crate::Scheme::Sip),
                host_with_port: Domain::from("example.com").into(),
                ..Default::default()
            },
            version: Version::V2,
            headers: Default::default(),
            body: vec![],
        };

        // Test ToTree
        let tree = request.to_tree();

        // Test ToGrammar
        let grammar = Request::grammar();

        // Test that grammar matches the tree
        assert_eq!(
            grammar.matches(&tree),
            true,
            "Grammar should match the generated tree"
        );

        // Test round-trip
        let bytes = tree.to_bytes();
        let request_str = std::str::from_utf8(&bytes).unwrap();

        // Generated request must match parsed original
        let parsed_request = Request::try_from(request_str).unwrap();
        assert_eq!(parsed_request, request, "Request should match");
    }

    #[test]
    fn test_response_aglaea() {
        // Create a simple SIP response
        let response = Response {
            status_code: 200.into(),
            version: Version::V2,
            headers: Default::default(),
            body: vec![],
        };

        // Test ToTree
        let tree = response.to_tree();

        // Test ToGrammar
        let grammar = Response::grammar();

        // Test that grammar matches the tree
        assert_eq!(
            grammar.matches(&tree),
            true,
            "Grammar should match the generated tree"
        );

        // Test bytes generation
        let bytes = tree.to_bytes();
        let response_str = std::str::from_utf8(&bytes).unwrap();
        let parsed_response = Response::try_from(response_str).unwrap();
        assert_eq!(parsed_response, response, "Response should match");
    }

    #[test]
    fn test_sip_message_aglaea() {
        // Create a SIP request message
        let request = Request {
            method: Method::Invite,
            uri: crate::Uri {
                scheme: Some(crate::Scheme::Sip),
                host_with_port: Domain::from("example.com").into(),
                ..Default::default()
            },
            version: Version::V2,
            headers: Default::default(),
            body: vec![],
        };
        let sip_message = SipMessage::Request(request);

        // Test ToTree
        let tree = sip_message.to_tree();

        // Test ToGrammar
        let grammar = SipMessage::grammar();

        // Test that grammar matches the tree
        assert_eq!(
            grammar.matches(&tree),
            true,
            "Grammar should match the generated tree"
        );

        // Test bytes generation
        let bytes = tree.to_bytes();
        let sip_message_str = std::str::from_utf8(&bytes).unwrap();
        let parsed_sip_message = SipMessage::try_from(sip_message_str).unwrap();
        assert_eq!(parsed_sip_message, sip_message, "SIP message should match");
    }

    #[test]
    fn test_real_register_request_roundtrip() {
        // Real SIP REGISTER request from RFC examples
        let sip_msg = concat!(
            "REGISTER sips:ss2.biloxi.example.com SIP/2.0\r\n",
            "Via: SIP/2.0/TLS client.biloxi.example.com:5061;branch=z9hG4bKnashd92\r\n",
            "Max-Forwards: 70\r\n",
            "From: Bob <sips:bob@biloxi.example.com>;tag=ja743ks76zlflH\r\n",
            "To: Bob <sips:bob@biloxi.example.com>\r\n",
            "Call-ID: 1j9FpLxk3uxtm8tn@biloxi.example.com\r\n",
            "CSeq: 2 REGISTER\r\n",
            "Contact: <sips:bob@client.biloxi.example.com>\r\n",
            "Authorization: Digest username=\"bob\", realm=\"atlanta.example.com\" nonce=\"ea9c8e88df84f1cec4341ae6cbe5a359\", opaque=\"\" uri=\"sips:ss2.biloxi.example.com\", response=\"dfe56131d1958046689d83306477ecc\"\r\n",
            "Content-Length: 0\r\n\r\n"
        );


        // Step 1: Parse bytes to struct
        let request = Request::try_from(sip_msg).expect("Failed to parse original request");

        // Step 2: Convert struct to aglaea tree
        let tree = request.to_tree();
        println!("Generated tree for request");

        // Verify grammar matches
        let grammar = Request::grammar();
        assert_eq!(
            grammar.matches(&tree),
            true,
            "Grammar should match the generated tree"
        );

        // Step 3: Convert aglaea tree to bytes
        let generated_bytes = tree.to_bytes();
        let generated_str = std::str::from_utf8(&generated_bytes).unwrap();

        // Generated request must match parsed original
        let parsed_request = Request::try_from(generated_str).unwrap();
        assert_eq!(parsed_request, request, "Request should match");
    }

    #[test]
    fn test_real_unauthorized_response_roundtrip() {
        // Real SIP 401 Unauthorized response from RFC examples
        let sip_msg = concat!(
            "SIP/2.0 401 Unauthorized\r\n",
            "Via: SIP/2.0/TLS client.biloxi.example.com:5061;branch=z9hG4bKnashds7;received=192.0.2.201\r\n",
            "From: Bob <sips:bob@biloxi.example.com>;tag=a73kszlfl\r\n",
            "To: Bob <sips:bob@biloxi.example.com>;tag=1410948204\r\n",
            "Call-ID: 1j9FpLxk3uxtm8tn@biloxi.example.com\r\n",
            "CSeq: 1 REGISTER\r\n",
            "WWW-Authenticate: Digest realm=\"atlanta.example.com\", qop=\"auth\", nonce=\"ea9c8e88df84f1cec4341ae6cbe5a359\", opaque=\"\", stale=FALSE, algorithm=MD5\r\n",
            "Content-Length: 0\r\n\r\n"
        );

        println!("Original SIP response:\n{}", sip_msg);

        // Step 1: Parse bytes to struct
        let response = Response::try_from(sip_msg).expect("Failed to parse original response");
        println!(
            "Parsed response: status={}, version={}",
            response.status_code, response.version
        );

        // Step 2: Convert struct to aglaea tree
        let tree = response.to_tree();
        println!("Generated tree for response");

        // Verify grammar matches
        let grammar = Response::grammar();
        assert_eq!(
            grammar.matches(&tree),
            true,
            "Grammar should match the generated tree"
        );

        // Step 3: Convert aglaea tree to bytes
        let generated_bytes = tree.to_bytes();
        let generated_str = std::str::from_utf8(&generated_bytes).unwrap();
        println!("Generated SIP response from tree:\n{}", generated_str);

        let parsed_response = Response::try_from(generated_str).unwrap();
        assert_eq!(parsed_response, response, "Response should match");
    }

    #[test]
    fn test_real_sip_message_roundtrip() {
        // Test with a real INVITE request
        let sip_msg = concat!(
            "INVITE sip:bob@biloxi.example.com SIP/2.0\r\n",
            "Via: SIP/2.0/UDP pc33.atlanta.example.com;branch=z9hG4bK776asdhds\r\n",
            "Max-Forwards: 70\r\n",
            "To: Bob <sip:bob@biloxi.example.com>\r\n",
            "From: Alice <sip:alice@atlanta.example.com>;tag=1928301774\r\n",
            "Call-ID: a84b4c76e66710@pc33.atlanta.example.com\r\n",
            "CSeq: 314159 INVITE\r\n",
            "Contact: <sip:alice@pc33.atlanta.example.com>\r\n",
            "Content-Type: application/sdp\r\n",
            "Content-Length: 0\r\n\r\n"
        );

        println!("Original SIP INVITE:\n{}", sip_msg);

        // Step 1: Parse bytes to SipMessage
        let sip_message = SipMessage::try_from(sip_msg).expect("Failed to parse original message");

        // Step 2: Convert struct to aglaea tree
        let tree = sip_message.to_tree();

        // Verify grammar matches
        let grammar = SipMessage::grammar();
        assert_eq!(
            grammar.matches(&tree),
            true,
            "Grammar should match the generated tree"
        );

        // Step 3: Convert aglaea tree to bytes
        let generated_bytes = tree.to_bytes();
        let generated_str = std::str::from_utf8(&generated_bytes).unwrap();

        let new_message = SipMessage::try_from(generated_str).unwrap();
        assert_eq!(new_message, sip_message, "SipMessage should match");
    }
}

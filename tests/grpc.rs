use std::fs;
use tonic::Request;

// Include the generated gRPC code
tonic::include_proto!("commands");

#[tokio::test]
async fn test_grpc_set_and_get() {
    let mut client = basic_client::BasicClient::connect("http://[::1]:50051")
        .await
        .expect("Failed to connect to gRPC server");

    let set_request = SetRequest {
        key: "key1".to_string(),
        value: "value1".to_string(),
    };
    let set_response = client
        .set(Request::new(set_request))
        .await
        .expect("Failed to set key-value pair");
    assert!(set_response.into_inner().success);

    let get_request = GetRequest {
        key: "key1".to_string(),
    };
    let get_response = client
        .get(Request::new(get_request))
        .await
        .expect("Failed to get value for key");
    assert_eq!(get_response.into_inner().value, "value1");
}

#[tokio::test]
async fn test_grpc_get_non_existent_key() {
    let mut client = basic_client::BasicClient::connect("http://[::1]:50051")
        .await
        .expect("Failed to connect to gRPC server");

    let get_request = GetRequest {
        key: "non_existent_key".to_string(),
    };
    let get_response = client.get(Request::new(get_request)).await;

    assert!(get_response.is_err());
}

#[tokio::test]
async fn test_grpc_set_empty_value() {
    let mut client = basic_client::BasicClient::connect("http://[::1]:50051")
        .await
        .expect("Failed to connect to gRPC server");

    let set_request = SetRequest {
        key: "key_empty".to_string(),
        value: "".to_string(),
    };
    let set_response = client
        .set(Request::new(set_request))
        .await
        .expect("Failed to set key with empty value");
    assert!(set_response.into_inner().success);

    let get_request = GetRequest {
        key: "key_empty".to_string(),
    };
    let get_response = client
        .get(Request::new(get_request))
        .await
        .expect("Failed to get value for key with empty value");
    assert_eq!(get_response.into_inner().value, "");
}

// #[tokio::test]
// async fn test_grpc_concurrent_requests() {
//     let client = basic_client::BasicClient::connect("http://[::1]:50051")
//         .await
//         .expect("Failed to connect to gRPC server");
//     let client = std::sync::Arc::new(client);

//     let mut handles = vec![];

//     for i in 0..10 {
//         let client = client.clone();
//         let handle = tokio::task::spawn(async move {
//             let key = format!("key{}", i);
//             let value = format!("value{}", i);

//             let set_request = SetRequest {
//                 key: key.clone(),
//                 value: value.clone(),
//             };
//             client
//                 .set(Request::new(set_request))
//                 .await
//                 .expect("Failed to set key-value pair");

//             let get_request = GetRequest { key };
//             let get_response = client
//                 .get(Request::new(get_request))
//                 .await
//                 .expect("Failed to get value for key");
//             assert_eq!(get_response.into_inner().value, value);
//         });
//         handles.push(handle);
//     }

//     for handle in handles {
//         handle.await.unwrap();
//     }
// }

#[tokio::test]
async fn test_cleanup() {
    let test_db_path = "./test_db";
    if std::path::Path::new(test_db_path).exists() {
        fs::remove_dir_all(test_db_path).expect("Failed to clean up test database");
    }
}

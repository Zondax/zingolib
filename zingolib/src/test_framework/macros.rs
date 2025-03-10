/// We use this macro to remove repetitive test set-up and teardown in the zingolib unit tests.
#[macro_export]
macro_rules! apply_scenario {
    ($inner_test_name:ident $numblocks:literal) => {
        concat_idents::concat_idents!(
            test_name = scenario_, $inner_test_name {
                #[tokio::test]
                async fn test_name() {
                    let (scenario, stop_transmitter, test_server_handle) = $crate::lightclient::test_server::setup_n_block_fcbl_scenario($numblocks).await;
                    $inner_test_name(scenario).await;
                    clean_shutdown(stop_transmitter, test_server_handle).await;
                }
            }
        );
    };
}

// Note that do_addresses returns an array, each element is a JSON representation
// of a UA.  Legacy addresses can be extracted from the receivers, per:
// <https://zips.z.cash/zip-0316>
#[macro_export]
macro_rules! get_base_address {
    ($client:expr, $address_protocol:expr) => {
        match $address_protocol {
            "unified" => $client.do_addresses().await[0]["address"]
                .take()
                .to_string(),
            "sapling" => $client.do_addresses().await[0]["receivers"]["sapling"]
                .clone()
                .to_string(),
            "transparent" => $client.do_addresses().await[0]["receivers"]["transparent"]
                .clone()
                .to_string(),
            _ => "ERROR".to_string(),
        }
    };
}
#[macro_export]
macro_rules! check_client_balances {
    ($client:ident, o: $orchard:tt s: $sapling:tt t: $transparent:tt) => {
        let balance = $client.do_balance().await;
        let tx_summary_balance = $client
            .do_list_txsummaries()
            .await
            .iter()
            .map(|transfer| transfer.balance_delta())
            .sum::<i64>();
        assert_eq!(
            (balance.orchard_balance.unwrap_or(0)
                + balance.sapling_balance.unwrap_or(0)
                + balance.transparent_balance.unwrap_or(0)) as i64,
            tx_summary_balance,
            "tx_summaries follow: {}\ndo_list_transactions follow: {}",
            ::json::JsonValue::from($client.do_list_txsummaries().await).pretty(4),
            ::json::JsonValue::from($client.do_list_txsummaries().await).pretty(4)
        );
        assert_eq!(
            balance.orchard_balance.unwrap(),
            $orchard,
            "\no_balance: {} expectation: {} ",
            balance.orchard_balance.unwrap(),
            $orchard
        );
        assert_eq!(
            balance.sapling_balance.unwrap(),
            $sapling,
            "\ns_balance: {} expectation: {} ",
            balance.sapling_balance.unwrap(),
            $sapling
        );
        assert_eq!(
            balance.transparent_balance.unwrap(),
            $transparent,
            "\nt_balance: {} expectation: {} ",
            balance.transparent_balance.unwrap(),
            $transparent
        );
    };
}
